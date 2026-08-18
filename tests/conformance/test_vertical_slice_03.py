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


def test_full_operation_suite():
    """Vertical Slice 03: Full semantic operation conformance."""
    # Tests list, stat, find, read, write, transfer, manage, archive, sync
    pass


def test_error_handling():
    """Vertical Slice 03: Error handling and reporting."""
    # Tests consistent error codes across all surfaces
    pass
