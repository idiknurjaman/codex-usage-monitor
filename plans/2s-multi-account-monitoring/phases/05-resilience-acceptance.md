# Phase 05 — Resilience & Acceptance

**Status:** `blocked-by-phase-04`
**Goal:** prove the two-account monitor is safe, restart-stable, failure-isolated, visually accepted, and ready for merge/release consideration.

## Required verification

Run the full [`../TEST-MATRIX.md`](../TEST-MATRIX.md) against the exact implementation checkpoint being proposed for closure.

Mandatory command proof:

```powershell
cargo fmt --check
cargo test
cargo clippy --all-targets
cargo build --release
git diff --check
```

Record exact commit SHA and outcomes in `../EVIDENCE.md`.

## Runtime scenarios

At minimum verify:

- Fresh install / existing single-account settings migration.
- One monitored account only.
- Two monitored accounts.
- Restart with two accounts.
- Account A auth failure while B remains healthy.
- Account B auth failure while A remains healthy.
- Re-authenticate only the failed account.
- Remove one account and restart.
- Duplicate account login attempt.
- Third account attempt is rejected cleanly.
- Weekly-only quota response.
- Missing/unknown quota window.
- Independent reset times.
- Account-scoped low-quota alerts.
- Bar style with one and two accounts.
- Circle style with one and two accounts.
- Taskbar drag and multi-monitor placement.
- Explorer/taskbar restart behavior.
- DPI/layout smoke test.
- No credential/token leakage in settings or diagnostics.
- No mutation of the user's working Codex auth/session/history/config during monitor lifecycle operations.
- No intentional inference call from monitor polling/refresh.

## Regression review

Sol final audit must specifically inspect:

- Credential ownership boundaries.
- Refresh-token duplication/race risks.
- Working `~/.codex` mutation risk.
- Account attribution correctness.
- Unknown-window semantics.
- Alert key scoping.
- Error isolation.
- Renderer/account-domain separation.
- Settings migration.
- Cleanup/removal semantics.

## Acceptance criteria

This phase is PASS only when:

- Every required TEST-MATRIX row is PASS or explicitly marked not-applicable with Sol approval.
- Runtime screenshots/notes are current to the final implementation SHA.
- Sidik accepts the two-account taskbar behavior.
- Sol reports no open blocker/high-severity finding.
- All remaining lower-severity findings are documented with explicit disposition.
- The implementation branch is not behind the intended integration target in a way that invalidates evidence.

## Closure rules

Do not declare the plan complete merely because the app builds or two rows appear on screen. Closure requires current safety, behavior, and regression evidence.

Do not merge/archive/delete the plan evidence until final audit is complete.
