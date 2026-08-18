# AgentFileOps Vertical Slice 02: Connection Resolution & Backend Strategy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make SSH/SFTP a canonical executable contract by adding connection models, alias-aware dual-namespace path resolution, conservative capability snapshots, and deterministic backend-strategy selection without exposing arbitrary shell execution.

**Architecture:** Extend the Rust reference core with pure connection/path/capability types first, then expose them through the `afo` conformance CLI. SFTP remains the baseline strategy. Shell acceleration is selected only when the required command capability and a proven shell-path mapping are both present. This slice deliberately does not open network sockets; live SSH/SFTP transport is the next slice so protocol semantics can be tested independently from network/library behavior.

**Tech Stack:** Rust 2021, serde, serde_json, thiserror, clap, Python pytest black-box conformance harness, JSON Schema Draft 2020-12.

**Spec:** `docs/superpowers/specs/2026-08-18-agentfs-product-design.md` plus current `ARCHITECTURE.md`, `SECURITY.md`, and `protocol/README.md` naming/safety amendments.

## Global Constraints

- Public identity is `AgentFileOps`; active Rust packages use `agent-file-ops-*`; CLI is `afo`.
- Public operations never accept arbitrary shell command strings.
- SFTP is the portable baseline for SSH-backed connections.
- Shell acceleration is advisory/conditional and never changes canonical semantics.
- Aliases are bookmarks, not permissions.
- Absolute mode is explicit.
- Relative paths may not escape the selected base.
- Recursive symlink following remains false by default.
- Credential values are never embedded in connection protocol objects; only `credential_ref` is permitted.
- Tests are written before implementation behavior.

---

### Task 1: Black-box connection and strategy conformance contract

**Files:**
- Create: `tests/conformance/test_vertical_slice_02.py`
- Modify: `.github/workflows/foundation.yml`

**Interfaces:**
- Consumes CLI binary `afo`.
- Produces expected JSON contracts for `resolve-connection-path` and `select-backend-strategy`.

- [ ] **Step 1: Write the failing connection-resolution tests**

The test invokes:

```bash
cargo run -q -p agent-file-ops-cli -- resolve-connection-path \
  --connection prod \
  --home /home/u1 \
  --sftp-home . \
  --shell-home /home/u1 \
  --alias web=domains/example.com/public_html \
  --base web \
  --path releases/app.zip
```

Expected JSON:

```json
{
  "connection":"prod",
  "logical_path":"web:releases/app.zip",
  "sftp_path":"domains/example.com/public_html/releases/app.zip",
  "shell_path":"/home/u1/domains/example.com/public_html/releases/app.zip",
  "base":"web",
  "follow_symlinks":false
}
```

Add failing cases for unknown alias, relative-base escape, and explicit absolute mode.

- [ ] **Step 2: Write failing strategy-selection tests**

Required behavior:

```json
{"operation":"copy","sftp":true,"shell":false,"commands":{},"shell_path_safe":false}
```

returns:

```json
{"strategy":"sftp-stream","accelerated":false}
```

and:

```json
{"operation":"copy","sftp":true,"shell":true,"commands":{"cp":true},"shell_path_safe":true}
```

returns:

```json
{"strategy":"shell-cp","accelerated":true}
```

If shell mapping is unsafe or `cp` is absent, it must fall back to `sftp-stream`.

Checksum behavior follows the same pattern using `sha256sum` vs `sftp-hash`.

- [ ] **Step 3: Add the new test file to CI**

Run both vertical slice files explicitly in `.github/workflows/foundation.yml`.

- [ ] **Step 4: Run the new tests and confirm RED**

Expected reason: CLI subcommands do not yet exist.

- [ ] **Step 5: Commit**

```bash
git add tests/conformance/test_vertical_slice_02.py .github/workflows/foundation.yml
git commit -m "test: define connection and backend strategy conformance"
```

---

### Task 2: Connection, capability, and resolved-path Rust models

**Files:**
- Create: `crates/agent-file-ops-core/src/connection.rs`
- Create: `crates/agent-file-ops-core/src/path_resolution.rs`
- Modify: `crates/agent-file-ops-core/src/lib.rs`

**Interfaces:**
- Produces:

```rust
pub struct ConnectionDescriptor {
    pub id: String,
    pub home: String,
    pub sftp_home: String,
    pub shell_home: Option<String>,
    pub aliases: BTreeMap<String, String>,
    pub capabilities: ConnectionCapabilities,
}

pub struct ConnectionCapabilities {
    pub sftp: bool,
    pub shell: bool,
    pub commands: BTreeMap<String, bool>,
}

pub struct ResolvedPath {
    pub connection: String,
    pub logical_path: String,
    pub sftp_path: String,
    pub shell_path: Option<String>,
    pub base: String,
    pub follow_symlinks: bool,
}

pub fn resolve_connection_path(
    connection: &ConnectionDescriptor,
    base: &str,
    path: &str,
    follow_symlinks: bool,
) -> Result<ResolvedPath, AgentFileOpsError>;
```

- [ ] **Step 1: Add unit tests for alias resolution and dual namespace mapping**

Tests cover `home`, named alias, `absolute`, unknown alias, and escape failure.

- [ ] **Step 2: Verify the tests fail because the types/functions do not exist**

Run:

```bash
cargo test -p agent-file-ops-core path_resolution -- --nocapture
```

- [ ] **Step 3: Implement the minimal models and resolver**

Rules:

- alias paths are relative to connection home;
- `sftp_home="."` means the SFTP namespace begins at account home;
- `shell_home` may be absent;
- shell path is `None` when a safe mapping cannot be established;
- `base="absolute"` preserves the absolute path for SFTP and shell only when their namespace supports the same absolute form;
- no `..` escape above home/alias base.

- [ ] **Step 4: Run core tests GREEN**

```bash
cargo test -p agent-file-ops-core --all-targets
```

- [ ] **Step 5: Commit**

```bash
git add crates/agent-file-ops-core/src
git commit -m "feat: add connection-aware path resolution"
```

---

### Task 3: Deterministic backend strategy selector

**Files:**
- Create: `crates/agent-file-ops-core/src/strategy.rs`
- Modify: `crates/agent-file-ops-core/src/lib.rs`

**Interfaces:**
- Produces:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendStrategy {
    pub strategy: String,
    pub accelerated: bool,
}

pub fn select_backend_strategy(
    operation: &str,
    capabilities: &ConnectionCapabilities,
    shell_path_safe: bool,
) -> Result<BackendStrategy, AgentFileOpsError>;
```

- [ ] **Step 1: Add failing strategy unit tests**

Required mappings:

```text
copy + shell/cp + safe shell mapping -> shell-cp
copy otherwise with SFTP -> sftp-stream
checksum + shell/sha256sum + safe shell mapping -> shell-sha256sum
checksum otherwise with SFTP -> sftp-hash
move + shell/mv + safe shell mapping -> shell-mv
move otherwise with SFTP -> sftp-rename-or-stream
```

If neither a safe shell strategy nor SFTP fallback exists, return `CapabilityUnavailable`.

- [ ] **Step 2: Run tests RED**

```bash
cargo test -p agent-file-ops-core strategy -- --nocapture
```

- [ ] **Step 3: Implement the selector with fixed operation/command mappings**

No caller-supplied command strings are accepted.

- [ ] **Step 4: Run tests GREEN**

```bash
cargo test -p agent-file-ops-core --all-targets
```

- [ ] **Step 5: Commit**

```bash
git add crates/agent-file-ops-core/src
git commit -m "feat: select safe filesystem backend strategies"
```

---

### Task 4: CLI rendering of the canonical contracts

**Files:**
- Modify: `crates/agent-file-ops-cli/src/main.rs`

**Interfaces:**
- Adds CLI subcommands:

```text
resolve-connection-path
select-backend-strategy
```

- [ ] **Step 1: Run black-box conformance tests and confirm they still fail**

```bash
python -m pytest tests/conformance/test_vertical_slice_02.py -v
```

- [ ] **Step 2: Implement `resolve-connection-path` argument parsing**

Parse repeatable `--alias name=value`; construct `ConnectionDescriptor`; emit `ResolvedPath` JSON.

- [ ] **Step 3: Implement `select-backend-strategy` argument parsing**

Accept booleans for SFTP/shell/path safety and repeatable `--command name=true|false`; emit `BackendStrategy` JSON.

- [ ] **Step 4: Run black-box tests GREEN**

```bash
python -m pytest tests/conformance/test_vertical_slice_02.py -v
```

- [ ] **Step 5: Run full Rust and conformance verification**

```bash
cargo test --workspace --all-targets
python -m pytest tests/conformance/test_vertical_slice_01.py tests/conformance/test_vertical_slice_02.py -v
python scripts/validate_foundation.py
```

- [ ] **Step 6: Commit**

```bash
git add crates/agent-file-ops-cli/src/main.rs
git commit -m "feat: expose connection and backend conformance CLI"
```

---

## Self-review

- Spec coverage: connection identity, aliases, dual SFTP/shell path namespaces, SFTP baseline, constrained shell acceleration, no raw credentials, no arbitrary shell are covered.
- Deliberate exclusion: live network authentication, host-key verification implementation, and actual SFTP transport are deferred to Vertical Slice 03 because this slice must prove canonical semantics independent of a transport library.
- Type consistency: `ConnectionDescriptor`, `ConnectionCapabilities`, `ResolvedPath`, and `BackendStrategy` are defined once and consumed by later tasks.
- Placeholder scan: no TBD/TODO implementation steps remain.

## Execution handoff

The user already requested continuation in this session, so execute inline with the repository's TDD and verifier gates. Do not claim GREEN until fresh CI or equivalent command evidence is available.
