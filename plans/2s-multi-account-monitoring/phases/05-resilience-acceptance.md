# Phase 05 — Resilience & Acceptance

**Status:** `blocked-by-phase-04`
**Goal:** prove the amended collection-driven monitor is safe, restart-stable, failure-isolated, visually accepted, and ready for merge/release consideration under the max-four retained-account product policy.

## Required verification

Run the full [`../TEST-MATRIX.md`](../TEST-MATRIX.md) against the exact implementation checkpoint proposed for closure.

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

- Fresh install / existing settings migration.
- First launch with one working Codex account auto-discovered without manual Add.
- Working Codex identity switch A → B: B becomes active; A remains retained; no duplicate identity.
- Manual `Add monitor account...` while another account remains active in Codex.
- Manual login resolving to an already-known identity reconciles ownership instead of creating a duplicate row.
- One, two, three, and four retained-account registry/runtime smoke tests.
- Fifth manual `Add monitor account...` is visibly disabled and no browser/OAuth flow starts.
- Four retained accounts + normal Codex switch to unknown fifth identity: new identity recognized current-only; no retained account silently evicted.
- Restart with multiple retained accounts.
- One account auth failure while all others remain healthy.
- Re-authenticate only the selected failed account.
- Transaction rollback on cancel/timeout/identity mismatch remains safe.
- Remove inactive retained account and restart.
- Remove currently active account from monitor: normal Codex remains logged in; current-only presentation remains valid.
- Weekly-only quota response.
- Missing/unknown quota window.
- Independent reset times.
- Account-scoped low-quota alerts.
- Bar and Circle style with one/two/four representative accounts.
- Active blue identity outline follows runtime current account after switch.
- Whole-account tooltip opens from identity, quota rows, percentage/reset areas and shows correct account data.
- Context menu has direct per-account submenus, no nested `Manage accounts >`, and Add disabled at capacity.
- Same-initial identities remain independent.
- Taskbar drag and multi-monitor placement.
- Explorer/taskbar restart behavior.
- 100/125/150/200% DPI/layout smoke test.
- No credential/token/email/account-ID leakage in settings, diagnostics, screenshots, tooltip, or evidence.
- No mutation of normal working Codex auth/session/history/config during monitor lifecycle operations.
- No intentional inference call from monitor polling/refresh.

## Regression review

Sol final audit must specifically inspect:

- stable identity vs runtime active role vs monitor credential ownership separation;
- collection/N-capable architecture with max-four enforced as policy rather than fixed shape;
- absence of `Slot1/Slot2/...` capacity coupling, `account_a/account_b`, per-slot polling, or fixed per-slot menu action routing;
- automatic discovery and duplicate reconciliation;
- active-account change attribution;
- monitor-owner acquisition without working refresh-token copying;
- refresh-token duplication/race risks;
- working `~/.codex` mutation risk;
- current-only overflow behavior at full retained capacity;
- unknown-window semantics;
- alert key scoping;
- error isolation;
- tooltip/menu account attribution;
- renderer/account-domain separation;
- settings migration and historical two-slot compatibility;
- cleanup/removal semantics for active and inactive identities.

## Acceptance criteria

This phase is PASS only when:

- every current TEST-MATRIX row is PASS or explicitly N/A with Sol approval;
- runtime evidence is current to the final implementation SHA;
- historical max-two evidence is not reused as proof of amended max-four behavior;
- Sidik accepts account discovery, retention, active indication, tooltip, menu, and representative multi-account taskbar behavior;
- Sol reports no open blocker/high-severity finding;
- all remaining lower-severity findings have explicit disposition;
- branch/integration state does not invalidate the evidence.

## Closure rules

Do not declare the plan complete merely because four accounts can be stored or rendered. Closure requires current safety, attribution, lifecycle, polling, UI, and regression evidence.

Do not merge/archive/delete plan evidence until final audit is complete.
