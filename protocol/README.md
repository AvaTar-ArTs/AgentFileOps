# AgentFileOps Protocol

Canonical protocol definition, schemas, and reference implementations.

## Core Contracts

The AgentFileOps protocol defines:

- **Connection Model**: SSH/SFTP transport with credential and known-hosts references (no embedded secrets)
- **Path Semantics**: Normalized path resolution with explicit symlink follow/no-follow controls
- **Operation Taxonomy**: Semantic operations (list, stat, find, read, write, transfer, manage, archive, sync) with no arbitrary shell exposure
- **Risk Classification**: L0 (read-only) through L4 (high-impact destructive) for safety-first enforcement
- **Planning & Approval**: Preflight plans, target snapshots, expiration windows, and fingerprints for high-risk mutations
- **Audit Trail**: Immutable logging of all operations with actors, timestamps, and outcomes

## Schema Files

- `connection.schema.json` - Connection specification with transport reference
- `path.schema.json` - Path normalization and base selection
- `operation.schema.json` - Semantic operation definitions
- `plan.schema.json` - Preflight planning and approval workflows
- `result.schema.json` - Normalized operation results
- `audit.schema.json` - Audit trail schema
- `ssh-transport.schema.json` - SSH/SFTP transport configuration

## Principles

1. **No Arbitrary Shell**: Never expose `exec()`, `shell()`, or raw command execution
2. **Protocol First**: Implementations are replaceable; protocol is canonical
3. **Safety by Default**: High-risk operations require explicit plans and snapshots
4. **Credential Security**: Credentials referenced, never embedded
5. **Semantic Parity**: All surfaces (MCP, CLI, SDK, daemon) validate against shared conformance fixtures
