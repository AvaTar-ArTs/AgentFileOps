---
name: protocol-conformance
description: Use when validating semantic parity between AgentFileOps schemas, implementations, CLIs, SDKs, gateways, or provider adapters.
---

# Protocol Conformance

## Contract

The protocol schemas are authoritative. Every implementation surface must produce equivalent normalized behavior for the same fixture, including errors and safety decisions.

## Evidence ladder

1. Schema validation.
2. Unit tests for canonical logic.
3. Black-box CLI or adapter tests.
4. Cross-surface fixture parity.
5. Transport/integration tests.
6. CI result and recorded verification evidence.

A lower level does not prove a higher level.

## Vertical slices

| Slice | Scope | Current state |
|---|---|---|
| 01 | path normalization and risk classification | real foundation tests |
| 02 | connection, path base, backend strategy | implementation/tests required |
| 03 | SSH/SFTP operations and errors | implementation/tests required |

## Test requirements

Each test must:

- name one observable behavior;
- exercise real code rather than only mocks;
- assert success and fail-closed cases;
- identify the protocol/schema involved;
- record whether it is implemented, skipped, or blocked.

A test containing only pass is not conformance evidence.

## Parity checklist

Compare operation name, input normalization, resolved target, risk level, strategy, output envelope, error code, and audit fields. Differences must be intentional, documented, and covered by a compatibility rule.

## Example verification

    python scripts/validate_foundation.py
    cargo test --workspace --all-targets
    python -m pytest tests/conformance/test_vertical_slice_01.py -v

Add slice-specific commands only when the slice contains real assertions.

## Failure behavior

Stop on schema drift, missing fixture fields, inconsistent error mapping, accidental secret exposure, or a green suite with no meaningful assertions. Report the smallest reproducible case.

## Verification

Run the foundation validator, the narrowest real conformance slice, and the full workspace suite. Report passed, failed, and skipped tests separately; skipped placeholders are not conformance evidence.
