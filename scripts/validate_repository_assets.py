from __future__ import annotations

import re
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"
MARKDOWN_LINK = re.compile(r"!\[[^]]*\]\(([^)]+)\)")


def check_png(path: Path) -> None:
    raw = path.read_bytes()
    if not raw.startswith(PNG_SIGNATURE):
        raise SystemExit(f"invalid PNG signature: {path}")
    if len(raw) < 24:
        raise SystemExit(f"truncated PNG header: {path}")
    width, height = struct.unpack(">II", raw[16:24])
    if width < 400 or height < 200:
        raise SystemExit(f"unexpectedly small PNG: {path} ({width}x{height})")
    if len(raw) < 100_000:
        raise SystemExit(f"PNG is suspiciously small/truncated: {path} ({len(raw)} bytes)")


def check_markdown_images(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    for target in MARKDOWN_LINK.findall(text):
        if target.startswith(("http://", "https://", "#")):
            continue
        candidate = (path.parent / target).resolve()
        if not candidate.is_file():
            raise SystemExit(f"missing image referenced by {path}: {target}")


def main() -> int:
    required = [
        ROOT / "docs/assets/generated/agentfileops-hero.png",
        ROOT / "docs/assets/generated/agentfileops-use-cases.png",
        ROOT / "docs/assets/generated/agentfileops-typography-poster.png",
        ROOT / "docs/assets/generated/agentfileops-workflow-visual.png",
        ROOT / "docs/assets/generated/agentfileops-protocol-hero.png",
        ROOT / "docs/assets/generated/agentfileops-poster-square.png",
        ROOT / "docs/assets/generated/agentfileops-social-og.png",
        ROOT / "docs/assets/use-case-legend.svg",
        ROOT / "docs/design/architecture.svg",
        ROOT / "docs/design/data-flow.svg",
        ROOT / "docs/design/risk-levels.svg",
    ]
    for path in required:
        if not path.is_file():
            raise SystemExit(f"missing repository asset: {path.relative_to(ROOT)}")
        if path.suffix == ".png":
            check_png(path)

    check_markdown_images(ROOT / "README.md")
    check_markdown_images(ROOT / "docs/assets/README.md")

    print(f"AgentFileOps repository asset validation: PASS ({len(required)} assets)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
