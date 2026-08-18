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


def test_normalize_home_relative_path():
    proc = run_cli(
        "normalize-path",
        "--base",
        "home",
        "--path",
        "domains/./avatararts.org/assets/../releases",
    )
    assert parse_stdout(proc) == {
        "base": "home",
        "path": "domains/avatararts.org/releases",
        "follow_symlinks": False,
    }


def test_relative_path_cannot_escape_selected_base():
    proc = run_cli(
        "normalize-path",
        "--base",
        "home",
        "--path",
        "../../etc/passwd",
    )
    assert proc.returncode != 0
    error = json.loads(proc.stderr)
    assert error["code"] == "path_escape"


def test_absolute_mode_must_be_explicit_and_absolute():
    proc = run_cli(
        "normalize-path",
        "--base",
        "absolute",
        "--path",
        "home/user/file.txt",
    )
    assert proc.returncode != 0
    error = json.loads(proc.stderr)
    assert error["code"] == "absolute_path_required"


def test_absolute_path_is_normalized_without_losing_root():
    proc = run_cli(
        "normalize-path",
        "--base",
        "absolute",
        "--path",
        "/home/user/sites/./app/../public",
    )
    assert parse_stdout(proc) == {
        "base": "absolute",
        "path": "/home/user/sites/public",
        "follow_symlinks": False,
    }


def test_risk_classification_matches_agent_file_ops_policy():
    cases = {
        "list": "level_0",
        "checksum": "level_0",
        "mkdir": "level_1",
        "copy-new": "level_1",
        "overwrite": "level_2",
        "symlink": "level_2",
        "delete": "level_3",
        "recursive-delete": "level_4",
        "sync-delete": "level_4",
    }
    for operation, expected in cases.items():
        proc = run_cli("classify-risk", "--operation", operation)
        body = parse_stdout(proc)
        assert body == {"operation": operation, "risk": expected}


def test_unknown_operation_fails_closed():
    proc = run_cli("classify-risk", "--operation", "shell-anything")
    assert proc.returncode != 0
    error = json.loads(proc.stderr)
    assert error["code"] == "unknown_operation"


def test_overlong_path_fails_closed():
    proc = run_cli("normalize-path", "--base", "home", "--path", "a" * 4097)
    assert proc.returncode != 0
    error = json.loads(proc.stderr)
    assert error["code"] == "invalid_path"
