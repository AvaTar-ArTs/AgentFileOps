---
name: remote-deploy
description: Use when preparing or executing a remote deployment, artifact release, or server-side rollout through AgentFileOps.
---

# Safe Remote Deploy

## Contract

Deployment is a planned publication, not a file copy. Preserve target identity, artifact provenance, approval state, and post-deploy evidence.

## Required inputs

- source artifact and version;
- checksum or immutable digest;
- destination connection and logical path;
- target snapshot or expected state;
- rollout and rollback intent;
- approval policy.

Never accept raw credentials or an unbounded shell command.

## Workflow

1. Inspect artifact metadata and calculate a checksum.
2. Resolve the destination path and confirm the connection.
3. Snapshot target state and detect conflicts.
4. Classify the operation: new publication, replacement, or destructive cleanup.
5. Produce a preflight plan with files, bytes, risk, and rollback.
6. Obtain required approval.
7. Publish using the safest available semantic operation.
8. Verify checksum, file metadata, and expected service-facing state.
9. Emit an audit event with actor, artifact, target, decision, and result.

## Safety boundaries

- New publication is not permission to overwrite.
- Cleanup is a separate approved operation.
- Do not delete the previous release until rollback is verified.
- Do not follow symlinks during destructive cleanup by default.
- If verification fails, stop and preserve evidence.

## Example release record

    artifact: dist/site.tar.zst
    version: 2026.08.18
    sha256: <computed-digest>
    target: production:web/releases/2026.08.18
    risk: L1
    rollback: production:web/releases/2026.08.17

## Verification

Record the exact preflight, approval, publish, and post-deploy checks. A deployment is incomplete until the target state is verified.
