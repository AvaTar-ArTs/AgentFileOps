from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SCHEMA_FILES = [
    "protocol/schema/connection.schema.json",
    "protocol/schema/path.schema.json",
    "protocol/schema/operation.schema.json",
    "protocol/schema/plan.schema.json",
    "protocol/schema/result.schema.json",
    "protocol/schema/audit.schema.json",
]

REQUIRED = [
    "README.md",
    "ARCHITECTURE.md",
    "SECURITY.md",
    "CHANGELOG.md",
    "manifests/source-lock.json",
    "manifests/agents.json",
    "manifests/skills.json",
    "manifests/discovery.json",
    *SCHEMA_FILES,
    "skills/remote-file-operations/SKILL.md",
    "skills/remote-deploy/SKILL.md",
    "skills/artifact-publisher/SKILL.md",
    "skills/remote-sync/SKILL.md",
    "skills/protocol-conformance/SKILL.md",
    "tests/conformance/README.md",
    "crates/README.md",
    "packages/README.md",
    "packages/package-metadata.json",
    "sdk/python/README.md",
    "sdk/go/README.md",
    "cmd/agent-file-opsd/README.md",
    "presets/hostinger/README.md",
    "docs/ecosystem/AGENT_SKILL_REVIEW.md",
    "docs/distribution/DISCOVERY_SEO.md",
    "docs/verification/VERTICAL_SLICE_01.md",
]

ACTIVE_NAMING_SURFACES = [
    "ARCHITECTURE.md",
    "SECURITY.md",
    "protocol/README.md",
    "skills/remote-file-operations/SKILL.md",
    "skills/remote-deploy/SKILL.md",
    "skills/artifact-publisher/SKILL.md",
    "skills/remote-sync/SKILL.md",
    "skills/protocol-conformance/SKILL.md",
    "packages/README.md",
    "crates/README.md",
    "sdk/python/README.md",
    "sdk/go/README.md",
    "cmd/agent-file-opsd/README.md",
    "presets/hostinger/README.md",
    "docs/ecosystem/AGENT_SKILL_REVIEW.md",
    "docs/distribution/DISCOVERY_SEO.md",
    "docs/verification/VERTICAL_SLICE_01.md",
    ".github/workflows/foundation.yml",
]

STALE_ACTIVE_PATTERNS = [
    "# AgentFS",
    "AgentFS Protocol",
    "AgentFS Security",
    "AgentFS Orchestrator",
    "AgentFS Protocol Architect",
    "AgentFS Security Reviewer",
    "AgentFS Conformance Verifier",
    "AgentFS-connected",
    "canonical AgentFS",
    "AgentFS semantic",
    "AgentFS service",
    "AgentFS compatibility",
    "AgentFS provider",
    "@avatar-arts/agentfs",
    "agentfs-core",
    "agentfs-cli",
    "agentfsd",
    "agentfs.",
]


def load_json(relative: str):
    return json.loads((ROOT / relative).read_text(encoding="utf-8"))


def main() -> int:
    missing = [p for p in REQUIRED if not (ROOT / p).is_file()]
    if missing:
        raise SystemExit(f"missing required foundation files: {missing}")

    for path in REQUIRED:
        if path.endswith(".json"):
            load_json(path)

    for schema_path in SCHEMA_FILES:
        schema = load_json(schema_path)
        schema_id = schema.get("$id", "")
        title = schema.get("title", "")
        if "AgentFS" in title or "agentfs.dev" in schema_id:
            raise SystemExit(f"retired schema identity found in {schema_path}")
        if "AvaTar-ArTs/AgentFileOps" not in schema_id:
            raise SystemExit(f"schema id does not use AgentFileOps canonical source: {schema_path}")

    operation = load_json("protocol/schema/operation.schema.json")
    operations = operation["properties"]["operation"]["enum"]
    forbidden = {"exec", "shell", "command", "run_command", "ssh_exec"}
    overlap = forbidden.intersection(operations)
    if overlap:
        raise SystemExit(f"public arbitrary-shell-like operations forbidden: {sorted(overlap)}")

    path_schema = load_json("protocol/schema/path.schema.json")
    if "follow_symlinks" not in path_schema["properties"]:
        raise SystemExit("PathSpec must expose explicit follow_symlinks semantics")

    source_lock = load_json("manifests/source-lock.json")
    source_ids = {entry["id"] for entry in source_lock["sources"]}
    required_sources = {"source.superagents", "source.superskills", "source.agent-skills"}
    if not required_sources.issubset(source_ids):
        raise SystemExit("ecosystem source lock is incomplete")

    agents = load_json("manifests/agents.json")
    agent_ids = {entry["id"] for entry in agents["agents"]}
    for expected in {
        "agentfileops.orchestrator",
        "agentfileops.protocol-architect",
        "agentfileops.security-reviewer",
        "agentfileops.conformance-verifier",
    }:
        if expected not in agent_ids:
            raise SystemExit(f"missing AgentFileOps agent: {expected}")

    skills = load_json("manifests/skills.json")
    skill_ids = {entry["id"] for entry in skills["skills"]}
    for expected in {
        "agentfileops.remote-filesystem",
        "agentfileops.safe-remote-deploy",
        "agentfileops.remote-sync",
        "agentfileops.artifact-publisher",
        "agentfileops.protocol-conformance",
    }:
        if expected not in skill_ids:
            raise SystemExit(f"missing AgentFileOps skill: {expected}")

    for entry in skills["skills"]:
        path = entry.get("path")
        if not path or not (ROOT / path).is_file():
            raise SystemExit(f"skill catalog path is missing for {entry['id']}: {path}")

    discovery = load_json("manifests/discovery.json")
    if discovery.get("product") != "AgentFileOps":
        raise SystemExit("discovery manifest must identify AgentFileOps")
    if discovery.get("category_phrase") != "Remote File Operations for AI Agents":
        raise SystemExit("canonical category phrase drifted")
    expected_keywords = {"mcp", "remote-filesystem", "ssh", "sftp", "file-transfer"}
    npm_keywords = set(discovery["npm"]["keywords"])
    if not expected_keywords.issubset(npm_keywords):
        raise SystemExit("core npm discovery keywords are incomplete")
    if discovery["skills_sh"]["primary"] != "remote-file-operations":
        raise SystemExit("primary skills.sh discovery name drifted")

    package_metadata = load_json("packages/package-metadata.json")
    if set(package_metadata["keywords"]) != set(discovery["npm"]["keywords"]):
        raise SystemExit("npm package keyword metadata drifted from discovery manifest")

    for active_surface in ACTIVE_NAMING_SURFACES:
        text = (ROOT / active_surface).read_text(encoding="utf-8")
        for stale in STALE_ACTIVE_PATTERNS:
            if stale in text:
                raise SystemExit(
                    f"retired active identifier {stale!r} found in: {active_surface}"
                )

    retired_paths = [
        "cmd/agentfsd",
        "crates/agentfs-core",
        "crates/agentfs-cli",
        "skills/remote-filesystem",
    ]
    leftovers = [p for p in retired_paths if (ROOT / p).exists()]
    if leftovers:
        raise SystemExit(f"retired AgentFS/discovery paths still exist: {leftovers}")

    print("AgentFileOps foundation validation: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
