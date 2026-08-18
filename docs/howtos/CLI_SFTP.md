# How to use the `afo` SFTP commands

Build the CLI:

```bash
cargo build --release -p agent-file-ops-cli
```

Every live command requires:

- `--host` and `--port`;
- `--username`;
- an explicit `--known-hosts` file;
- `--credential`, which is a private-key file path or `env:VARIABLE` whose value is a private-key file path.

Unknown or mismatched server keys fail closed. No command accepts an arbitrary
shell command string.

Set common values in your shell:

```bash
AFO=target/release/afo
HOST=example.com
USER_NAME=deploy
KNOWN_HOSTS=$HOME/.ssh/known_hosts
KEY=$HOME/.ssh/agentfileops_deploy
REMOTE=/home/deploy/releases
```

List a directory:

```bash
$AFO sftp list \
  --host "$HOST" --port 22 --username "$USER_NAME" \
  --known-hosts "$KNOWN_HOSTS" --credential "$KEY" \
  --path "$REMOTE" --limit 100
```

Read metadata:

```bash
$AFO sftp stat \
  --host "$HOST" --username "$USER_NAME" \
  --known-hosts "$KNOWN_HOSTS" --credential "$KEY" \
  --path "$REMOTE/app.zip"
```

Read at most 64 KiB. The response contains bytes as a JSON array:

```bash
$AFO sftp read \
  --host "$HOST" --username "$USER_NAME" \
  --known-hosts "$KNOWN_HOSTS" --credential "$KEY" \
  --path "$REMOTE/manifest.json" --offset 0 --limit 65536
```

Create a new remote file from a local file. Existing destinations return a
conflict error and are never overwritten:

```bash
$AFO sftp write-new \
  --host "$HOST" --username "$USER_NAME" \
  --known-hosts "$KNOWN_HOSTS" --credential "$KEY" \
  --path "$REMOTE/manifest.json" --data-file ./manifest.json
```

The current CLI surface intentionally omits overwrite, delete, recursive
operations, archive, sync, and shell execution.
