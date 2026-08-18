---
name: remote-deploy
description: Safely deploy build artifacts to remote servers over AgentFileOps SSH/SFTP file operations. Use for remote deployment, release publishing, artifact transfer, archive activation, or web/server updates where preflight, checksum verification, approvals, and rollback-aware file operations are preferable to arbitrary shell commands.
---

# Remote Deploy

Use AgentFileOps to model deployment as a verified sequence of remote file operations rather than an unrestricted shell session.

## Runtime prerequisite

Use this skill only when the connected AgentFileOps runtime exposes the semantic operations needed by the deployment plan. If a required capability is unavailable, report the missing capability rather than replacing it with arbitrary `ssh exec` behavior.

## Workflow

1. Inspect connection capabilities, aliases, destination state, and available space where supported.
2. Identify the artifact and expected integrity metadata.
3. Transfer to a new/staging destination first when practical.
4. Verify size/checksum after transfer.
5. Inspect archives before extraction and preflight collisions.
6. Preflight any overwrite, move, activation, or deletion step.
7. Require the appropriate approval for replacement/destructive actions.
8. Activate through a defined semantic file operation.
9. Verify the final filesystem state and deployment-specific evidence available to the workflow.
10. Preserve audit evidence and rollback-relevant paths.

## Search intents covered

remote deploy, remote deployment, SSH deployment, SFTP deployment, AI agent deployment, MCP deployment, DevOps agent, release publishing, remote server update, artifact deployment.

## Safety rules

- Never turn a missing semantic capability into unrestricted shell execution.
- Do not overwrite production content before the replacement set is preflighted.
- Do not delete previous artifacts merely because a new upload succeeded.
- Do not claim deployment success from transfer success alone.
- Preserve checksums, plan ids/fingerprints, and verification evidence when available.
