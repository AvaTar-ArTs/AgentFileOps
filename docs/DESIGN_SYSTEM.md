# AgentFileOps Design System

**Give agents files, not a shell.**

Version 1.0 · 2026-08-18

## Purpose

This document defines the visual language for AgentFileOps documentation, dashboards, diagrams, and operator-facing surfaces. It emphasizes semantic clarity, visible safety controls, and operational trust.

## Brand

- **Promise:** Safe, semantic remote filesystem operations for AI agents.
- **Category:** Remote file operations for AI agents and MCP clients.
- **Tone:** Technical, calm, precise, trustworthy, empowering.
- **Primary message:** Give agents files, not a shell.
- **Supporting message:** SSH/SFTP, sync, deployment, archives, checksums, approvals, and audit trails without arbitrary shell execution.

## Color palette

| Token | Hex | Use |
|---|---|---|
| Navy | #0F1E37 | Headers, dark surfaces, primary text |
| Cyan | #00C8FF | Links, focus, active/read states |
| Green | #108040 | Approved, additive, and success states |
| Amber | #FFB400 | Review-required modifications |
| Orange | #FF8C42 | Destructive single-target warnings |
| Red | #DC3545 | Critical operations and errors |
| Ink | #162033 | Body text |
| Muted | #6C757D | Captions and secondary text |
| Cloud | #F4F7FA | Page backgrounds |
| Line | #DDE5EC | Borders and dividers |

Spacing uses a 4px base with preferred steps of 4, 8, 16, 24, 32, and 48px. Controls use an 8px radius, cards use 12px, and pills use 999px. Maximum content width is 1180px.

## Typography

Use Inter or a system sans-serif stack for interface text and headings:

    Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif

Use a system monospace stack for paths, commands, identifiers, and protocol fields:

    ui-monospace, SFMono-Regular, Menlo, Consolas, monospace

Recommended scale:

| Element | Size | Weight |
|---|---:|---:|
| H1 | clamp(2rem, 5vw, 3.5rem) | 750 |
| H2 | 1.75rem | 700 |
| H3 | 1.1rem | 700 |
| Body | 1rem / 1.6 | 400 |
| Caption | .8rem / 1.4 | 400 |

## Risk model

Risk must always be communicated with text and, where useful, an icon; color alone is insufficient.

| Level | Meaning | Examples | Approval |
|---|---|---|---|
| L0 | Read-only | list, stat, find, read, checksum | Automatic |
| L1 | Additive | mkdir, new upload, copy to a new path | Policy-dependent |
| L2 | Replacement | overwrite, move, chmod, symlink | Review recommended |
| L3 | Destructive | Single delete | Explicit approval |
| L4 | High impact | Recursive delete, bulk delete, sync-with-delete | Staged approval |

## Components

Buttons require visible labels, a minimum 44px touch target, and a 2px cyan focus ring. Primary actions use navy and cyan; approval actions use green; destructive actions use red.

Cards use a one-pixel border, 12px radius, 24px padding, and one clear primary action. Status badges should use explicit labels such as:

- ✓ Connected
- ⚠ Review required
- ✕ Blocked
- ○ Inactive

Inputs use a two-pixel border, 8px radius, 12px vertical padding, and a cyan focus state. Progress bars use a neutral track with a cyan-to-green fill.

## Layout

Use a centered container with a maximum width of 1180px. Desktop layouts may use a 12-column grid with 24px gutters. Tablet layouts may use two columns. Mobile layouts stack content into one column.

Keep operational status, target information, and risk context adjacent to the action they qualify.

## Diagram rules

Architecture diagrams should read top-to-bottom:

Agent or MCP client → AgentFileOps protocol → safety and risk engine → connection/backend resolver → SSH/SFTP transport → remote filesystem.

Data-flow diagrams should distinguish:

Descriptor → validator → executor → result

The audit trail should visibly record the decision, actor, target fingerprint, and timestamp. Repository SVGs are self-contained, accessible, and text-readable.

## Accessibility

- Meet WCAG AA for normal text and controls.
- Preserve visible keyboard focus.
- Provide meaningful titles and descriptions in SVGs.
- Never use color as the only state indicator.
- Use navy or ink for normal text on light backgrounds.
- Use cyan-on-navy primarily for large text, borders, and accents.
- Respect prefers-reduced-motion.
- Keep interactive targets at least 44px where practical.

## Motion

Default transitions are 180–240ms. Pulse and spin are reserved for connection and loading indicators. Disable nonessential animation when reduced motion is requested.

## Voice and microcopy

Prefer:

- “Operation requires approval.”
- “Could not connect. Verify the credential reference and known-host policy.”
- “Delete operation rejected. The target is read-only.”

Avoid:

- “Danger!!!”
- “Invalid input.”
- “Operation failed.”

## Repository assets

- [Design tokens](design/tokens.json)
- [Architecture diagram](design/architecture.svg)
- [Risk classification diagram](design/risk-levels.svg)
- [Operation data-flow diagram](design/data-flow.svg)
- [Pages dashboard](site/index.html)

This document describes presentation. Protocol schemas, security policy, and conformance tests remain authoritative for behavior.
