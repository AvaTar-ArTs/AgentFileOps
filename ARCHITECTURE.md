# AgentFS Architecture

## Invariant

```text
AgentFS Protocol != Rust Core != MCP Gateway != Daemon != SDKs != Skills != Provider Presets
```

The protocol owns behavior. Every other component renders or implements that behavior.

## Boundaries

### `protocol/`
Canonical schemas, examples, error/risk semantics, and conformance fixtures. No dependency on MCP, Rust, Node, Go, Python, or any provider.

### `crates/`
Rust native engine family. Owns high-performance reference implementation mechanics, not protocol policy invention.

Planned crates:

- `agentfs-core`
- `agentfs-ssh`
- `agentfs-archive`
- `agentfs-sync`
- `agentfs-cli`

### `packages/`
TypeScript/npm surfaces.

Planned packages:

- `@avatar-arts/agentfs-schema`
- `@avatar-arts/agentfs-client`
- `@avatar-arts/agentfs-mcp`
- `@avatar-arts/agentfs`

The MCP package translates canonical operations into MCP tools. It must not redefine path, risk, archive, sync, or audit semantics.

### `cmd/agentfsd/`
Deployable daemon/gateway track. Go is favored for a low-dependency single-binary service, but daemon language is not part of the protocol contract.

### `sdk/`
Language SDKs. SDK behavior is generated/adapted from protocol contracts and verified with shared fixtures.

### `skills/`
Procedural guidance for agents. Skills may explain safe behavior but never become an alternate authorization or filesystem implementation layer.

### `presets/`
Provider configuration/documentation recipes. Presets never contain credentials and may not redefine AgentFS semantics.

## Control flow

```text
request
  -> surface adapter (MCP / CLI / SDK)
  -> canonical AgentFS operation
  -> capability + path resolution
  -> risk / preflight policy
  -> backend strategy selection
  -> filesystem execution
  -> result normalization
  -> audit event
  -> verifier / conformance evidence
```

## Shell boundary

AgentFS does not expose arbitrary remote shell execution. SSH shell capabilities may be used internally only through constrained templates for operations already represented by AgentFS semantics. If safe translation cannot be proven, implementations fall back to portable filesystem strategies.

## Cross-language rule

Changing a canonical operation requires protocol/schema and conformance review before or alongside implementation updates. An implementation-specific convenience must remain implementation-specific unless deliberately promoted into the protocol through design review.

## Review stack

Major changes use:

- AgentFS Orchestrator
- AgentFS Protocol Architect
- AgentFS Security Reviewer
- AgentFS Conformance Verifier

These adapt `superAgents`, `superSkills`, and `agent-skills` contracts recorded in `manifests/source-lock.json`.