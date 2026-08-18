# AgentFS Product Architecture Design

## Status

Approved product direction, formalized from the prior Hostinger File Bridge research and the decision to generalize it into a provider- and language-agnostic remote filesystem platform for AI agents.

## Product thesis

> **AgentFS gives agents files, not a shell.**

AgentFS is a safety-first remote filesystem control plane for AI agents, coding assistants, automation systems, and human operators. It exposes structured file operations across remote systems without requiring arbitrary shell access.

The product is defined by its **protocol, schemas, capability model, safety model, and behavioral contracts**. Individual language implementations are replaceable components.

AgentFS is not:

- a Hostinger-specific utility;
- a Python-only MCP server;
- a thin SSH command wrapper;
- an unrestricted remote shell;
- a single package tied to one runtime.

## Core differentiator

Most SSH-oriented agent tools expose arbitrary command execution. AgentFS instead exposes semantic filesystem operations:

```text
connections
list
stat
find
read
write
transfer
manage
archive
sync
```

The safety model is built around operation semantics rather than command strings.

## Target users

- AI coding agents publishing build artifacts;
- developers managing remote servers without raw shell tooling;
- agencies managing multiple customer hosting accounts;
- creator workflows publishing generated media and websites;
- homelab/NAS operators;
- release automation systems;
- backup/migration workflows;
- AI workflow builders needing remote storage destinations;
- teams that need auditable approvals for remote mutations.

## Product architecture

```text
                             AgentFS
                                |
                     Protocol + Schemas
                                |
            +-------------------+-------------------+
            |                   |                   |
        Core Engine         Agent Gateway        Skills
            |                   |                   |
          Rust             TypeScript/MCP       SKILL.md
            |                   |                   |
    filesystem logic       MCP / HTTP / npm     operator guidance
            |
      +-----+------+----------------+----------------+
      |            |                |                |
    SSH/SFTP      Local          Rclone          Future
      |                         adapter          backends
      |
  provider presets
  Hostinger / VPS / NAS / cPanel / Plesk / generic SSH

Additional first-class surfaces:

Go daemon     Python SDK     JS/TS SDK     CLI     Docker/OCI
```

## Product layers

### 1. AgentFS Protocol

The protocol is the canonical product contract.

It defines:

- connection descriptors;
- logical path specifications;
- filesystem operation schemas;
- capability discovery;
- risk levels;
- preflight plans;
- plan fingerprints;
- mutation result schemas;
- audit event schemas;
- transfer ticket contracts;
- archive and sync semantics;
- error codes.

Example operation:

```json
{
  "operation": "copy",
  "source": {
    "connection": "production",
    "base": "home",
    "path": "build/app.zip"
  },
  "destination": {
    "connection": "production",
    "base": "web",
    "path": "releases/app.zip"
  },
  "dry_run": true
}
```

Protocol versioning is independent of any implementation language.

### 2. Rust Core

Recommended canonical engine implementation.

Responsibilities:

- logical path normalization/resolution;
- filesystem operation planning;
- risk classification;
- plan fingerprints;
- SFTP/SSH transport abstraction;
- hashing/checksums;
- archive inspection/extraction safety;
- sync manifests and diffing;
- locking and atomic-write strategies;
- audit event generation;
- transfer streaming primitives.

Rust is selected for memory safety, native performance, concurrency, static binaries, and suitability for security-sensitive filesystem infrastructure.

The Rust crate must not depend on MCP.

### 3. TypeScript Gateway

Responsibilities:

- MCP server;
- npm/npx distribution;
- Streamable HTTP transport;
- tool schemas;
- browser upload/download endpoints;
- Node.js SDK;
- configuration UX;
- remote gateway mode;
- communication with the Rust core.

Recommended package namespace:

```text
@avatar-arts/agentfs
@avatar-arts/agentfs-mcp
@avatar-arts/agentfs-client
@avatar-arts/agentfs-schema
```

The npm surface must not contain filesystem semantics that diverge from the protocol.

### 4. Go Daemon

A standalone remote-service implementation optimized for deployment simplicity.

Potential executable:

```text
agentfsd
```

Primary use cases:

- single-binary deployment;
- VPS/server installation;
- remote AgentFS gateway;
- low-dependency Docker images;
- environments where Node/Python are undesirable.

The Go daemon may initially consume the AgentFS protocol without implementing every advanced core optimization.

### 5. Python SDK

Python is a first-class SDK and automation surface, not the identity of the project.

Responsibilities:

- Python client SDK;
- workflow integration;
- notebook/research usage;
- scripting;
- backwards migration utilities from Hostinger File Bridge;
- optional reference adapters.

Potential package:

```text
agentfs-python
```

### 6. Agent Skills

Skills teach agents how to use AgentFS correctly.

Initial skills:

```text
remote-filesystem
remote-deploy
safe-server-file-ops
artifact-publisher
remote-sync
```

Initial pack:

```text
AgentFS Operator Pack
```

Skills contain procedural knowledge only. They do not carry credentials or substitute for the AgentFS runtime/MCP server.

## Repository structure

Recommended monorepo:

```text
AgentFS/
├── README.md
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
│
├── protocol/
│   ├── README.md
│   ├── schema/
│   │   ├── connection.schema.json
│   │   ├── path.schema.json
│   │   ├── operation.schema.json
│   │   ├── plan.schema.json
│   │   ├── result.schema.json
│   │   └── audit.schema.json
│   └── examples/
│
├── crates/
│   ├── agentfs-core/
│   ├── agentfs-ssh/
│   ├── agentfs-archive/
│   ├── agentfs-sync/
│   └── agentfs-cli/
│
├── packages/
│   ├── schema/
│   ├── client/
│   ├── mcp/
│   └── npx/
│
├── cmd/
│   └── agentfsd/
│
├── sdk/
│   ├── python/
│   └── go/
│
├── skills/
│   ├── remote-filesystem/
│   ├── remote-deploy/
│   ├── safe-server-file-ops/
│   ├── artifact-publisher/
│   └── remote-sync/
│
├── presets/
│   ├── generic-ssh/
│   ├── hostinger/
│   ├── cpanel/
│   ├── plesk/
│   └── nas/
│
├── apps/
│   └── gateway/
│
├── docs/
│   ├── architecture/
│   ├── protocol/
│   ├── providers/
│   ├── security/
│   ├── migration/
│   └── superpowers/
│
├── tests/
│   ├── conformance/
│   ├── fixtures/
│   └── integration/
│
└── .github/
    └── workflows/
```

## Path model

AgentFS uses logical paths rather than assuming a backend-native path string.

```json
{
  "connection": "production",
  "base": "web",
  "path": "assets/releases/app.zip",
  "follow_symlinks": false
}
```

Base modes:

- `home` — account/user home;
- named alias/bookmark;
- `absolute` — explicit backend absolute-path mode;
- backend-defined logical roots in future adapters.

Aliases are conveniences, not permissions.

## Connection model

A connection describes a filesystem endpoint and capability source.

Initial backend type:

```text
ssh
```

Initial transport capabilities:

```text
sftp
ssh-shell acceleration
```

Future adapters may include:

- local filesystem;
- rclone remotes;
- WebDAV;
- SMB;
- S3-compatible storage where appropriate.

Do not duplicate cloud-object-store ecosystems merely for feature count. AgentFS should first dominate the remote server filesystem use case.

## Provider presets

Providers are configuration/documentation recipes, not hardcoded product boundaries.

Examples:

```text
Hostinger
DigitalOcean
Hetzner
Linode/Akamai
Vultr
AWS EC2
generic VPS
cPanel
Plesk
NAS
Raspberry Pi
```

A preset may define:

- typical SSH port;
- path conventions;
- suggested aliases;
- capability notes;
- deployment instructions;
- known provider caveats.

No provider preset contains credentials.

## Safety model

### Principle

> Structured filesystem capabilities instead of arbitrary shell execution.

No MCP or public API may expose `exec(command: string)`.

Shell use is internal acceleration only, with fixed templates and validated/quoted arguments.

### Risk levels

- Level 0: read-only;
- Level 1: additive mutation;
- Level 2: replacement/mutation;
- Level 3: destructive;
- Level 4: high-impact destructive.

Examples:

```text
list/stat/find/read             -> L0
mkdir/new upload/new copy       -> L1
overwrite/move/chmod            -> L2
single delete                   -> L3
recursive delete/sync --delete  -> L4
```

### Preflight

High-risk operations produce machine-readable plans before execution.

Plans contain:

- operation id;
- canonical inputs;
- resolved targets;
- discovered target metadata;
- risk level;
- planned actions;
- counts/estimated bytes;
- conflicts;
- symlink encounters;
- strategy;
- expiry;
- fingerprint.

Execution revalidates the plan fingerprint.

### Symlinks

Recursive operations default to `follow_symlinks=false`.

Links are represented explicitly and never silently traversed during destructive work.

### Archives

Extraction must defend against:

- absolute paths;
- `..` traversal;
- symlink/hardlink escapes;
- devices/FIFOs;
- decompression bombs;
- dangerous collisions.

### Transfers

Large binary transfers use streams or short-lived signed tickets rather than MCP JSON/base64.

### Auditing

Every mutation emits a structured audit event.

Audit records never contain:

- passwords;
- private keys;
- full bearer tokens;
- full signed transfer tickets;
- secret material.

## Capability model

Capability discovery is conservative.

A connection may report:

```json
{
  "filesystem": true,
  "sftp": true,
  "shell": true,
  "commands": {
    "cp": true,
    "mv": true,
    "find": true,
    "du": true,
    "tar": true,
    "sha256sum": true,
    "rsync": true
  }
}
```

A command is available only after positive detection.

When an accelerated path cannot be proven safe, the implementation falls back to the portable strategy.

## MCP surface

Initial semantic tools:

```text
fs_connections
fs_list
fs_stat
fs_find
fs_read
fs_write
fs_transfer
fs_manage
fs_archive
fs_sync
```

MCP is an adapter over the AgentFS protocol, not the system of record.

## Distribution model

### GitHub

Canonical source:

```text
AvaTar-ArTs/AgentFS
```

### npm / npx

Primary developer onboarding:

```bash
npx @avatar-arts/agentfs
```

### Rust

Potential crates:

```text
agentfs-core
agentfs-cli
```

### Python

Potential:

```bash
uvx agentfs
pipx run agentfs
```

### Docker / OCI

```text
ghcr.io/avatar-arts/agentfs
```

### MCP Registry

Register after the generalized implementation and conformance suite are stable enough for public use.

### skills.sh

Example install:

```bash
npx skills add AvaTar-ArTs/AgentFS --skill remote-filesystem
```

## Open-source and hosted split

### AgentFS Open

Open-source baseline:

- protocol/schemas;
- CLI;
- local/stdio MCP;
- SSH/SFTP;
- aliases;
- transfers;
- archives;
- sync;
- local audit log;
- skills.

### AgentFS Gateway

Potential hosted/commercial layer:

- remote MCP endpoint;
- encrypted connection vault;
- teams/workspaces;
- OAuth/SSO;
- shared profiles;
- central policy templates;
- approval UI;
- centralized audit history;
- upload/download portal;
- scheduled sync;
- multi-user permissions;
- managed deployment.

The open-source product must remain genuinely useful without the hosted service.

## Migration from Hostinger File Bridge

`AvaTar-ArTs/hostinger-file-bridge` becomes a provider-specific predecessor/reference implementation.

Migration goals:

1. extract the semantic filesystem model;
2. preserve the researched C+ account-filesystem design;
3. move Hostinger-specific configuration into `presets/hostinger`;
4. retain SFTP, transfer-ticket, path, archive, sync, and approval research;
5. do not copy Python-specific architecture into the protocol layer;
6. document compatibility/migration rather than silently abandoning the predecessor.

## Conformance

Every implementation claiming AgentFS compatibility must pass shared protocol tests.

Initial conformance areas:

- path normalization;
- alias behavior;
- absolute mode;
- risk classification;
- preflight/fingerprint stability;
- symlink semantics;
- archive traversal defenses;
- sync manifest classification;
- result/error schemas;
- audit redaction requirements.

## Versioning

Use independent versions for:

```text
AgentFS Protocol
AgentFS Rust core
AgentFS MCP/npm packages
AgentFS daemon
AgentFS SDKs
AgentFS skills
```

Compatibility tables document which implementation versions support which protocol versions.

Protocol `0.x` may evolve rapidly until conformance behavior stabilizes.

## Initial release sequence

### Milestone 0 — Foundation

- monorepo structure;
- protocol schemas;
- architecture docs;
- Rust workspace;
- TypeScript workspace;
- Go/Python SDK placeholders with explicit status;
- skill skeletons;
- provider preset skeletons;
- conformance harness.

### Milestone 1 — SSH/SFTP vertical slice

- connection model;
- path resolution;
- list/stat/read;
- upload/download;
- copy/move/mkdir;
- checksums;
- basic policy/audit;
- MCP adapter;
- CLI.

### Milestone 2 — Safety and workflow depth

- preflight plans;
- fingerprints;
- archive safety;
- sync;
- symlinks;
- browser transfer tickets;
- integration fixtures.

### Milestone 3 — Distribution

- npm/npx;
- crates.io where appropriate;
- PyPI SDK;
- Docker/GHCR;
- skills.sh;
- MCP Registry;
- release docs.

### Milestone 4 — Gateway

- hosted remote MCP;
- encrypted secrets/vault integration;
- team policies;
- approval UI;
- centralized audit.

## Success criteria

AgentFS is ready for its first public technical preview when:

1. protocol schemas are versioned and documented;
2. Rust core executes a complete SSH/SFTP filesystem vertical slice;
3. TypeScript MCP exposes the semantic tool surface;
4. CLI and MCP produce equivalent operation results;
5. no public interface accepts arbitrary shell command strings;
6. destructive operations are preflighted and fingerprinted;
7. large transfers avoid MCP JSON payloads;
8. shared conformance tests pass across at least two implementation surfaces;
9. Hostinger works only through a provider preset/configuration example;
10. README presents AgentFS as provider- and language-agnostic infrastructure;
11. skills.sh content teaches safe operator behavior without embedding runtime logic;
12. CI verifies protocol schemas, Rust, TypeScript, and conformance fixtures.

## Branding direction

Name: **AgentFS**

Primary line:

> **Give agents files, not a shell.**

Supporting description:

> A safety-first remote filesystem layer for AI agents. Connect SSH/SFTP systems and let agents browse, transfer, synchronize, archive, deploy, and manage files through structured operations instead of arbitrary shell commands.

## Final architecture invariant

```text
AgentFS Protocol != Rust Core != MCP Gateway != Skills != Provider Presets
```

The protocol defines behavior. Implementations execute it. MCP exposes it to agents. Skills teach correct usage. Provider presets make deployment convenient.
