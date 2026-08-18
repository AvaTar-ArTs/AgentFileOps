# AgentFileOps Examples

These examples are narrative contracts: they show how intent becomes a safe operation and what evidence should exist afterward. They are illustrative and do not grant access to a real server.

## Example 1: Read a deployment manifest

**Intent:** inspect a known release manifest without changing the server.

    operation: read
    connection: production
    base: web
    path: releases/current/manifest.json
    max_bytes: 1048576

**Expected reasoning:**

1. Resolve the production connection reference.
2. Resolve base web and normalize the logical path.
3. Classify read as L0.
4. Select bounded SFTP read.
5. Return content plus normalized metadata.
6. Record an audit event without secrets.

**Expected result shape:**

    status: succeeded
    risk: L0
    changed: false
    target: production:web/releases/current/manifest.json
    bytes_read: <measured>
    audit_event: recorded

## Example 2: Publish a new artifact

**Intent:** publish a new build without replacing an existing release.

    operation: write
    connection: production
    base: web
    path: releases/2026.08.18/app.zip
    source: dist/app.zip
    expected_absence: true
    sha256: <computed-digest>

**Expected reasoning:**

1. Inspect and hash the local artifact.
2. Resolve the destination and verify capability.
3. Confirm the destination does not already exist.
4. Classify the action as L1 additive.
5. Create a preflight record with artifact, target, bytes, and digest.
6. Transfer using a bounded semantic write.
7. Verify the remote digest and metadata.
8. Record the result and audit event.

**Failure rule:** if the destination exists, stop with a conflict. Do not silently convert a new publication into an overwrite.

## Example 3: Deploy with review

**Intent:** replace the active release after an explicit review.

    operation: deploy
    connection: production
    base: web
    target: releases/current
    source: releases/2026.08.18
    risk: L2
    approval: required
    rollback: releases/2026.08.17

**Expected reasoning:**

1. Resolve the target and capture its snapshot.
2. Produce a deterministic diff and replacement plan.
3. Show files, bytes, conflicts, and rollback target.
4. Obtain approval for the exact plan fingerprint.
5. Execute the approved semantic operations.
6. Verify the resulting target state.
7. Preserve the prior release until rollback safety is confirmed.

## Example 4: Block a dangerous sync

**Intent:** synchronize a tree while deleting remote files not present locally.

    source: local:./dist
    destination: production:web/current
    direction: push
    deletion: true
    dry_run: false

**Expected decision:**

    status: blocked
    risk: L4
    reason: deletion-enabled synchronization requires staged approval
    next_action: generate dry-run manifest and request review

The system should show the exact deletion set, target fingerprint, conflict list, and approval requirements before any destructive action.

## Example 5: Skill-to-runtime handoff

Skills provide procedural guidance. The runtime and protocol enforce behavior.

    skill: remote-sync
      ↓ describes manifest, dry-run, deletion policy, and review
    protocol
      ↓ defines canonical fields and result semantics
    implementation
      ↓ resolves paths, classifies risk, selects backend, executes
    conformance
      ↓ proves normalized behavior and fail-closed errors

A skill may recommend a safe action, but it cannot authorize a mutation or bypass the protocol.
