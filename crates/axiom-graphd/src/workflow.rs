//! Read-only selected export from one verified catalog vector.
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::PathBuf;

use graph_core::config::ServiceConfig;
use graph_core::error::{AxiomError, ErrorCode};
use graph_core::locks::{default_lock_path, LockMode, SolutionGuard};
use graph_core::paths::{AxiomHome, PathEnvironment};
use graph_export::{pointer, reader, sha256_hex};
use serde_json::{json, Value};

use crate::{commands::render, runtime, serve};

/// Strict argv selection; no natural-language parsing or source mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportCommand {
    pub solution: String,
    pub projects: Vec<String>,
    pub nodes: Vec<String>,
    pub edges: Vec<String>,
    pub exact_projection: bool,
    pub catalog_generation: Option<String>,
    pub project_generations: BTreeMap<String, String>,
    pub out: PathBuf,
    pub format: String,
}

fn invalid(message: &str) -> AxiomError {
    AxiomError::new(ErrorCode::ValidationError, message)
}

fn vector_matches(raw: &[u8], expected: &BTreeMap<String, String>) -> bool {
    serde_json::from_slice::<Value>(raw)
        .ok()
        .and_then(|v| v["projects"].as_array().cloned())
        .is_some_and(|members| {
            expected.iter().all(|(project, generation)| {
                members.iter().any(|m| {
                    m["project_id"].as_str() == Some(project)
                        && m["generation_id"].as_str() == Some(generation)
                })
            })
        })
}

fn selected_catalog(lane: &std::path::Path, command: &ExportCommand) -> Result<String, AxiomError> {
    let current = pointer::read(lane)
        .map_err(runtime::export_error)?
        .ok_or_else(|| AxiomError::new(ErrorCode::NotReady, "catalog publication missing"))?
        .generation_id;
    let preferred = command.catalog_generation.as_ref().unwrap_or(&current);
    if command.project_generations.is_empty() {
        return Ok(preferred.clone());
    }
    let raw = std::fs::read(
        lane.join("generations")
            .join(preferred)
            .join("manifest.json"),
    )
    .map_err(|error| runtime::storage_error("selected catalog snapshot unavailable", &error))?;
    if sha256_hex(&raw) == *preferred && vector_matches(&raw, &command.project_generations) {
        return Ok(preferred.clone());
    }
    if command.catalog_generation.is_some() {
        return Err(invalid("explicit catalog and project vector disagree"));
    }
    let entries = std::fs::read_dir(lane.join("generations"))
        .map_err(|error| runtime::storage_error("retained catalog inventory", &error))?;
    for (index, entry) in entries.enumerate() {
        if index >= 2048 {
            return Err(AxiomError::new(
                ErrorCode::NotReady,
                "retained catalog search budget exceeded",
            ));
        }
        let entry =
            entry.map_err(|error| runtime::storage_error("retained catalog entry", &error))?;
        let id = entry.file_name().to_string_lossy().to_string();
        if !graph_export::manifest::is_sha256_hex(&id) {
            continue;
        }
        if let Ok(raw) = std::fs::read(entry.path().join("manifest.json")) {
            if sha256_hex(&raw) == id && vector_matches(&raw, &command.project_generations) {
                return Ok(id);
            }
        }
    }
    Err(AxiomError::new(
        ErrorCode::NotReady,
        "requested project-generation vector is no longer retained",
    ))
}

/// Load exact selected members under the native guard; never mix newer pointers.
pub fn export(command: &ExportCommand) -> Result<Value, AxiomError> {
    axiom_platform::atomic_file::reject_links(&command.out)
        .map_err(|_| invalid("export destination has a linked ancestor"))?;
    if command.out.exists() {
        return Err(invalid("export requires a new output directory"));
    }
    let home = AxiomHome::resolve(&PathEnvironment::for_current_process())?;
    home.verify_destination()?;
    let config = ServiceConfig::load(None)?;
    let database = config
        .storage()
        .database_path()
        .map(PathBuf::from)
        .unwrap_or(home.instance_db(config.daemon().instance_id())?);
    let connection =
        rusqlite::Connection::open_with_flags(database, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|error| runtime::storage_error("export registration", &error))?;
    let solution = runtime::solution(&connection, &command.solution)?;
    let projects = runtime::projects(&connection, &command.solution)?;
    let wanted: BTreeSet<_> =
        if command.projects.is_empty() && !command.project_generations.is_empty() {
            command.project_generations.keys().cloned().collect()
        } else {
            command.projects.iter().cloned().collect()
        };
    if (!command.projects.is_empty() && wanted.len() != command.projects.len())
        || wanted
            .iter()
            .any(|id| !projects.iter().any(|p| &p.id == id))
    {
        return Err(invalid(
            "export project selection is duplicate or outside the solution",
        ));
    }
    if !command.project_generations.is_empty()
        && wanted != command.project_generations.keys().cloned().collect()
    {
        return Err(invalid(
            "requested generation vector must cover exactly the selected projects",
        ));
    }
    let host = runtime::catalog_host_repo(&home, &solution, &projects)?;
    let host_project = projects
        .iter()
        .find(|p| p.repo_id == host)
        .ok_or_else(|| invalid("catalog host is not registered"))?;
    let host_root = serve::resolve_one_project(&home, host_project)?;
    let lane = PathBuf::from(host_root)
        .join(".axiom/graph")
        .join(&command.solution)
        .join("_catalog/live");
    let guard_root = home.instance_guard(config.daemon().instance_id())?;
    let admission = SolutionGuard::acquire(&guard_root.join("admission.lock"), LockMode::Exclusive)
        .map_err(runtime::guard_error)?;
    let data = SolutionGuard::acquire(&guard_root.join("data.lock"), LockMode::Shared)
        .map_err(runtime::guard_error)?;
    admission.release().map_err(runtime::guard_error)?;
    let catalog_id = selected_catalog(&lane, command)?;
    let raw = std::fs::read(
        lane.join("generations")
            .join(&catalog_id)
            .join("manifest.json"),
    )
    .map_err(|error| runtime::storage_error("export catalog snapshot unavailable", &error))?;
    if sha256_hex(&raw) != catalog_id {
        return Err(invalid("catalog checksum mismatch"));
    }
    let catalog: Value =
        serde_json::from_slice(&raw).map_err(|_| invalid("catalog JSON invalid"))?;
    let fields: BTreeSet<_> = catalog
        .as_object()
        .ok_or_else(|| invalid("catalog must be an object"))?
        .keys()
        .map(String::as_str)
        .collect();
    if fields
        != BTreeSet::from([
            "schema_version",
            "solution_id",
            "analysis_profile",
            "projects",
            "coverage",
        ])
        || catalog["analysis_profile"].as_str() != Some(&solution.profile)
        || !matches!(
            catalog["coverage"].as_str(),
            Some("complete_for_profile" | "partial" | "unsupported")
        )
        || graph_export::canonical::canonical_document_value(&catalog)
            .map_err(runtime::export_error)?
            != raw
    {
        return Err(invalid(
            "catalog schema/profile/canonical encoding mismatch",
        ));
    }
    if catalog["solution_id"].as_str() != Some(&command.solution) || catalog["schema_version"] != 1
    {
        return Err(invalid("catalog identity/schema mismatch"));
    }
    let members = catalog["projects"]
        .as_array()
        .ok_or_else(|| invalid("catalog has no member vector"))?;
    let mut member_ids = BTreeSet::new();
    for member in members {
        let keys: BTreeSet<_> = member
            .as_object()
            .ok_or_else(|| invalid("catalog member must be an object"))?
            .keys()
            .map(String::as_str)
            .collect();
        let id = member["project_id"]
            .as_str()
            .ok_or_else(|| invalid("catalog member project missing"))?;
        if keys != BTreeSet::from(["project_id", "generation_id", "source_fingerprint"])
            || !graph_export::manifest::is_identifier(id)
            || !member_ids.insert(id)
            || !member["generation_id"]
                .as_str()
                .is_some_and(graph_export::manifest::is_sha256_hex)
            || !member["source_fingerprint"]
                .as_str()
                .is_some_and(graph_export::manifest::is_sha256_hex)
        {
            return Err(invalid("catalog member schema/identity mismatch"));
        }
    }
    if member_ids != projects.iter().map(|p| p.id.as_str()).collect() {
        return Err(invalid(
            "catalog vector differs from registered project membership",
        ));
    }
    let mut graphs = Vec::new();
    let mut documents = Vec::new();
    let mut vector = serde_json::Map::new();
    let mut selected_ids = Vec::new();
    let selected_nodes: BTreeSet<_> = command.nodes.iter().cloned().collect();
    let selected_edges: BTreeSet<_> = command.edges.iter().cloned().collect();
    let mut observed_nodes = BTreeSet::new();
    let mut observed_edges = BTreeSet::new();
    for project in &projects {
        if !wanted.is_empty() && !wanted.contains(&project.id) {
            continue;
        }
        let matches: Vec<_> = members
            .iter()
            .filter(|m| m["project_id"].as_str() == Some(&project.id))
            .collect();
        if matches.len() != 1 {
            return Err(invalid(
                "selected project is missing or duplicate in catalog",
            ));
        }
        let member = matches[0];
        let generation = member["generation_id"]
            .as_str()
            .ok_or_else(|| invalid("catalog member identity missing"))?;
        let root = serve::resolve_one_project(&home, project)?;
        let project_lane = PathBuf::from(root)
            .join(".axiom/graph")
            .join(&command.solution)
            .join(&project.id)
            .join("live");
        let guard = SolutionGuard::acquire(&default_lock_path(&project_lane), LockMode::Shared)
            .map_err(runtime::guard_error)?;
        let snapshot = reader::load_pinned_verified(&project_lane, generation, &guard)
            .map_err(runtime::export_error)?;
        guard.release().map_err(runtime::guard_error)?;
        if snapshot.manifest().solution_id != command.solution
            || snapshot.manifest().project_id != project.id
            || member["source_fingerprint"].as_str()
                != Some(&snapshot.manifest().source_fingerprint)
        {
            return Err(invalid(
                "project identity/source fingerprint disagrees with catalog",
            ));
        }
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for entry in &snapshot.manifest().files {
            if !matches!(entry.role.as_str(), "nodes" | "edges") {
                continue;
            }
            let records = snapshot
                .parse_shard(&entry.path)
                .map_err(runtime::export_error)?;
            if entry.role == "nodes" {
                nodes.extend(records);
            } else {
                edges.extend(records);
            }
        }
        if !selected_nodes.is_empty() {
            nodes.retain(|node| {
                node["id"]
                    .as_str()
                    .is_some_and(|id| selected_nodes.contains(id))
            });
            edges.retain(|edge| {
                edge["source_id"]
                    .as_str()
                    .is_some_and(|id| selected_nodes.contains(id))
                    && edge["target_id"]
                        .as_str()
                        .is_some_and(|id| selected_nodes.contains(id))
            });
        }
        if command.exact_projection || !selected_edges.is_empty() {
            edges.retain(|edge| {
                edge["id"]
                    .as_str()
                    .is_some_and(|id| selected_edges.contains(id))
            });
        }
        observed_edges.extend(
            edges
                .iter()
                .filter_map(|edge| edge["id"].as_str().map(str::to_owned)),
        );
        observed_nodes.extend(
            nodes
                .iter()
                .filter_map(|node| node["id"].as_str().map(str::to_owned)),
        );
        documents.push(json!({"project_id":project.id,"generation_id":generation,"manifest":snapshot.manifest(),"nodes":nodes,"edges":edges}));
        vector.insert(project.id.clone(), json!(generation));
        selected_ids.push(project.id.clone());
        graphs.push(render::ProjectGraph::new(project.id.clone(), snapshot));
    }
    data.release().map_err(runtime::guard_error)?;
    if graphs.is_empty()
        || selected_nodes.iter().any(|id| !observed_nodes.contains(id))
        || selected_edges.iter().any(|id| !observed_edges.contains(id))
    {
        return Err(invalid(
            "export selection is empty or a node is outside the selected published scope",
        ));
    }
    let artifact = json!({"schema_version":1,"solution_id":command.solution,"catalog_generation_id":catalog_id,
        "generation_vector":vector,"selection":{"projects":selected_ids,"node_ids":command.nodes,"edge_ids":command.edges,"projection":if command.exact_projection { "exact" } else { "induced" }},
        "completeness":if selected_nodes.is_empty() && selected_edges.is_empty() && !command.exact_projection { "selected_projects" } else { "filtered_projection" },
        "source_fidelity":"published_source_not_staged_verified","freshness":"not_checked",
        "limits":["Static graph coverage is not semantic accuracy; out-of-scope and dynamic consumers remain unknown"],"projects":documents});
    let mut files = vec![(
        "graph.json",
        serde_json::to_vec_pretty(&artifact).map_err(|_| invalid("export serialization failed"))?,
    )];
    if command.format == "html" {
        let freshness = render::FreshnessInput::new(0, Vec::new(), render::VerificationMode::None);
        let mut diagram = render::build_diagram(&command.solution, &graphs, &freshness)?;
        if !selected_nodes.is_empty() {
            render::select_nodes(&mut diagram, &selected_nodes);
        }
        if command.exact_projection || !selected_edges.is_empty() {
            render::select_edges(&mut diagram, &selected_edges);
        }
        files.push(("result.html", diagram.result_html.into_bytes()));
        files.push((
            "summary.json",
            render::summary_json_bytes(&diagram.summary)?,
        ));
    }
    // Claim the new directory atomically. A concurrent human destination is never overwritten.
    std::fs::create_dir(&command.out)
        .map_err(|error| runtime::storage_error("export destination", &error))?;
    let mut receipts = Vec::new();
    for (name, bytes) in files {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(command.out.join(name))
            .map_err(|error| runtime::storage_error("export artifact", &error))?;
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| runtime::storage_error("export synchronization", &error))?;
        receipts.push(json!({"name":name,"bytes":bytes.len(),"sha256":sha256_hex(&bytes)}));
    }
    Ok(
        json!({"schema_version":1,"solution_id":command.solution,"format":command.format,"catalog_generation_id":catalog_id,
        "generation_vector":vector,"completeness":artifact["completeness"],"artifacts":receipts}),
    )
}
