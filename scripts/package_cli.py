#!/usr/bin/env python3
"""Build a small, reproducible AgentFileOps CLI release bundle."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import platform
import shutil
import subprocess
import tempfile
import tarfile
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


def staged_files(staging: Path) -> list[Path]:
    return [path for path in sorted(staging.rglob("*")) if path.is_file()]


def create_zip(archive: Path, staging: Path) -> None:
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as bundle:
        for path in staged_files(staging):
            relative = path.relative_to(staging.parent).as_posix()
            info = zipfile.ZipInfo(relative, date_time=(2020, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            mode = 0o755 if relative.startswith(f"{staging.name}/bin/") else 0o644
            info.external_attr = mode << 16
            bundle.writestr(info, path.read_bytes())


def create_tar_gz(archive: Path, staging: Path) -> None:
    with archive.open("wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as bundle:
                for path in staged_files(staging):
                    relative = path.relative_to(staging.parent).as_posix()
                    info = bundle.gettarinfo(str(path), arcname=relative)
                    info.mtime = 0
                    info.uid = 0
                    info.gid = 0
                    info.uname = "root"
                    info.gname = "root"
                    with path.open("rb") as handle:
                        bundle.addfile(info, handle)


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
    archives = [
        output_dir / f"{identifier}.zip",
        output_dir / f"{identifier}.tar.gz",
    ]
    binary_name = binary.name

    manifest = {
        "product": "AgentFileOps",
        "package": "afo-cli",
        "version": version,
        "platform": platform_name(),
        "binary": f"bin/{binary_name}",
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
        "sha256": "see adjacent checksum files",
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

        create_zip(archives[0], staging)
        create_tar_gz(archives[1], staging)

    for archive in archives:
        checksum = sha256(archive)
        archive.with_name(f"{archive.name}.sha256").write_text(
            f"{checksum}  {archive.name}\n", encoding="utf-8"
        )
        print(f"created {archive}")
        print(f"sha256 {checksum}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
