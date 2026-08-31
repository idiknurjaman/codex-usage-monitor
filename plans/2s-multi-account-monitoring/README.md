# 2S Multi-Account Monitoring

**Status:** `in-progress`
**Current phase:** `02-account-login`
**Implementation branch:** `feat/2s-multi-account-monitoring`
**Plan authored from checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1`

## Goal

Monitor usage for up to two Codex accounts in one native Windows taskbar widget while leaving the user's normal Codex workspace, sessions, history, config, and active credential untouched.

Target UI:

```text
[S]  5h ○ 32% · 2h
     7d ○ 74% · 6d

[N]  5h ○ 81% · 4h
     7d ○ 43% · 3d
```

The displayed percentage is remaining quota.

## Product authority and execution

- **Sidik:** final product authority and runtime acceptance.
- **Luna:** sole implementation writer/executor for plan work.
- **Sol:** architecture review, phase-gate audit, and final closure audit.

Luna may proceed from this plan without asking Sidik to choose implementation details already resolved here. Any unresolved credential/auth contradiction is a hard stop and must be surfaced instead of guessed through.

## In scope

- Add and persist up to two monitored Codex accounts.
- Separate monitor credential ownership per account.
- Poll 5h and 7d rate limits independently per account.
- Derive and display a one-character account initial.
- Preserve existing Bar and Circle widget styles.
- Isolate account failures and alerts.
- Re-authenticate an individual monitored account.
- Remove a monitored account without touching the normal Codex login.
- Keep monitoring read-only with respect to the user's working Codex state.

## Out of scope

- Switching the active Codex account.
- Auto-switching when quota is low.
- Routing Codex requests between accounts.
- Changing `~/.codex` sessions, history, project state, or config.
- Copying monitor credentials into the active Codex credential.
- Proxying inference traffic.
- Supporting more than two Codex accounts in this iteration.

## Non-negotiable architecture rules

1. **Monitoring and account switching are separate domains.** This plan implements monitoring only.
2. The user's normal Codex workspace remains the single source of truth for normal Codex sessions/history/config.
3. Only monitor credentials may be isolated. A monitor auth root must never become a second user workspace or source of truth for sessions/history/config.
4. Never duplicate a refresh token into multiple independently refreshing owners.
5. Prefer Codex-managed authentication/refresh behavior over 2S implementing OAuth refresh itself.
6. A quota poll must not make an inference request or intentionally consume Codex usage.
7. Credentials and raw tokens must never be written to `settings.json`, logs, diagnostics, screenshots, or the evidence ledger.
8. One account failing must not invalidate, hide, or pause another healthy account.
9. Unknown/missing quota windows remain `None`/unavailable, never synthetic `0%`.
10. Existing canonical model semantics remain `used_percentage: Option<f64>`; conversion to remaining happens at presentation/alert boundaries.

## Current checkpoint

At the authored checkpoint:

- Codex percentage semantics have been repaired to canonical used values with explicit missing windows.
- Bar and Circle widget styles exist and persist.
- Account initial rendering exists for the currently active local Codex identity.
- Circle progress is monotonic with remaining percentage.
- Multi-account monitoring has not started.
- Existing Codex auth refresh fallback can invoke `codex exec "."`; that behavior must not be reused as the multi-account refresh strategy because monitoring must not intentionally consume quota.

## Phase order

| # | Phase | Gate |
|---|---|---|
| 00 | [Auth spike](./phases/00-auth-spike.md) | Prove isolated, zero-inference account auth/rate-limit read without mutating working Codex state |
| 01 | [Account registry](./phases/01-account-registry.md) | Stable account model and credential ownership boundary |
| 02 | [Account login](./phases/02-account-login.md) | Add/re-auth/remove account flows for max two accounts |
| 03 | [Multi-account polling](./phases/03-multi-account-polling.md) | Independent usage state, failure isolation, account-scoped alerts |
| 04 | [Taskbar UI](./phases/04-taskbar-ui.md) | Bar/Circle rendering of two accounts without regressions |
| 05 | [Resilience & acceptance](./phases/05-resilience-acceptance.md) | Full Definition of Done and runtime acceptance |

Phases 00 and 01 have passed their Sol phase gates. Phases 03–05 remain blocked until their preceding phase passes.

## Hard stops

Stop implementation and report to Sol/Sidik if any of these occur:

- Safe per-account credential isolation cannot be achieved without splitting the user's normal Codex sessions/history/config.
- The proposed refresh method requires inference traffic.
- Adding a monitor account mutates or replaces the normal active Codex credential.
- Two monitor processes/sessions would race on the same refresh token.
- Rate-limit reads cannot be attributed to the intended account reliably.
- A required auth mechanism depends on undocumented behavior that cannot be proven by a focused runtime spike.

Do not paper over a hard stop with file copying, token duplication, hidden account switching, or a second user-facing `CODEX_HOME`.

## Definition of Done

The plan is complete only when all rows in [`TEST-MATRIX.md`](./TEST-MATRIX.md) are PASS, [`EVIDENCE.md`](./EVIDENCE.md) contains current evidence for the final implementation checkpoint, Sidik has accepted the runtime UI/behavior, and Sol has completed final audit.

Required command proof at the final checkpoint:

```powershell
cargo fmt --check
cargo test
cargo clippy --all-targets
cargo build --release
git diff --check
```

Existing unrelated Clippy warnings may be recorded, but the plan must introduce no new warning caused by its changes.

## Next authorized action

Execute **Phase 02 only**. Do not implement polling fan-out, account switching, or later phases.
