# AgentFileOps Vertical Slice 03 — SSH/SFTP Transport Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first real AgentFileOps SSH/SFTP transport with strict host-key verification, credential references, capability discovery plumbing, and safe `list`, `stat`, bounded `read`, and additive `write` operations.

**Architecture:** `agent-file-ops-ssh` implements transport mechanics behind the existing protocol-first core. `russh` owns SSH session/authentication and host-key callbacks; `russh-sftp` owns async SFTP filesystem operations. The public core continues to expose semantic operations, never arbitrary shell commands.

**Tech Stack:** Rust 2021, Tokio, `russh`, `russh-sftp`, `serde`, `thiserror`, pytest black-box conformance, GitHub Actions, ephemeral OpenSSH/SFTP integration fixture.

**Spec:** `docs/architecture/ADR-0002-russh-transport.md` plus the product design checkpoint `docs/checkpoints/2026-08-18-agentfileops-checkpoint-01.md`.

## Global Constraints

- Product identity is `AgentFileOps`; active package identifiers use `agent-file-ops-*`.
- No public `exec(command)` or caller-provided shell command strings.
- Unknown SSH host keys are rejected by default.
- Mismatched SSH host keys are rejected.
- Protocol operations carry credential references, never raw credential material.
- SFTP is the baseline filesystem transport.
- Reads are bounded.
- Writes in this slice are additive only and fail if the destination exists.
- Recursive mutation, delete, overwrite, archive and sync are out of scope.
- `follow_symlinks=false` remains the recursive/default safety posture.
- No production credentials in CI.
- No completion claim without fresh unit/conformance/integration evidence.

---

## File Structure

Create:

```text
crates/agent-file-ops-ssh/
  Cargo.toml
  src/lib.rs
  src/config.rs
  src/host_key.rs
  src/credentials.rs
  src/session.rs
  src/sftp_ops.rs
  tests/policy.rs

tests/conformance/test_vertical_slice_03.py

tests/integration/ssh/
  README.md
  sshd_config
  fixture.sh

protocol/schema/ssh-transport.schema.json

docs/verification/VERTICAL_SLICE_03.md
```

Modify:

```text
Cargo.toml
protocol/schema/connection.schema.json
scripts/validate_foundation.py
.github/workflows/foundation.yml
CHANGELOG.md
```

Responsibility boundaries:

- `config.rs`: typed SSH transport configuration only.
- `host_key.rs`: strict known-host verification policy only.
- `credentials.rs`: credential-reference resolution interface only.
- `session.rs`: SSH connection/authentication/session lifecycle.
- `sftp_ops.rs`: bounded semantic SFTP operations, no policy invention.
- `lib.rs`: public transport exports and normalized error mapping.

---

### Task 1: Canonical SSH transport contract

**Files:**
- Create: `protocol/schema/ssh-transport.schema.json`
- Modify: `protocol/schema/connection.schema.json`
- Modify: `scripts/validate_foundation.py`
- Test: `tests/conformance/test_vertical_slice_03.py`

**Interfaces:**
- Consumes: existing `ConnectionDescriptor`, `PathSpec`, AgentFileOps naming/discovery invariants.
- Produces: canonical transport config fields `known_hosts_ref`, `credential_ref`, `connect_timeout_seconds`, `operation_timeout_seconds`, and bounded read contract.

- [ ] **Step 1: Write failing black-box contract tests**

Add tests asserting:

```python
def test_ssh_transport_schema_requires_known_hosts_and_credential_refs():
    schema = load_schema("ssh-transport.schema.json")
    assert "known_hosts_ref" in schema["required"]
    assert "credential_ref" in schema["required"]


def test_transport_schema_has_no_raw_secret_fields():
    text = json.dumps(load_schema("ssh-transport.schema.json"))
    for forbidden in ["private_key", "password", "passphrase", "secret_value"]:
        assert forbidden not in text


def test_read_contract_is_bounded():
    schema = load_schema("ssh-transport.schema.json")
    assert schema["properties"]["inline_read_bytes"]["maximum"] == 16777216
```

- [ ] **Step 2: Run RED**

```bash
python -m pytest tests/conformance/test_vertical_slice_03.py -v
```

Expected: FAIL because `ssh-transport.schema.json` does not exist.

- [ ] **Step 3: Add minimal canonical schema**

Required shape:

```json
{
  "required": [
    "known_hosts_ref",
    "credential_ref"
  ],
  "properties": {
    "known_hosts_ref": {"type": "string", "minLength": 1},
    "credential_ref": {"type": "string", "minLength": 1},
    "connect_timeout_seconds": {"type": "integer", "minimum": 1, "maximum": 120, "default": 20},
    "operation_timeout_seconds": {"type": "integer", "minimum": 1, "maximum": 600, "default": 60},
    "inline_read_bytes": {"type": "integer", "minimum": 1, "maximum": 16777216, "default": 1048576}
  }
}
```

- [ ] **Step 4: Extend foundation validation**

Validator must reject:

- missing SSH transport schema;
- `AgentFS` schema identity;
- raw secret fields;
- missing bounded-read maximum.

- [ ] **Step 5: Run GREEN**

```bash
python scripts/validate_foundation.py
python -m pytest tests/conformance/test_vertical_slice_03.py -v
```

- [ ] **Step 6: Commit**

```bash
git add protocol/schema tests/conformance scripts/validate_foundation.py
git commit -m "feat: define SSH transport contract"
```

---

### Task 2: Transport config, credential references and strict host-key policy

**Files:**
- Create: `crates/agent-file-ops-ssh/Cargo.toml`
- Create: `crates/agent-file-ops-ssh/src/config.rs`
- Create: `crates/agent-file-ops-ssh/src/credentials.rs`
- Create: `crates/agent-file-ops-ssh/src/host_key.rs`
- Create: `crates/agent-file-ops-ssh/src/lib.rs`
- Create: `crates/agent-file-ops-ssh/tests/policy.rs`
- Modify: root `Cargo.toml`

**Interfaces:**

Produces:

```rust
pub struct SshTransportConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub known_hosts_path: PathBuf,
    pub credential_ref: CredentialRef,
    pub connect_timeout: Duration,
    pub operation_timeout: Duration,
    pub inline_read_bytes: u64,
}

pub enum CredentialRef {
    SshAgent,
    KeyFile(PathBuf),
}

pub struct StrictHostKeyVerifier {
    host: String,
    port: u16,
    known_hosts_path: PathBuf,
}

impl StrictHostKeyVerifier {
    pub fn verify(&self, key: &ssh_key::PublicKey) -> Result<(), TransportError>;
}
```

- [ ] **Step 1: Write failing Rust tests**

Required tests:

```rust
#[test]
fn rejects_zero_inline_read_limit() { ... }

#[test]
fn credential_ref_does_not_store_raw_secret_material() { ... }

#[test]
fn missing_known_hosts_file_fails_closed() { ... }
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p agent-file-ops-ssh --test policy
```

Expected: FAIL because crate/types do not exist.

- [ ] **Step 3: Add dependencies**

Pin compatible current versions after Cargo resolution:

```toml
russh = "0.62"
russh-sftp = "2.3"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "fs", "time", "net", "io-util"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
```

Do not add a raw shell/process dependency.

- [ ] **Step 4: Implement strict verifier**

Use:

```rust
russh::keys::known_hosts::check_known_hosts_path(
    &self.host,
    self.port,
    key,
    &self.known_hosts_path,
)
```

Rules:

- `Ok(true)` -> accept;
- `Ok(false)` -> `UnknownHostKey`;
- key-changed error -> `HostKeyMismatch`;
- missing/unreadable known-host data -> fail closed.

No automatic `learn_known_hosts*` call.

- [ ] **Step 5: Run GREEN**

```bash
cargo test -p agent-file-ops-ssh --test policy
```

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml crates/agent-file-ops-ssh
git commit -m "feat: add strict SSH transport policy"
```

---

### Task 3: Async SSH session and capability plumbing

**Files:**
- Create: `crates/agent-file-ops-ssh/src/session.rs`
- Modify: `crates/agent-file-ops-ssh/src/lib.rs`
- Test: `crates/agent-file-ops-ssh/tests/session_policy.rs`

**Interfaces:**

Produces:

```rust
pub struct AgentFileOpsSshSession { ... }

impl AgentFileOpsSshSession {
    pub async fn connect(config: SshTransportConfig) -> Result<Self, TransportError>;
    pub async fn open_sftp(&self) -> Result<SftpSession, TransportError>;
    pub async fn close(self) -> Result<(), TransportError>;
}
```

Authentication behavior:

```text
CredentialRef::SshAgent
  -> connect to SSH agent
  -> enumerate identities
  -> authenticate without copying private key bytes into protocol objects

CredentialRef::KeyFile(path)
  -> load key from configured secret path
  -> authenticate using that local secret reference
```

- [ ] **Step 1: Write failing session-policy tests**

Test that:

- handler delegates every server key to `StrictHostKeyVerifier`;
- no unconditional host-key acceptance path exists;
- auth failure maps to normalized `AuthenticationFailed`;
- connection timeout maps to `ConnectionTimeout`.

- [ ] **Step 2: Run RED**

```bash
cargo test -p agent-file-ops-ssh session_policy
```

- [ ] **Step 3: Implement minimal async session**

Use a `russh::client::Handler` whose `check_server_key` calls the strict verifier.

Do not implement a generic `exec` method.

- [ ] **Step 4: Open SFTP subsystem**

Open a session channel, request the `sftp` subsystem, and construct `russh_sftp::client::SftpSession` over the channel stream.

- [ ] **Step 5: Run GREEN**

```bash
cargo test -p agent-file-ops-ssh
```

- [ ] **Step 6: Commit**

```bash
git add crates/agent-file-ops-ssh
git commit -m "feat: add async SSH and SFTP session"
```

---

### Task 4: Safe SFTP list/stat/read/write operations

**Files:**
- Create: `crates/agent-file-ops-ssh/src/sftp_ops.rs`
- Modify: `crates/agent-file-ops-ssh/src/lib.rs`
- Test: `crates/agent-file-ops-ssh/tests/sftp_contract.rs`

**Interfaces:**

Produces semantic transport methods:

```rust
pub async fn list(&self, path: &str) -> Result<Vec<RemoteEntry>, TransportError>;
pub async fn lstat(&self, path: &str) -> Result<RemoteStat, TransportError>;
pub async fn stat(&self, path: &str) -> Result<RemoteStat, TransportError>;
pub async fn read_bounded(&self, path: &str, max_bytes: u64) -> Result<Vec<u8>, TransportError>;
pub async fn write_new<R>(&self, path: &str, source: R) -> Result<WriteResult, TransportError>
where R: AsyncRead + Unpin + Send;
```

Rules:

- `read_bounded` returns `ReadLimitExceeded` rather than silently truncating unless the protocol explicitly requests truncation later;
- `write_new` checks destination absence and fails with `Conflict` if it exists;
- the create/open mode used for additive writes must not silently overwrite an existing file;
- no recursive operation in this task.

- [ ] **Step 1: Write failing semantic tests**

Required cases:

- list maps entries consistently;
- `lstat` preserves symlink identity;
- bounded read rejects file larger than limit;
- additive write rejects existing destination;
- successful write reports bytes written.

- [ ] **Step 2: Run RED**

```bash
cargo test -p agent-file-ops-ssh sftp_contract
```

- [ ] **Step 3: Implement minimal operations**

Use only `russh-sftp` filesystem APIs and async stream I/O.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p agent-file-ops-ssh
```

- [ ] **Step 5: Commit**

```bash
git add crates/agent-file-ops-ssh
git commit -m "feat: add safe SFTP file operations"
```

---

### Task 5: Ephemeral OpenSSH/SFTP integration fixture

**Files:**
- Create: `tests/integration/ssh/README.md`
- Create: `tests/integration/ssh/sshd_config`
- Create: `tests/integration/ssh/fixture.sh`
- Create: `crates/agent-file-ops-ssh/tests/integration.rs`
- Modify: `.github/workflows/foundation.yml`

**Interfaces:**
- Consumes: real `AgentFileOpsSshSession` and SFTP operations.
- Produces: fresh evidence for connection, host-key rejection/acceptance, auth, list/stat/read/write.

- [ ] **Step 1: Build a test-only SSH fixture**

Fixture requirements:

- localhost only;
- random/high port;
- generated test host key;
- generated test user key;
- dedicated temporary filesystem root;
- deterministic known_hosts line;
- no production secret references;
- no privileged external host dependency.

- [ ] **Step 2: Write failing integration tests**

Required cases:

```text
unknown host key -> reject
wrong host key   -> reject
correct key      -> connect
bad credential   -> reject
correct key auth -> connect
list             -> expected fixture files
lstat symlink    -> identifies link
bounded read     -> enforces limit
write_new        -> creates new file
write_new again  -> conflict
```

- [ ] **Step 3: Run RED**

```bash
cargo test -p agent-file-ops-ssh --test integration -- --ignored
```

Expected: fixture/session integration not yet wired.

- [ ] **Step 4: Wire fixture and CI**

Add a `vertical-slice-03` GitHub Actions job that:

```text
installs/starts local OpenSSH server fixture
runs cargo test for agent-file-ops-ssh
runs pytest vertical slice 03
tears fixture down
```

- [ ] **Step 5: Run GREEN**

```bash
cargo test --workspace --all-targets
python scripts/validate_foundation.py
python -m pytest tests/conformance/test_vertical_slice_01.py -v
python -m pytest tests/conformance/test_vertical_slice_02.py -v
python -m pytest tests/conformance/test_vertical_slice_03.py -v
```

- [ ] **Step 6: Commit**

```bash
git add tests/integration .github/workflows/foundation.yml crates/agent-file-ops-ssh
git commit -m "test: verify SSH SFTP transport end to end"
```

---

### Task 6: Verification record and changelog

**Files:**
- Create: `docs/verification/VERTICAL_SLICE_03.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Record exact verification commands**

Checklist:

```text
[ ] foundation validator
[ ] Rust workspace tests
[ ] Vertical Slice 01 conformance
[ ] Vertical Slice 02 conformance
[ ] Vertical Slice 03 conformance
[ ] ephemeral SSH/SFTP integration
[ ] unknown host rejected
[ ] mismatched host rejected
[ ] correct host/auth accepted
[ ] bounded read verified
[ ] additive write conflict verified
[ ] no arbitrary shell public API detected
```

- [ ] **Step 2: Record actual evidence only**

Do not check boxes based on code inspection. Each checked box must have a fresh CI/local command result.

- [ ] **Step 3: Update changelog**

Record the SSH/SFTP transport slice as complete only after the verifier gate is green.

- [ ] **Step 4: Commit**

```bash
git add docs/verification/VERTICAL_SLICE_03.md CHANGELOG.md
git commit -m "docs: record SSH SFTP transport verification"
```

---

## Self-review

### Spec coverage

Covered:

- strict host-key verification;
- credential references;
- async SSH/SFTP transport;
- SFTP baseline;
- bounded reads;
- additive writes;
- no arbitrary shell API;
- integration fixture;
- fresh verification evidence.

Deliberately deferred:

- overwrite;
- delete;
- transfer tickets;
- archive;
- sync;
- destructive preflight plans;
- persistent audit subsystem;
- TypeScript MCP gateway.

### Placeholder scan

No `TBD`, `TODO`, or unspecified implementation placeholders are permitted by this plan. Dependency patch versions are resolved by Cargo within the declared compatible release line and captured by the lockfile.

### Type consistency

`SshTransportConfig`, `CredentialRef`, `StrictHostKeyVerifier`, `AgentFileOpsSshSession`, `RemoteEntry`, `RemoteStat`, `WriteResult`, and `TransportError` are the canonical names used throughout this slice.

## Execution handoff

Plan saved at:

`docs/superpowers/plans/2026-08-18-agentfileops-vertical-slice-03.md`

Execution mode for this conversation: **inline**, with TDD and verifier checkpoints.
