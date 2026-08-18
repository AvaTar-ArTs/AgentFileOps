# How to verify AgentFileOps

This is the normal local verification sequence. Use `python3` when your shell
does not provide a `python` alias.

```bash
python3 scripts/validate_foundation.py
python3 scripts/validate_skills.py
python3 scripts/validate_source_lock.py
python3 scripts/validate_repository_assets.py

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets

python3 -m pytest tests/conformance/test_vertical_slice_01.py \
  tests/conformance/test_vertical_slice_02.py \
  tests/conformance/test_vertical_slice_03.py -v -ra
```

The Rust workspace tests include the VS03 OpenSSH fixture. The fixture needs
`sshd` and `ssh-keygen`; Ubuntu CI installs `openssh-server` automatically.

Run only the live transport tests with:

```bash
cargo test -p agent-file-ops-ssh --test integration -- --nocapture
```

The test server listens only on localhost, uses generated temporary Ed25519
keys, and removes its temporary filesystem after each test.

## Evidence boundary

Passing these commands proves the implemented contracts and the tested SFTP
slice. It does not certify delete, recursive sync, archive, deployment
rollback, MCP, Docker, or production credential-management workflows.
