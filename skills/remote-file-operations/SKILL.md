---
name: remote-file-operations
description: Safely inspect, transfer, organize, archive, synchronize, deploy, and manage remote server files over SSH/SFTP with AgentFileOps. Use for remote filesystem, file transfer, server file management, remote sync, artifact publishing, or deployment tasks where structured MCP/agent operations are safer than arbitrary shell commands.
---

# Remote File Operations

Use AgentFileOps semantic remote filesystem capabilities instead of raw shell commands whenever the task can be represented as file operations.

## Discoverability / task aliases

This skill covers tasks commonly described as:

- remote file operations;
- safe remote filesystem;
- SSH/SFTP file management;
- remote server file management;
- filesystem MCP operations;
- remote file transfer;
- remote sync;
- remote deploy;
- artifact publishing;
- server file automation.

## Core rule

> Give agents files, not a shell.

Do not request or construct arbitrary remote shell command strings for normal filesystem work.

## Operating sequence

1. Identify the intended connection and logical base.
2. Inspect aliases and capabilities before inventing paths.
3. Use `list`, `stat`, or `find` to establish current state.
4. Prefer named aliases for known locations; use `home` for account-relative navigation.
5. Use explicit `absolute` mode only when the user or workflow intentionally needs an absolute backend path.
6. Classify the requested change by risk.
7. Use dry-run or preflight for recursive, replacement, destructive, archive-extraction, deployment, or sync operations.
8. Execute only against the preflighted target set when approval is required.
9. Verify final size, checksum, stat, or directory state after mutation.
10. Preserve audit evidence and report what was actually verified.

## Risk behavior

- L0 read-only: proceed with bounded reads and discovery.
- L1 additive: create new content without replacing existing content.
- L2 replacement/mutation: require preflight when existing content can be affected.
- L3 destructive: exact target must be explicit.
- L4 high-impact: require machine-readable plan, counts/bytes, fingerprint, target revalidation, and approval.

Never silently escalate L1 into overwrite when a destination appears during execution.

## Path behavior

- Aliases are bookmarks, not permission boundaries.
- Never use `..` as a substitute for selecting a broader base.
- Absolute paths require explicit absolute mode.
- Recursive traversal defaults to not following symlinks.
- If SFTP and shell namespaces differ, rely on AgentFileOps resolution rather than rewriting paths manually.

## SSH/SFTP and remote servers

SSH/SFTP is a transport/backend capability, not permission to expose unrestricted shell execution. Prefer structured AgentFileOps operations for VPS, NAS, hosted web servers, cPanel/Plesk environments, and generic SSH/SFTP systems.

## Transfers

For large binary files, prefer streaming or scoped browser upload/download tickets instead of embedding binary content in tool-call JSON. Verify transfer integrity when a checksum is available or practical.

## Archives

Inspect and preflight before extraction. Reject or surface traversal, absolute members, unsafe links, device files, dangerous collisions, and suspicious decompression behavior.

## Sync

Start with a dry-run manifest/diff. Deletion is disabled by default. Any sync that removes destination content is high-impact and must use the approved plan/fingerprint flow.

## Deployment and artifact publishing

Treat deployment as a composed file operation: inspect target, transfer artifact, verify integrity, preflight replacement/extraction, activate only through a defined semantic operation, then verify resulting remote state.

## Completion standard

Do not claim a remote file operation succeeded merely because a write call returned. Verify the resulting remote state with a fresh read, stat, list, checksum, or deployment-specific health evidence as appropriate.
