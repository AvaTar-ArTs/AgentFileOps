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


def parse_error(proc: subprocess.CompletedProcess[str]) -> dict:
    assert proc.returncode != 0
    return json.loads(proc.stderr)


def test_resolves_alias_across_sftp_and_shell_namespaces():
    proc = run_cli(
        "resolve-connection-path",
        "--connection",
        "prod",
        "--home",
        "/home/u1",
        "--sftp-home",
        ".",
        "--shell-home",
        "/home/u1",
        "--alias",
        "web=domains/example.com/public_html",
        "--base",
        "web",
        "--path",
        "releases/app.zip",
    )
    assert parse_stdout(proc) == {
        "connection": "prod",
        "logical_path": "web:releases/app.zip",
        "sftp_path": "domains/example.com/public_html/releases/app.zip",
        "shell_path": "/home/u1/domains/example.com/public_html/releases/app.zip",
        "base": "web",
        "follow_symlinks": False,
    }


def test_unknown_alias_fails_closed():
    proc = run_cli(
        "resolve-connection-path",
        "--connection",
        "prod",
        "--home",
        "/home/u1",
        "--sftp-home",
        ".",
        "--base",
        "missing",
        "--path",
        "file.txt",
    )
    assert parse_error(proc)["code"] == "unknown_alias"


def test_alias_relative_path_cannot_escape_base():
    proc = run_cli(
        "resolve-connection-path",
        "--connection",
        "prod",
        "--home",
        "/home/u1",
        "--sftp-home",
        ".",
        "--alias",
        "web=domains/example.com/public_html",
        "--base",
        "web",
        "--path",
        "../../secrets.txt",
    )
    assert parse_error(proc)["code"] == "path_escape"


def test_explicit_absolute_mode_preserves_absolute_path():
    proc = run_cli(
        "resolve-connection-path",
        "--connection",
        "prod",
        "--home",
        "/home/u1",
        "--sftp-home",
        "/home/u1",
        "--shell-home",
        "/home/u1",
        "--base",
        "absolute",
        "--path",
        "/var/www/app/releases/app.zip",
    )
    assert parse_stdout(proc) == {
        "connection": "prod",
        "logical_path": "absolute:/var/www/app/releases/app.zip",
        "sftp_path": "/var/www/app/releases/app.zip",
        "shell_path": "/var/www/app/releases/app.zip",
        "base": "absolute",
        "follow_symlinks": False,
    }


def test_copy_prefers_sftp_when_shell_is_unavailable():
    proc = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
    )
    assert parse_stdout(proc) == {"strategy": "sftp-stream", "accelerated": False}


def test_copy_uses_constrained_shell_acceleration_when_safe():
    proc = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
        "--shell",
        "--shell-path-safe",
        "--command",
        "cp",
    )
    assert parse_stdout(proc) == {"strategy": "shell-cp", "accelerated": True}


def test_copy_falls_back_when_shell_path_mapping_is_not_safe():
    proc = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
        "--shell",
        "--command",
        "cp",
    )
    assert parse_stdout(proc) == {"strategy": "sftp-stream", "accelerated": False}


def test_checksum_strategy_uses_sha256sum_only_when_safe_and_available():
    accelerated = run_cli(
        "select-backend-strategy",
        "--operation",
        "checksum",
        "--sftp",
        "--shell",
        "--shell-path-safe",
        "--command",
        "sha256sum",
    )
    assert parse_stdout(accelerated) == {
        "strategy": "shell-sha256sum",
        "accelerated": True,
    }

    fallback = run_cli(
        "select-backend-strategy",
        "--operation",
        "checksum",
        "--sftp",
    )
    assert parse_stdout(fallback) == {"strategy": "sftp-hash", "accelerated": False}


def test_missing_backend_capability_fails_closed():
    proc = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
    )
    assert parse_error(proc)["code"] == "capability_unavailable"
