# Changelog

All notable AgentFileOps changes should be recorded here. Protocol changes, compatibility changes, safety changes, naming changes, and verification evidence are release-significant.

## Unreleased

### Added

- Approved provider- and language-agnostic AgentFileOps product architecture.
- Protocol-first repository boundary.
- Connection, path, operation, preflight-plan, result, and audit schemas for protocol 0.1 foundation.
- AgentFileOps agent roster adapted from superAgents and agent-skills.
- AgentFileOps skill catalog composed with superSkills process contracts.
- `remote-filesystem` operator skill.
- `protocol-conformance` skill.
- Pinned ecosystem source lock.
- Cross-language workspace boundaries for Rust, TypeScript/npm/MCP, Go daemon/SDK, and Python SDK.
- Hostinger provider preset boundary.
- Shared conformance-suite contract.
- Security invariants for semantic filesystem operations and constrained shell acceleration.
- First Rust path-normalization and risk-classification conformance slice.

### Changed

- Renamed the public product from the temporary `AgentFS` working name to **AgentFileOps** after repository/name-collision research.
- Migrated active Rust crates to `agent-file-ops-*` names and the CLI binary to `afo`.
- Migrated active agent IDs and skill IDs to the `agentfileops.*` namespace.
- Migrated active skills, protocol docs, architecture docs, security docs, SDK docs, provider presets, npm planning, and validation output to AgentFileOps naming.
- Renamed the daemon track from `agentfsd` to `agent-file-opsd`.
- Retained `AgentFS` only where it documents historical naming or migration context.

### Status

Foundation plus first conformance slice. No production-ready AgentFileOps SSH/SFTP runtime is claimed yet.