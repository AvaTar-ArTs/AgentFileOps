# Rust implementation family

Rust is the recommended canonical high-performance engine implementation, not the definition of AgentFileOps itself.

Planned workspace crates:

- `agent-file-ops-core`: logical operations, planning, risk, normalized results
- `agent-file-ops-ssh`: SSH/SFTP transport and constrained acceleration
- `agent-file-ops-archive`: archive inspection/safety/execution
- `agent-file-ops-sync`: manifests, diffing, synchronization
- `agent-file-ops-cli`: native CLI over the same operation contracts

Rules:

1. Consume protocol-generated/adapted types where practical.
2. Do not depend on MCP.
3. No public arbitrary-shell API.
4. Shared conformance fixtures decide semantic parity.
5. Protocol changes are reviewed before Rust behavior becomes canonical by precedent.
6. Active crate/package identifiers use the `agent-file-ops-*` namespace.