# AgentFileOps Python SDK

The Python SDK is a small automation wrapper around the verified `afo` policy
CLI. It supports path normalization, risk classification, and backend strategy
selection without exposing arbitrary command execution.

```python
from agentfileops import Client

client = Client(binary="afo")
print(client.normalize_path("home", "domains/example.com/public_html"))
print(client.classify_risk("write-new"))
```

Live SSH/SFTP methods will be added only after the Rust transport integration
fixture is part of the supported release gate.
