# ADR-0001: AgentFS Public Naming Collision

## Status

Open. No repository rename is authorized by this ADR.

## Context

During the first executable-slice verification, repository discovery found an established public project named `tursodatabase/agentfs` as well as several other repositories using `agentfs`.

The existing Turso project describes a filesystem for AI agents centered on persistent agent state, SQLite/Turso storage, SDKs, CLI and mounting behavior. The AvaTar-ArTs project has a different thesis: a safety-first remote filesystem control plane that gives agents structured access to existing SSH/SFTP/server filesystems without exposing arbitrary shell execution.

Although the products are technically distinct, using the same public product name creates avoidable discovery, package, registry and branding ambiguity.

## Decision for current development

1. Keep the repository `AvaTar-ArTs/AgentFS` unchanged while architecture and conformance work continues.
2. Treat `AgentFS` as a provisional repository/codename, not a guaranteed public package identity.
3. Do not publish crates, npm packages, PyPI packages, MCP Registry entries or skills.sh marketing under an unqualified `AgentFS` name until naming review is resolved.
4. Prefer scoped/internal identifiers during development, such as `@avatar-arts/agentfs-*` and repository-qualified skill sources.
5. The protocol and implementation architecture must remain name-agnostic enough to support a later product rename without semantic changes.

## Naming criteria for eventual resolution

A public name should communicate:

- remote/server filesystem operations;
- agent-safe semantics;
- structured operations rather than shell execution;
- provider independence;
- room for MCP, CLI, SDK and hosted gateway surfaces.

It should be checked across GitHub, npm, crates.io, PyPI, skills.sh/MCP discovery and basic web search before adoption.

## Non-decision

This ADR does not choose a replacement name. That should be a dedicated naming/market review rather than a rushed implementation-side rename.
