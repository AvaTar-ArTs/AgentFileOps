# AgentFileOps

> **Give agents files, not a shell.**
>
> **Safe Remote Filesystem, SSH/SFTP, Sync & Deployment Operations for AI Agents**

AgentFileOps is a safety-first, provider- and language-agnostic **remote filesystem operations layer for AI agents and MCP clients**. It gives coding assistants, automation systems, and human operators structured tools for **SSH/SFTP file transfer, remote server file management, synchronization, archives, deployment, artifact publishing, checksums, approvals, and audit trails** without exposing arbitrary shell execution.

**Category phrase:** Remote File Operations for AI Agents.

## Why AgentFileOps

Many remote-agent tools begin with a shell and then ask the agent to construct commands. AgentFileOps takes the opposite approach: model the operation first, classify its risk, resolve paths safely, execute through the best available filesystem backend, and verify the resulting state.

```text
AI agent / MCP client / operator
              |
       AgentFileOps Protocol
              |
     semantic file operations
              |
  SSH / SFTP / VPS / NAS / server
```

The product is deliberately broader than a basic filesystem wrapper. `FileOps` includes transfer planning, sync, archives, deployment, checksums, approvals, audit trails, remote operations, and future backend adapters.

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

## Example operator experience

The planned CLI identity is deliberately compact:

```bash
afo ls prod:/var/www
afo sync ./dist prod:web/releases
afo deploy ./build.zip production:web
```

Planned npm bootstrap:

```bash
npx @avatar-arts/agent-file-ops
```

These examples define product direction, not a claim that every command/package is published today.

## Distribution namespace

```text
GitHub
AvaTar-ArTs/AgentFileOps

npm
@avatar-arts/agent-file-ops
@avatar-arts/agent-file-ops-mcp
@avatar-arts/agent-file-ops-client
@avatar-arts/agent-file-ops-schema

Rust
agent-file-ops-core
agent-file-ops-cli

Python
agent-file-ops

Go
agent-file-ops-go

Docker
ghcr.io/avatar-arts/agent-file-ops

skills.sh
remote-file-operations
safe-remote-filesystem
ssh-sftp-file-management
remote-deploy
artifact-publisher
remote-sync
server-file-management
```

The canonical machine-readable naming and keyword map lives in `manifests/discovery.json`.

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
manifests/     source locks, discovery, capability and review contracts
docs/          architecture, ecosystem, security, migration, distribution
tests/         shared conformance and integration fixtures
```

## Provider neutrality

Hostinger is a preset and migration source, not a product boundary. The same AgentFileOps protocol should work with generic SSH/SFTP servers, VPSes, NAS systems, cPanel/Plesk hosts, and future adapters.

## Search / discovery vocabulary

AgentFileOps deliberately targets the vocabulary people use when looking for this capability:

**AI agents · MCP · Model Context Protocol · remote filesystem · filesystem MCP · SSH · SFTP · file transfer · remote file operations · remote server · server file management · sync · remote sync · deployment · remote deploy · DevOps · artifact publishing · archives · checksums · automation · secure filesystem · agent tools · VPS · NAS**

The brand stays memorable while repository descriptions, package metadata, MCP metadata, skills, docs, and release pages carry the descriptive search language.

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
