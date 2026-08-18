# How to run a local OpenSSH target

For real transport development, use the built-in ephemeral fixture instead of
production credentials:

```bash
cargo test -p agent-file-ops-ssh --test integration -- --nocapture
```

The fixture creates a temporary directory containing:

- an Ed25519 SSH host key;
- an Ed25519 client key and authorized-key file;
- a temporary `known_hosts` entry;
- `hello.txt` and a symlink for metadata tests;
- a random high localhost port.

It then verifies:

1. an empty `known_hosts` file is rejected;
2. a mismatched host key is rejected;
3. an unauthorized client key is rejected;
4. the authorized key connects successfully;
5. SFTP list/stat/lstat work;
6. reads exceeding the requested bound fail;
7. `write_new` succeeds once and returns a conflict on reuse.

The fixture process and files are cleaned up automatically. No external host,
password, private production key, or arbitrary shell command is involved.

To inspect the fixture contract and CI assumptions, see:

- `crates/agent-file-ops-ssh/tests/integration.rs`;
- `tests/integration/ssh/README.md`;
- `docs/verification/VERTICAL_SLICE_03.md`.
