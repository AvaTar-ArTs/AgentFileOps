# Skill: Remote Filesystem Operations

## Purpose

Implement semantic remote filesystem operations: list, stat, find, read, write, transfer with AgentFileOps safety contracts.

## Conformance

- All operations use AgentFileOps protocol schemas
- No arbitrary shell exposure
- Risk classification enforced per operation
- Path normalization and symlink controls applied
- SSH/SFTP transport with credential references

## Operations

- `list` - L0 read-only
- `stat` - L0 read-only
- `find` - L0 read-only
- `read` - L0 read-only
- `write` - L1 additive or L2 replacement
- `transfer` - L1/L2 depending on target mode
