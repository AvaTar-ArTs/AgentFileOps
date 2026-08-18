---
name: remote-sync
description: Use when reconciling local and remote filesystem trees, especially when overwrites, conflicts, or deletions are possible.
---

# Remote Sync

## Contract

Sync is a manifest-driven reconciliation process. It is not an implicit mirror and must never silently delete.

## Required manifest

    source
    destination
    direction
    include/exclude rules
    follow_symlinks
    conflict policy
    deletion policy
    checksum or fingerprint policy
    dry_run

## Workflow

1. Resolve both endpoints and normalize paths.
2. Enumerate source and destination state.
3. Build a deterministic manifest and diff.
4. Classify each action by risk.
5. Present counts, paths, bytes, conflicts, and deletions.
6. Require explicit deletion approval when deletion is enabled.
7. Execute only the approved fingerprint.
8. Reconcile and verify the resulting state.
9. Write an audit event.

## Safe defaults

- dry_run is true until approved;
- deletion is disabled unless explicitly enabled;
- symlinks are not followed unless explicitly requested and supported;
- conflicts stop the plan rather than choosing silently;
- an expired or changed fingerprint invalidates approval.

## Example dry-run intent

    source: local:./dist
    destination: production:web/releases/current
    direction: push
    deletion: false
    dry_run: true

## Failure behavior

Stop on endpoint ambiguity, unsupported direction, changed target state, checksum mismatch, or an unapproved deletion. Preserve the manifest and diff for review.

## Verification

Run the sync dry-run and compare the approved fingerprint with the post-reconciliation manifest. Record counts, conflicts, deletions, and the resulting audit event.
