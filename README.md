# AgentFileOps

> **Give agents files, not a shell.**

AgentFileOps is a provider- and language-agnostic remote filesystem operations layer for AI agents, coding assistants, automation systems, and human operators. It exposes structured remote file capabilities instead of arbitrary shell execution.

## What AgentFileOps is

AgentFileOps defines a protocol, schemas, safety model, capability model, conformance suite, and multiple implementation surfaces.

```text
AgentFileOps Protocol
      |
      +-- Rust core engine
      +-- TypeScript MCP / npm gateway
      +-- Go daemon
      +-- Python SDK
      +-- CLI (`afo`)
      +-- Agent Skills
      +-- provider presets
```

The protocol is canonical. Implementations are replaceable.

## Initial semantic operations

```text
connections
list
stat
find
read
write
transfer
manage
archive
sync
```

No public AgentFileOps interface should expose `exec(command: string)`.

## Safety model

AgentFileOps classifies operations by semantic risk:

- L0 read-only: list, stat, find, read, checksum
- L1 additive: mkdir, upload new file, copy to new path
- L2 replacement: overwrite, move, chmod, symlink creation
- L3 destructive: single delete
- L4 high impact: recursive delete, bulk delete, sync with deletion

High-risk mutations use preflight plans, target snapshots, expirations, and fingerprints before execution. Recursive operations do not follow symlinks by default.

## Ecosystem-driven development

AgentFileOps is developed using the AvaTar-ArTs agent ecosystem:

- **superAgents** provides orchestration, policy boundaries, catalog locking, and verification roles.
- **superSkills** provides curated procedural contracts such as brainstorming, TDD, verification, MCP development, catalog synchronization, and changelog discipline.
- **agent-skills** provides the broader specialist bench for architecture, security, testing, API design, DevOps, workflow orchestration, capability translation, and code review.

See `docs/ecosystem/AGENT_SKILL_REVIEW.md` and `manifests/source-lock.json`.

## Repository map

```text
protocol/      canonical schemas and examples
crates/        Rust implementation family
packages/      TypeScript / npm / MCP surfaces
cmd/           deployable daemons
sdk/           language SDKs
skills/        skills.sh / agent procedural guidance
agents/        AgentFileOps-specific review and orchestration roles
presets/       provider recipes, never credentials
manifests/     source locks, capability and review contracts
docs/          architecture, ecosystem, security, migration
tests/         shared conformance and integration fixtures
```

## Provider neutrality

Hostinger is a preset and migration source, not a product boundary. The same AgentFileOps protocol should work with generic SSH/SFTP servers, VPSes, NAS systems, cPanel/Plesk hosts, and future adapters.

## Search / discovery terms

AgentFileOps is designed around the remote-file operations people actually need from agents: **remote filesystem**, **SSH**, **SFTP**, **file transfer**, **remote server**, **sync**, **deployment**, **archives**, **artifact publishing**, **MCP**, and **AI agent automation**.

## Current status

**Foundation + first Rust conformance slice.** The product architecture is approved, the canonical path/risk contracts are implemented in Rust, and broader SSH/SFTP runtime work remains under active development. Production readiness must only be claimed after conformance, integration, and security verification evidence exists.

## Naming history

The project began as `hostinger-file-bridge`, was generalized briefly under the working name `AgentFS`, and was renamed to **AgentFileOps** after collision research found existing AgentFS projects. Historical ADR/spec filenames may retain `agentfs` where they document that evolution.

## Source architecture

The original product-design checkpoint is preserved at:

`docs/superpowers/specs/2026-08-18-agentfs-product-design.md`

New implementation and distribution identifiers use `AgentFileOps` / `agent-file-ops`.

## License

License selection is intentionally deferred until the product/repository licensing strategy is finalized.
