//! Monotonic operational measurements kept outside deterministic graph content.
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use serde::Serialize;

static NEXT: AtomicU64 = AtomicU64::new(1);

/// A measured execution, including explicitly uninstrumented observations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkCycle {
    pub schema_version: u32,
    pub cycle_id: String,
    pub operation: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub processing_ms: u64,
    pub scan_ms: Option<u64>,
    pub analyze_ms: Option<u64>,
    pub validate_ms: Option<u64>,
    pub publish_ms: Option<u64>,
    pub queue_wait_ms: Option<u64>,
    pub change_observed_to_publish_ms: Option<u64>,
    pub unavailable_reason: String,
    pub files_changed: Option<usize>,
    pub files_analyzed: Option<usize>,
    pub nodes: Option<usize>,
    pub edges: Option<usize>,
    pub generation_before: BTreeMap<String, Option<String>>,
    pub generation_after: BTreeMap<String, Option<String>>,
}

impl WorkCycle {
    pub fn start(operation: &str) -> (Self, Instant) {
        let started_at = graph_store::migrations::utc_timestamp();
        let cycle = Self {
            schema_version: 1,
            cycle_id: format!("{}-{}-{}", std::process::id(), started_at, NEXT.fetch_add(1, Ordering::Relaxed)),
            operation: operation.to_owned(), status: "pending".to_owned(), started_at,
            finished_at: None, processing_ms: 0,
            scan_ms: None, analyze_ms: None, validate_ms: None, publish_ms: None,
            queue_wait_ms: None, change_observed_to_publish_ms: None,
            unavailable_reason: "Phase, queue and source observation clocks are not instrumented; failure counts are unknown".to_owned(),
            files_changed: None, files_analyzed: None, nodes: None, edges: None,
            generation_before: BTreeMap::new(), generation_after: BTreeMap::new(),
        };
        (cycle, Instant::now())
    }

    pub fn finish(&mut self, start: Instant, status: &str) {
        self.processing_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        self.finished_at = Some(graph_store::migrations::utc_timestamp());
        self.status = status.to_owned();
    }
}
