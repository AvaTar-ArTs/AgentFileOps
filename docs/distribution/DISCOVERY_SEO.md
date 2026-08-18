# AgentFileOps Discovery & SEO Strategy

## Canonical identity

**Product:** AgentFileOps  
**Tagline:** Give agents files, not a shell.  
**Category phrase:** Remote File Operations for AI Agents  
**SEO subtitle:** Safe Remote Filesystem, SSH/SFTP, Sync & Deployment Operations for AI Agents

## Positioning

AgentFileOps should be discoverable as a safety-first alternative to remote-agent products that expose arbitrary SSH command execution.

Primary positioning sentence:

> Remote file operations without handing your agent a shell.

Long description:

> AgentFileOps is a safety-first remote filesystem, SSH/SFTP, file transfer, sync, archive, deployment, artifact publishing, checksum, approval, and audit layer for AI agents and MCP clients without arbitrary shell access.

## Search vocabulary

Use naturally across repository description, README headings, package metadata, MCP Registry metadata, skills.sh descriptions, release notes, docs titles, landing pages, and launch copy:

- AI agents
- agent tools
- MCP
- Model Context Protocol
- filesystem MCP
- remote filesystem
- remote file operations
- SSH
- SFTP
- SSH file transfer
- SFTP file management
- file transfer
- remote server
- remote server file management
- server file management
- remote sync
- filesystem sync
- deployment
- remote deploy
- DevOps
- automation
- artifact publisher
- artifact publishing
- archives
- checksums
- secure filesystem
- VPS
- NAS

Do not keyword-stuff headings or prose. The product name stays compact; descriptive surfaces carry search vocabulary.

## GitHub discovery

Recommended description:

> Safety-first remote filesystem operations for AI agents and MCP clients. SSH/SFTP, file transfer, sync, archives, deployments, approvals and audit logs without arbitrary shell access.

Recommended topics:

```text
ai-agents
mcp
model-context-protocol
remote-filesystem
ssh
sftp
file-transfer
remote-server
deployment
sync
devops
agent-tools
filesystem
automation
security
```

The connected GitHub API currently used by this project does not expose repository topic/description mutation, so `manifests/discovery.json` remains the canonical source until those settings are applied through GitHub UI or an available settings API.

## npm namespace

```text
@avatar-arts/agent-file-ops
@avatar-arts/agent-file-ops-mcp
@avatar-arts/agent-file-ops-client
@avatar-arts/agent-file-ops-schema
```

Recommended npm keywords:

```text
mcp
model-context-protocol
ai-agent
agents
remote-filesystem
filesystem
ssh
sftp
file-transfer
remote-server
remote-file-operations
sync
deployment
devops
automation
secure-filesystem
agent-tools
artifact-publishing
```

## Rust / CLI

```text
agent-file-ops-core
agent-file-ops-cli
```

CLI:

```text
afo
```

Example future UX:

```bash
afo ls prod:/var/www
afo sync ./dist prod:web/releases
afo deploy ./build.zip production:web
```

## Python / Go / Docker

```text
Python: agent-file-ops
Go: agent-file-ops-go
Daemon: agent-file-opsd
Docker: ghcr.io/avatar-arts/agent-file-ops
```

## skills.sh discovery

Skills should use task intent instead of relying only on the product brand.

Primary:

```text
remote-file-operations
```

Recommended discoverable names/aliases:

```text
safe-remote-filesystem
ssh-sftp-file-management
remote-deploy
artifact-publisher
remote-sync
server-file-management
```

`skills/remote-file-operations/SKILL.md` is the primary operator skill. `skills/protocol-conformance/SKILL.md` remains the technical parity skill.

## MCP discovery

Recommended display name:

```text
AgentFileOps
```

Recommended category language:

```text
remote filesystem operations
filesystem MCP
SSH/SFTP file operations
safe remote file management for AI agents
```

Do not publish registry metadata until the implementation and current registry schema have been re-verified. MCP metadata must point to real, tested tools rather than planned capabilities.

## Differentiation language

Prefer:

- Give agents files, not a shell.
- Remote file operations without handing your agent a shell.
- Structured filesystem capabilities instead of arbitrary shell execution.
- Safety-first remote filesystem operations for AI agents and MCP clients.

Avoid framing the product as merely:

- a Python SFTP script;
- a Hostinger utility;
- an SSH command-execution server;
- a generic filesystem wrapper.

## Naming boundaries

Canonical active forms:

```text
AgentFileOps
agent-file-ops
agentfileops
afo
agent-file-opsd
```

`AgentFS` is historical only.

Avoid as primary product names due to collision/genericity concerns:

```text
AgentFS
RemoteFS
AgentBridge
FileWarden
```

## Canonical machine-readable source

`manifests/discovery.json` owns the current distribution names, search phrases, keywords, topics, descriptions, and aliases. Individual package metadata should be generated or reviewed against that manifest to prevent discovery drift.
