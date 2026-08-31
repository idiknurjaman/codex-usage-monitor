# Phase 02 — Account Login & Lifecycle

**Status:** `in-progress`
**Goal:** provide explicit Add, Re-authenticate, and Remove flows for monitored Codex accounts without changing the user's active Codex account.

## User flow

Context menu target:

```text
Accounts >
  S   Connected
  N   Connected
  ─────────────
  Add account...
  Manage accounts >
```

The exact Windows menu structure may adapt to native limitations, but the actions and safety boundaries below are required.

## Add account

1. User explicitly chooses `Add account...`.
2. 2S starts an auth session using the Phase 00 approved mechanism.
3. Supported Codex/ChatGPT login UI opens.
4. Login completes or is cancelled/times out.
5. 2S reads stable account identity.
6. Duplicate account check runs.
7. If valid and capacity remains, registry entry is committed.
8. Initial usage read runs.
9. Widget refreshes.

No active working Codex credential is replaced during this flow.

## Re-authenticate account

- Re-auth applies only to the selected monitored account.
- Existing usage for the other account remains visible and polling continues.
- Re-auth must not replace account identity silently with a different account. If login resolves to a different stable identity, require an explicit replace/remove decision rather than mutating the registry behind the user's back.

## Remove account

- Remove deletes only monitor-owned auth/registry state for that monitored account.
- It must never call normal Codex logout or delete the user's working `~/.codex` state.
- Removing one account keeps the other account untouched.

## Tasks

- [ ] Add `Accounts` context-menu surface.
- [ ] Add explicit `Add account...` action.
- [ ] Add cancel/timeout handling for active login.
- [ ] Commit registry entry only after successful identity read and duplicate check.
- [ ] Reject third account with a clear local message/state.
- [ ] Add per-account `Re-authenticate` action.
- [ ] Add per-account `Remove` action.
- [ ] Ensure failure/cancel leaves prior registry/auth state consistent.
- [ ] Add tests around lifecycle state transitions.

## Acceptance criteria

- User can add Account A, then a distinct Account B.
- Duplicate login does not create a second registry entry.
- Third account cannot be added in this iteration.
- Cancelling login leaves current monitored accounts unchanged.
- Re-auth for B cannot break A.
- Removing B cannot log out or modify the normal working Codex account.
- Restart reconstructs registry state without exposing tokens in settings.

## Hard stops

- No `Make Active`, `Switch`, `Use this account`, or automatic switching command may be introduced.
- Do not reuse the working Codex auth file as a scratch target for Add Account.
- Do not silently overwrite an account when stable identity changes during re-auth.

## Evidence

Record lifecycle tests and one manual two-account login/remove/restart walkthrough in `../EVIDENCE.md`. Never include raw email/token values; use initials plus redacted/opaque identifiers.
