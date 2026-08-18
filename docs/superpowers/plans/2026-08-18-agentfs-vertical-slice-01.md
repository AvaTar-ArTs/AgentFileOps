# AgentFS Vertical Slice 01 Implementation Plan

## Goal

Deliver the first executable AgentFS capability while preserving the project invariant:

```text
protocol -> conformance -> implementation -> adapter -> verification
```

The first slice proves two canonical behaviors across a native implementation surface:

1. logical path normalization/resolution semantics;
2. operation risk classification.

These are deliberately chosen before live SSH/SFTP because every later remote operation depends on them.

## Control stack

Reviewed sources:

- `AvaTar-ArTs/superAgents` — orchestration, verification, policy/audit discipline;
- `AvaTar-ArTs/superSkills` — TDD, verification, MCP-development and changelog contracts;
- `AvaTar-ArTs/agent-skills` — workflow-orchestrator, capability-atlas, system-architect, security-engineer, testing-specialist, code-review.

Required workflow:

1. RED: add black-box conformance tests first and prove they fail for missing behavior.
2. GREEN: create the minimum Rust workspace/core/CLI necessary to satisfy them.
3. REFACTOR: separate protocol-facing models from CLI rendering without altering behavior.
4. VERIFY: run full foundation + conformance + Rust tests before claiming the slice works.
5. REVIEW: compare implementation against this plan and AgentFS protocol schemas.

## Slice API

The native CLI is only a conformance harness at this stage, not the final UX.

### Normalize path

```bash
agentfs-core-cli normalize-path --base home --path 'domains/./avatararts.org/assets/../releases'
```

Expected JSON:

```json
{
  "base": "home",
  "path": "domains/avatararts.org/releases",
  "follow_symlinks": false
}
```

Rules:

- relative bases reject attempts to traverse above the selected base;
- `.` is removed;
- internal `..` cancels one prior segment;
- NUL is rejected;
- `base=absolute` requires `/` and normalizes without silently converting relative input;
- relative bases reject leading `/`.

### Classify risk

```bash
agentfs-core-cli classify-risk --operation list
```

Canonical initial mapping:

```text
list/stat/find/read/checksum -> level_0
mkdir/touch/write-new/copy-new -> level_1
overwrite/move/rename/chmod/symlink -> level_2
delete -> level_3
recursive-delete/sync-delete/bulk-delete -> level_4
```

Unknown operations must produce a structured error and non-zero exit code.

## Files

RED phase:

- `tests/conformance/test_vertical_slice_01.py`
- `.github/workflows/foundation.yml` update to install Rust and run the conformance test.

GREEN phase:

- `Cargo.toml`
- `crates/agentfs-core/Cargo.toml`
- `crates/agentfs-core/src/lib.rs`
- `crates/agentfs-cli/Cargo.toml`
- `crates/agentfs-cli/src/main.rs`

Later slices will add SSH/SFTP and MCP. This slice must not smuggle transport concerns into the path/risk core.

## Acceptance

A slice is green only when fresh CI or equivalent local evidence shows:

- foundation validator passes;
- conformance test passes;
- `cargo test --workspace` passes;
- traversal-above-base is rejected;
- absolute mode is explicit;
- risk levels match the protocol policy;
- unknown operations fail closed.
