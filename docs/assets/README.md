# AgentFileOps Visual Gallery

A small campaign system for explaining AgentFileOps as a safety-first remote filesystem protocol.

These bitmap visuals are used for product storytelling, onboarding, and documentation headers. Exact architecture and risk diagrams remain SVG-native so their labels and relationships stay reviewable.

## Product hero

![AgentFileOps product hero](generated/agentfileops-hero.png)

Use this image when introducing the core promise: an agent sends structured file intent through a guarded protocol instead of receiving an unrestricted shell.

## Product use cases

![AgentFileOps use cases](generated/agentfileops-use-cases.png)

Use this image to explain the same semantic operation reaching different destinations—VPS, NAS, shared hosting, or another SSH/SFTP target—with approved and review-required paths made visible.

## Typography poster

![Give agents files, not a shell](generated/agentfileops-typography-poster.png)

The primary message is deliberately short:

> Give agents files, not a shell.

The poster is suitable for project landing pages, social previews, talks, and repository documentation.

## Lifecycle visual

![AgentFileOps operation lifecycle](generated/agentfileops-workflow-visual.png)

Use this visual as the editorial companion to the canonical flow:

`intent → operation → resolution → risk review → backend execution → result → audit`

For exact technical documentation, pair it with:

- [Architecture diagram](../design/architecture.svg)
- [Data-flow diagram](../design/data-flow.svg)
- [Risk-level diagram](../design/risk-levels.svg)
- [Design system](../DESIGN_SYSTEM.md)

## Asset map

| Asset | Primary job | Recommended placement |
|---|---|---|
| `agentfileops-hero.png` | Establish the product category and promise | README, landing page, project overview |
| `agentfileops-use-cases.png` | Show provider-neutral deployment destinations | README, docs landing page, presentations |
| `agentfileops-typography-poster.png` | Make the product message memorable | Social preview, talks, docs cover |
| `agentfileops-workflow-visual.png` | Explain the operation lifecycle at a glance | Examples, architecture overview, onboarding |

## Provenance

Generated on 2026-08-18 with the built-in image-generation workflow using the AgentFileOps navy/cyan/green visual language, amber review state, and red high-risk state. Generated imagery is illustrative; protocol claims, labels, and safety controls remain defined by the repository's Markdown, JSON, Rust, tests, and SVG sources.
