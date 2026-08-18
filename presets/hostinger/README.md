# Hostinger preset

Hostinger is an AgentFileOps provider recipe, not an AgentFileOps product boundary.

This preset exists to document common Hostinger SSH/SFTP configuration patterns and migration from `AvaTar-ArTs/hostinger-file-bridge`.

Typical connection shape:

```yaml
id: hostinger-main
backend: ssh
host: <your-host>
port: <your-ssh-port>
username: <your-hosting-user>
aliases:
  home: .
  domains: domains
  web: domains/example.com/public_html
```

Never commit credentials, passwords, private keys, passphrases, or unverified host keys.

AgentFileOps must discover actual capabilities at runtime rather than assuming every Hostinger plan has identical shell access or path namespaces.

The previous Hostinger File Bridge research remains useful for SFTP, transfer tickets, archive safety, sync planning, and account-scoped path semantics, but Python-specific or one-root assumptions are not canonical AgentFileOps behavior.