---
name: artifact-publisher
description: Publish build, release, media, or creative artifacts to remote AgentFileOps targets with file transfer, checksum verification, destination planning, and audit-aware placement. Use for artifact publishing, release uploads, remote file delivery, build distribution, or creator asset publishing over SSH/SFTP without arbitrary shell execution.
---

# Artifact Publisher

Use AgentFileOps to publish artifacts as explicit, verifiable remote file operations.

## Runtime prerequisite

Use only the semantic transfer and filesystem operations exposed by the connected AgentFileOps runtime. If remote placement, verification, or activation requires a capability the runtime does not provide, surface that gap instead of improvising unrestricted shell commands.

## Workflow

1. Identify the local/source artifact, destination connection, logical base, and final path.
2. Inspect the destination and determine whether the operation is new, replacement, or conflict-prone.
3. Prefer streaming or scoped upload tickets for large binary artifacts.
4. Supply expected size/checksum when known.
5. Upload to a staging/partial location and finalize atomically where supported.
6. Verify final size and checksum or equivalent integrity evidence.
7. Record artifact metadata and remote path in the audit trail.
8. If publishing a release set, verify every required artifact rather than sampling one file.

## Search intents covered

artifact publisher, artifact publishing, release upload, build artifact upload, remote file delivery, SSH file publishing, SFTP artifact transfer, AI agent artifact publishing, creator asset publishing, release automation.

## Safety rules

- New-file publishing must not silently become overwrite.
- Large binary data should not be embedded into MCP JSON/tool-call text.
- A successful HTTP/upload response is not final verification.
- Preserve checksums and destination identity whenever practical.
