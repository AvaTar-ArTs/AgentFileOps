---
name: artifact-publisher
description: Use when packaging, versioning, checksumming, approving, or distributing generated artifacts to one or more remote destinations.
---

# Artifact Publisher

## Contract

An artifact is a named, immutable release unit with provenance. Publication must be reproducible and verifiable.

## Artifact record

    artifact_id
    version
    source
    created_at
    sha256
    media_type
    size_bytes
    intended_targets
    retention_policy

Do not include credentials or signed secret material in the record.

## Workflow

1. Inspect and package the artifact deterministically.
2. Calculate and record its digest, size, and media type.
3. Validate target capabilities and destination paths.
4. Create a publication plan and classify risk.
5. Obtain required approval for replacement or cleanup.
6. Transfer using streaming or a short-lived ticket for large binaries.
7. Verify the remote digest and metadata.
8. Record publication results and retention decisions.

## Multi-target rules

Each target has its own resolved path, capability check, risk, approval, and result. One target's success does not imply another target succeeded.

## Example

    artifact: build/app.zip
    version: 0.4.0
    sha256: <computed-digest>
    targets:
      - production:web/releases/app-0.4.0.zip
      - staging:web/releases/app-0.4.0.zip

## Failure behavior

Stop on nondeterministic packaging, digest mismatch, target conflict, unsupported capability, expired approval, or partial publication. Report successful and failed targets separately and preserve retry/rollback information.

## Verification

Verify the local digest, remote digest, target metadata, per-target publication result, and retention decision. A multi-target publication is complete only when every intended target has an explicit result.
