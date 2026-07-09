---
type: adr
title: "ADR-0001: Record architecture decisions"
description: "Record architecture decisions using lightweight ADRs."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0001]
timestamp: 2026-07-09
---

# ADR-0001: Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR template](../template/), [ADR index](../)

## Context

gh-qwt is a greenfield project, so many foundational choices will be made before there is much implementation history to inspect. Those choices affect command semantics, filesystem layout, release packaging, and integration with GitHub CLI and Git. Future maintainers need to know not only what was chosen, but why competing options were rejected.

A heavyweight architecture process would slow the project down, while relying on issue comments or commit messages would make rationale hard to find. We need a lightweight, durable format that lives with the code and can be reviewed in the same workflow as implementation changes.

## Decision

We will record architecture decisions as lightweight Nygard-style ADRs stored in `docs/development/adr/NNNN-slug/README.md`. Each ADR will document one decision, its context, consequences, alternatives, and references. ADRs are immutable after acceptance except for typo fixes or metadata maintenance. If a decision changes, we will create a new ADR that supersedes the old one rather than rewriting history.

## Consequences

### Positive
- Maintainers can recover the rationale for foundational decisions without searching chat logs or issues.
- Decisions are versioned, reviewable, and colocated with project documentation.
- Small ADRs encourage focused discussion and reduce architectural drift.

### Negative
- Contributors must spend time writing and updating decision records.
- The ADR set can become stale if superseding decisions are not recorded promptly.

### Neutral
- ADRs describe decisions; they do not replace user documentation or implementation specs.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| No ADRs | Rationale would be scattered across issues, commits, and memory, making later maintenance harder. |
| A single decisions document | A growing monolithic document would be harder to review, link to, and supersede decision by decision. |
| A wiki | Wiki pages are outside the normal code-review workflow and may drift from the repository state. |

## References

- Michael Nygard, “Documenting Architecture Decisions”
- [ADR template](../template/)
