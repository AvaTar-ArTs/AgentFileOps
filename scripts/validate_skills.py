from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "manifests" / "skills.json"


def main() -> int:
    catalog = json.loads(CATALOG.read_text(encoding="utf-8"))
    errors: list[str] = []

    for entry in catalog["skills"]:
        path = ROOT / entry["path"]
        if not path.is_file():
            errors.append(f"missing skill file: {entry['path']}")
            continue

        text = path.read_text(encoding="utf-8")
        match = re.match(r"^---\nname: ([a-z0-9-]+)\ndescription: (.+)\n---\n", text)
        if not match:
            errors.append(f"invalid frontmatter: {entry['path']}")
            continue

        if match.group(1) != entry["id"].removeprefix("agentfileops."):
            errors.append(f"catalog/name mismatch: {entry['path']}")
        if not match.group(2).startswith("Use when "):
            errors.append(f"description must start with 'Use when': {entry['path']}")

        required_sections = ["## Contract", "## Failure behavior", "## Verification"]
        for section in required_sections:
            if section not in text:
                errors.append(f"missing {section} in {entry['path']}")

    if errors:
        raise SystemExit("\n".join(errors))

    print(f"AgentFileOps skill contract validation: PASS ({len(catalog['skills'])} skills)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
