#!/usr/bin/env python3
"""Build a small, reproducible AgentFileOps CLI release bundle."""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import shutil
import subprocess
import tempfile
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def package_version() -> str:
    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    for line in cargo.splitlines():
        if line.startswith("version = "):
            return line.split('"', 2)[1]
    raise RuntimeError("workspace version not found")


def platform_name() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower().replace("aarch64", "arm64")
    return f"{system}-{machine}"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=ROOT / "dist")
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    binary = ROOT / "target" / "release" / ("afo.exe" if platform.system() == "Windows" else "afo")
    if not args.skip_build:
        subprocess.run(
            ["cargo", "build", "--release", "-p", "agent-file-ops-cli"],
            cwd=ROOT,
            check=True,
        )
    if not binary.is_file():
        raise SystemExit(f"release binary not found: {binary}")

    version = package_version()
    identifier = f"agentfileops-cli-{version}-{platform_name()}"
    output_dir = args.output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    archive = output_dir / f"{identifier}.zip"

    manifest = {
        "product": "AgentFileOps",
        "package": "afo-cli",
        "version": version,
        "platform": platform_name(),
        "binary": "bin/afo",
        "supported": [
            "path normalization and containment",
            "risk classification",
            "connection alias/path resolution",
            "capability-aware backend strategy selection",
            "normalized JSON output and errors",
        ],
        "not_production_claims": [
            "live SSH/SFTP transport",
            "MCP, SDK, and daemon adapters",
            "destructive operations, sync, and recovery",
        ],
        "sha256": "generated-after-archive",
    }

    with tempfile.TemporaryDirectory(prefix="agentfileops-package-") as temporary:
        staging = Path(temporary) / identifier
        (staging / "bin").mkdir(parents=True)
        shutil.copy2(binary, staging / "bin" / binary.name)
        for relative in ["README.md", "LICENSE", "SECURITY.md", "RELEASE_SCOPE.md"]:
            shutil.copy2(ROOT / relative, staging / relative)
        shutil.copytree(ROOT / "protocol" / "schema", staging / "protocol" / "schema")
        shutil.copytree(ROOT / "docs" / "verification", staging / "docs" / "verification")
        (staging / "PACKAGE.json").write_text(
            json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
        )

        with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as bundle:
            for path in sorted(staging.rglob("*")):
                if path.is_file():
                    bundle.write(path, path.relative_to(staging.parent))

    checksum = sha256(archive)
    archive.with_suffix(".zip.sha256").write_text(
        f"{checksum}  {archive.name}\n", encoding="utf-8"
    )
    print(f"created {archive}")
    print(f"sha256 {checksum}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
