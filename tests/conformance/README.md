# AgentFS Conformance Suite

Shared conformance fixtures are the executable parity layer across AgentFS implementations.

Every implementation claiming AgentFS compatibility should consume the same canonical fixtures for:

- path normalization and explicit absolute mode;
- alias behavior;
- risk classification;
- preflight/fingerprint stability and invalidation;
- symlink semantics;
- archive traversal/link/device defenses;
- sync manifest classification and delete policy;
- normalized results/errors;
- audit redaction;
- capability fallback behavior.

## Fixture shape

A fixture should contain:

```json
{
  "protocol_version": "0.1",
  "case": "copy-new-target",
  "input": {},
  "environment": {},
  "expected": {},
  "invariants": []
}
```

Surface-specific test harnesses may translate the fixture into native types, but expected semantic output must remain comparable.

## Drift policy

A failed parity test is not automatically an implementation bug. It may reveal protocol ambiguity. When ambiguity exists, stop and resolve the protocol/specification explicitly before changing multiple implementations to match whichever one happened to run first.