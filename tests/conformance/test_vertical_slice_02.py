from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def run_cli(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "agent-file-ops-cli",
            "--",
            *args,
        ],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def parse_stdout(proc: subprocess.CompletedProcess[str]) -> dict:
    assert proc.returncode == 0, proc.stderr
    return json.loads(proc.stdout)


def test_connection_validation():
    """Vertical Slice 02: Connection specification validation."""
    # Placeholder for VS02 connection strategy tests
    # Tests SSH transport, known_hosts_ref, credential_ref, etc.
    pass


def test_path_base_selection():
    """Vertical Slice 02: Path base selection strategy."""
    # Tests home, absolute, and root base modes
    pass


def test_backend_strategy():
    """Vertical Slice 02: Backend-specific strategy."""
    # Tests SSH/SFTP backend capabilities and constraints
    pass
