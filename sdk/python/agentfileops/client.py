"""Small automation client for the verified AgentFileOps CLI contract.

This SDK intentionally exposes policy/conformance operations only. It does not
accept or execute arbitrary shell commands.
"""

from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from typing import Iterable, Mapping, Optional


class AgentFileOpsError(RuntimeError):
    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{code}: {message}")
        self.code = code
        self.message = message


@dataclass(frozen=True)
class Client:
    binary: str = "afo"

    def _run(self, *args: str) -> Mapping[str, object]:
        result = subprocess.run(
            [self.binary, *args],
            check=False,
            capture_output=True,
            text=True,
        )
        stream = result.stdout if result.returncode == 0 else result.stderr
        try:
            body = json.loads(stream)
        except json.JSONDecodeError as error:
            raise AgentFileOpsError(
                "invalid_cli_response", stream.strip() or str(error)
            ) from error
        if result.returncode:
            raise AgentFileOpsError(body.get("code", "cli_error"), body.get("message", "CLI failed"))
        return body

    def normalize_path(self, base: str, path: str, follow_symlinks: bool = False) -> Mapping[str, object]:
        return self._run(
            "normalize-path", "--base", base, "--path", path,
            *("--follow-symlinks",) if follow_symlinks else (),
        )

    def classify_risk(self, operation: str) -> Mapping[str, object]:
        return self._run("classify-risk", "--operation", operation)

    def select_backend_strategy(
        self,
        operation: str,
        *,
        sftp: bool = False,
        shell: bool = False,
        shell_path_safe: bool = False,
        commands: Optional[Iterable[str]] = None,
    ) -> Mapping[str, object]:
        args = ["select-backend-strategy", "--operation", operation]
        if sftp:
            args.append("--sftp")
        if shell:
            args.append("--shell")
        if shell_path_safe:
            args.append("--shell-path-safe")
        for command in commands or ():
            args.extend(("--command", command))
        return self._run(*args)
