# AgentFileOps Rename Verification Gate

This file exists to trigger a pull-request validation run after the repository and Rust package namespace moved from the temporary `AgentFS` working name to `AgentFileOps` / `agent-file-ops-*`.

Verification target:

- foundation contract validation
- source-lock review
- Rust workspace resolution under the new crate names
- black-box vertical-slice conformance tests against `agent-file-ops-cli`

Historical architecture filenames may retain `agentfs` when they document the earlier design checkpoint or naming collision.
