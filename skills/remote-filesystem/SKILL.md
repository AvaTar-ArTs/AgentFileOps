---
name: remote-filesystem
description: Safely inspect, transfer, organize, archive, synchronize, and manage files on AgentFS-connected remote systems. Use when the task is about remote files or deployments and structured AgentFS operations are preferable to arbitrary shell commands.
---

# Remote Filesystem Operator

Use AgentFS semantic filesystem capabilities instead of raw shell commands whenever the task can be represented as file operations.

## Core rule

> Give agents files, not a shell.

Do not request or construct arbitrary remote shell command strings for normal filesystem work.

## Operating sequence

1. Identify the intended connection and logical base.
2. Inspect aliases/capabilities before inventing paths.
3. Use `list`, `stat`, or `find` to establish current state.
4. Prefer named aliases for known locations; use `home` for account-relative navigation.
5. Use explicit `absolute` mode only when the user/workflow intentionally needs an absolute backend path.
6. Classify the requested change by risk.
7. Use dry-run/preflight for recursive, replacement, destructive, archive-extraction, or sync operations.
8. Execute only against the preflighted target set when approval is required.
9. Verify final size, checksum, stat, or directory state after mutation.
10. Preserve audit evidence and report what was actually verified.

## Risk behavior

- L0 read-only: proceed with bounded reads/discovery.
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
- If SFTP and shell namespaces differ, rely on AgentFS resolution rather than rewriting paths manually.

## Transfers

For large binary files, prefer streaming or scoped browser upload/download tickets instead of embedding binary content in tool-call JSON.

Verify transfer integrity when a checksum is available or practical.

## Archives

Inspect/preflight before extraction. Reject or surface traversal, absolute members, unsafe links, device files, dangerous collisions, and suspicious decompression behavior.

## Sync

Start with a dry-run manifest/diff. Deletion is disabled by default. Any sync that removes destination content is high-impact and must use the approved plan/fingerprint flow.

## Completion standard

Do not claim a remote operation succeeded merely because a write call returned. Verify the resulting remote state with a fresh read/stat/list/checksum as appropriate.