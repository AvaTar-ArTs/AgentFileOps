---
name: remote-sync
description: Plan and safely synchronize local or remote directory trees with AgentFileOps using manifest diffs, SSH/SFTP transfer, deletion previews, integrity checks, and high-impact approvals. Use for remote sync, server sync, backup/migration, deployment mirroring, or filesystem synchronization without arbitrary shell execution.
---

# Remote Sync

Use AgentFileOps synchronization as a planned filesystem diff, not a blind recursive copy.

## Runtime prerequisite

Use this skill only when the connected AgentFileOps runtime exposes the required listing/stat/transfer/sync semantics. Shell-accelerated rsync may be used internally only when AgentFileOps has detected and constrained that backend strategy.

## Workflow

1. Resolve source and destination connection/base/path identities.
2. Build bounded source and destination manifests.
3. Classify unchanged, create, update, conflict, and delete candidates.
4. Present a dry-run summary with item counts, bytes, conflicts, symlinks, and selected strategy.
5. Keep deletion disabled unless explicitly requested.
6. Treat sync-with-delete as high-impact and require plan/fingerprint approval and target revalidation.
7. Transfer changed/new content using the safest supported backend.
8. Verify checksums or metadata according to the selected sync policy.
9. Re-list/re-stat the destination and report drift or partial completion.

## Search intents covered

remote sync, filesystem sync, server sync, SSH sync, SFTP sync, AI agent sync, MCP sync, backup automation, remote migration, deployment mirroring, rsync agent.

## Safety rules

- Never infer deletion from absence without an approved sync-delete policy.
- Do not follow symlinks recursively by default.
- Do not hide conflicts by overwriting both sides into apparent parity.
- If the target changes after preflight, invalidate the destructive plan.
- Report the chosen strategy, including whether rsync acceleration or portable SFTP diffing was used.
