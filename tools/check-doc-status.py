#!/usr/bin/env python3
"""Validate product documentation links, JSON examples and engine command coverage.

Does not infer release readiness or require historical no-installer boilerplate.
"""
from __future__ import annotations

import argparse
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import tempfile

REFERENCE = "docs/reference/axiom-graphd.md"
CLI_SOURCE = "crates/axiom-graphd/src/cli.rs"
INTERNAL = re.compile(r"\b(?:[A-Z]-\d{3}|V2-\d{3}|CP-\d{2})\b|\bcrates/|\bevidence/|Owner:|Spec baseline:")
SECRET = re.compile(r"AKIA[0-9A-Z]{16}|gh[pousr]_[A-Za-z0-9]{30,}|-----BEGIN [A-Z ]*PRIVATE KEY-----")
LINK = re.compile(r"!?\[[^\]\n]*\]\(([^\s)]+)(?:\s+\"[^\"]*\")?\)")


def check_reference(text: str, cli: str) -> list[str]:
    issues = []
    try:
        table = text.split("<!-- BEGIN COMMAND STATUS -->", 1)[1].split("<!-- END COMMAND STATUS -->", 1)[0]
        documented = set(re.findall(r"^\|\s*([a-z]+)\s*\|\s*available\s*\|", table, re.M))
        region = cli.split("pub const ACCEPTED_VERBS: &[&str] = &[", 1)[1].split("];", 1)[0]
        accepted = set(re.findall(r'"([a-z][a-z0-9-]*)"', region))
        if documented != accepted:
            issues.append(f"Command coverage differs: documented={sorted(documented)}, accepted={sorted(accepted)}")
        for name in accepted:
            if not re.search(r"^##\s+\d+\.\s+`" + name + r"`\s*$", text, re.M):
                issues.append(f"Missing command section: {name}")
    except IndexError:
        issues.append("Missing command status markers or accepted-verb declaration")
    return issues


def check_text(path: Path, text: str, product: bool) -> list[str]:
    issues = []
    prose = re.sub(r"```.*?```", "", text, flags=re.S)
    for link in LINK.findall(prose):
        if link.startswith("#") or re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", link):
            continue
        target = link.split("#", 1)[0].split("?", 1)[0]
        if target and not (path.parent / target).exists():
            issues.append(f"{path}: broken local link {link}")
    for block in re.findall(r"```json\s*\n(.*?)```", text, re.S):
        try:
            json.loads(block)
        except ValueError as exc:
            issues.append(f"{path}: invalid JSON example: {exc}")
    if SECRET.search(text):
        issues.append(f"{path}: possible embedded credential")
    if product and (INTERNAL.search(text) or re.search(r"^\s*cargo(?:\s|$)", text, re.M)):
        issues.append(f"{path}: internal task/source/build material in product guide")
    return issues


def run(root: Path) -> list[str]:
    issues = check_reference((root / REFERENCE).read_text(encoding="utf-8"),
                             (root / CLI_SOURCE).read_text(encoding="utf-8"))
    paths = [root / "README.md", *sorted((root / "docs").rglob("*.md")),
             *sorted((root / "examples/first-graph").rglob("*.md")), root / "benchmarks/README.md"]
    for p in paths:
        product = p.is_relative_to(root / "docs") and "development" not in p.relative_to(root).parts
        issues.extend(check_text(p, p.read_text(encoding="utf-8"), product))
    for p in sorted((root / "examples/first-graph").rglob("*.html")):
        issues.extend(check_html(p, p.read_text(encoding="utf-8")))
    return issues


def check_html(path: Path, text: str) -> list[str]:
    """Check resources for file-openable examples without executing page code."""
    issues = []

    class Links(HTMLParser):
        def handle_starttag(self, tag, attrs):
            for key, value in attrs:
                if key not in {"href", "src"} or not value or value.startswith("#"):
                    continue
                if re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", value):
                    if key == "src":
                        issues.append(f"{path}: example depends on external resource {value}")
                    continue
                target = value.split("#", 1)[0].split("?", 1)[0]
                if not (path.parent / target).is_file():
                    issues.append(f"{path}: broken HTML resource {value}")

    Links().feed(text)
    return issues


def self_test() -> list[str]:
    failures = []
    cli = 'pub const ACCEPTED_VERBS: &[&str] = &["help", "build"];'
    ref = ("<!-- BEGIN COMMAND STATUS -->\n| help | available | Help |\n"
           "| build | available | Build |\n<!-- END COMMAND STATUS -->\n"
           "## 1. `help`\n\n## 2. `build`\n")
    if check_reference(ref, cli):
        failures.append("Valid command fixture rejected")
    for label, bad in [("missing verb", ref.replace("| build | available | Build |\n", "")),
                       ("invented verb", ref.replace("| build |", "| install |")),
                       ("missing section", ref.replace("## 2. `build`", "Build")),
                       ("missing marker", ref.replace("<!-- BEGIN COMMAND STATUS -->", ""))]:
        if not check_reference(bad, cli):
            failures.append(label + " was not detected")
    with tempfile.TemporaryDirectory() as folder:
        p = Path(folder) / "guide.md"
        p.write_text("# Guide\n", encoding="utf-8")
        clean = '# Start\n\n[Guide](guide.md)\n\n```json\n{"id":"demo"}\n```\n'
        if check_text(p, clean, True):
            failures.append("Clean product guide rejected")
        for label, bad in [("broken link", "[Missing](missing.md)"),
                           ("broken image", "![Missing](missing.png)"),
                           ("invalid JSON", '```json\n{"x":}\n```'),
                           ("task code", "Benchmark (F-030)"),
                           ("source internals", "See crates/graph-core/src/lib.rs"),
                           ("Cargo onboarding", "```text\ncargo test\n```"),
                           ("credential", "AKIAIOSFODNN7EXAMPLE")]:
            if not check_text(p, bad, True):
                failures.append(label + " was not detected")
        if check_text(p, "# Developer\n\nF-030\n```text\ncargo test\n```", False):
            failures.append("Legitimate contributor material rejected")
        if check_html(p, '<a href="guide.md">Guide</a>'):
            failures.append("Valid offline HTML link rejected")
        for label, html in [("HTML link", '<a href="missing.md">Missing</a>'),
                            ("HTML image", '<img src="missing.svg" alt="Missing">'),
                            ("remote resource", '<script src="https://example.com/script.js"></script>')]:
            if not check_html(p, html):
                failures.append(label + " was not detected")
    return failures


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    try:
        issues = self_test() if args.self_test else run(args.root.resolve())
    except (OSError, ValueError, IndexError) as exc:
        issues = [str(exc)]
    for issue in issues:
        print("FAIL " + issue)
    if not issues:
        print("PASS documentation " + ("negative controls" if args.self_test else "links, JSON, commands and editorial boundaries"))
    return 2 if issues else 0


if __name__ == "__main__":
    raise SystemExit(main())
