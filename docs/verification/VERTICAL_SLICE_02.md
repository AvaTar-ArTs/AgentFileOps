# Vertical Slice 02: Connection Resolution & Backend Strategy

## Purpose

Validate the semantic decisions that must happen before AgentFileOps touches a remote host. This slice is intentionally network-free: it proves connection/path/capability behavior independently of an SSH library or live credentials.

## Evidence

`tests/conformance/test_vertical_slice_02.py` exercises the public `afo` JSON CLI and verifies:

- named connection identity and dual SFTP/shell namespace mapping;
- unknown aliases fail closed;
- alias-relative paths cannot escape their selected base;
- absolute paths require explicit absolute mode and preserve the filesystem root;
- SFTP is the baseline strategy when shell acceleration is unavailable or unsafe;
- shell acceleration requires both a safe shell-path mapping and the required command;
- checksum selection uses `sha256sum` only when advertised, otherwise SFTP hashing;
- missing backend capabilities return normalized errors.

## Boundary

This slice does not prove SSH authentication, host-key callbacks, SFTP packet handling, remote reads, writes, transfers, or recovery behavior. Those belong to Vertical Slice 03 and require a disposable integration fixture.

## Verification

```bash
python -m pytest tests/conformance/test_vertical_slice_02.py -v -ra
```
