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

**Status:** `BLOCKED`

**Evidence checkpoint:** `ed2e8fb0dd9a556f087a0d2f7c6b384a50bd14db`

#### Harness and authority map

- Installed Codex runtime: `codex-cli 0.151.0-alpha.7.1`.
- `codex login status`: working ChatGPT login present; no credential content was recorded.
- The smallest spike used the installed `codex app-server` over stdio. No production source or account-manager harness was added.
- Generated app-server schema exposes `account/login/start`, `account/login/cancel`, `account/read`, and `account/rateLimits/read`.
- The managed `chatgpt` auth mode is Codex-owned persistence/refresh. The `chatgptAuthTokens` mode is marked unstable/internal and requires an external host to own refresh; it is not an approved foundation.
- No auth mechanism is approved for Phase 01 because isolated credential ownership was not proven.

#### Read-only working-account runtime proof

The sequential JSON-RPC smoke test sent only `initialize`, `initialized`, `account/read` with `refreshToken:false`, and `account/rateLimits/read`. It did not send `command/exec`, `codex exec`, an inference request, login, logout, or refresh.

- `initialize`: PASS; the runtime reported its `codexHome` as the normal working `~/.codex`.
- `account/read(refreshToken:false)`: PASS for the existing working account; account object present, type `chatgpt`, plan type present. Email was intentionally omitted.
- `account/rateLimits/read`: PASS for that same working account. The snapshot returned a 5-hour window (`usedPercent=92`, `windowDurationMins=300`, `resetsAt=1788128406`) and weekly window (`usedPercent=30`, `windowDurationMins=10080`, `resetsAt=1788655142`). These are working-account observations, not Account A/B monitor proof.

#### Credential ownership and isolation

**BLOCKED.** The default app-server session resolved to the user's working `~/.codex`, so using it for a second monitor account would share the active credential/workspace boundary. No separate credential-owned monitor auth root was created because no approved owner boundary or second-account login was available. No `auth.json` copy, refresh-token duplication, hidden account switch, or second user-facing `CODEX_HOME` was attempted.

#### Account A/B distinct rate-limit proof

**BLOCKED.** There is one working-account read only. Account A as an isolated monitor owner, Account B as a distinct isolated owner, deterministic A/B attribution, and independent refresh ownership were not exercised or proven.

#### Refresh, restart, and zero-inference proof

**BLOCKED.** No refresh or re-auth was run because the only available app-server root was the working root. The app-server schema documents a Codex-managed refresh path through `account/read(refreshToken:true)`, but this was not exercised in an isolated owner. The existing monitor fallback still invokes `codex exec .` at `src/poller.rs:437-449`; it was not invoked during this spike, but the required zero-inference refresh alternative is therefore not proven.

No A+B monitor state was created, so restart survival and monitor-auth deletion without working-state deletion are also not proven.

#### Working Codex pre/post proof

The targeted baseline was captured before the app-server read and repeated afterward. No raw credential/token/email/content was recorded.

| Working state | Before | After | Result |
|---|---|---|---|
| `~/.codex/auth.json` | 4111 bytes; SHA-256 `E6E52E27882C5C85192862EBA73B125CFE04C436DB5578C330BC4FF84581C4BF` | identical | PASS |
| `~/.codex/config.toml` | 8630 bytes; SHA-256 `6F77B6DFCAAC205FBEFA00F75A130EC4DAFB7173B66F5739811BAD37A89561C5` | identical | PASS |
| `~/.codex/session_index.jsonl` | 33989 bytes; SHA-256 `00FF3B78B93CDCE25E12FD0DB826E4AE5AC811647F042F3FCB1766159F26B1EA` | identical | PASS |
| `~/.codex/history.jsonl` | absent | absent | PASS |
| `~/.codex/sessions/` | 354 files; 4134752524 bytes | 354 files; 4136598346 bytes | INCONCLUSIVE |

The sessions aggregate changed while this active Codex task continued writing its own session files. Because that concurrent writer cannot be separated from the read-only app-server lifecycle in this run, SEC-01 is not promoted to PASS.

#### Required Phase 00 IDs

| ID | Disposition | Evidence boundary |
|---|---|---|
| AUTH-01 | BLOCKED | No isolated monitor Account A login/owner was created. |
| AUTH-02 | BLOCKED | No distinct Account B login/owner or A/B attribution was created. |
| AUTH-03 | BLOCKED | Refresh was not exercised; zero-inference refresh remains unproven. |
| AUTH-04 | BLOCKED | No isolated A+B state existed to restart. |
| AUTH-06 | PARTIAL | This evidence contains no raw secret; full monitor settings/log audit was not possible because no monitor owner was created. |
| POLL-03 | PARTIAL | This run used no inference request; refresh-path zero-inference proof is missing. |
| SEC-01 | BLOCKED | Credential/config/index remained stable, but sessions were concurrently written by the active Codex task. |
| SEC-02 | BLOCKED | No two monitor owners were created, so refresh-token non-duplication cannot be proven. |

**Decision:** `BLOCKED`. The app-server account/rate-limit read interface is a viable candidate for further investigation, but Phase 00 acceptance is not met. Do not begin Phase 01.

**Next authorized action:** Sol/Sidik review this bounded result and, only if approved, define a credential-owned isolated test boundary and provide two distinct test accounts or an equivalent owner-approved runtime fixture. No production account registry, login lifecycle, multi-account polling, account switching, or two-account UI work is authorized.

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
