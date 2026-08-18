from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOCK = ROOT / "manifests" / "source-lock.json"
COMMIT_RE = re.compile(r"^[0-9a-f]{40}$")


def main() -> int:
    data = json.loads(LOCK.read_text(encoding="utf-8"))

    if data.get("version") != 1:
        raise SystemExit("source-lock version must be 1")
    if not data.get("generated_at"):
        raise SystemExit("source-lock generated_at is required")
    if not isinstance(data.get("sources"), list) or not data["sources"]:
        raise SystemExit("source-lock sources must be a non-empty list")

    ids: set[str] = set()
    repositories: set[str] = set()
    required_pinned = {
        "source.superagents",
        "source.superskills",
        "source.agent-skills",
    }

    for source in data["sources"]:
        source_id = source.get("id")
        repository = source.get("repository")
        if not source_id or not repository:
            raise SystemExit("every source-lock entry requires id and repository")
        if source_id in ids:
            raise SystemExit(f"duplicate source-lock id: {source_id}")
        if repository in repositories:
            raise SystemExit(f"duplicate source-lock repository: {repository}")
        ids.add(source_id)
        repositories.add(repository)

        commit = source.get("commit")
        if source_id in required_pinned:
            if not isinstance(commit, str) or not COMMIT_RE.fullmatch(commit):
                raise SystemExit(f"{source_id} must be pinned to a 40-character commit")
        elif commit is not None and not COMMIT_RE.fullmatch(commit):
            raise SystemExit(f"invalid commit for {source_id}: {commit!r}")

        if not isinstance(source.get("uses"), list) or not source["uses"]:
            raise SystemExit(f"{source_id} must declare non-empty uses")

    missing = required_pinned - ids
    if missing:
        raise SystemExit(f"missing required pinned sources: {sorted(missing)}")

    policy = data.get("policy", {})
    required_policy = {
        "imports_are_metadata_or_adapted_contracts",
        "remote_content_is_not_executed_by_catalog_sync",
        "source_updates_require_review",
        "protocol_remains_authoritative",
    }
    missing_policy = [key for key in required_policy if policy.get(key) is not True]
    if missing_policy:
        raise SystemExit(f"source-lock policy is incomplete: {missing_policy}")

    print(f"AgentFileOps source-lock validation: PASS ({len(ids)} sources)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
