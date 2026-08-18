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


def test_connection_validation():
    """VS02: resolve a named connection and preserve both path namespaces."""
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

    missing = run_cli(
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
    assert missing.returncode != 0
    assert json.loads(missing.stderr)["code"] == "unknown_alias"


def test_path_base_selection():
    """VS02: aliases remain bounded and absolute mode remains explicit."""
    escaped = run_cli(
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
        "../../secret",
    )
    assert escaped.returncode != 0
    assert json.loads(escaped.stderr)["code"] == "path_escape"

    absolute = run_cli(
        "resolve-connection-path",
        "--connection",
        "prod",
        "--home",
        "/home/u1",
        "--sftp-home",
        "/srv/sftp",
        "--shell-home",
        "/home/u1",
        "--base",
        "absolute",
        "--path",
        "/var/www/./app/../public",
        "--follow-symlinks",
    )
    assert parse_stdout(absolute) == {
        "connection": "prod",
        "logical_path": "absolute:/var/www/public",
        "sftp_path": "/var/www/public",
        "shell_path": "/var/www/public",
        "base": "absolute",
        "follow_symlinks": True,
    }


def test_backend_strategy():
    """VS02: SFTP is the safe baseline; shell is conditional acceleration."""
    baseline = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
    )
    assert parse_stdout(baseline) == {
        "strategy": "sftp-stream",
        "accelerated": False,
    }

    accelerated = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
        "--shell",
        "--shell-path-safe",
        "--command",
        "cp",
    )
    assert parse_stdout(accelerated) == {
        "strategy": "shell-cp",
        "accelerated": True,
    }

    unsafe = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
        "--sftp",
        "--shell",
        "--command",
        "cp",
    )
    assert parse_stdout(unsafe) == {
        "strategy": "sftp-stream",
        "accelerated": False,
    }

    checksum = run_cli(
        "select-backend-strategy",
        "--operation",
        "checksum",
        "--sftp",
        "--shell",
        "--shell-path-safe",
        "--command",
        "sha256sum",
    )
    assert parse_stdout(checksum) == {
        "strategy": "shell-sha256sum",
        "accelerated": True,
    }

    unavailable = run_cli(
        "select-backend-strategy",
        "--operation",
        "copy",
    )
    assert unavailable.returncode != 0
    assert json.loads(unavailable.stderr)["code"] == "capability_unavailable"
