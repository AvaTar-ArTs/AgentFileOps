# AgentFileOps Architecture

## Invariant

```text
AgentFileOps Protocol != Rust Core != MCP Gateway != Daemon != SDKs != Skills != Provider Presets
```

The protocol owns behavior. Every other component renders or implements that behavior.

## Boundaries

### `protocol/`
Canonical schemas, examples, error/risk semantics, and conformance fixtures. No dependency on MCP, Rust, Node, Go, Python, or any provider.

### `crates/`
Rust native engine family. Owns high-performance reference implementation mechanics, not protocol policy invention.

Planned crates:

- `agent-file-ops-core`
- `agent-file-ops-ssh`
- `agent-file-ops-archive`
- `agent-file-ops-sync`
- `agent-file-ops-cli`

### `packages/`
TypeScript/npm surfaces.

Planned packages:

- `@avatar-arts/agent-file-ops-schema`
- `@avatar-arts/agent-file-ops-client`
- `@avatar-arts/agent-file-ops-mcp`
- `@avatar-arts/agent-file-ops`

The MCP package translates canonical operations into MCP tools. It must not redefine path, risk, archive, sync, or audit semantics.

### `cmd/agent-file-opsd/`
Deployable daemon/gateway track. Go is favored for a low-dependency single-binary service, but daemon language is not part of the protocol contract.

### `sdk/`
Language SDKs. SDK behavior is generated/adapted from protocol contracts and verified with shared fixtures.

### `skills/`
Procedural guidance for agents. Skills may explain safe behavior but never become an alternate authorization or filesystem implementation layer.

### `presets/`
Provider configuration/documentation recipes. Presets never contain credentials and may not redefine AgentFileOps semantics.

## Control flow

```text
request
  -> surface adapter (MCP / CLI / SDK)
  -> canonical AgentFileOps operation
  -> capability + path resolution
  -> risk / preflight policy
  -> backend strategy selection
  -> filesystem execution
  -> result normalization
  -> audit event
  -> verifier / conformance evidence
```

## Shell boundary

AgentFileOps does not expose arbitrary remote shell execution. SSH shell capabilities may be used internally only through constrained templates for operations already represented by AgentFileOps semantics. If safe translation cannot be proven, implementations fall back to portable filesystem strategies.

## Cross-language rule

Changing a canonical operation requires protocol/schema and conformance review before or alongside implementation updates. An implementation-specific convenience must remain implementation-specific unless deliberately promoted into the protocol through design review.

## Review stack

Major changes use:

- AgentFileOps Orchestrator
- AgentFileOps Protocol Architect
- AgentFileOps Security Reviewer
- AgentFileOps Conformance Verifier

These adapt `superAgents`, `superSkills`, and `agent-skills` contracts recorded in `manifests/source-lock.json`.

## Naming rule

Active product and package identifiers use `AgentFileOps`, `agent-file-ops`, or `agentfileops` as appropriate for the surface. `AgentFS` is retained only in historical ADRs, migration notes, or filenames whose purpose is to document the rename.