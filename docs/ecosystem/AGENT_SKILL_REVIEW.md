# AgentFileOps Ecosystem Review

## Purpose

This review applies the AvaTar-ArTs agent/skill ecosystem to AgentFileOps before runtime implementation. The goal is to preserve useful boundaries across languages and hosts instead of allowing each package to invent its own interpretation of AgentFileOps.

## Sources reviewed

Pinned source identities are recorded in `manifests/source-lock.json`.

### superAgents

Relevant control-plane roles:

- `superagents.orchestrator`: normalize requests, select capabilities, apply approval boundaries, produce execution plans.
- `superagents.verifier`: inspect artifacts and require evidence before completion claims.

Relevant runtime assets include routing, policy, execution/audit schemas, catalog validation, catalog locks, and changelog checks.

### superSkills

Curated contracts directly relevant to AgentFileOps:

- `process.brainstorming`
- `process.test-driven-development`
- `process.verification`
- `research.source-backed`
- `integration.mcp-development`
- `integration.catalog-synchronization`
- `release.changelog-discipline`

AgentFileOps adds product-specific skills beneath `skills/`, but those skills must compose with these process contracts rather than replacing them.

### agent-skills specialist bench

Mandatory review lenses:

- System Architect: component boundaries, technology trade-offs, ADRs, evolvability.
- Security Engineer: defense in depth, least privilege, fail-secure defaults, secrets isolation, auditing, threat modeling.
- Testing Specialist: contract-first tests, TDD, integration fixtures, conformance, low flakiness.
- Workflow Orchestrator: cross-language coordination and documentation/runtime parity.
- Capability Atlas: preserve one canonical capability while translating it to MCP, CLI, SDK, skill, daemon, and provider-specific renderings.
- Code Reviewer: compare implementation to plan/spec and classify deviations and defects.

## Review findings

### 1. Protocol must remain the stable unit

AgentFileOps follows Capability Atlas logic: preserve the canonical function and treat Rust, TypeScript, Go, Python, MCP, CLI, and skills as host-specific renderings. No implementation language owns semantics.

### 2. Orchestration and verification are separate responsibilities

The project must not let the same layer both perform a risky filesystem mutation and decide that its own result is trustworthy. Planning/risk classification, execution, and verification remain explicit stages.

### 3. Security belongs in the protocol contract

Host-key verification, path semantics, symlink behavior, approval levels, transfer integrity, archive traversal defenses, redaction, and audit requirements must be testable protocol/conformance rules. They must not live only in one Rust or TypeScript implementation.

### 4. Tests must prove parity across surfaces

The same canonical operation fixture should be consumable by Rust core, TypeScript MCP, CLI, and SDK tests. Surface-specific tests are necessary but insufficient without shared conformance fixtures.

### 5. Provider presets are translations, not forks

Hostinger, cPanel, Plesk, generic VPS, and NAS support should be configuration/documentation overlays. Provider-specific code requires an explicit capability reason and should never redefine core filesystem semantics.

### 6. Skills carry procedure, not runtime authority

AgentFileOps skills teach safe operator behavior: inspect before mutation, prefer aliases when known, use explicit absolute mode intentionally, dry-run recursive work, verify checksums, preview sync deletion, inspect archives, and preserve audit evidence. Skills must never embed credentials or silently bypass runtime policy.

## AgentFileOps review gates

Every major implementation slice should pass these gates:

1. **Orchestrator gate**: request mapped to canonical AgentFileOps capability and surfaces.
2. **Architecture gate**: implementation does not move protocol semantics into a host-specific package.
3. **Security gate**: threat boundaries and destructive behavior reviewed.
4. **TDD/conformance gate**: failing contract test exists before implementation and shared fixtures are updated when protocol changes.
5. **Code-review gate**: implementation compared to spec/plan; deviations classified.
6. **Verifier gate**: fresh commands/evidence captured before completion or release claims.
7. **Changelog gate**: public behavior changes recorded.

## Recommended AgentFileOps-specific agents

- `agentfileops.orchestrator`
- `agentfileops.protocol-architect`
- `agentfileops.security-reviewer`
- `agentfileops.conformance-verifier`

Their machine-readable definitions are in `manifests/agents.json`.

## Recommended AgentFileOps-specific skills

- `agentfileops.remote-filesystem`
- `agentfileops.safe-remote-deploy`
- `agentfileops.remote-sync`
- `agentfileops.artifact-publisher`
- `agentfileops.protocol-conformance`

The first operator and conformance skills are scaffolded in this repository. Remaining skills should be added as their runtime capabilities become real and testable.

## Naming hygiene gate

Active product, package, agent, skill, daemon, CI, and documentation surfaces use `AgentFileOps`, `agent-file-ops`, or `agentfileops` according to host conventions. `AgentFS` is allowed only in historical ADRs, migration records, or source filenames whose purpose is to preserve the rename history.

## Drift rule

If two surfaces disagree, do not merge behavior by intuition. Compare both against the protocol schema, documented invariants, and conformance fixture. If the protocol itself is ambiguous, update the protocol through an explicit design/ADR before changing implementations.