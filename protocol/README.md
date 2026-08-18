# AgentFileOps Protocol

The AgentFileOps protocol is the canonical behavioral contract for remote filesystem operations. Runtime languages, MCP tools, CLI commands, SDKs, skills, and provider presets must translate this contract without redefining it.

## Protocol invariants

1. Public operations are semantic filesystem operations, never arbitrary shell command execution.
2. Paths are logical `PathSpec` objects, not untyped backend-native strings.
3. Aliases are bookmarks, not permissions.
4. Absolute paths require explicit absolute mode.
5. Recursive operations default to `follow_symlinks=false`.
6. Mutations have semantic risk levels L0-L4.
7. Replacement/destructive/high-impact operations support preflight and target snapshots.
8. High-impact execution is tied to expiring plan fingerprints and revalidation.
9. Large binary transfers use streams/tickets rather than MCP JSON/base64.
10. Mutations emit redacted audit events.
11. Implementations may accelerate operations internally with shell capabilities, but shell acceleration must not change the protocol result or safety contract.
12. Active protocol metadata and implementation identifiers use AgentFileOps naming; AgentFS is historical only.

## Schemas

- `schema/connection.schema.json`
- `schema/path.schema.json`
- `schema/operation.schema.json`
- `schema/plan.schema.json`
- `schema/result.schema.json`
- `schema/audit.schema.json`

Protocol versioning is independent of individual package versions.

## Conformance

Shared fixtures under `tests/conformance/` are the executable interpretation of these schemas and invariants. When documentation, schema, and a runtime disagree, the discrepancy must be resolved explicitly rather than silently normalized by one implementation.