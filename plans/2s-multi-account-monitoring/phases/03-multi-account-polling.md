# Phase 03 — Multi-Account Polling

**Status:** `in-progress — authorized`
**Goal:** poll the account collection independently, preserve stable attribution and runtime active role, and isolate failures/alerts without account-position hard-coding.

## Preconditions

Phase 02 amended gate is `PASS` at implementation checkpoint `537b2bbad951ccbb43f04ba9067b55b304f4d232` with deterministic proof checkpoint `2dd532525e48710dd03e4bea37819d052b117fc2`. This phase is authorized; Phase 04 remains blocked.

The account collection may contain up to four retained identities plus, at full retained capacity, a current-only active identity discovered from normal Codex. Polling code must consume account/capability state rather than assume A/B or slot positions.

## Polling model

Conceptually:

```text
poll cycle
  for each effective account identity
    resolve approved credential source
    read quota
    publish result by stable account id
```

Credential source selection is separate from active-role presentation:

- retained account with monitor owner → use that isolated monitor owner;
- current-only/active account without monitor owner → working Codex read path may provide current usage;
- inactive retained account without monitor owner → explicit unavailable/re-auth state; do not borrow another account's credential.

If an account is both active and has its own monitor owner, source selection must be deterministic and must not create token-copy/refresh races. Active role itself does not decide identity ownership.

## Requirements

- Poll the approved read-only Codex usage interface.
- No `codex exec` or inference request for monitor refresh/polling.
- Keep canonical `used_percentage: Option<f64>` semantics.
- Convert used percentage to remaining only at display/alert boundaries.
- Keep missing session/weekly windows unavailable rather than zero.
- Associate every result/error with a stable account ID before publishing state.
- Poll failures are per-account and categorized.
- One account failure must not pause or invalidate another.
- Preserve last-known usage only if UI explicitly marks it stale; otherwise show unavailable.
- Refresh/auth lifecycle is scoped to the selected monitor owner.
- Poll orchestration iterates the account collection; no `poll_account_a/b`, `poll_slot_1/2`, or fixed four-account branching.
- Runtime product capacity remains four retained accounts; polling capability itself must not encode four as its structural shape.

## Account-scoped alerts

Alert deduplication keys must include stable account identity, provider, quota window, threshold, and reset identity, conceptually:

```text
<account-id>:codex:<window>:<threshold>:<reset-key>
```

Initial/name is presentation only and is never a safe alert key.

## Automatic active-role updates

Working Codex identity observation may change independently from the polling interval.

When current identity changes:

- active marker moves by stable identity matching;
- quota results remain attached to their account IDs;
- the old active account does not lose its retained monitor state;
- the new active account does not steal another account's monitor owner;
- an unknown current-only identity at full retained capacity may be polled only through the safe current working source until retention/owner capacity is available.

## Tasks

- [ ] Introduce collection-driven per-account poll orchestration.
- [ ] Resolve credential source per account without identity ambiguity.
- [ ] Aggregate results while preserving healthy accounts when another fails.
- [ ] Add per-account connection/error/stale state.
- [ ] Add monitor-owner refresh behavior using the Phase 00 approved mechanism.
- [ ] Remove multi-account dependency on `cli_refresh_codex_token()` / `codex exec "."`.
- [ ] Account-scope low-quota alert deduplication.
- [ ] Handle weekly-only/missing-window responses independently per account.
- [ ] Handle different reset times independently.
- [ ] Preserve active-role changes independently from quota attribution.
- [ ] Add tests across 1, 2, and 4 retained accounts plus mixed success/failure.

## Acceptance criteria

- Distinct accounts can show distinct 5h/7d values in one process.
- Polling works through collection iteration with no position-specific branches.
- One account can fail while all other healthy accounts continue updating.
- Auth expiry for one monitor owner does not pause another.
- Missing 5h on one account never renders as `0% remaining`.
- Account attribution remains stable across refresh, active-account changes, and restart.
- Alert deduplication remains independent per stable identity/window/reset.
- Current-only active overflow at retained capacity never borrows or evicts another account's credential owner.
- Poll/refresh path makes zero intentional inference requests.

## Hard stops

- Do not solve auth failure by copying a token from another account or from the normal working credential.
- Do not temporarily replace the working Codex auth file to perform a poll.
- Do not add account switching as a fallback.
- Do not reintroduce account-position coupling for convenience.

## Evidence

Record current tests plus runtime proof of distinct simultaneous account values, one-account failure isolation, active-role switch attribution, and a four-retained-account smoke test in `../EVIDENCE.md` at the exact implementation checkpoint.
