---
name: remote-file-operations
description: Use when an agent needs to inspect, read, write, transfer, or manage remote files through AgentFileOps semantics.
---

# Remote File Operations

## Contract

Translate user intent into a canonical semantic operation. Never invent or expose an arbitrary remote shell command.

## Before execution

1. Identify connection, logical base, target path, and operation.
2. Normalize and resolve the path without allowing base escape.
3. Discover backend capabilities; prefer SFTP and portable strategies.
4. Classify risk using the protocol's L0–L4 model.
5. Produce a preflight plan for L2–L4 or whenever policy requires review.
6. Ask for approval before an explicit-approval operation.

## Operation policy

| Operation | Default | Required evidence |
|---|---|---|
| list, stat, find, read | L0 | resolved target |
| mkdir, new upload, copy-new | L1 | destination absence |
| overwrite, move, chmod, symlink | L2 | target snapshot and review |
| delete | L3 | explicit approval and target identity |
| recursive delete, bulk delete, sync-delete | L4 | staged approval, fingerprint, rollback/restore plan |

## Input and output

Inputs are protocol descriptors and credential references. Outputs are normalized result envelopes and audit events. Never place passwords, private keys, or bearer tokens in an operation, fixture, log, or result.

## Example intent

    operation: read
    connection: production
    base: web
    path: releases/app.json
    max_bytes: 1048576

The example is an intent shape, not permission to access a real server.

## Failure behavior

Fail closed on unknown operations, ambiguous paths, missing capabilities, unknown host keys, credential resolution errors, or expired fingerprints. Explain the rejected boundary and the safe next action.

## Verification

Run the narrowest relevant conformance test, then the full foundation validator. Do not claim completion from a successful placeholder test.
