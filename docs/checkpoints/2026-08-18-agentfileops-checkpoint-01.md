# AgentFileOps Checkpoint 01

**Captured:** 2026-08-18 09:03 -04:00  
**Repository:** `AvaTar-ArTs/AgentFileOps`  
**Canonical branch:** `main`  
**Main commit at capture:** `5f516d2d29c4a004742bfcecd460088317113269`  
**Current verification PR:** `#2 Verify discovery metadata and connection strategy slice`  
**Verification branch:** `verify/discovery-and-slice-02`

## Executive state

AgentFileOps has evolved from the original Hostinger-specific upload bridge into a provider- and language-agnostic remote file-operations product for AI agents and MCP clients.

The canonical positioning is:

> **AgentFileOps**  
> **Give agents files, not a shell.**  
> **Remote File Operations for AI Agents**

The repository is no longer organized around one Hostinger directory, one Python implementation, or one MCP transport. The product is protocol-first and treats Rust, TypeScript/MCP, Go, Python, CLI, skills, daemons, and provider presets as renderings of one canonical behavior contract.

## Product architecture

```text
AgentFileOps Protocol
        |
        +-- Rust core/reference implementation
        +-- TypeScript MCP / npm gateway
        +-- Go daemon
        +-- Python SDK
        +-- CLI (`afo`)
        +-- Agent Skills / skills.sh surfaces
        +-- provider presets
```

Canonical invariant:

```text
Protocol != implementation language != MCP != daemon != SDK != skill != provider preset
```

Hostinger is now a provider preset and migration source, not a product boundary.

## Public identity and discovery

Canonical public identifiers:

- Product: `AgentFileOps`
- CLI: `afo`
- Rust crates: `agent-file-ops-core`, `agent-file-ops-cli`
- npm family: `@avatar-arts/agent-file-ops*`
- Python package target: `agent-file-ops`
- Go daemon target: `agent-file-opsd`
- Docker target: `ghcr.io/avatar-arts/agent-file-ops`
- Agent/skill IDs: `agentfileops.*`

Canonical discovery metadata is stored in `manifests/discovery.json`.

Primary search/category language includes:

- remote file operations
- remote filesystem
- filesystem MCP
- SSH
- SFTP
- file transfer
- remote server
- sync
- deployment
- artifact publishing
- AI agents
- MCP / Model Context Protocol
- DevOps / automation
- safe filesystem operations

Primary skills.sh discovery skill:

- `remote-file-operations`

Task-oriented skill family now includes:

- `remote-file-operations`
- `remote-deploy`
- `artifact-publisher`
- `remote-sync`
- `protocol-conformance`

## Naming migration status

The temporary `AgentFS` identity was retired because of public-project namespace collisions.

Current active naming:

```text
AgentFileOps
agent-file-ops-*
agentfileops.*
afo
agent-file-opsd
```

Historical-only naming:

```text
AgentFS
agentfs-*
agentfs.*
agentfsd
```

Historical ADR/spec filenames may retain `agentfs` when they explicitly document project evolution. Active schemas, docs, packages, skills, manifests, validation output, runtime errors, and CI labels use AgentFileOps naming.

## Agent / skill control stack

AgentFileOps development is explicitly composed from:

- `AvaTar-ArTs/superAgents`
- `AvaTar-ArTs/superSkills`
- `AvaTar-ArTs/agent-skills`

Pinned source identities are kept in `manifests/source-lock.json`.

Current AgentFileOps-specific control roles:

- `agentfileops.orchestrator`
- `agentfileops.protocol-architect`
- `agentfileops.security-reviewer`
- `agentfileops.conformance-verifier`

Review gates:

1. Orchestrator / intent-to-capability mapping
2. Architecture boundary review
3. Security review
4. TDD / conformance-first implementation
5. Code-review comparison against plan/spec
6. Fresh verifier evidence before completion claims
7. Changelog / discovery metadata maintenance

## Security model

Core security principle:

> Structured filesystem capabilities instead of arbitrary shell execution.

Required behavior includes:

- no public arbitrary `exec(command)` operation;
- strict SSH host-key verification when transport is implemented;
- secret references rather than raw credentials in protocol operations;
- explicit absolute-path mode;
- aliases as bookmarks rather than permission boundaries;
- bounded reads/discovery;
- recursive symlink following disabled by default;
- semantic operation risk levels;
- preflight + target snapshots for replacement/destructive work;
- fingerprinted approvals for high-impact mutations;
- archive traversal/link/device/decompression defenses;
- SSRF controls for URL import;
- streaming transfer integrity verification;
- append-only redacted audit events;
- shell acceleration only through fixed operation templates after positive capability discovery and safe path mapping.

## Protocol state

Canonical JSON Schemas currently exist for:

- connection descriptor
- path specification
- semantic operation
- operation plan
- normalized result
- audit event

Schema titles and IDs have been migrated to AgentFileOps and repository-backed identifiers. The connection schema now models logical home, SFTP home, shell home, aliases, capabilities, command availability, and credential references.

## Rust implementation state

### Vertical Slice 01

Implemented semantic core for:

- path normalization;
- explicit absolute mode;
- path-escape rejection;
- risk classification L0-L4;
- fail-closed unknown operations;
- `afo` black-box conformance harness.

### Vertical Slice 02

Implemented semantic core for:

- `ConnectionDescriptor`;
- `ConnectionCapabilities`;
- alias-aware logical path resolution;
- dual SFTP/shell path namespaces;
- explicit absolute-path availability checks;
- safe backend strategy selection;
- shell acceleration only when capability and path mapping are both safe;
- deterministic SFTP fallback;
- fail-closed unavailable capabilities.

Current strategy examples:

```text
copy:
  shell + cp + safe mapping -> shell-cp
  otherwise SFTP           -> sftp-stream

checksum:
  shell + sha256sum + safe mapping -> shell-sha256sum
  otherwise SFTP                  -> sftp-hash

move:
  shell + mv + safe mapping -> shell-mv
  otherwise SFTP            -> sftp-rename-or-stream
```

No caller-supplied shell command strings are accepted.

## Validation and conformance

`scripts/validate_foundation.py` currently guards:

- required foundation files;
- JSON parseability;
- no public arbitrary-shell-like protocol operation;
- explicit symlink semantics;
- source-lock completeness;
- AgentFileOps agent IDs;
- AgentFileOps skill IDs;
- naming hygiene across active surfaces;
- retired path absence;
- canonical schema identity;
- discovery metadata and package keyword parity;
- expected skill paths.

Conformance suites:

- `tests/conformance/test_vertical_slice_01.py`
- `tests/conformance/test_vertical_slice_02.py`

## Distribution preparation

Prepared but not yet published:

- GitHub discovery/topic plan
- npm package naming + keyword metadata
- crates.io metadata
- Python naming target
- Go daemon naming target
- Docker image target
- skills.sh task-oriented names
- MCP discovery/category metadata

`docs/distribution/DISCOVERY_SEO.md` and `manifests/discovery.json` are the canonical distribution/discovery references.

## Repository hygiene

User removed obsolete branches:

- `verify/agentfileops-rename`
- `verify/vertical-slice-01`

Current verification branch retained:

- `verify/discovery-and-slice-02`

The user also enabled GitHub's automatic deletion of merged PR head branches.

PR #1 is closed as superseded. PR #2 is the current verification gate and is mergeable.

## Verification status

**Important:** implementation presence is not equivalent to verified runtime correctness.

At checkpoint time:

- GitHub PR #2 exists and is mergeable.
- The connector has not surfaced a GitHub Actions workflow run for the PR head.
- Therefore Rust workspace tests and Vertical Slice 01/02 conformance are **not yet claimed green**.
- No live SSH/SFTP transport test has run.
- No production credentials are present or required in the repository.

This checkpoint intentionally preserves that distinction.

## Current strengths

1. Clear product differentiation: files, not arbitrary shell.
2. Protocol-first architecture avoids language lock-in.
3. Strong naming/discovery discipline before public distribution.
4. Agent/skill ecosystem integrated as development control plane.
5. Safety semantics are protocol concerns rather than implementation afterthoughts.
6. Shared conformance direction prevents Rust/TS/Go/Python drift.
7. Hostinger-specific research is preserved as a reusable provider preset instead of discarded.
8. First two Rust slices establish executable semantics before transport complexity.

## Current risks / gaps

1. CI evidence is still missing through the connector.
2. `main` branch protection was not verified as enabled at checkpoint time.
3. No real SSH/SFTP transport backend exists yet.
4. Strict host-key verification and credential-reference resolution are specified but not implemented.
5. Capability probing is modeled but not connected to a real SSH session.
6. `list/stat/read/write` remote operations are not yet real.
7. No ephemeral OpenSSH/SFTP integration fixture exists yet.
8. TypeScript MCP gateway is planned, not implemented.
9. Go daemon and Python SDK are placeholders.
10. Archive, sync, transfer-ticket, approval-plan, and persistent-audit subsystems remain future slices.
11. GitHub repository description/topics are canonically documented but were not directly mutable through the available connector action.
12. Public package names still require availability checks immediately before publication.

## Continue-from-here order

The recommended continuation sequence is:

### Vertical Slice 03 — real SSH/SFTP transport

1. transport interface / trait;
2. SSH/SFTP connection configuration;
3. strict known-host verification;
4. credential-reference abstraction;
5. capability probing;
6. safe SFTP session lifecycle;
7. `list`, `stat`, bounded `read`, additive `write`;
8. normalized errors/results;
9. black-box conformance fixtures;
10. ephemeral SSH/SFTP integration fixture.

### Vertical Slice 04 — MCP gateway

Translate the proven semantic/transport core to TypeScript/MCP without changing protocol meaning.

### Later slices

- transfer tickets + browser streaming;
- preflight plans/fingerprints;
- safe copy/move/checksum acceleration;
- archive inspect/create/extract;
- sync/diff/delete policy;
- audit persistence;
- Go daemon;
- Python SDK;
- packaging/registry/skills.sh publication.

## Resume invariant

When work resumes from this checkpoint:

```text
Do not broaden shell access.
Do not let a provider redefine protocol semantics.
Do not claim verification without fresh evidence.
Do not let package/skill/schema names drift from AgentFileOps.
Add conformance expectations before transport/runtime behavior.
```
