#!/usr/bin/env python3
"""Run sample source with an installed engine in an isolated, new output folder."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def dump(p: Path, obj: object) -> None:
    p.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--engine", default="axge")
    ap.add_argument("--out", type=Path, required=True, help="New folder on local disk")
    args = ap.parse_args()
    found = shutil.which(args.engine)
    if found is None:
        ap.error("Engine not found. Install Axiom or pass --engine with its executable path.")
    engine = str(Path(found).resolve(strict=True))
    requested = args.out.absolute()
    if requested.exists() or requested.is_symlink():
        ap.error("Output already exists. Choose a new directory; nothing was changed.")
    out = requested.parent.resolve(strict=True) / requested.name
    out.mkdir()  # Exclusive claim; never overwrite a concurrent creator.
    sample = Path(__file__).resolve().parent
    source = sample / "source"
    inputs = {p.relative_to(source).as_posix(): sha(p) for p in source.rglob("*.cs")}
    shutil.copytree(source, out / "workspace", ignore=shutil.ignore_patterns(".axiom"))
    shutil.copyfile(sample / "solution.json", out / "solution.json")
    dump(out / "bindings.json", {"bindings": {"demo-shop-repo": str(out / "workspace")}})
    env = dict(os.environ, AXIOM_HOME=str(out / "state"))
    commands = []

    def call(label: str, words: list[str]) -> dict:
        start = time.monotonic()
        result = subprocess.run([engine, *words, "--json"], cwd=out, env=env,
                                capture_output=True, text=True, encoding="utf-8", timeout=90)
        commands.append({"name": label, "argv": ["axge", *words, "--json"],
                         "exit_code": result.returncode,
                         "wall_ms": round((time.monotonic() - start) * 1000, 3)})
        (out / f"{label}.stderr.txt").write_text(result.stderr, encoding="utf-8")
        if result.returncode:
            raise RuntimeError(f"{label}: {result.stdout} {result.stderr}")
        obj = json.loads(result.stdout)
        dump(out / f"{label}.json", obj)
        return obj

    version = call("version", ["version"])
    register = ["solution", "register", "--config", "solution.json", "--bindings", "bindings.json"]
    call("preview", [*register, "--dry-run"])
    if (out / "state/config/bindings.json").exists():
        raise RuntimeError("Preview persisted bindings")
    call("register", [*register, "--apply"])
    build = call("build", ["build", "--solution", "demo-shop"])
    call("status", ["status", "--solution", "demo-shop"])
    query = call("query", ["query", "context", "--solution", "demo-shop", "--symbol",
                           "Demo.Shop.OrderService", "--max-nodes", "20", "--max-bytes", "8192"])
    if not query.get("records") or "OrderService" not in json.dumps(query):
        raise RuntimeError("Known sample declaration was not returned")
    for fmt in ("html", "json"):
        receipt = call(f"export-{fmt}", ["export", "--solution", "demo-shop", "--out", fmt, "--format", fmt])
        for item in receipt["artifacts"]:
            p = out / fmt / item["name"]
            if sha(p) != item["sha256"] or p.stat().st_size != item["bytes"]:
                raise RuntimeError("Export receipt verification failed")
    before = {p.name: sha(p) for p in (out / "html").iterdir()}
    refusal = subprocess.run([engine, "export", "--solution", "demo-shop", "--out", "html",
                              "--format", "html", "--json"], cwd=out, env=env,
                             capture_output=True, text=True, encoding="utf-8", timeout=30)
    if refusal.returncode == 0 or before != {p.name: sha(p) for p in (out / "html").iterdir()}:
        raise RuntimeError("Existing destination was not refused unchanged")
    dump(out / "destination-refusal.json", {"exit_code": refusal.returncode, "body": json.loads(refusal.stdout)})
    for rel, digest in inputs.items():
        if sha(source / rel) != digest or sha(out / "workspace" / rel) != digest:
            raise RuntimeError("Source changed")
    graph = json.loads((out / "json/graph.json").read_text(encoding="utf-8"))
    nodes = sum(len(p["nodes"]) for p in graph["projects"])
    edges = sum(len(p["edges"]) for p in graph["projects"])
    if nodes < 3 or edges < 1:
        raise RuntimeError("Expected a nonempty declaration/containment graph")
    dump(out / "run.json", {"engine_version": version, "engine_sha256": sha(Path(engine)),
         "source_sha256": inputs, "solution_id": "demo-shop", "project_ids": ["app"],
         "nodes": nodes, "edges": edges, "commands": commands,
         "engine_cycle": build.get("cycle"), "generation_vector": graph.get("generation_vector"),
         "source_fidelity": graph.get("source_fidelity"), "platform": sys.platform,
         "coverage": "declarations and containment; complete method-call analysis outside this profile",
         "checks": {"preview_did_not_persist_bindings": True, "source_unchanged": True,
                    "known_symbol_returned": True, "artifact_receipts_verified": True,
                    "existing_destination_refused_unchanged": True}})
    print(f"Built demo-shop: {nodes} nodes, {edges} edges.\nOpen {out / 'html/result.html'}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, RuntimeError, subprocess.TimeoutExpired) as exc:
        print(f"Demo failed: {exc}", file=sys.stderr)
        raise SystemExit(2)
