# 2S Multi-Account Monitoring — Test Matrix

**Plan:** `2s-multi-account-monitoring`
**Status:** `Phase 03 PASS; Phase 04 in progress`
**Product policy:** maximum four retained accounts; collection-driven/N-capable implementation

Evidence must be current to the exact implementation checkpoint under review. Historical PASS results prove only their historical checkpoints and do not prove the amended account model.

| ID | Area | Scenario | Required result | Evidence |
|---|---|---|---|---|
| AUTH-01 | Working account | First launch while normal Codex is authenticated as A | A identity discovered automatically; normal credential unchanged | pending |
| AUTH-02 | Manual add | Add distinct monitor account B while Codex remains A | B receives isolated monitor owner; normal Codex remains A | pending |
| AUTH-03 | Refresh | Monitor credential requires refresh | Refresh succeeds without inference; other owners unchanged | pending |
| AUTH-04 | Ownership | Restart with multiple retained accounts | Monitor owners restore by stable identity; no token copying | pending |
| AUTH-05 | Cleanup | Remove inactive retained account B | Only B monitor owner/retention removed; working Codex and others unchanged | pending |
| AUTH-06 | Security | Inspect settings/logs/diagnostics/UI/evidence | No raw token, OAuth code, email, or account ID leakage | pending |
| AUTH-07 | Duplicate | Manual login resolves to an already-known identity | Reconcile/attach owner to one identity row; no duplicate account | pending |
| AUTH-08 | Capacity | Four retained accounts already exist | `Add monitor account...` disabled; no OAuth/browser login starts | pending |
| AUTH-09 | Auto-discovery boundary | Working A is auto-discovered without monitor owner | No working refresh-token copy; independent monitoring claimed only after safe owner exists | pending |
| AUTH-10 | Full-capacity current switch | Four retained accounts; normal Codex changes to unknown E | E recognized current-only; no silent eviction/owner theft | pending |
| AUTH-11 | Owner uniqueness | Two identities attempt same monitor owner | Rejected/reconciled; one owner belongs to at most one identity | pending |
| AUTH-12 | Re-auth missing owner | Retained B requires re-auth but old credential is absent | Re-auth can start; B identity validated; A/others untouched | pending |
| AUTH-13 | Transaction rollback | Cancel/timeout/failure after possible auth mutation | Previous owner restored or new owner cleared; rollback failure explicit | pending |
| AUTH-14 | Identity mismatch | Re-auth B resolves to different identity X | B not silently replaced; owner state safely rolled back/reconciled | pending |
| DATA-01 | Semantics | Codex API reports used 81/55 | Canonical used values remain 81/55; UI remaining 19/45 | pending |
| DATA-02 | Missing window | Weekly-only response | Session unavailable, not 0%; weekly correct | pending |
| DATA-03 | Attribution | Multiple accounts return different quota values | Values stay attached to correct stable identities | Class R — PASS: Sidik observed distinct A/B usage with correct attribution; Class F/S result-application proof also PASS |
| DATA-04 | Reset | Accounts have different reset times | Each account/window keeps its own countdown/reset | Class S — PASS: per-account reset state and collection scheduling are implemented; no separate reset-time runtime claim |
| DATA-05 | Active role | Working identity changes A → B | Active marker moves to B; quota ownership does not move between identities | Class R — PASS: A↔B switches preserved attribution and retention |
| POLL-01 | Collection | Poll 1, 2, and 4 retained accounts | Same collection-driven orchestration; no fixed account branches | Class F/S — PASS: collection plan iterates identities and independent owners |
| POLL-02 | Isolation | A poll succeeds; B fails; C/D healthy | Healthy accounts continue updating; B failure account-scoped | Class R + F/S — PASS: A remained healthy while B became partial/unavailable; deterministic scoped-failure test PASS |
| POLL-03 | Auth isolation | B monitor auth expires | Only B requests re-auth; other accounts continue | PASS: Phase 00 Class R owner refresh/isolation proof + Phase 03 Class S account-scoped failure regression; deliberately forced expiry runtime event not performed |
| POLL-04 | Zero inference | Observe monitor poll/refresh process behavior | No `codex exec` or intentional inference request | PASS: Phase 00 Class R zero-inference refresh proof + Phase 03 Class S source/path regression |
| POLL-05 | Current-only | Unknown active E at full retained capacity | Safe working-account usage may display; no retained owner borrowed/evicted | Class R + F — PASS: ownerless current-only behavior observed; full-capacity no-eviction fixture PASS |
| ALERT-01 | Scope | Multiple accounts cross same threshold | Each stable identity can notify independently | Class F/S — PASS: opaque per-identity alert fixture PASS |
| ALERT-02 | Dedup | Same account/window repeatedly polled before reset | Notification emitted once per identity/window/threshold/reset | Class F/S — PASS: per-identity/window/threshold/reset dedup fixture PASS |
| LIFE-01 | Auto discovery | Startup with working A | A appears without manual Add | pending |
| LIFE-02 | Active switch | Working Codex changes A → B | B active; A remains retained; no duplicate row | pending |
| LIFE-03 | Manual add | Add B while A active | B added/owner provisioned without switching Codex | pending |
| LIFE-04 | Cancel | Cancel/timeout manual Add | Registry and credential ownership remain consistent | pending |
| LIFE-05 | Re-auth | Re-auth B as B | B recovers; all other accounts untouched | pending |
| LIFE-06 | Remove inactive | Remove B while A active | B removed only; A active/healthy | pending |
| LIFE-07 | Remove active | Remove current A from monitor | Normal Codex stays A; retained/monitor ownership removed; A remains current-only while active | pending |
| LIFE-08 | Restart | Restart after mixed auto/manual discovery | Retained identities restore; active role recomputed from current Codex | pending |
| LIFE-09 | Max four | Retain A/B/C/D through valid paths | Four retained identities supported without structural slot hard-coding | pending |
| LIFE-10 | Current overflow | With A/B/C/D retained, Codex changes to E | E current-only; A/B/C/D retained; user can free capacity explicitly | pending |
| UI-01 | Single account | Bar style | Existing accepted single-account visual remains readable | pending |
| UI-02 | Single account | Circle style | Initial + two quota rows remain readable | pending |
| UI-03 | Active marker | Current account in widget | Subtle blue outline/ring on identity chip; inactive chips neutral | pending |
| UI-04 | Tooltip target | Hover initial, 5h, weekly, bar, %, reset areas | Same account tooltip opens from entire account block | pending |
| UI-05 | Tooltip content | Current and monitored account tooltips | Name, ACTIVE badge when current, role, 5h/weekly remaining, exact reset, connection status | pending |
| UI-06 | Tooltip privacy | Inspect tooltip | No email, opaque ID, token, OAuth material by default | pending |
| UI-07 | Two accounts | Bar/Circle | Both account groups readable; attribution correct | pending |
| UI-08 | Four accounts | Representative Bar/Circle runtime state | Four retained accounts readable; no action/identity collision | pending |
| UI-09 | Context menu | Accounts menu with multiple identities | Direct per-account submenu; no nested `Manage accounts >` | pending |
| UI-10 | Menu action | Account submenu | `Re-authenticate` + `Remove from monitor` route to selected identity | pending |
| UI-11 | Add disabled | Four retained accounts | `Add monitor account...` visibly disabled | pending |
| UI-12 | Same initial | Distinct accounts derive same initial | Both render/manage independently by stable identity | pending |
| UI-13 | Failure state | One account unavailable | Healthy accounts remain readable; failed account explicit | pending |
| UI-14 | Overflow state | Four retained + current-only E | Current identity remains understandable; no retained account silently dropped | pending |
| UI-15 | DPI | 100/125/150/200% smoke test | No material clipping/overlap in accepted layouts | pending |
| UI-16 | Taskbar | Drag/multi-monitor placement | Existing placement behavior preserved | pending |
| UI-17 | Shell | Explorer/taskbar restart | Widget recovers using existing reliability mechanism | pending |
| ARCH-01 | Account shape | Inspect account/auth types | No capacity encoded as `Slot1|Slot2|Slot3|Slot4` variants | pending |
| ARCH-02 | Routing | Inspect lifecycle/menu/poll/render code | No `account_a/b`, per-slot poll functions, or fixed per-slot command branches | pending |
| ARCH-03 | Policy | Inspect capacity enforcement | Max four retained accounts enforced as policy/validation | pending |
| MIG-01 | Settings | Existing settings without registry | App starts with valid working-account behavior | pending |
| MIG-02 | Two-slot metadata | Existing Phase-01/02 `slot-1/slot-2` settings | Migrates/reconstructs safely into generalized owner-handle model | pending |
| BUILD-01 | Formatting | `cargo fmt --check` | PASS | Class S — PASS at `acb870d204d83bcba71534aeab5910784e74b1c1` |
| BUILD-02 | Tests | `cargo test` | PASS | Class S — PASS: 76/76 tests at `acb870d204d83bcba71534aeab5910784e74b1c1` |
| BUILD-03 | Lint | `cargo clippy --all-targets` | No new plan-caused warnings/errors | Class S — PASS exit; existing warnings only |
| BUILD-04 | Release build | `cargo build --release` | PASS | Class S — PASS: optimized release build completed |
| BUILD-05 | Diff hygiene | `git diff --check` | PASS | Class S — PASS |
| SEC-01 | Working state | Compare normal Codex state before/after lifecycle | Auth/session/history/config source of truth not mutated by monitor management | pending |
| SEC-02 | Token ownership | Review monitor owners | No refresh token duplicated across independently refreshing owners | pending |
| SEC-03 | Active role | Inspect persistence | Active/current role not persisted as credential ownership | pending |

## Required screenshots/runtime artifacts

- One-account Bar.
- One-account Circle.
- Two-account representative state.
- Four-retained-account representative state.
- Active blue-ring state.
- Whole-account tooltip for current account.
- Whole-account tooltip for monitored account.
- Direct Accounts context menu and one account submenu.
- Fifth manual Add disabled state.
- One healthy + one re-auth/unavailable state.
- Full-capacity current-only overflow state if exercised by the final implementation.

Screenshots must not expose email, account IDs, access/refresh tokens, OAuth codes, or other credential material.
