from __future__ import annotations

import json
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]


def run_cli(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["cargo", "run", "-q", "-p", "agent-file-ops-cli", "--", *args],
        cwd=ROOT, text=True, capture_output=True, check=False,
    )


def parse_stdout(proc: subprocess.CompletedProcess[str]) -> dict:
    assert proc.returncode == 0, proc.stderr
    return json.loads(proc.stdout)


@pytest.mark.skip(reason="Vertical Slice 02 contract tests are not implemented yet")
def test_connection_validation():
    """VS02: connection identity, credential references, and host-key policy."""
    raise AssertionError("Implement connection validation before claiming VS02 conformance")


@pytest.mark.skip(reason="Vertical Slice 02 contract tests are not implemented yet")
def test_path_base_selection():
    """VS02: home, named alias, absolute, and root base modes."""
    raise AssertionError("Implement path-base selection before claiming VS02 conformance")


@pytest.mark.skip(reason="Vertical Slice 02 contract tests are not implemented yet")
def test_backend_strategy():
    """VS02: capability-aware SFTP and constrained shell strategy selection."""
    raise AssertionError("Implement backend strategy tests before claiming VS02 conformance")
