# ADR-0002: Rust SSH/SFTP Transport Uses russh + russh-sftp

**Status:** Accepted for Vertical Slice 03  
**Date:** 2026-08-18

## Context

AgentFileOps needs a real SSH/SFTP transport that preserves the product's core invariant:

> Give agents files, not a shell.

The transport must support async server/gateway use, strict host-key verification, public-key/agent authentication, SFTP file operations, bounded streaming, and optional constrained shell capability probing without turning a raw shell into the public API.

## Options reviewed

### `russh` + `russh-sftp`

Strengths:

- Rust-native async/Tokio SSH implementation;
- explicit client `check_server_key` hook;
- default server-key handler rejects keys rather than accepting unknown keys;
- built-in known-host checking helpers including `check_known_hosts_path`;
- SSH key and agent support in the same ecosystem;
- high-level async SFTP client through `russh-sftp`;
- direct access to SSH channels later for constrained capability probes;
- fits a future long-running MCP/daemon runtime without blocking-thread wrappers.

Risks:

- lower-level than a subprocess wrapper;
- transport behavior must be wrapped carefully so callers never receive arbitrary command execution;
- library API versions must be pinned and conformance-tested.

### `ssh2`

Strengths:

- mature libssh2 binding;
- direct SFTP handle;
- explicit `KnownHosts` verification support.

Trade-offs:

- synchronous core API makes it less natural for the planned async MCP/daemon architecture;
- would require blocking isolation or a separate concurrency layer;
- introduces a native libssh2 dependency boundary.

### OpenSSH subprocess / `openssh` crate

Strengths:

- delegates SSH behavior to the system OpenSSH client;
- can use strict known-host handling.

Trade-offs:

- process/shell boundary is closer to the public application surface;
- system-binary availability becomes part of the runtime contract;
- harder to preserve one portable in-process SFTP/session abstraction;
- raw-command escape hatches are contrary to the product's strongest safety differentiator.

## Decision

Use **`russh` + `russh-sftp`** for the first native Rust SSH/SFTP transport implementation.

The transport crate will be named:

```text
agent-file-ops-ssh
```

The transport is an implementation of AgentFileOps protocol semantics, not a new semantic authority.

## Host-key policy

AgentFileOps MUST NOT accept unknown server keys automatically.

Default policy:

```text
known_hosts file required
+ exact host/port/key verification
+ unknown -> reject
+ mismatch -> reject
```

`russh::keys::known_hosts::check_known_hosts_path` is the preferred verification primitive for the initial implementation.

No runtime code may use a handler equivalent to unconditional `Ok(true)` outside isolated test fixtures.

Trust-on-first-use, host-key learning, or enrollment are future explicit administrator workflows and are not part of normal filesystem calls.

## Credential boundary

Protocol operations contain only a credential reference.

Initial credential source abstraction:

```text
CredentialRef
├── ssh-agent
├── key-file-secret-reference
└── password-secret-reference (supported later only if required)
```

Raw private keys, passwords, or passphrases are never MCP operation arguments.

## Public API boundary

The SSH crate may internally open channels for capability discovery, but it MUST NOT expose:

```text
exec(command: String)
run_shell(command: String)
ssh_exec(command: String)
```

Any shell-assisted behavior must correspond to a canonical AgentFileOps operation and use a fixed internal command template.

## Initial SFTP operations

Vertical Slice 03 will implement only:

- connect / close;
- connection health;
- SFTP `list`;
- SFTP `stat` / `lstat` distinction;
- bounded read;
- additive write that refuses an existing destination;
- remote home / namespace discovery needed by `ResolvedPath`;
- capability probe plumbing.

Overwrite, delete, recursive mutation, archive, sync, and arbitrary shell are out of scope for this slice.

## Testing

Three levels:

1. pure unit tests for config, policy, and normalized mapping;
2. black-box conformance tests against the `afo` surface;
3. ephemeral OpenSSH/SFTP integration fixture in CI with generated test-only keys and known_hosts data.

Production Hostinger credentials are never required by CI.

## Research references

Reviewed 2026-08-18:

- `Eugeny/russh` official repository and current docs;
- `russh::client::Handler::check_server_key`;
- `russh::keys::known_hosts::check_known_hosts_path`;
- `russh-sftp::client::SftpSession`;
- `ssh2::KnownHosts` and `ssh2::Sftp` for comparison;
- `openssh` crate strict-known-host/subprocess model for comparison.

## Consequences

Positive:

- async-native architecture;
- strict host-key policy can be enforced inside the transport;
- SFTP remains the baseline;
- constrained SSH acceleration remains possible later;
- no dependency on a system `scp` binary.

Costs:

- AgentFileOps owns more session lifecycle and authentication plumbing;
- integration fixtures become mandatory before production-readiness claims;
- dependency/API drift must be controlled with Cargo lockfiles and CI.
