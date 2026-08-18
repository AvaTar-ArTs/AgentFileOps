# AgentFS Security Policy

AgentFS is security-sensitive infrastructure because it mediates remote filesystem access for agents and automation.

## Core invariant

**Structured filesystem capabilities instead of arbitrary shell execution.**

No public AgentFS protocol, MCP tool, CLI passthrough, SDK method, or provider preset should expose unrestricted `exec(command)` behavior.

## Required controls

- strict SSH host-key verification by default;
- credentials referenced from deployment secret stores, never embedded in protocol operations;
- explicit absolute-path mode;
- bounded reads and discovery;
- recursive symlink following disabled by default;
- operation risk classification;
- preflight and target revalidation for replacement/destructive work;
- fingerprinted approval for high-impact operations;
- archive traversal/link/device/decompression defenses;
- SSRF controls for URL import;
- streaming transfer integrity checks;
- destination locking and partial-then-finalize writes where supported;
- append-only, secret-redacted audit events.

## Secrets

Never commit or log:

- passwords;
- SSH private keys;
- private-key passphrases;
- bearer/OAuth tokens;
- complete signed transfer tickets;
- raw authorization headers;
- secret-manager values.

## Shell acceleration

Backends may use SSH shell commands internally only when:

1. the operation already exists as a canonical AgentFS semantic operation;
2. capability is positively detected;
3. backend path mapping is proven safe;
4. the command is a fixed template with separately validated/quoted arguments;
5. the result preserves AgentFS safety and normalized-result semantics.

Otherwise, fall back to a portable filesystem strategy.

## Reporting vulnerabilities

Until a private disclosure channel is configured, avoid posting exploit details for an unpatched vulnerability in a public issue. Repository maintainers should establish a GitHub Security Advisory/private reporting workflow before public technical preview.