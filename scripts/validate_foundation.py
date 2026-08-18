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
        "agentfs.orchestrator",
        "agentfs.protocol-architect",
        "agentfs.security-reviewer",
        "agentfs.conformance-verifier",
    }:
        if expected not in agent_ids:
            raise SystemExit(f"missing AgentFS agent: {expected}")

    print("AgentFS foundation validation: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
