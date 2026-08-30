# Phase 00 — Auth Spike

**Status:** `planned`
**Goal:** prove a safe, zero-inference mechanism for authenticating and reading rate limits for a second Codex account without mutating the user's working Codex state.

## Why this phase exists

Multi-account is blocked on credential ownership. The existing monitor can read the active local `auth.json`, and its current auth-recovery path may run `codex exec "."`. That is not an acceptable multi-account foundation because monitoring must not intentionally consume quota and must not race refresh tokens.

Codex app-server exposes account login/read/rate-limit functionality. This phase must prove the exact Windows runtime behavior before any production architecture is committed.

## Required spike

Create the smallest possible experimental path, behind development-only code or a temporary isolated harness, that can:

1. Start a Codex-managed auth session for a monitor account.
2. Login through the supported ChatGPT browser/device flow.
3. Read account identity.
4. Read account rate limits for 5h and 7d windows.
5. Restart the monitor process and read the same account again.
6. Demonstrate token refresh/re-auth behavior without any inference command.
7. Run while the user's normal Codex installation remains logged into another account.

## Isolation proof

Capture pre/post signatures for the user's normal Codex state. At minimum prove that the spike did not replace or mutate the working active credential and did not create a second working session/history/config source of truth.

Do not record token contents. Evidence may contain file metadata/hashes for non-secret comparison where safe, account initials/opaque IDs, command exit status, and rate-limit values.

A monitor-specific auth directory is acceptable only if it is credential-owned and does not become a second user workspace. If Codex writes incidental non-credential files there, document them and prove they are not used as sessions/history/config source of truth. If that boundary cannot be kept clean, stop.

## Tasks

- [ ] Map current Codex auth read/refresh flow in this repo.
- [ ] Map the official app-server account login/read/rate-limit interface used by the installed Codex version.
- [ ] Implement a minimal isolated spike, not production account management.
- [ ] Login Account A and read identity + rate limits.
- [ ] Login Account B independently and read identity + rate limits.
- [ ] Restart and prove both isolated auth sessions survive as intended.
- [ ] Prove no `codex exec`/inference request is used for polling or refresh.
- [ ] Prove working Codex auth/session/history/config remain unchanged by the monitor spike.
- [ ] Record findings in `../EVIDENCE.md`.

## Acceptance criteria

PASS only if all are true:

- Two distinct accounts can be authenticated and rate limits can be read deterministically.
- Each monitor auth owner has an independent refresh lifecycle.
- No refresh token is copied between owners.
- Normal working Codex credentials are not replaced or edited by the spike.
- No inference request is required to keep monitoring credentials usable.
- Account identity can be associated with the usage response reliably.
- Removing the spike auth state would not remove the user's normal Codex working state.

## Hard fail

If any acceptance item cannot be proven, mark Phase 00 `blocked`, write the contradiction into `EVIDENCE.md`, and stop. Do not proceed to Phase 01.

## Deliverable

A concise implementation recommendation in `EVIDENCE.md` identifying the approved credential/auth ownership mechanism, its storage boundary, refresh behavior, and any known limitations.
