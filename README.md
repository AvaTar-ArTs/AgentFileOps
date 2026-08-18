# AgentFS

> **Give agents files, not a shell.**

AgentFS is a provider- and language-agnostic remote filesystem control plane for AI agents, coding assistants, automation systems, and human operators. It exposes structured filesystem capabilities instead of arbitrary shell execution.

## What AgentFS is

AgentFS defines a protocol, schemas, safety model, capability model, conformance suite, and multiple implementation surfaces.

```text
AgentFS Protocol
      |
      +-- Rust core engine
      +-- TypeScript MCP / npm gateway
      +-- Go daemon
      +-- Python SDK
      +-- CLI
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

No public AgentFS interface should expose `exec(command: string)`.

## Safety model

AgentFS classifies operations by semantic risk:

- L0 read-only: list, stat, find, read, checksum
- L1 additive: mkdir, upload new file, copy to new path
- L2 replacement: overwrite, move, chmod, symlink creation
- L3 destructive: single delete
- L4 high impact: recursive delete, bulk delete, sync with deletion

High-risk mutations use preflight plans, target snapshots, expirations, and fingerprints before execution. Recursive operations do not follow symlinks by default.

## Ecosystem-driven development

AgentFS is developed using the AvaTar-ArTs agent ecosystem:

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
agents/        AgentFS-specific review and orchestration roles
presets/       provider recipes, never credentials
manifests/     source locks, capability and review contracts
docs/          architecture, ecosystem, security, migration
tests/         shared conformance and integration fixtures
```

## Provider neutrality

Hostinger is a preset and migration source, not a product boundary. The same AgentFS protocol should work with generic SSH/SFTP servers, VPSes, NAS systems, cPanel/Plesk hosts, and future adapters.

## Current status

**Foundation / protocol design.** The product architecture is approved. Runtime implementations are not yet production-ready and must not be represented as complete until conformance and verification evidence exist.

## Source architecture

The product design is in:

`docs/superpowers/specs/2026-08-18-agentfs-product-design.md`

## License

License selection is intentionally deferred until the product/repository licensing strategy is finalized.