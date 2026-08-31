# Phase 01 — Account Registry

**Status:** `complete/PASS — capacity policy superseded`
**Goal:** introduce a stable multi-account domain model and move account identity/auth ownership out of the window renderer.

## Historical gate status

Phase 01 passed Sol review at implementation checkpoint `84c8e14d2ce3cef06be2cfd3925d575e5cb9076c` for these durable boundaries:

- stable identity separate from credential ownership;
- non-secret persisted metadata;
- deterministic registry ordering;
- duplicate detection by stable identity rather than initial;
- unique credential-owner ownership;
- renderer does not own auth/JWT parsing;
- legacy current-Codex identity remains ephemeral when no registry exists.

Those boundaries remain authoritative.

The original max-two/`Slot1|Slot2` capacity shape is **superseded** by the owner-approved product amendment in [`../PRODUCT-AMENDMENT-2026-08-31.md`](../PRODUCT-AMENDMENT-2026-08-31.md). Phase 01 is not reopened as a separate execution phase; Phase 02 is explicitly authorized to reconcile these implementation details while preserving the Phase 01 invariants above.

## Current canonical model requirements

The account domain must support a collection of stable account identities without exposing credentials to UI code.

Recommended logical shape:

```text
KnownAccount
- id                   stable identity
- initial/name          presentation metadata
- enabled/retained      persistent monitor intent
- auth_handle           optional opaque monitor credential owner
- connection_state      connected | reauth_required | unavailable
- usage                 optional per-account UsageData
- last_success_at       optional timestamp
- last_error            optional categorized error

ActiveAccountRole
- runtime-only identity match against current working Codex
```

Names may differ if the Rust code suggests a better shape, but responsibilities must remain separate.

## Architecture requirements

- `window.rs` must not parse JWTs or open Codex auth files to derive account identity.
- UI receives identity/usage/active-role projections through account/application state.
- Credential/token material must not be serialized into `settings.json`.
- Registry metadata may persist only non-secret fields needed to identify retained accounts and reconnect to approved credential owners.
- Account collection capability must be N-capable; current runtime product policy is **four retained accounts**.
- Capacity must be enforced by policy/validation, not by `Slot1|Slot2|Slot3|Slot4` type variants.
- Account order is deterministic and persisted.
- Duplicate detection uses stable account identity, never the one-letter initial.
- Same-initial accounts are valid.
- Monitor credential owner is optional until safe independent ownership exists.
- Active status is runtime-derived from working Codex identity and is never persisted as credential ownership.

## Reconciliation delegated to Phase 02

Phase 02 is authorized to:

- replace `MAX_MONITORED_ACCOUNTS = 2` with current max-four retained policy;
- replace fixed `MonitorAuthHandle::{Slot1, Slot2}` with a validated collection-friendly logical owner handle/index or equivalent;
- preserve uniqueness of credential ownership;
- preserve serialization compatibility where practical and add explicit migration when required;
- add automatic working-account discovery and runtime active-role projection;
- keep manual add as a second discovery/owner-provisioning path.

## Current acceptance invariants

- Existing single-account users start without losing settings.
- Renderer does not own credential parsing.
- Registry can represent a variable account collection subject to current max-four policy.
- Same-initial accounts do not collide.
- Duplicate stable identities reconcile rather than produce duplicate rows.
- No secret appears in settings serialization or diagnostics.
- Existing usage semantics remain unchanged.
- Active role and monitor credential ownership remain separate concepts.
- No account-position hard-coding is required by downstream polling/menu/rendering.

## Hard stops

- Do not use token copying to convert the working active credential into monitor ownership.
- Do not introduce account switching.
- Do not invent another auth storage strategy if Phase 00 evidence becomes inconvenient; re-open the auth proof instead.
- Do not preserve a fixed-slot type shape merely by expanding two variants to four.

## Evidence

Historical Phase 01 evidence remains valid for the durable identity/ownership boundaries at its exact checkpoint. New capacity/generalization evidence belongs to the amended Phase 02 implementation checkpoint and must be recorded as current evidence before Phase 02 can pass.
