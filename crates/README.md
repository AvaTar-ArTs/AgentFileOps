# Rust implementation family

Rust is the recommended canonical high-performance engine implementation, not the definition of AgentFS itself.

Planned workspace crates:

- `agentfs-core` — logical operations, planning, risk, normalized results
- `agentfs-ssh` — SSH/SFTP transport and constrained acceleration
- `agentfs-archive` — archive inspection/safety/execution
- `agentfs-sync` — manifests, diffing, synchronization
- `agentfs-cli` — native CLI over the same operation contracts

Rules:

1. Consume protocol-generated/adapted types where practical.
2. Do not depend on MCP.
3. No public arbitrary-shell API.
4. Shared conformance fixtures decide semantic parity.
5. Protocol changes are reviewed before Rust behavior becomes canonical by precedent.