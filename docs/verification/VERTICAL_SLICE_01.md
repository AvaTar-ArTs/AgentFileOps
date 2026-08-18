# Vertical Slice 01: Path Normalization & Risk Classification

## Purpose

Validate core semantics of path handling and operation risk classification across all implementation surfaces.

## Test Coverage

### Path Normalization
- Relative path resolution with multiple bases (home, absolute, root)
- Path canonicalization (remove ./, ../, etc.)
- Escape prevention (prevent ../../etc/passwd attacks)
- Symlink handling (explicit follow/no-follow controls)

### Risk Classification
- L0 (read-only): list, stat, find, read, checksum
- L1 (additive): mkdir, write new file, copy to new path
- L2 (replacement): overwrite, move, chmod, symlink creation
- L3 (destructive): single delete
- L4 (high-impact): recursive delete, bulk delete, sync with deletion

## Conformance

All implementation surfaces (Rust, TypeScript, Python, Go) must produce identical results for these tests.
