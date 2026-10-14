"""Bind checked archives to native source proof before preparing Release assets."""
import argparse
import hashlib
import json
import shutil
import re
import tomllib
from pathlib import Path

LANES = {"windows-x64", "linux-x64", "macos-x64"}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def collect(root, revision, output):
    if output.exists():
        raise ValueError("output must be a new directory")
    selected = []
    seen = set()
    for directory in root.iterdir():
        manifests = list(directory.rglob("candidate-manifest.json"))
        proofs = []
        for proof_path in directory.rglob("native-report.json"):
            value = json.loads(proof_path.read_text(encoding="utf-8"))
            if "platform" in value:
                proofs.append(proof_path)
        if not manifests or not proofs:
            continue
        if len(manifests) != 1 or len(proofs) != 1:
            raise ValueError("duplicate candidate or native proof")
        candidate, report = manifests[0], proofs[0]
        manifest = json.loads(candidate.read_text(encoding="utf-8"))
        proof = json.loads(report.read_text(encoding="utf-8"))
        lane = manifest["platform"]
        if lane not in LANES or lane in seen:
            raise ValueError("unsupported or duplicate native lane")
        seen.add(lane)
        if manifest["source_revision"] != revision or proof["source_revision"] != revision:
            raise ValueError("source drift")
        if proof["platform"] != lane or proof["execution"] != "native" or proof.get("migration_verified") is not True:
            raise ValueError("native proof incomplete")
        for kind in ("archive", "sbom"):
            entry = manifest[kind]
            name = entry["name"]
            if Path(name).name != name or "/" in name or "\\" in name:
                raise ValueError("unsafe asset name")
            path = candidate.parent / name
            if sha(path) != entry["sha256"]:
                raise ValueError("asset hash changed")
            selected.append((path, name))
        selected.extend([(candidate, lane + "-manifest.json"), (report, lane + "-native-report.json")])
    if seen != LANES:
        raise ValueError("three native lanes are required")
    output.mkdir(parents=True)
    for path, name in selected:
        shutil.copyfile(path, output / name)
    (output / "SOURCE-REVISION.txt").write_text(revision + "\n", encoding="utf-8", newline="\n")
    repository = Path(__file__).resolve().parents[1]
    version = tomllib.loads((repository / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]["package"]["version"]
    changelog = (repository / "Changelog.md").read_text(encoding="utf-8")
    entries = re.findall(r"^## " + re.escape(version) + r"\s*\n(.*?)(?=^## |\Z)", changelog, re.M | re.S)
    if len(entries) != 1:
        raise ValueError("require exactly one release changelog entry")
    changes = re.sub(r"^### ", "## ", entries[0].strip(), flags=re.M)
    (output / "RELEASE-NOTES.md").write_text(
        "# Axiom core " + version + "\n\n" + changes + "\n\n"
        "Native Windows x64, Linux x64 and Mac Intel assets verified by GitHub Actions. "
        "Use axge for the graph engine and axm engine for private installation operations. "
        "Mac ARM and live AI-host verification remain outside this release's native evidence.\n",
        encoding="utf-8", newline="\n")
    (output / "SHA256SUMS").write_text("".join(sha(path) + "  " + path.name + "\n" for path in sorted(output.iterdir())), encoding="utf-8", newline="\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    collect(args.input, args.source_revision, args.out)
    print("OK source-bound checked Release assets and SHA256SUMS")
