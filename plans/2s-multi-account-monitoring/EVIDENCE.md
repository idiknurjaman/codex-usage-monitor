# 2S Multi-Account Monitoring — Evidence Ledger

## Current state

- **Plan:** `2s-multi-account-monitoring`
- **Lifecycle:** `planned`
- **Current phase:** `00-auth-spike`
- **Implementation branch:** `feat/2s-multi-account-monitoring`
- **Plan authoring checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1`
- **Implementation checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1` until Phase 00 creates a newer implementation commit

## Baseline evidence

Current branch already contains:

- explicit `used_percentage: Option<f64>` semantics;
- correct weekly-only/unavailable-window handling;
- remaining conversion at presentation boundaries for Codex;
- persisted Bar/Circle widget style;
- current-account initial derivation/rendering;
- monotonic Circle remaining progress.

Multi-account credential ownership, registry, login lifecycle, and polling are not yet implemented.

## Open findings / blockers

### BLOCKER-00 — monitor credential isolation not yet proven

Need runtime proof that two Codex accounts can be independently authenticated/read/refreshed for monitoring without replacing the user's working Codex credential or splitting the normal Codex sessions/history/config source of truth.

### BLOCKER-01 — existing Codex auth refresh path can invoke inference

Existing single-account fallback may invoke `codex exec "."` to force local Codex token refresh. Multi-account monitoring must not use this strategy. Phase 00 must prove a zero-inference alternative.

### FINDING-01 — identity parsing currently lives in UI layer

Current initial derivation is in `window.rs`. Phase 01 must move identity/auth concerns into an account domain so rendering consumes account identity rather than parsing credential files.

### FINDING-02 — current initial is startup-scoped

Current single-account initial is computed at startup. Multi-account work must make identity lifecycle explicit rather than relying on startup-only renderer state.

## Phase evidence

### Phase 00 — Auth Spike

**Status:** `not-run`

Record:

- approved auth mechanism;
- credential ownership/storage boundary;
- whether app-server/other Codex-managed refresh is used;
- proof of Account A and Account B distinct rate-limit reads;
- restart proof;
- working Codex pre/post mutation proof;
- zero-inference proof;
- PASS/BLOCKED decision.

### Phase 01 — Account Registry

**Status:** `blocked-by-phase-00`

Evidence pending.

### Phase 02 — Account Login & Lifecycle

**Status:** `blocked-by-phase-01`

Evidence pending.

### Phase 03 — Multi-Account Polling

**Status:** `blocked-by-phase-02`

Evidence pending.

### Phase 04 — Taskbar UI

**Status:** `blocked-by-phase-03`

Evidence pending.

### Phase 05 — Resilience & Acceptance

**Status:** `blocked-by-phase-04`

Evidence pending.

## Final audit

**Sol verdict:** `NOT READY — plan not executed`

Do not change this to PASS until the final implementation checkpoint has current TEST-MATRIX evidence and Sidik runtime acceptance.
