# AgentFileOps

> **Give agents files, not a shell.**
>
> Safe remote filesystem, SSH/SFTP, sync, and deployment operations for AI agents.

AgentFileOps is a protocol-first remote filesystem operations layer for AI agents, MCP clients, coding assistants, automation systems, and human operators. It provides structured file capabilities for remote servers without exposing arbitrary shell execution as the public interface.

## Project status

**Foundation and first Rust conformance slices are in progress.**

The repository currently contains:

- canonical protocol and schema foundations;
- Rust core and CLI workspace components;
- path, risk, discovery, and backend-strategy contracts;
- black-box conformance fixtures for Vertical Slices 01–03;
- provider-neutral SSH/SFTP transport planning;
- safety, audit, and source-lock manifests;
- ecosystem review documentation;
- design-system, diagram, dashboard, and GitHub Pages assets.

The project is not yet claiming production-ready SSH/SFTP runtime coverage. Runtime transport, deeper integration, security evidence, and broader conformance must be demonstrated by fresh verification results.

## Why AgentFileOps

Many remote-agent tools start with a shell and ask the agent to construct commands. AgentFileOps reverses that relationship:

    agent intent
        ↓
    semantic operation
        ↓
    path and capability resolution
        ↓
    risk classification and preflight
        ↓
    approved backend execution
        ↓
    verified result and audit event

The product promise is simple: give an agent structured file operations instead of an unrestricted remote shell.

## What the protocol defines

The protocol is the authoritative product contract. Implementations are replaceable adapters.

Initial semantic operation families:

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

The public interface must not expose an arbitrary exec(command) operation. Shell acceleration, where permitted internally, is constrained by capability detection, fixed strategies, validated arguments, and safety policy.

## Safety model

Operations are classified semantically:

| Level | Meaning | Examples | Default control |
|---|---|---|---|
| L0 | Read-only | list, stat, find, read, checksum | Automatic |
| L1 | Additive | mkdir, new upload, copy to a new path | Policy-dependent |
| L2 | Replacement | overwrite, move, chmod, symlink | Review recommended |
| L3 | Destructive | Single delete | Explicit approval |
| L4 | High impact | Recursive delete, bulk delete, sync with deletion | Staged approval |

High-risk mutations are designed to use preflight plans, resolved target snapshots, expiry windows, and fingerprints before execution. Recursive operations default to follow_symlinks=false. Credentials are represented by references, not embedded secrets.

Safety is enforced by the protocol, implementation, validators, and tests. Documentation alone is not a security boundary.

## Agent ecosystem operating model

AgentFileOps is developed as part of the AvaTar-ArTs agent ecosystem. The repositories have distinct responsibilities:

### superAgents — orchestration and governance

[AvaTar-ArTs/superAgents](https://github.com/AvaTar-ArTs/superAgents) supplies agent roles and governance patterns for:

- orchestration;
- verification ownership;
- policy boundaries;
- audit-oriented review;
- catalog locking;
- handoff and completion discipline.

### superSkills — curated procedural contracts

[ AvaTar-ArTs/superSkills ](https://github.com/AvaTar-ArTs/superSkills) supplies reviewed, reusable procedural contracts for:

- brainstorming and requirements shaping;
- test-driven development;
- systematic debugging;
- verification before completion;
- MCP and agent-surface development;
- catalog synchronization;
- changelog discipline.

A superSkill describes how work should be performed. It does not replace the AgentFileOps protocol or runtime.

### agent-skills — specialist capability bench

[ AvaTar-ArTs/agent-skills ](https://github.com/AvaTar-ArTs/agent-skills) supplies specialist perspectives for:

- system architecture;
- protocol and schema design;
- security engineering;
- testing and conformance;
- workflow orchestration;
- capability analysis;
- DevOps;
- code review.

AgentFileOps adapts relevant contracts and records their provenance. Imported material is metadata or an adapted contract; it is not executed blindly.

### How the workflow fits together

    superAgents
        ↓ assigns ownership, policy, and verification roles
    superSkills
        ↓ supplies the procedural method and quality gates
    agent-skills
        ↓ supplies specialist review lenses
    AgentFileOps protocol
        ↓ remains the canonical technical contract
    implementations and conformance suites
        ↓ produce evidence
    release or completion decision

The repository applies the same principle as the product itself: structured contracts first, execution second, evidence before claims.

## Provenance and source locking

External ecosystem sources are pinned in [manifests/source-lock.json](manifests/source-lock.json):

- [superAgents pinned source](https://github.com/AvaTar-ArTs/superAgents/tree/9d56534c686242cc82aa4033bc574c14fc6856fe)
- [superSkills pinned source](https://github.com/AvaTar-ArTs/superSkills/tree/2cb68b62354efb3acc36140de5282f7b8bb119d3)
- [agent-skills pinned source](https://github.com/AvaTar-ArTs/agent-skills/tree/af71bdf644030f447c2bcd69575ea19e92b964c7)

The source lock records the exact commit, intended use, and policy:

- source updates require review;
- remote catalog content is not executed by synchronization;
- imported contracts remain adapted and bounded;
- AgentFileOps protocol definitions remain authoritative.

The ecosystem review is documented in [docs/ecosystem/AGENT_SKILL_REVIEW.md](docs/ecosystem/AGENT_SKILL_REVIEW.md).

## Repository map

    protocol/       canonical schemas, examples, and protocol documentation
    crates/         Rust implementation family
    packages/       TypeScript, npm, and MCP surfaces
    cmd/            deployable daemons
    sdk/            language SDKs
    skills/         agent procedural guidance
    agents/         AgentFileOps-specific roles, when present
    presets/        provider recipes; never credentials
    manifests/      discovery, capability, source-lock, and review contracts
    docs/           architecture, ecosystem, security, verification, and design
    tests/          conformance and integration fixtures
    .github/        CI and Pages workflows

Some directories represent planned expansion surfaces. Their existence in the map is not a claim that every adapter or package has been released.

## Current implementation surfaces

| Surface | Role | Status |
|---|---|---|
| Rust core | canonical path, risk, and filesystem logic | Foundation in progress |
| Rust CLI | executable conformance surface | Active development |
| TypeScript/MCP | agent gateway and npm surface | Planned/expanding |
| Python SDK | automation and migration surface | Planned/expanding |
| Go daemon | deployable service surface | Planned |
| Agent skills | operator guidance and procedural usage | Present and expanding |
| SSH/SFTP transport | remote filesystem backend | Design/conformance work in progress |

The machine-readable discovery vocabulary is maintained in [manifests/discovery.json](manifests/discovery.json).

## Quick start for contributors

Requirements:

- Rust toolchain with Cargo;
- Python 3 with pytest;
- a checkout of this repository.

Run the foundation validator:

    python scripts/validate_foundation.py

Run the Rust workspace tests:

    cargo test --workspace --all-targets

Run the available conformance slices:

    python -m pytest tests/conformance/test_vertical_slice_01.py -v
    python -m pytest tests/conformance/test_vertical_slice_02.py -v
    python -m pytest tests/conformance/test_vertical_slice_03.py -v

Do not describe a slice as complete from source inspection alone. Use fresh command or CI evidence and record the result in the relevant verification document.

Vertical Slice plans:

- [Vertical Slice 01](docs/superpowers/plans/2026-08-18-agentfs-vertical-slice-01.md)
- [Vertical Slice 02](docs/superpowers/plans/2026-08-18-agentfileops-vertical-slice-02.md)
- [Vertical Slice 03](docs/superpowers/plans/2026-08-18-agentfileops-vertical-slice-03.md)

## Documentation and visual system

- [Architecture](ARCHITECTURE.md)
- [Security policy](SECURITY.md)
- [Foundation audit](AUDIT_REPORT.md)
- [Design system](docs/DESIGN_SYSTEM.md)
- [Architecture SVG](docs/design/architecture.svg)
- [Risk-level SVG](docs/design/risk-levels.svg)
- [Data-flow SVG](docs/design/data-flow.svg)
- [Operations dashboard](docs/site/index.html)
- [Product design checkpoint](docs/superpowers/specs/2026-08-18-agentfs-product-design.md)

The dashboard and diagrams communicate project structure and safety concepts. Protocol schemas, security policy, implementation behavior, and conformance tests remain authoritative.

## Provider neutrality

Hostinger is a migration and research source, not a product boundary. The protocol is intended for generic SSH/SFTP servers, VPS providers, NAS systems, cPanel/Plesk hosting, and future backends.

Provider presets may contain path conventions, capability notes, deployment guidance, and known caveats. They must never contain credentials.

## Planned distribution

The following names are discovery and packaging targets, not a claim that all packages are currently published:

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

## Naming history

The project began as hostinger-file-bridge, was generalized under the working name AgentFS, and was renamed to AgentFileOps after collision research. Historical ADR and specification filenames may retain agentfs while active implementation and distribution identifiers use AgentFileOps and agent-file-ops.

## License

License selection is intentionally deferred while the repository and product licensing strategy are finalized. Do not treat the repository as licensed for redistribution until a license file and policy are published.

## Contributing principles

Before changing protocol or implementation behavior:

1. Read the relevant protocol and architecture documents.
2. Identify the applicable superAgent ownership and verification role.
3. Select the relevant superSkill procedural contract.
4. Consult the appropriate agent-skills specialist perspective.
5. Write or update tests before claiming behavior.
6. Run the narrowest relevant validator, then the broader suite.
7. Record actual evidence in the changelog or verification record.
8. Keep credentials and secret material out of source, fixtures, logs, manifests, and commits.

When a skill says verification is required, “looks correct” is not verification. A green command or CI result is the evidence.

## Design language

The visual identity is documented in [docs/DESIGN_SYSTEM.md](docs/DESIGN_SYSTEM.md). The core idea is visible safety: make operation semantics, risk, approval, target, and result understandable before execution.

---

**AgentFileOps**  
Safe remote filesystem operations for AI agents and MCP clients.  
**Give agents files, not a shell.**
