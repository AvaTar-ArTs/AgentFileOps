# Changelog

All notable AgentFileOps changes should be recorded here. Protocol changes, compatibility changes, safety changes, naming changes, discovery changes, and verification evidence are release-significant.

## Unreleased

### Added

- Approved provider- and language-agnostic AgentFileOps product architecture.
- Protocol-first repository boundary.
- Connection, path, operation, preflight-plan, result, and audit schemas for protocol 0.1 foundation.
- AgentFileOps agent roster adapted from superAgents and agent-skills.
- AgentFileOps skill catalog composed with superSkills process contracts.
- Primary `remote-file-operations` operator skill with remote filesystem, SSH/SFTP, file transfer, server management, deployment, and sync discovery language.
- `remote-deploy` procedural skill.
- `artifact-publisher` procedural skill.
- `remote-sync` procedural skill.
- `protocol-conformance` skill.
- `manifests/discovery.json` as the canonical product/distribution/search metadata source.
- `docs/distribution/DISCOVERY_SEO.md` for GitHub, npm, MCP, skills.sh, Rust, Python, Go, Docker, and launch-positioning guidance.
- Canonical npm package metadata template and keyword catalog.
- Rust workspace discovery metadata for MCP, SSH, SFTP, filesystem, and AI-agent discovery.
- Pinned ecosystem source lock.
- Cross-language workspace boundaries for Rust, TypeScript/npm/MCP, Go daemon/SDK, and Python SDK.
- Hostinger provider preset boundary.
- Shared conformance-suite contract.
- Security invariants for semantic filesystem operations and constrained shell acceleration.
- First Rust path-normalization and risk-classification conformance slice.

### Changed

- Renamed the public product from the temporary `AgentFS` working name to **AgentFileOps** after repository/name-collision research.
- Established **Remote File Operations for AI Agents** as the canonical category phrase.
- Established **Safe Remote Filesystem, SSH/SFTP, Sync & Deployment Operations for AI Agents** as the SEO subtitle.
- Expanded discovery vocabulary across MCP, remote filesystem, filesystem MCP, SSH, SFTP, file transfer, remote server, server file management, sync, deployment, DevOps, automation, artifact publishing, and secure filesystem intents.
- Migrated the primary operator skill path from `skills/remote-filesystem/` to `skills/remote-file-operations/`.
- Migrated active Rust crates to `agent-file-ops-*` names and the CLI binary to `afo`.
- Migrated active agent IDs and skill IDs to the `agentfileops.*` namespace.
- Migrated active skills, protocol docs, architecture docs, security docs, SDK docs, provider presets, npm planning, and validation output to AgentFileOps naming.
- Renamed the daemon track from `agentfsd` to `agent-file-opsd`.
- Retained `AgentFS` only where it documents historical naming or migration context.

### Status

Foundation plus first conformance slice. Task-oriented skills and discovery metadata exist, but no production-ready AgentFileOps SSH/SFTP runtime is claimed yet.
