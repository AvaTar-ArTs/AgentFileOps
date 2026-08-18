# Vertical Slice 03 verification

Vertical Slice 03 is the first live transport slice. It proves the Rust
transport against an ephemeral localhost OpenSSH server.

## Covered behavior

- unknown host key is rejected;
- mismatched host key is rejected;
- valid public-key authentication succeeds;
- invalid public-key authentication fails;
- SFTP directory listing works;
- `stat` and `lstat` preserve file-type identity;
- bounded reads reject data beyond the requested limit;
- additive writes create a new file and reject a second write to the same path;
- no generic shell or `exec` API is exposed.

## Run locally

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p agent-file-ops-ssh --test integration -- --nocapture
cargo test --workspace --all-targets
```

The fixture requires `sshd` and `ssh-keygen` on the local machine. CI installs
`openssh-server` on Ubuntu before running the workspace tests.

The broader delete, recursive sync, archive, transfer-ticket, and persistent
audit workflows remain outside this slice.
