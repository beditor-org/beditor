# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for Beditor.

An ADR documents a significant architectural decision: what was decided, why, and what trade-offs were accepted. The goal is to give future contributors (and your future self) context that code alone cannot provide.

## Format

Each ADR is a markdown file with the following structure:

```
# Title

## Status: <status>

## Context
Why this decision was needed. What problem we were solving.

## Decision
What we decided to do.

## Consequences
What becomes easier or harder as a result.
```

Optionally, a **Considered Options** section can list alternatives that were evaluated and rejected at the time of writing — but only alternatives that were *not* tried in practice. If an approach was tried and abandoned, it gets its own ADR (see below).

## Statuses

| Status | Meaning |
|---|---|
| **Proposed** | Decision is being considered, not yet settled |
| **Accepted** | Active decision, currently in effect |
| **Superseded by ADR-XXX** | This decision was replaced; see the linked ADR |
| **Deprecated** | No longer relevant but kept for historical context |

## Naming

Files are numbered sequentially: `001-short-description.md`, `002-short-description.md`, etc.

## How to handle a change of direction

When an accepted decision is abandoned in favour of a different approach:

1. **Do not edit or delete the old ADR.** It documents what was true at that point in time and why it was chosen.
2. Change the old ADR's status to `Superseded by ADR-XXX`.
3. Create a new ADR that explains why the previous approach was abandoned and what replaces it.

This gives a chronological history: a reader can trace not just *what* the current approach is, but *why* earlier approaches were dropped.

## Further reading

- [Original ADR concept by Michael Nygard](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [adr.github.io — community resources and tooling](https://adr.github.io/)
- [Markdown Architectural Decision Records (MADR)](https://adr.github.io/madr/) — a more detailed template if needed in the future
