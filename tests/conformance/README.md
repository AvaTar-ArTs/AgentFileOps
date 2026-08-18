# AgentFileOps Conformance Tests

## Purpose

Validate semantic parity across all implementation surfaces using shared fixtures.

## Vertical Slices

### Slice 01: Path Normalization & Risk Classification
Tests path resolution with multiple bases (home, absolute, root), symlink handling, and escape prevention.
Also validates risk classification (L0-L4) for all operations.

### Slice 02: Connection, Path, Backend Strategy
Tests SSH/SFTP connection setup, transport security, credential handling, and backend-specific behaviors.

### Slice 03: Full Operation Suite
Tests all semantic operations: list, stat, find, read, write, transfer, manage, archive, sync.

## Running Tests

```bash
pytest tests/conformance/ -v
```
