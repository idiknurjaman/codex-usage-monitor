# 2S Multi-Account Monitoring — Test Matrix

**Plan:** `2s-multi-account-monitoring`
**Status:** `Plan complete; Phase 05 PASS`
**Product policy:** maximum four retained accounts; collection-driven/N-capable implementation

Evidence must be current to the exact implementation checkpoint under review. Historical PASS results prove only their historical checkpoints and do not prove the amended account model.

| ID | Area | Scenario | Required result | Evidence |
|---|---|---|---|---|
| AUTH-01 | Working account | First launch while normal Codex is authenticated as A | A identity discovered automatically; normal credential unchanged | Class R — PASS bounded: Phase 02/03 owner runtime auto-discovered the working account without manual Add; normal Codex remained the working source |
| AUTH-02 | Manual add | Add distinct monitor account B while Codex remains A | B receives isolated monitor owner; normal Codex remains A | Class R — PASS bounded: Phase 02 owner runtime completed manual Add while normal Codex remained on A |
| AUTH-03 | Refresh | Monitor credential requires refresh | Refresh succeeds without inference; other owners unchanged | Class R — PASS bounded: Phase 00 direct pinned `codex-login` refresh proof; no inference path |
| AUTH-04 | Ownership | Restart with multiple retained accounts | Monitor owners restore by stable identity; no token copying | Class R — PASS bounded: Phase 00 direct harness restored A/B independently; Phase 02 restart retained identities and recomputed active role |
| AUTH-05 | Cleanup | Remove inactive retained account B | Only B monitor owner/retention removed; working Codex and others unchanged | Class R — PASS bounded: Phase 02/03 owner runtime removed inactive B while A remained healthy |
| AUTH-06 | Security | Inspect settings/logs/diagnostics/UI/evidence | No raw token, OAuth code, email, or account ID leakage | Class R/F/S — PASS: Phase 00–04 evidence and source/tests preserve secret-safe surfaces |
| AUTH-07 | Duplicate | Manual login resolves to an already-known identity | Reconcile/attach owner to one identity row; no duplicate account | Class R — PASS bounded: known B manual Add reconciled to one row; deterministic dedupe tests also PASS |
| AUTH-08 | Capacity | Four retained accounts already exist | `Add monitor account...` disabled; no OAuth/browser login starts | Class F/S — PASS: four-account fixture and preflight test reject before login dispatch; no synthetic OAuth |
| AUTH-09 | Auto-discovery boundary | Working A is auto-discovered without monitor owner | No working refresh-token copy; independent monitoring claimed only after safe owner exists | Class R + F/S — PASS: owner runtime plus source/test proof preserve working-owner separation |
| AUTH-10 | Full-capacity current switch | Four retained accounts; normal Codex changes to unknown E | E recognized current-only; no silent eviction/owner theft | Class F — PASS: full-capacity current-only fixture preserves A/B/C/D and owners |
| AUTH-11 | Owner uniqueness | Two identities attempt same monitor owner | Rejected/reconciled; one owner belongs to at most one identity | Class F/S — PASS: registry add/reconstruction invariant tests |
| AUTH-12 | Re-auth missing owner | Retained B requires re-auth but old credential is absent | Re-auth can start; B identity validated; A/others untouched | Class F/S — PASS: missing-owner re-auth start and identity validation state tests |
| AUTH-13 | Transaction rollback | Cancel/timeout/failure after possible auth mutation | Previous owner restored or new owner cleared; rollback failure explicit | Class F/S — PASS: cancel, timeout, mutation, failure, and rollback-failure tests |
| AUTH-14 | Identity mismatch | Re-auth B resolves to different identity X | B not silently replaced; owner state safely rolled back/reconciled | Class F/S — PASS: identity-mismatch rollback tests |
| DATA-01 | Semantics | Codex API reports used 81/55 | Canonical used values remain 81/55; UI remaining 19/45 | Class S — PASS: poller preserves canonical used values and presentation converts to remaining |
| DATA-02 | Missing window | Weekly-only response | Session unavailable, not 0%; weekly correct | Class S — PASS: weekly-only and missing-window tests |
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
| LIFE-01 | Auto discovery | Startup with working A | A appears without manual Add | Class R — PASS bounded: owner runtime auto-discovery showed A without Add |
| LIFE-02 | Active switch | Working Codex changes A → B | B active; A remains retained; no duplicate row | Class R — PASS bounded: A→B switch moved active role and retained A without duplication |
| LIFE-03 | Manual add | Add B while A active | B added/owner provisioned without switching Codex | Class R — PASS bounded: manual Add/reconcile completed while A remained current |
| LIFE-04 | Cancel | Cancel/timeout manual Add | Registry and credential ownership remain consistent | Class F/S — PASS: transactional cancel/timeout rollback tests; no separate real cancel event claimed |
| LIFE-05 | Re-auth | Re-auth B as B | B recovers; all other accounts untouched | Class R — PASS bounded: selected-account re-auth completed with other account/normal Codex unchanged |
| LIFE-06 | Remove inactive | Remove B while A active | B removed only; A active/healthy | Class R — PASS bounded: inactive B removal left A healthy |
| LIFE-07 | Remove active | Remove current A from monitor | Normal Codex stays A; retained/monitor ownership removed; A remains current-only while active | Class R — PASS bounded: current removal preserved normal Codex and current-only state |
| LIFE-08 | Restart | Restart after mixed auto/manual discovery | Retained identities restore; active role recomputed from current Codex | Class R — PASS bounded: A/B restart restored retained rows and recomputed active role |
| LIFE-09 | Max four | Retain A/B/C/D through valid paths | Four retained identities supported without structural slot hard-coding | Class F/S — PASS: four-account collection/capacity tests |
| LIFE-10 | Current overflow | With A/B/C/D retained, Codex changes to E | E current-only; A/B/C/D retained; user can free capacity explicitly | Class F — PASS: full-capacity current-only no-eviction fixture |
| UI-01 | Single account | Bar style | Existing accepted single-account visual remains readable | Class R — PASS bounded: real Bar rendering accepted; earlier unchanged-renderer proof remains applicable; no extra Phase 05 claim |
| UI-02 | Single account | Circle style | Initial + two quota rows remain readable | Class R — PASS bounded: real Circle rendering accepted; earlier unchanged-renderer proof remains applicable |
| UI-03 | Active marker | Current account in widget | Subtle blue outline/ring on identity chip; inactive chips neutral | Class R — PASS: blue ring followed runtime Codex active identity; Class F/S supports state seam |
| UI-04 | Tooltip target | Hover initial, 5h, weekly, bar, %, reset areas | Same account tooltip opens from entire account block | Class R — PASS: whole account-block hover accepted; Class S hit-test proof current |
| UI-05 | Tooltip content | Current and monitored account tooltips | Name, ACTIVE badge when current, role, 5h/weekly remaining, exact reset, connection status | Class R — PASS: current-checkpoint tooltip hierarchy/status accepted |
| UI-06 | Tooltip privacy | Inspect tooltip | No email, opaque ID, token, OAuth material by default | Class R — PASS bounded: account-scoped privacy-safe tooltip accepted |
| UI-07 | Two accounts | Bar/Circle | Both account groups readable; attribution correct | Class R — PASS bounded: real two-account rendering and attribution accepted |
| UI-08 | Four accounts | Representative Bar/Circle runtime state | Four retained accounts readable; no action/identity collision | Class F — PASS: deterministic four-account fixture; no four-real-account claim |
| UI-09 | Context menu | Accounts menu with multiple identities | Direct per-account submenu; no nested `Manage accounts >` | Class R — PASS bounded: direct Accounts menu accepted; dynamic route tests remain Class S |
| UI-10 | Menu action | Account submenu | `Re-authenticate` + `Remove from monitor` route to selected identity | Class R — PASS bounded: existing direct action walkthrough remains applicable |
| UI-11 | Add disabled | Four retained accounts | `Add monitor account...` visibly disabled | Class F/S — PASS: deterministic preflight and fixture proof; no new runtime capacity claim |
| UI-12 | Same initial | Distinct accounts derive same initial | Both render/manage independently by stable identity | Class F/S — PASS: same-initial deterministic proof; no real same-initial claim |
| UI-13 | Failure state | One account unavailable | Healthy accounts remain readable; failed account explicit | Class R — PASS bounded: inactive ownerless re-auth-required and independent-owner health accepted; transient isolation remains Class F/S |
| UI-14 | Overflow state | Four retained + current-only E | Current identity remains understandable; no retained account silently dropped | Class F/S — PASS: current-only overflow fixture; no real E claim |
| UI-15 | DPI | 100/125/150/200% smoke test | No material clipping/overlap in accepted layouts | Class F/S — PASS for scalable geometry; no deliberate owner multi-DPI walkthrough claimed |
| UI-16 | Taskbar | Drag/multi-monitor placement | Existing placement behavior preserved | Class R — PASS bounded: diagnostic runtime recorded two taskbars, selection of index 1 with repositioning, then return to index 0 |
| UI-17 | Shell | Explorer/taskbar restart | Widget recovers using existing reliability mechanism | Class R — PASS bounded: controlled Explorer restart produced a new 2S process, a new taskbar selection with `count=2`, and `window shown` |
| UI-18 | Theme | Tooltip/menu theme | Tooltip has no visible stroke; native HMENU follows Windows light/dark theme when supported and falls back safely | Class R — PASS bounded: Sidik confirmed light-theme tooltip/native menu behavior; bar/background theme is outside plan and non-blocking |
| UI-19 | Three accounts | Real taskbar smoke after normal Codex discovery | Third account renders as a distinct block; returning to account 1 leaves account 3 retained and ownerless-inactive without credential borrowing | Class R — PASS bounded: third account discovered by becoming current; after return to account 1 it remained retained and entered `Re-authentication required` |
| ARCH-01 | Account shape | Inspect account/auth types | No capacity encoded as `Slot1|Slot2|Slot3|Slot4` variants | Class S — PASS: collection/N-capable account model with max-four policy |
| ARCH-02 | Routing | Inspect lifecycle/menu/poll/render code | No `account_a/b`, per-slot poll functions, or fixed per-slot command branches | Class S — PASS: lifecycle/menu/poll/render paths iterate stable-identity collection |
| ARCH-03 | Policy | Inspect capacity enforcement | Max four retained accounts enforced as policy/validation | Class F/S — PASS: max-four policy and preflight tests |
| MIG-01 | Settings | Existing settings without registry | App starts with valid working-account behavior | Class F/S — PASS: empty-registry legacy fallback and settings round-trip tests |
| MIG-02 | Two-slot metadata | Existing Phase-01/02 `slot-1/slot-2` settings | Migrates/reconstructs safely into generalized owner-handle model | Class F/S — PASS: typed legacy slot metadata and owner-continuity tests |
| BUILD-01 | Formatting | `cargo fmt --check` | PASS | Class S — PASS at `1cda2b33c7ed30166f364b26687a8876d347865d` |
| BUILD-02 | Tests | `cargo test` | PASS | Class S — PASS: 91/91 tests at `1cda2b33c7ed30166f364b26687a8876d347865d` |
| BUILD-03 | Lint | `cargo clippy --all-targets` | No new plan-caused warnings/errors | Class S — PASS exit; existing warnings only at `1cda2b33c7ed30166f364b26687a8876d347865d` |
| BUILD-04 | Release build | `cargo build --release` | PASS | Class S — PASS: optimized release build at `1cda2b33c7ed30166f364b26687a8876d347865d` |
| BUILD-05 | Diff hygiene | `git diff --check` | PASS | Class S — PASS at `1cda2b33c7ed30166f364b26687a8876d347865d` |
| SEC-01 | Working state | Compare normal Codex state before/after lifecycle | Auth/session/history/config source of truth not mutated by monitor management | Class R — PASS bounded: Phase 00 controlled quiet interval proved stable normal markers; owner-attributed normal Codex switching is not attributed to monitor |
| SEC-02 | Token ownership | Review monitor owners | No refresh token duplicated across independently refreshing owners | Class R/S — PASS: Phase 00 independent owner/refresh/deletion proof plus current source boundaries |
| SEC-03 | Active role | Inspect persistence | Active/current role not persisted as credential ownership | Class F/S — PASS: runtime active matching and secret-free metadata tests |

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
