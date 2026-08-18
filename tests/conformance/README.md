# AgentFileOps Conformance Tests

## Purpose

Validate semantic parity across all implementation surfaces using shared fixtures.

## Vertical Slices

### Slice 01: Path Normalization & Risk Classification
Tests path resolution with multiple bases (home, absolute, root), symlink handling, and escape prevention.
Also validates risk classification (L0-L4) for all operations.

### Slice 02: Connection Resolution & Backend Strategy
Tests connection identity, alias-aware path resolution, explicit absolute paths, capability-aware backend selection, and safe SFTP fallback. This slice is semantic and does not open network sockets.

### Slice 03: SSH/SFTP Operation Suite
Tests all semantic operations: list, stat, find, read, write, transfer, manage, archive, sync.

Slice 01 and Slice 02 are implemented black-box contracts. Slice 03 remains explicitly skipped until live SSH/SFTP behavior and integration fixtures are available.

## Running Tests

```bash
pytest tests/conformance/ -v
```
