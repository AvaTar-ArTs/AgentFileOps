# AgentFileOps

<p align="center">
  <strong>Give agents files, not a shell.</strong><br>
  Safe remote filesystem operations for AI agents and MCP clients
</p>

<p align="center">
  <a href="https://github.com/AvaTar-ArTs/AgentFileOps/actions/workflows/foundation.yml"><img src="https://github.com/AvaTar-ArTs/AgentFileOps/actions/workflows/foundation.yml/badge.svg?branch=main" alt="Foundation CI"></a>
  <a href="https://github.com/AvaTar-ArTs/AgentFileOps/actions/workflows/pages.yml"><img src="https://github.com/AvaTar-ArTs/AgentFileOps/actions/workflows/pages.yml/badge.svg?branch=main" alt="Pages deployment"></a>
  <a href="https://github.com/AvaTar-ArTs/AgentFileOps/blob/main/SECURITY.md"><img src="https://img.shields.io/badge/security-policy-0F1E37" alt="Security policy"></a>
  <a href="https://github.com/AvaTar-ArTs/AgentFileOps"><img src="https://img.shields.io/badge/status-foundation%20in%20progress-00C8FF" alt="Foundation in progress"></a>
</p>

> [!WARNING]
> AgentFileOps is under active development. The protocol, safety model, and first Rust conformance slices are being built now. Do not treat planned package names, CLI examples, or future SSH/SFTP runtime work as released production features.

## What is AgentFileOps?

AgentFileOps is a protocol-first remote filesystem layer for:

- AI agents and MCP clients;
- coding assistants and automation systems;
- deployment and artifact-publishing workflows;
- operators managing VPS, NAS, hosting, and SSH/SFTP targets.

It provides structured filesystem capabilities—rather than an unrestricted shell—for planning, classifying, executing, and verifying remote file operations.

## Why it exists

A shell asks an agent to invent commands. AgentFileOps asks an agent to declare intent:

    intent
      ↓
    semantic operation
      ↓
    path and capability resolution
      ↓
    risk and approval policy
      ↓
    backend execution
      ↓
    verified result and audit event

The protocol remains the canonical contract. Rust, TypeScript/MCP, Go, Python, CLI, and skill surfaces are replaceable implementations or adapters.

## At a glance

| Area | Current direction |
|---|---|
| Protocol | Canonical schemas for connections, paths, operations, plans, results, and audit events |
| Operations | connections, list, stat, find, read, write, transfer, manage, archive, sync |
| Transport | SSH/SFTP baseline; provider-neutral connection model |
| Safety | L0–L4 semantic risk levels, preflight plans, fingerprints, approvals |
| Security | Credential references, strict host-key policy, bounded reads, no public arbitrary exec |
| Verification | Foundation validator, Rust workspace tests, Vertical Slices 01–03 |
| Distribution | Rust core/CLI first; other SDK, MCP, daemon, and package surfaces are staged |

## Safety model

| Level | Meaning | Examples | Control |
|---|---|---|---|
| L0 | Read-only | list, stat, find, read, checksum | Automatic |
| L1 | Additive | mkdir, new upload, copy to a new path | Policy-dependent |
| L2 | Replacement | overwrite, move, chmod, symlink | Review recommended |
| L3 | Destructive | single delete | Explicit approval |
| L4 | High impact | recursive delete, bulk delete, sync with deletion | Staged approval |

Safety rules:

- no public exec(command) interface;
- credentials are references, never embedded secrets;
- recursive operations default to follow_symlinks=false;
- high-risk mutations require a plan, target resolution, and revalidation;
- unknown or mismatched SSH host keys fail closed;
- behavior must be proven by tests or CI evidence, not documentation alone.

See [SECURITY.md](SECURITY.md) for the security policy and [ARCHITECTURE.md](ARCHITECTURE.md) for the system invariants.

## Current capabilities

### Foundation implemented

- Rust workspace and core path/risk contracts;
- discovery and naming manifest;
- source-lock and ecosystem provenance manifest;
- foundation validation script;
- protocol documentation and architecture records;
- conformance fixture structure;
- security and audit design;
- visual documentation and Pages dashboard.

### In progress

- connection and backend-strategy conformance;
- real SSH/SFTP transport;
- strict host-key verification implementation;
- bounded remote reads and additive writes;
- ephemeral OpenSSH/SFTP integration fixtures;
- MCP and SDK adapter surfaces.

### Not released yet

- published npm, Python, Go, or Docker packages;
- a stable public CLI distribution;
- destructive delete, archive, and sync runtime behavior;
- production-readiness certification.

## Quick start

### Inspect the repository

    git clone https://github.com/AvaTar-ArTs/AgentFileOps.git
    cd AgentFileOps

### Run foundation validation

    python scripts/validate_foundation.py

### Run Rust tests

    cargo test --workspace --all-targets

### Run conformance tests

    python -m pytest tests/conformance/test_vertical_slice_01.py -v
    python -m pytest tests/conformance/test_vertical_slice_02.py -v
    python -m pytest tests/conformance/test_vertical_slice_03.py -v

The commands above are verification entry points for the current repository. They do not imply that every planned runtime surface is complete.

## Repository layout

    protocol/       canonical schemas and examples
    crates/         Rust implementation family
    packages/       TypeScript, npm, and MCP surfaces
    cmd/            deployable daemons
    sdk/            language SDKs
    skills/         agent procedural guidance
    presets/        provider recipes; never credentials
    manifests/      discovery, capability, and source-lock contracts
    docs/           architecture, ecosystem, verification, and design
    tests/          conformance and integration fixtures
    .github/        CI and Pages workflows

## Documentation

For a complete evidence-based tree and ownership map, see [Repository Index](docs/REPOSITORY_INDEX.md).

| Need | Start here |
|---|---|
| Understand the architecture | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Review security boundaries | [SECURITY.md](SECURITY.md) |
| Read the foundation audit | [AUDIT_REPORT.md](AUDIT_REPORT.md) |
| Explore protocol contracts | [protocol/README.md](protocol/README.md) |
| Review discovery metadata | [manifests/discovery.json](manifests/discovery.json) |
| Review source provenance | [manifests/source-lock.json](manifests/source-lock.json) |
| Review ecosystem roles | [Agent Skill Review](docs/ecosystem/AGENT_SKILL_REVIEW.md) |
| Review the design language | [Design System](docs/DESIGN_SYSTEM.md) |
| Study end-to-end operation narratives | [Examples](docs/examples/README.md) |
| Open the dashboard | [AgentFileOps Dashboard](docs/site/index.html) |

Conformance plans:

- [Vertical Slice 01](docs/superpowers/plans/2026-08-18-agentfs-vertical-slice-01.md)
- [Vertical Slice 02](docs/superpowers/plans/2026-08-18-agentfileops-vertical-slice-02.md)
- [Vertical Slice 03](docs/superpowers/plans/2026-08-18-agentfileops-vertical-slice-03.md)

## How AgentFileOps is built

AgentFileOps uses the AvaTar-ArTs agent ecosystem as a reviewed development system:

- [superAgents](https://github.com/AvaTar-ArTs/superAgents) provides orchestration, ownership, policy boundaries, and verification roles.
- [superSkills](https://github.com/AvaTar-ArTs/superSkills) provides procedural contracts for brainstorming, TDD, debugging, verification, MCP development, and changelog discipline.
- [agent-skills](https://github.com/AvaTar-ArTs/agent-skills) provides specialist perspectives for architecture, security, testing, DevOps, capability analysis, and code review.
- [manifests/source-lock.json](manifests/source-lock.json) pins the reviewed source commits and records what each source is allowed to contribute.

The working rule is:

    contract first → specialist review → implementation → verification evidence

Imported ecosystem material is adapted and bounded. The AgentFileOps protocol remains authoritative.

## Provider neutrality

Hostinger is a migration and research source, not a product boundary. The same protocol is intended for generic SSH/SFTP servers, VPS providers, NAS systems, cPanel/Plesk hosting, and future backends.

Provider presets may contain path conventions, capability notes, deployment guidance, and known caveats. They must never contain credentials.

## Contributing

Before changing protocol or implementation behavior:

1. Read the relevant protocol and architecture documents.
2. Identify the applicable verification and review role.
3. Select the relevant procedural skill.
4. Write or update tests before claiming behavior.
5. Run the narrowest relevant validator, then the broader suite.
6. Record actual verification evidence.
7. Keep credentials and secret material out of source, fixtures, logs, and commits.

Start with [AUDIT_REPORT.md](AUDIT_REPORT.md), [SECURITY.md](SECURITY.md), and the relevant conformance plan.

## License

AgentFileOps is licensed under the MIT License. See [LICENSE](LICENSE).

## Naming history

The project began as hostinger-file-bridge, was generalized under the working name AgentFS, and was renamed to AgentFileOps after collision research. Historical specification filenames may retain agentfs while active implementation and distribution identifiers use AgentFileOps and agent-file-ops.

---

<p align="center">
  <strong>AgentFileOps</strong><br>
  Safe remote filesystem operations for AI agents and MCP clients
</p>
