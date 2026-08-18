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


@pytest.mark.skip(reason="Vertical Slice 03 contract tests are not implemented yet")
def test_full_operation_suite():
    """VS03: semantic list, stat, find, read, write, transfer, archive, and sync."""
    raise AssertionError("Implement operation-suite tests before claiming VS03 conformance")


@pytest.mark.skip(reason="Vertical Slice 03 contract tests are not implemented yet")
def test_error_handling():
    """VS03: normalized errors and fail-closed reporting across surfaces."""
    raise AssertionError("Implement error-handling tests before claiming VS03 conformance")
