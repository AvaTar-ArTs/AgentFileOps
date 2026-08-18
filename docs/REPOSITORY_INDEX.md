# AgentFileOps Repository Index

This index is the map for the current main branch. It separates what exists, what is authoritative, and what remains planned.

## Start here

1. [README](../README.md) — project landing page and contributor entry point.
2. [Architecture](../ARCHITECTURE.md) — boundaries and canonical control flow.
3. [Security](../SECURITY.md) — trust boundaries and forbidden behavior.
4. [Protocol](../protocol/README.md) — schemas and semantic operation vocabulary.
5. [Audit report](../AUDIT_REPORT.md) — evidence-based readiness assessment.
6. [Changelog](../CHANGELOG.md) — historical changes and verification notes.

## Inventory

| Domain | Paths | Role | Evidence level |
|---|---|---|---|
| Protocol | protocol/ | schemas, examples, semantic contracts | authoritative |
| Rust core | crates/agent-file-ops-core/ | path, risk, connection, strategy logic | implemented foundation |
| CLI | crates/agent-file-ops-cli/ | JSON command/conformance surface | implemented foundation |
| SSH/SFTP | crates/agent-file-ops-ssh/ | transport configuration and operations | partial |
| Tests | tests/ | black-box and integration evidence | VS01 real; VS02/03 incomplete |
| Validation | scripts/validate_foundation.py | metadata and safety invariants | executable |
| Skills | skills/ | agent-facing procedural contracts | present; needs enrichment |
| Agents | manifests/agents.json | review and verification roles | declared |
| Source lock | manifests/source-lock.json | pinned ecosystem provenance | authoritative metadata |
| Distribution | packages/, sdk/, cmd/ | future adapters and package surfaces | staged |
| Provider recipes | presets/ | provider-specific guidance | non-authoritative |
| Visual system | docs/DESIGN_SYSTEM.md, docs/design/ | visual language and diagrams | documentation |
| Dashboard | docs/site/ | static project status surface | documentation/deployment |
| Workflows | .github/workflows/ | foundation CI and Pages deployment | executable |

## Canonical flow

    request
      ↓
    surface adapter
      ↓
    canonical operation
      ↓
    capability + path resolution
      ↓
    risk and preflight policy
      ↓
    backend strategy
      ↓
    filesystem execution
      ↓
    normalized result
      ↓
    audit event
      ↓
    conformance evidence

If a document or visual omits one of these boundaries, it is incomplete or intentionally scoped.

## Agent ecosystem

| Source | Contribution | Locked in |
|---|---|---|
| [superAgents](https://github.com/AvaTar-ArTs/superAgents) | orchestration, policy, verification roles | manifests/source-lock.json |
| [superSkills](https://github.com/AvaTar-ArTs/superSkills) | procedural contracts and quality gates | manifests/source-lock.json |
| [agent-skills](https://github.com/AvaTar-ArTs/agent-skills) | specialist architecture, security, testing, and review lenses | manifests/source-lock.json |

## Status vocabulary

- **Authoritative:** defines behavior or provenance.
- **Implemented foundation:** present and exercised in the current foundation scope.
- **Partial:** types, scaffolding, or a bounded subset exists; runtime coverage is incomplete.
- **Planned:** described as a target but not available as a supported implementation.
- **Evidence required:** must be backed by fresh tests, CI, or integration results before being called complete.

## Review rule

When changing a protocol contract, update the schema, implementation surface, conformance evidence, relevant skill, and narrative documentation together. Do not let README claims, diagrams, manifests, and tests drift apart.
