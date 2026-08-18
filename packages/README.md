# TypeScript / npm surfaces

TypeScript owns developer-facing Node.js integrations and the MCP gateway, while the AgentFS protocol owns filesystem semantics.

Planned packages:

- `@avatar-arts/agentfs-schema`
- `@avatar-arts/agentfs-client`
- `@avatar-arts/agentfs-mcp`
- `@avatar-arts/agentfs` (npx/bootstrap entry)

The MCP package should expose the semantic tool family:

`fs_connections`, `fs_list`, `fs_stat`, `fs_find`, `fs_read`, `fs_write`, `fs_transfer`, `fs_manage`, `fs_archive`, `fs_sync`.

Rules:

- never expose arbitrary remote `exec`;
- generated/adapted schema types must remain traceable to `protocol/`;
- large binary transfer uses streaming/ticket flows;
- MCP annotations are hints, never the sole enforcement layer;
- shared conformance fixtures must validate normalized behavior.