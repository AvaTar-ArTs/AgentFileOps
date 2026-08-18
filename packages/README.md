# TypeScript / npm surfaces

TypeScript owns developer-facing Node.js integrations and the MCP gateway, while the AgentFileOps protocol owns filesystem semantics.

Planned packages:

- `@avatar-arts/agent-file-ops-schema`
- `@avatar-arts/agent-file-ops-client`
- `@avatar-arts/agent-file-ops-mcp`
- `@avatar-arts/agent-file-ops` (npx/bootstrap entry)

The MCP package should expose the semantic tool family:

`fs_connections`, `fs_list`, `fs_stat`, `fs_find`, `fs_read`, `fs_write`, `fs_transfer`, `fs_manage`, `fs_archive`, `fs_sync`.

Recommended discovery keywords:

`mcp`, `model-context-protocol`, `ai-agent`, `remote-filesystem`, `ssh`, `sftp`, `file-transfer`, `remote-server`, `sync`, `deployment`, `devops`, `automation`, `secure-filesystem`.

Rules:

- never expose arbitrary remote `exec`;
- generated/adapted schema types must remain traceable to `protocol/`;
- large binary transfer uses streaming/ticket flows;
- MCP annotations are hints, never the sole enforcement layer;
- shared conformance fixtures must validate normalized behavior;
- active package metadata must use the AgentFileOps name and `agent-file-ops` namespace.