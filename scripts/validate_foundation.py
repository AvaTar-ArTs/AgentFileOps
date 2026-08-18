from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = [
    "README.md",
    "ARCHITECTURE.md",
    "SECURITY.md",
    "CHANGELOG.md",
    "manifests/source-lock.json",
    "manifests/agents.json",
    "manifests/skills.json",
    "protocol/schema/connection.schema.json",
    "protocol/schema/path.schema.json",
    "protocol/schema/operation.schema.json",
    "protocol/schema/plan.schema.json",
    "protocol/schema/result.schema.json",
    "protocol/schema/audit.schema.json",
    "skills/remote-filesystem/SKILL.md",
    "skills/protocol-conformance/SKILL.md",
    "tests/conformance/README.md",
    "crates/README.md",
    "packages/README.md",
    "sdk/python/README.md",
    "sdk/go/README.md",
    "cmd/agent-file-opsd/README.md",
    "presets/hostinger/README.md",
    "docs/ecosystem/AGENT_SKILL_REVIEW.md",
    "docs/verification/VERTICAL_SLICE_01.md",
]

ACTIVE_NAMING_SURFACES = [
    "ARCHITECTURE.md",
    "SECURITY.md",
    "protocol/README.md",
    "skills/remote-filesystem/SKILL.md",
    "skills/protocol-conformance/SKILL.md",
    "packages/README.md",
    "crates/README.md",
    "sdk/python/README.md",
    "sdk/go/README.md",
    "cmd/agent-file-opsd/README.md",
    "presets/hostinger/README.md",
    "docs/ecosystem/AGENT_SKILL_REVIEW.md",
    "docs/verification/VERTICAL_SLICE_01.md",
    ".github/workflows/foundation.yml",
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
        "agentfileops.protocol-conformance",
    }:
        if expected not in skill_ids:
            raise SystemExit(f"missing AgentFileOps skill: {expected}")

    for active_surface in ACTIVE_NAMING_SURFACES:
        text = (ROOT / active_surface).read_text(encoding="utf-8")
        if "AgentFS" in text:
            raise SystemExit(
                f"retired AgentFS name found in active surface: {active_surface}"
            )
        if "agentfs" in text.lower():
            raise SystemExit(
                f"retired agentfs identifier found in active surface: {active_surface}"
            )

    retired_paths = [
        "cmd/agentfsd",
        "crates/agentfs-core",
        "crates/agentfs-cli",
    ]
    leftovers = [p for p in retired_paths if (ROOT / p).exists()]
    if leftovers:
        raise SystemExit(f"retired AgentFS paths still exist: {leftovers}")

    print("AgentFileOps foundation validation: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
