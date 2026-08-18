---
name: protocol-conformance
description: Preserve canonical AgentFS behavior across Rust, TypeScript, Go, Python, MCP, CLI, skills, and provider presets. Use when adding or changing a capability, translating it to another surface, reviewing parity, or diagnosing behavioral drift.
---

# AgentFS Protocol Conformance

Treat the AgentFS protocol as authoritative and every runtime/package as a rendering of that contract.

## Canonical workflow

1. State the canonical capability in one sentence.
2. Identify the protocol schema and invariant that defines it.
3. List every affected surface.
4. Add or update the shared conformance fixture first.
5. Run the fixture against existing implementations and record expected failures.
6. Implement the smallest host-specific adaptation that restores parity.
7. Run surface-specific tests.
8. Run shared conformance tests across every available implementation.
9. Record intentional deltas that a host cannot express exactly.
10. Verify before claiming parity.

## Capability matrix

For each changed capability, record:

- canonical operation;
- trigger/use case;
- inputs;
- normalized outputs;
- risk level;
- Rust rendering;
- TypeScript/MCP rendering;
- CLI rendering;
- Go/Python rendering where implemented;
- skill guidance implications;
- provider preset implications;
- parity tests;
- known drift.

## Rules

- Do not copy semantics from one implementation into the protocol merely because that implementation shipped first.
- Do not allow provider presets to redefine core behavior.
- Do not flatten safety behavior to accommodate a weaker host surface without documenting the delta.
- Protocol-breaking changes require explicit versioning and changelog entries.
- If two implementations disagree and the protocol is ambiguous, stop and resolve the protocol through design/ADR before merging either behavior as canonical.

## Completion standard

A capability is conformant only when shared fixtures validate the canonical behavior and each supported surface either passes or has an explicit documented compatibility delta.