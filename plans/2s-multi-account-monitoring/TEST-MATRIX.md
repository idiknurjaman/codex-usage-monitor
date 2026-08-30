# 2S Multi-Account Monitoring — Test Matrix

**Plan:** `2s-multi-account-monitoring`
**Status:** `not-run`

Evidence must be current to the exact implementation checkpoint under review. Historical PASS results do not prove a later checkpoint.

| ID | Area | Scenario | Required result | Evidence |
|---|---|---|---|---|
| AUTH-01 | Isolation | Add monitor Account A while working Codex is already authenticated | Monitor auth succeeds; working credential unchanged | pending |
| AUTH-02 | Isolation | Add distinct monitor Account B | B is independent from A and working Codex | pending |
| AUTH-03 | Refresh | Monitor credential requires refresh | Refresh succeeds without inference request | pending |
| AUTH-04 | Ownership | Restart with A+B configured | Both monitor auth owners restore correctly; no token copying | pending |
| AUTH-05 | Cleanup | Remove B | Only B monitor state is removed; working Codex and A unchanged | pending |
| AUTH-06 | Security | Inspect settings/logs/diagnostics | No raw token/credential content present | pending |
| AUTH-07 | Duplicate | Login same stable account twice | Duplicate rejected/merged without duplicate registry entry | pending |
| AUTH-08 | Capacity | Attempt third monitored account | Rejected cleanly; A+B remain intact | pending |
| DATA-01 | Semantics | Codex API reports used 81/55 | Canonical values remain used 81/55; UI remaining 19/45 | pending |
| DATA-02 | Missing window | Weekly-only response | Session remains unavailable, not 0%; weekly correct | pending |
| DATA-03 | Attribution | A+B return different quota values | Values remain attached to correct stable account | pending |
| DATA-04 | Reset | A+B have different reset times | Each account/window shows its own countdown | pending |
| POLL-01 | Isolation | A poll succeeds; B fails | A updates normally; B shows account-scoped failure | pending |
| POLL-02 | Isolation | B auth expires | A polling continues; only B requests re-auth | pending |
| POLL-03 | Zero inference | Observe poll/refresh process behavior | No `codex exec` or intentional inference request | pending |
| ALERT-01 | Scope | A and B both cross same threshold | Each account can notify independently | pending |
| ALERT-02 | Dedup | Same account/window polled repeatedly before reset | Notification emitted once for reset/threshold identity | pending |
| UI-01 | Single account | Bar style | Existing accepted single-account visual remains readable | pending |
| UI-02 | Single account | Circle style | Initial + two quota rows render correctly | pending |
| UI-03 | Two accounts | Bar style | Both account groups readable; no clipping | pending |
| UI-04 | Two accounts | Circle style | Matches locked `[S] 5h ○ ... / 7d ○ ...` hierarchy | pending |
| UI-05 | Same initial | Two distinct accounts both derive `S` | Both render; registry identity does not collide | pending |
| UI-06 | Failure state | One account unavailable | Healthy account remains readable; failed account clearly unavailable | pending |
| UI-07 | DPI | 100/125/150/200% smoke test | No material clipping/overlap | pending |
| UI-08 | Taskbar | Drag/multi-monitor placement | Existing placement behavior preserved | pending |
| UI-09 | Shell | Explorer/taskbar restart | Widget recovers using existing reliability mechanism | pending |
| LIFE-01 | Add | Successful Add Account | Registry commit only after identity/duplicate validation | pending |
| LIFE-02 | Cancel | Cancel/timeout login | Existing registry unchanged | pending |
| LIFE-03 | Re-auth | Re-auth B as B | B recovers; A untouched | pending |
| LIFE-04 | Identity change | Re-auth B resolves to different stable account | No silent identity replacement | pending |
| MIG-01 | Settings | Existing settings without account registry | App starts with valid single-account behavior | pending |
| BUILD-01 | Formatting | `cargo fmt --check` | PASS | pending |
| BUILD-02 | Tests | `cargo test` | PASS | pending |
| BUILD-03 | Lint | `cargo clippy --all-targets` | No new plan-caused warnings/errors | pending |
| BUILD-04 | Release build | `cargo build --release` | PASS | pending |
| BUILD-05 | Diff hygiene | `git diff --check` | PASS | pending |
| SEC-01 | Working state | Compare working Codex state before/after monitor lifecycle | Auth/session/history/config source of truth not mutated by monitor account management | pending |
| SEC-02 | Token ownership | Review persisted monitor auth owners | No refresh token is duplicated across independently refreshing owners | pending |

## Required screenshots/runtime artifacts

- One-account Bar.
- One-account Circle.
- Two-account Bar.
- Two-account Circle.
- One healthy + one re-auth/unavailable state.

Screenshots must not expose emails, account IDs, access/refresh tokens, or other credential material.
