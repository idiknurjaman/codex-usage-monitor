# Phase 03 — Multi-Account Polling

**Status:** `blocked-by-phase-02`
**Goal:** poll both monitored Codex accounts independently, preserve account attribution, and isolate failures/alerts.

## Polling model

Each account owns an independent poll attempt:

```text
poll cycle
  ├─ account S -> auth -> rate limits -> S usage state
  └─ account N -> auth -> rate limits -> N usage state
```

A provider-level failure must not collapse the entire multi-account result.

## Requirements

- Poll the approved read-only rate-limit interface from Phase 00.
- No `codex exec` or inference request for refresh/polling.
- Keep canonical `used_percentage: Option<f64>` semantics.
- Convert Codex used percentage to remaining only at display/alert boundaries.
- Keep missing session/weekly windows unavailable rather than zero.
- Associate every usage result with a stable account ID before publishing state.
- Poll failures are per-account and categorized.
- Preserve last known usage only if the UI clearly distinguishes stale/unavailable state; otherwise show unavailable. Do not silently present stale values as current.
- A refresh/auth failure for one account must not pause polling for another.

## Account-scoped alerts

Alert deduplication keys must include stable account identity, provider, quota window, threshold/reset identity, for example conceptually:

```text
<account-id>:codex:session:<reset-key>
```

The one-letter initial is not a safe alert key.

## Tasks

- [ ] Introduce per-account poll orchestration.
- [ ] Add result aggregation that preserves successful accounts when another fails.
- [ ] Add per-account connection/error state.
- [ ] Add per-account refresh behavior using the Phase 00 approved mechanism.
- [ ] Remove multi-account dependency on `cli_refresh_codex_token()` / `codex exec "."`.
- [ ] Account-scope low-quota alert deduplication.
- [ ] Handle weekly-only responses correctly per account.
- [ ] Handle independently different reset times.
- [ ] Add tests for mixed success/failure, auth-required, weekly-only, unavailable windows, and alert isolation.

## Acceptance criteria

- Two accounts can show different 5h/7d values in the same process.
- Account S can fail while Account N continues to update.
- A 10% alert for S does not suppress a separate 10% alert for N.
- Missing 5h on one account never renders as `0% remaining`.
- Poll/refresh path makes zero intentional inference requests.
- Account attribution remains stable across refresh and process restart.

## Hard stops

- Do not solve auth failure by copying a token from another account.
- Do not temporarily replace the working Codex auth file to perform a poll.
- Do not add account switching as a fallback.

## Evidence

Record current tests plus a runtime proof of simultaneous distinct account values and one-account failure isolation in `../EVIDENCE.md`.
