# Release Scope

## Current package

The supported 0.1 package is the platform-specific `afo` command-line policy and conformance engine. It provides:

- normalized path resolution with explicit bases;
- escape, NUL-byte, and path-length rejection;
- operation risk classification;
- connection alias/path resolution;
- capability-aware backend strategy selection;
- normalized JSON output and errors.

The package includes the protocol schemas, security policy, and VS01/VS02 verification records needed to understand and validate these behaviors.

The reproducible packager emits both `.zip` and `.tar.gz` archives plus adjacent SHA-256 files. Tagged GitHub Actions builds produce the same artifact family for Ubuntu, macOS, and Windows.

## Experimental repository components

The following remain source-visible but are not production claims in 0.1:

- live SSH authentication and host-key callback integration;
- real SFTP list/stat/read/write execution;
- MCP, Python SDK, Go daemon, deployment, archive, and sync adapters;
- destructive operations and production recovery behavior.

Vertical Slice 03 and live transport integration are the release gate for expanding this boundary.
