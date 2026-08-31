# Plans

This directory contains active implementation plans for this fork.

## Active

- [`2s-multi-account-monitoring`](./2s-multi-account-monitoring/README.md) — collection-driven Codex account monitoring with automatic current-account discovery, manual monitor-account add, runtime active-account attribution, and a product policy of up to four retained accounts without account switching.

`2S` remains the project codename. It is not a two-slot capacity contract.

## Current 2S authority amendments

For the active 2S plan, read both amendments before executing or gating current work:

1. [`PRODUCT-AMENDMENT-2026-08-31.md`](./2s-multi-account-monitoring/PRODUCT-AMENDMENT-2026-08-31.md) — product/account model, max-four policy, active-role and credential boundaries, menu/tooltip direction.
2. [`PROOF-CONTRACT-AMENDMENT-2026-09-01.md`](./2s-multi-account-monitoring/PROOF-CONTRACT-AMENDMENT-2026-09-01.md) — authoritative proof modality for Phase 02 onward. Real credential-sensitive behavior is proven with available real accounts; higher-cardinality collection/capacity behavior may use deterministic non-secret fixtures. This amendment supersedes older phase/test wording that requires three, four, or five distinct real accounts solely for count/capacity proof.

The max-four product requirement is unchanged. Only the accepted proof mode changes where additional real-account fixtures are unavailable and do not add credential-specific coverage.

## Execution convention

Each plan owns its scope, hard stops, phases, test matrix, evidence ledger, and approved product amendments. Execute phases in order unless the plan explicitly authorizes reconciliation work inside the current phase. A phase is not complete until its acceptance criteria are proven and recorded in the evidence ledger using the currently authoritative proof mode.

For the 2S plan, Sidik is final product authority, Luna is the implementation writer/executor, and Sol owns architecture review, plan authoring/amendment, phase-gate audit, and final closure audit.
