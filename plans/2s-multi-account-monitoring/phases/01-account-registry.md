# Phase 01 — Account Registry

**Status:** `in-progress`
**Goal:** introduce a stable multi-account domain model and move account identity/auth ownership out of the window renderer.

## Preconditions

- Phase 00 is PASS.
- `EVIDENCE.md` names one approved monitor credential/auth mechanism.
- Working Codex state remains outside 2S ownership.

## Required model

Introduce an account domain that can represent up to two monitored Codex accounts without exposing credentials to UI code.

Recommended logical shape:

```text
MonitoredAccount
- id                 stable, non-secret local identifier
- initial            one uppercase display character
- enabled            bool
- auth_handle         opaque reference to credential owner
- connection_state    connected | reauth_required | unavailable
- usage               optional per-account UsageData
- last_success_at      optional timestamp
- last_error           optional categorized error
```

Names may differ if the codebase suggests a better Rust shape, but these responsibilities must remain separate.

## Architecture requirements

- `window.rs` must not parse JWTs or open Codex auth files to derive account identity.
- UI receives account identity/usage through domain state.
- Credential/token material must not be serialized into `settings.json`.
- Account registry metadata may persist only non-secret fields needed to reconnect to an approved credential owner.
- Maximum account count is two for this plan.
- Account order is deterministic and persisted.
- Duplicate detection uses a stable account identity when available, not the one-letter initial.
- Two accounts with the same initial are valid.

## Tasks

- [ ] Add account-domain types in an appropriate module instead of `window.rs`.
- [ ] Move current Codex initial derivation out of the renderer.
- [ ] Add stable account identity derivation based on the approved Phase 00 mechanism.
- [ ] Add persisted non-secret account registry metadata.
- [ ] Enforce maximum two monitored Codex accounts.
- [ ] Add deterministic ordering.
- [ ] Add duplicate-account detection independent of initial.
- [ ] Preserve current single-account behavior through an adapter/migration path.
- [ ] Add unit tests for identity derivation, duplicate detection, max-two enforcement, and same-initial accounts.

## Acceptance criteria

- Existing single-account users start without losing settings.
- Renderer no longer owns credential parsing.
- Registry can represent zero, one, or two monitored accounts.
- Same-initial accounts do not collide.
- No secret appears in settings serialization or diagnostics.
- Existing usage semantics remain unchanged.

## Hard stops

- Do not implement login UI in this phase.
- Do not implement polling fan-out in this phase.
- Do not introduce account switching.
- Do not invent another auth storage strategy if Phase 00 evidence becomes inconvenient. Re-open Phase 00 instead.

## Evidence

Record changed files, tests, and the final account model in `../EVIDENCE.md` before marking this phase complete.
