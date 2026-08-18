# OpenSSH/SFTP integration fixture

The authoritative fixture is implemented by
`crates/agent-file-ops-ssh/tests/integration.rs`. Each test:

- creates a temporary remote filesystem;
- generates temporary Ed25519 host and client keys;
- starts a localhost-only `sshd` on a random high port;
- uses an explicit `known_hosts` file;
- verifies unknown and mismatched host keys fail closed;
- verifies authentication, list, stat/lstat, bounded read, and additive write;
- kills the test server and removes the temporary directory on completion.

No production credentials or external hosts are used. Ubuntu CI installs
`openssh-server` before running the workspace tests.
