# Comparator Review: Agent-Safe Remote File Operations

**Date:** 2026-08-18

## Positioning

AgentFileOps should not compete with rclone on raw transfer performance or with Ansible on full infrastructure orchestration. Its useful boundary is a policy-controlled semantic gateway for agents: resolve a target, classify risk, produce a plan, execute only approved filesystem operations, and return normalized results and audit events.

## Comparable implementations

| Project | Implementation pattern | Useful precedent | Boundary AgentFileOps keeps |
|---|---|---|---|
| [MCP Filesystem Server](https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem) | TypeScript MCP tools with configured directories and client Roots | Explicit allowed roots and tool-scoped filesystem access | Remote SSH/SFTP, risk levels, plans, approvals, and audit records |
| [ssh-mcp](https://github.com/slepp/ssh-mcp) | Python MCP server around SSH, SCP, rsync, and remote sessions | Blocking dangerous SSH options and retaining operation transcripts | No public arbitrary `exec(command)` surface |
| [Ansible copy](https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/copy_module.html) | Python modules over SSH with metadata and safe-file-operation controls | Idempotent copy semantics, metadata handling, and backups | Agent-native semantic operations and preflight policy |
| [Ansible synchronize](https://docs.ansible.com/projects/ansible/latest/collections/ansible/posix/synchronize_module.html) | rsync-backed synchronization with delayed updates | Safer publication windows and explicit sync controls | Protocol-level risk classification and approval |
| [rclone](https://rclone.org/sftp/) | Go provider abstraction with SFTP, checksums, filters, concurrency, and sync | Transfer pipeline, retries, checksums, and bounded concurrency | Semantic intent and safety policy before transfer execution |

## Design decisions

1. Keep the Rust core protocol-first and deterministic. VS02 proves connection/path/backend semantics without network sockets.
2. Treat aliases as bookmarks, never as permissions. Root containment and path normalization remain mandatory.
3. Keep SFTP as the canonical safe backend. Shell acceleration is optional and allowed only with an advertised command and a proven safe path mapping.
4. Add MCP roots and tool annotations at the adapter boundary, not as a replacement for core path validation.
5. Add rclone-inspired checksums, retries, and concurrency only after bounded read/write semantics have integration evidence.
6. Add Ansible-inspired delayed updates, backups, and dry-run/preflight output for deployment and synchronization.
7. Preserve the explicit no-shell-command contract across CLI, MCP, SDK, and daemon surfaces.

## Known lessons from adjacent projects

Path containment needs independent runtime enforcement and tests. The MCP filesystem project documents allowed-directory boundaries, while its issue tracker also records path-validation and platform-specific edge cases. AgentFileOps therefore tests canonicalization, escape rejection, NUL rejection, and path length limits in the core rather than relying on descriptions or schema text alone.

## Scope boundary

This review does not claim that AgentFileOps has implemented live SSH/SFTP behavior. Vertical Slice 03 remains the required evidence boundary for host-key callbacks, authentication, SFTP packet operations, integration fixtures, and recovery behavior.
