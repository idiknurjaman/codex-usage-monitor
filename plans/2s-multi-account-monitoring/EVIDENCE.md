# 2S Multi-Account Monitoring — Evidence Ledger

## Current state

- **Plan:** `2s-multi-account-monitoring`
- **Lifecycle:** `in-progress`
- **Current phase:** `04-taskbar-ui`
- **Implementation branch:** `feat/2s-multi-account-monitoring`
- **Plan authoring checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1`
- **Implementation checkpoint:** `1cda2b33c7ed30166f364b26687a8876d347865d`
- **Deterministic proof/test checkpoint:** `1cda2b33c7ed30166f364b26687a8876d347865d`

## Baseline evidence

Current branch already contains:

- explicit `used_percentage: Option<f64>` semantics;
- correct weekly-only/unavailable-window handling;
- remaining conversion at presentation boundaries for Codex;
- persisted Bar/Circle widget style;
- current-account initial derivation/rendering;
- monotonic Circle remaining progress.

Phase 02 amended account model and proof closure passed Sol review at the checkpoints above. Phase 03 multi-account polling passed its Sol final gate; Phase 04 final collection taskbar UI is now active.

## Open findings / blockers

### BLOCKER-00 — monitor credential isolation (resolved in Phase 00)

Resolved by the direct pinned `codex-login` runtime proof recorded in the Phase 00 continuation below.

### BLOCKER-01 — existing Codex auth refresh path can invoke inference (resolved for monitor path)

The existing single-account fallback remains unchanged, but the accepted monitor path uses direct pinned `codex-login` refresh and does not use inference.

### FINDING-01 — identity parsing currently lives in UI layer

Resolved by the Phase 01 account-domain projection; `window.rs` now consumes registry state rather than parsing credential files.

### FINDING-02 — current initial is startup-scoped

Resolved for the amended Phase 02 source: current identity is re-observed through the existing normal polling cycle and represented as a runtime active role. Phase 03 still owns recurring independent retained-account polling.

## Phase evidence

### Phase 00 — Auth Spike

**Status:** `BLOCKED`

**Prior evidence checkpoint:** `5dcd0a77dedd43468c751bf39650a75d1d72f75a`

#### Harness and authority map

- Installed Codex runtime: `codex-cli 0.151.0-alpha.7.1`.
- `codex login status`: working ChatGPT login present; no credential content was recorded.
- The smallest spike used the installed `codex app-server` over stdio. No production source or account-manager harness was added.
- Generated app-server schema exposes `account/login/start`, `account/login/cancel`, `account/read`, and `account/rateLimits/read`.
- The managed `chatgpt` auth mode is Codex-owned persistence/refresh. The `chatgptAuthTokens` mode is marked unstable/internal and requires an external host to own refresh; it is not an approved foundation.
- No auth mechanism is approved for Phase 01 because isolated credential ownership was not proven.

#### Read-only working-account runtime proof

The sequential JSON-RPC smoke test sent only `initialize`, `initialized`, `account/read` with `refreshToken:false`, and `account/rateLimits/read`. It did not send `command/exec`, `codex exec`, an inference request, login, logout, or refresh.

- `initialize`: PASS; the runtime reported its `codexHome` as the normal working `~/.codex`.
- `account/read(refreshToken:false)`: PASS for the existing working account; account object present, type `chatgpt`, plan type present. Email was intentionally omitted.
- `account/rateLimits/read`: PASS for that same working account. The snapshot returned a 5-hour window (`usedPercent=92`, `windowDurationMins=300`, `resetsAt=1788128406`) and weekly window (`usedPercent=30`, `windowDurationMins=10080`, `resetsAt=1788655142`). These are working-account observations, not Account A/B monitor proof.

#### Credential ownership and isolation

**BLOCKED at the prior checkpoint.** The default app-server session resolved to the user's working `~/.codex`, so using it for a second monitor account would share the active credential/workspace boundary. No separate credential-owned monitor auth root was created at that checkpoint because no approved owner boundary or second-account login was available. No `auth.json` copy, refresh-token duplication, hidden account switch, or second user-facing `CODEX_HOME` was attempted.

#### Account A/B distinct rate-limit proof

**BLOCKED.** There is one working-account read only. Account A as an isolated monitor owner, Account B as a distinct isolated owner, deterministic A/B attribution, and independent refresh ownership were not exercised or proven.

#### Refresh, restart, and zero-inference proof

**BLOCKED.** No refresh or re-auth was run because the only available app-server root was the working root. The app-server schema documents a Codex-managed refresh path through `account/read(refreshToken:true)`, but this was not exercised in an isolated owner. The existing monitor fallback still invokes `codex exec .` at `src/poller.rs:437-449`; it was not invoked during this spike, but the required zero-inference refresh alternative is therefore not proven.

No A+B monitor state was created, so restart survival and monitor-auth deletion without working-state deletion are also not proven.

#### Working Codex pre/post proof

The targeted baseline was captured before the app-server read and repeated afterward. No raw credential/token/email/content was recorded.

| Working state | Before | After | Result |
|---|---|---|---|
| `~/.codex/auth.json` | 4111 bytes; SHA-256 `E6E52E27882C5C85192862EBA73B125CFE04C436DB5578C330BC4FF84581C4BF` | identical | PASS |
| `~/.codex/config.toml` | 8630 bytes; SHA-256 `6F77B6DFCAAC205FBEFA00F75A130EC4DAFB7173B66F5739811BAD37A89561C5` | identical | PASS |
| `~/.codex/session_index.jsonl` | 33989 bytes; SHA-256 `00FF3B78B93CDCE25E12FD0DB826E4AE5AC811647F042F3FCB1766159F26B1EA` | identical | PASS |
| `~/.codex/history.jsonl` | absent | absent | PASS |
| `~/.codex/sessions/` | 354 files; 4134752524 bytes | 354 files; 4136598346 bytes | INCONCLUSIVE |

The sessions aggregate changed while this active Codex task continued writing its own session files. Because that concurrent writer cannot be separated from the read-only app-server lifecycle in this run, SEC-01 is not promoted to PASS.

#### Required Phase 00 IDs

| ID | Disposition | Evidence boundary |
|---|---|---|
| AUTH-01 | BLOCKED | No isolated monitor Account A login/owner was created. |
| AUTH-02 | BLOCKED | No distinct Account B login/owner or A/B attribution was created. |
| AUTH-03 | BLOCKED | Refresh was not exercised; zero-inference refresh remains unproven. |
| AUTH-04 | BLOCKED | No isolated A+B state existed to restart. |
| AUTH-06 | PARTIAL | This evidence contains no raw secret; full monitor settings/log audit was not possible because no monitor owner was created. |
| POLL-03 | PARTIAL | This run used no inference request; refresh-path zero-inference proof is missing. |
| SEC-01 | BLOCKED | Credential/config/index remained stable, but sessions were concurrently written by the active Codex task. |
| SEC-02 | BLOCKED | No two monitor owners were created, so refresh-token non-duplication cannot be proven. |

**Decision:** `BLOCKED`. The app-server account/rate-limit read interface is a viable candidate for further investigation, but Phase 00 acceptance is not met. Do not begin Phase 01.

**Next authorized action:** Sol/Sidik review this bounded result and, only if approved, define a credential-owned isolated test boundary and provide two distinct test accounts or an equivalent owner-approved runtime fixture. No production account registry, login lifecycle, multi-account polling, account switching, or two-account UI work is authorized.

### Phase 00 continuation — strict keyring namespace attempt

**Date:** `2026-08-30` UTC runtime attempt

#### Authorized boundary and exact startup proof

The approved boundary was treated as Codex-managed auth storage only, never as a user workspace:

- `%LOCALAPPDATA%\CodexUsage\auth-spike\slot-1`
- `%LOCALAPPDATA%\CodexUsage\auth-spike\slot-2`

The installed runtime was `codex-cli 0.151.0-alpha.7.1`. Slot-1 was started with the exact strict-storage command:

```powershell
& {
  $env:CODEX_HOME = Join-Path $env:LOCALAPPDATA 'CodexUsage\auth-spike\slot-1'
  codex app-server --stdio --strict-config -c 'cli_auth_credentials_store="keyring"'
}
```

The process accepted the strict `keyring` configuration and completed `initialize`. Its reported `codexHome` ended in `CodexUsage\auth-spike\slot-1` (Windows packaged-runtime local-cache resolution). This proves parser/startup support for the requested keyring setting, not credential isolation.

#### Hard-stop result

Before any `account/login/start`, `account/read`, `account/rateLimits/read`, refresh, session, or inference request, the slot-1 namespace contained Codex runtime state beyond an auth-only boundary: 19 top-level entries including `goals_1.sqlite*`, `logs_2.sqlite*`, `memories_1.sqlite*`, `queue_1.sqlite*`, `state_5.sqlite*`, `installation_id`, `skills`, `.tmp`, and `tmp`. It did not contain `auth.json`, `history.jsonl`, or `sessions/`, but the additional Codex state/plugin/runtime workspace means the approved auth-only boundary was not preserved.

This is a hard stop under the continuation authorization. Slot-2 was not started, no browser login was initiated, and no fallback to `auto` was attempted. The slot-1 process was stopped after the startup/namespace inspection.

#### Account A proof

**NOT PROVEN.** No login was initiated and no Account A identity, 5-hour quota, weekly quota, refresh ownership, or account deletion proof exists.

#### Account B proof

**NOT PROVEN.** Slot-2 was not started after the slot-1 hard stop. No Account B identity, 5-hour quota, weekly quota, or distinct-owner proof exists.

#### Restart and refresh proof

**NOT PROVEN.** No authenticated slot was available to restart. `account/read { refreshToken: true }` was not called. No inference request or `codex exec "."` was invoked during this attempt, but that is an execution observation, not proof that an isolated refresh path works.

#### Working-Codex mutation proof

The approved slot-1 startup was run with `CODEX_HOME` pointed at the slot boundary. The normal working namespace remained unchanged against the prior baseline:

| Working state | Post-probe observation | Result |
|---|---|---|
| `~/.codex/auth.json` | 4111 bytes; SHA-256 `E6E52E27882C5C85192862EBA73B125CFE04C436DB5578C330BC4FF84581C4BF` | PASS against baseline |
| `~/.codex/config.toml` | 8630 bytes; SHA-256 `6F77B6DFCAAC205FBEFA00F75A130EC4DAFB7173B66F5739811BAD37A89561C5` | PASS against baseline |
| `~/.codex/session_index.jsonl` | 33989 bytes; SHA-256 `00FF3B78B93CDCE25E12FD0DB826E4AE5AC811647F042F3FCB1766159F26B1EA` | PASS against baseline |
| `~/.codex/history.jsonl` | absent | PASS against baseline |
| `~/.codex/sessions/` | active Codex session aggregate continued changing | INCONCLUSIVE; not attributed to slot-1, so SEC-01 remains blocked |

No password, raw access token, refresh token, email, or credential content was recorded. No production source was changed.

#### Required continuation handoff

```text
PHASE
00-auth-spike
HEAD
5dcd0a77dedd43468c751bf39650a75d1d72f75a (implementation/evidence checkpoint before this evidence-only continuation commit)
AUTH BOUNDARY
BLOCKED — strict keyring startup was accepted, but slot-1 created general Codex runtime/plugin state inside the auth-only namespace; slot-2 was not started.
ACCOUNT A PROOF
NOT PROVEN — no login or account read was allowed after the hard stop.
ACCOUNT B PROOF
NOT PROVEN — slot-2 was not started.
RESTART PROOF
NOT PROVEN — no authenticated isolated slot existed.
REFRESH PROOF
NOT PROVEN — account/read with refreshToken=true was not called; no inference was invoked.
WORKING-CODEX MUTATION PROOF
PARTIAL — auth/config/session index/history stayed at baseline; sessions aggregate remains inconclusive because the active Codex task was writing concurrently.
OPEN FINDINGS
The installed app-server does not preserve the approved auth-only namespace under strict keyring startup. Credential ownership, two-account distinction, restart survival, refresh ownership, deletion isolation, and zero-inference refresh remain unproven.
PASS-or-BLOCKED
BLOCKED
NEXT AUTHORIZED ACTION
Sol/Sidik must review this hard stop and approve a mechanism that provides auth-only ownership without creating a second Codex workspace. Do not start Phase 01, start slot-2, login, or implement production account management until that review resolves the boundary.
```

### Phase 00 continuation — direct `codex-login` auth-only harness

**Status:** `complete/PASS`

**Implementation checkpoint under test:** `d313c786bdc7bec3cdbc01f5e97c172f10af6c44` (no production source or dependency change)

#### CODEX AUTH SOURCE SHA

- Official OpenAI Codex source: `openai/codex`
- Pinned reviewed revision: `94cbbddafc1776d5e377bca1b05932c697e82238`
- Reviewed crate: `codex-rs/login` (`codex-login`)
- Harness was compiled as a temporary overlay member of the pinned `codex-rs` workspace so its existing workspace lock/resolution was used. The overlay was outside this repository and was not committed.
- Build command: `cargo check -p codex-phase00-direct-harness`
- Build result: `PASS` (`Finished dev profile`)

The pinned source exposes `AuthCredentialsStoreMode::Keyring`, `AuthKeyringBackendKind::{Direct, Secrets}`, `run_login_server`, `AuthManager::{auth, refresh_token, logout_with_revoke}`, and `CodexAuth::{get_token, get_account_id}`. The harness selected `Keyring + Secrets` after the Windows Direct backend rejected the managed OAuth payload at the platform's 2560-character keyring attribute limit. No fallback to `Auto` or `File` was used.

#### AUTH-ONLY BOUNDARY

The only approved namespaces used by the direct harness were:

- `%LOCALAPPDATA%\CodexUsage\auth-spike\slot-1`
- `%LOCALAPPDATA%\CodexUsage\auth-spike\slot-2`

Fresh construction/`AuthManager::auth()` inspection passed for both slots before login: `AUTH_PRESENT=false`, entry count `0` before and after, filesystem unchanged, and no `auth.json`, `history.jsonl`, `sessions`, SQLite runtime database, skills, plugin, or temp runtime entries.

The first Direct-keyring login attempt was stopped at the storage error:
`failed to write OAuth tokens to keyring: Attribute 'password encoded as UTF-16' is longer than a platform limit of 2560 chars`.
Slot-1 remained empty. This is a backend limitation, not a reason to use `Auto`.

With `Keyring + Secrets`, successful login created exactly one auth-owned encrypted file per authenticated slot: `secrets\codex_auth.age`. Slot-1 was `4209` bytes and slot-2 was `4325` bytes. No session/history/config/runtime state was created in either slot. After B cleanup, slot-2 retained only a `208`-byte empty encrypted container; its credential was removed and `AuthManager::auth()` returned no managed ChatGPT auth.

#### ACCOUNT A

- Browser login completed through `run_login_server`; no password, OAuth code, access token, refresh token, email, or credential content was captured in harness output/evidence.
- `AuthManager::auth()` returned managed `Chatgpt` auth.
- `CodexAuth::get_account_id()` produced opaque identity `ed81a109ecad9d86`.
- `CodexAuth::get_token()` returned a bearer token in memory only; output explicitly recorded `TOKEN_CONTENT_LOGGED=false`.
- Direct usage endpoint: `https://chatgpt.com/backend-api/wham/usage`, with bearer token and account header held in memory.
- Usage observation: primary 5-hour window `used_percent=19`, `remaining=81`, `reset_at=1788140799`; secondary weekly window `used_percent=61`, `remaining=39`, `reset_at=1788643352`.

#### ACCOUNT B

- Browser login completed independently in slot-2.
- `AuthManager::auth()` returned managed `Chatgpt` auth.
- `CodexAuth::get_account_id()` produced opaque identity `09c2de991a2a229b`, distinct from A.
- `CodexAuth::get_token()` returned a bearer token in memory only; output explicitly recorded `TOKEN_CONTENT_LOGGED=false`.
- Direct usage observation: primary 5-hour window `used_percent=100`, `remaining=0`, `reset_at=1788128406`; secondary weekly window `used_percent=31`, `remaining=69`, `reset_at=1788655142`.

The distinct opaque identities and different 5-hour/weekly values prove deterministic A/B attribution for this run. The two independent encrypted files and path-derived Codex keyring ownership are source/storage proof that the owners are separate. Raw refresh-token equality was intentionally never logged or persisted; direct raw-token comparison is therefore not claimed as evidence.

#### RESTART

New harness processes restored each account independently from its keyring owner:

- A restart/read returned `Chatgpt`, opaque identity `ed81a109ecad9d86`, bearer available in memory.
- B restart/read returned `Chatgpt`, opaque identity `09c2de991a2a229b`, bearer available in memory.
- After B deletion, a new A restart/read still returned A and A usage remained readable.
- After B deletion, a new B read returned `no managed ChatGPT auth`.

#### REFRESH

`AuthManager::refresh_token()` succeeded for A and B independently, preserving each opaque account identity. Reciprocal proof also passed:

- A refresh: B identity remained `09c2de991a2a229b` and B encrypted auth file SHA remained `E2E60271F708C8CCA0177803200566CBF5BF626EE9ED020860BD6574C31728B6`.
- B refresh: A identity remained `ed81a109ecad9d86` and A encrypted auth file SHA remained `4B89B718C98128AEBF38B3234110F7B05421D8FA612D7B1C30A26C394ADF5BCB`.

The direct harness has no `Command`/subprocess path and does not invoke app-server, TUI, `codex exec`, thread/session APIs, or inference. The pinned Codex refresh implementation calls the ChatGPT token refresh authority and persists the result through the selected auth storage. This is a source-plus-runtime zero-inference proof for this harness path.

#### DELETION ISOLATION

`AuthManager::logout_with_revoke()` for B returned `ACCOUNT_OWNED_CREDENTIAL_REMOVED=true`. A subsequent B read returned unauthenticated. A's encrypted auth file remained byte/SHA stable, A restart/read and direct usage remained successful, and no normal Codex state was targeted. The remaining `208`-byte encrypted B container is an empty auth-storage container, not an authenticated B credential.

#### WORKING CODEX MUTATION

During the controlled A-refresh/B-refresh/deletion interval, normal Codex metadata remained stable:

| Working state | Pre-cross-refresh | Final | Result |
|---|---|---|---|
| `~/.codex/auth.json` | 3980 bytes; SHA-256 `9017787DFDEACF53EC718F02D6A04F17E5635CAB89CC8F0954E888C010BB98ED` | identical | PASS for controlled interval |
| `~/.codex/config.toml` | 8630 bytes; SHA-256 `6F77B6DFCAAC205FBEFA00F75A130EC4DAFB7173B66F5739811BAD37A89561C5` | identical | PASS for controlled interval |
| `~/.codex/session_index.jsonl` | 35062 bytes; SHA-256 `BD7B7903DAE84DDBCC05D53630A738CA24D42C1491E1DDC26E31B8A74930A253` | identical | PASS for controlled interval |
| `~/.codex/history.jsonl` | absent | absent | PASS |

Owner attribution resolves the historical baseline ambiguity: Sidik confirms that he intentionally logged the normal Codex installation into a different account during the test period. That owner action occurred outside the monitor harness and explains the historical `~/.codex/auth.json` rewrite. The timestamp correlation is consistent with this: the owner chat is `03:46` local time (`20:46 UTC`) and the changed file has `LastWriteTimeUtc=20:45:53.3892576Z`, approximately seven seconds earlier. The historical hashes are not claimed to be identical.

The controlled A-refresh/B-refresh/deletion interval remains the authoritative monitor-mutation proof: normal `auth.json`, `config.toml`, `session_index.jsonl`, and `history.jsonl` stayed stable in that interval. Concurrent `session_index.jsonl` writes observed outside that interval belong to active normal Codex activity and are not attributed to the direct harness. **SEC-01: PASS.**

#### Required IDs

| ID | Disposition | Evidence |
|---|---|---|
| AUTH-01 | PASS bounded | A login, identity, direct usage, and auth-only slot boundary proven. |
| AUTH-02 | PASS bounded | B login and distinct identity/direct usage proven; B owner separate from A. |
| AUTH-03 | PASS bounded | A/B `AuthManager::refresh_token()` succeeded without inference. |
| AUTH-04 | PASS bounded | A/B restored in new processes; cross-refresh left the other owner unchanged. |
| AUTH-05 | PASS bounded | B logout-with-revoke removed B auth; A remained usable. |
| AUTH-06 | PASS | Harness output/evidence contains no raw secret/token/email content. |
| DATA-03 | PASS bounded | A/B opaque identities and quota values remained attributed to their slots. |
| POLL-03 | PASS bounded | Direct harness has no inference/subprocess path; refresh uses Codex auth authority. |
| SEC-01 | PASS | Owner-attributed normal Codex account-switch activity is separated from the controlled monitor interval; all four normal-state markers were stable during monitor refresh/deletion. |
| SEC-02 | PASS | Pinned Codex storage path uses independent auth owners; A/B had distinct identities and encrypted auth files, and reciprocal refresh left the other owner unchanged. No raw token content was logged or persisted. |

**Decision:** `PASS` — Sol phase-gate verdict recorded below. The direct `codex-login` hypothesis is accepted with `Keyring + Secrets`; auth-only, A/B, restart, refresh, deletion, SEC-01, and SEC-02 evidence passed. This transition authorizes Phase 01 only.

**Next authorized action:** Phase 00 is complete. Phase 01 is the active phase; login UI, polling fan-out, switching, and later phases remain out of scope.

#### Sol Phase 00 gate verdict

- **Verdict:** `PASS`
- **Phase 00:** `complete/PASS`
- **Approved mechanism:** direct pinned `codex-login` at source SHA `94cbbddafc1776d5e377bca1b05932c697e82238`, using `AuthCredentialsStoreMode::Keyring` with `AuthKeyringBackendKind::Secrets`.
- **Accepted boundary:** isolated credential-owned namespaces only; no normal Codex workspace ownership, no sessions/history/config ownership, and zero inference.
- **Phase 01:** `complete/PASS`. Phase 02 is authorized; polling fan-out, account switching, and later phases remain out of scope.

### Phase 01 — Account Registry

**Status:** `complete/PASS`

Implementation and acceptance evidence are recorded below at checkpoint `84c8e14d2ce3cef06be2cfd3925d575e5cb9076c`. Sol gate verdict: `PASS`.

### Phase 01 — Account Registry evidence

**Status:** `complete/PASS`

**Implementation checkpoint:** `84c8e14d2ce3cef06be2cfd3925d575e5cb9076c`

#### Scope and ownership

- Added `src/account.rs` as the account-domain owner for stable identity projection, typed monitor owner handles, monitored-account metadata, registry ordering, capacity, duplicate validation, and runtime connection/usage fields.
- `AccountIdentity.id` is independent from `MonitorAuthHandle::{Slot1, Slot2}`. The typed handle serializes as `slot-1`/`slot-2` and maps deterministically to clean production logical namespace keys `monitor-auth/slot-1`/`monitor-auth/slot-2`; no absolute path is persisted.
- `window.rs` no longer parses JWTs or opens Codex auth files. It consumes `AccountRegistry::display_initial()` and persists only explicit `AccountRegistryMetadata` through the existing settings path.
- The existing `poller.rs` Codex credential adapter now projects `account_id`/ID-token claims into `AccountIdentity` in memory. No token is stored in the account model or sent to the renderer.
- The accepted Phase 00 mechanism remains unchanged: direct pinned `codex-login`, `AuthCredentialsStoreMode::Keyring` with `AuthKeyringBackendKind::Secrets`, isolated credential-owned namespaces, zero inference, and no normal Codex workspace ownership. Phase 01 does not add the production `codex-login` dependency or implement login/lifecycle.

#### Account model and persistence proof

`AccountRegistry` represents zero, one, or two `MonitoredAccount` values. `try_add()` requires a real typed `MonitorAuthHandle`, rejects duplicate stable `id` values, rejects duplicate auth owners, and only then applies `MAX_MONITORED_ACCOUNTS = 2`. `from_metadata()` reconstructs through the same validation path. Vector insertion order is the persisted deterministic order. Two different stable identities may use the same initial.

Persisted `MonitoredAccountMetadata` contains only `id`, optional `initial`, `enabled`, and typed `auth_handle`. Runtime `connection_state`, `usage`, `last_success_at`, and `last_error` are not serialized. The focused serialization test verifies that `access_token`, `refresh_token`, `id_token`, email, and absolute path material do not appear in registry metadata.

When registry metadata is absent, the registry remains empty. The normal active Codex identity projection is used only as an ephemeral display fallback, preserving existing single-account initial behavior without entering `AppState.account_registry` or settings. Invalid/over-capacity persisted metadata is normalized through the same max-two/duplicate validation path.

#### Tests and build evidence

- `cargo fmt --check`: PASS.
- `cargo test`: PASS — 43 passed, 0 failed.
- `cargo clippy --all-targets`: PASS exit. Eight pre-existing warnings remain in `poller.rs`/`window.rs`; no new account-domain warning remains.
- `cargo build --release`: attempted; default `target\release\codex-usage.exe` was locked by running PID `13584` and was not terminated.
- `CARGO_TARGET_DIR=%TEMP%\codex-usage-phase01-release-target cargo build --release`: PASS — optimized release build.
- `git diff --check`: PASS.

#### Phase 01 acceptance matrix

| Requirement | Result | Evidence |
|---|---|---|
| Zero/one/two account representation | PASS | `AccountRegistry` model and empty/legacy fallback tests. |
| Maximum two accounts | PASS | `MAX_MONITORED_ACCOUNTS = 2` and full-registry length assertions. |
| Stable duplicate identity | PASS | Duplicate `id` rejected regardless of initial. |
| Unique monitor auth ownership | PASS | `Slot1 + Slot1` rejected in `try_add()` and `from_metadata()`; `Slot1 + Slot2` accepted. |
| Duplicate classification before capacity | PASS | Full registry reports duplicate identity/owner errors before capacity. |
| Deterministic persisted ordering | PASS | Vec insertion order round-trips through metadata. |
| Same-initial accounts | PASS | Two distinct IDs with initial `S` coexist. |
| Identity ownership outside renderer | PASS | JWT/auth parsing removed from `window.rs`; projection is adapter/domain-owned. |
| Non-secret settings metadata | PASS | Metadata-only serializer and no credential fields. |
| Legacy fallback remains ephemeral | PASS | Empty registry + working A displays A; settings round-trip keeps registry empty; working B then displays B. |
| Typed auth-owner handle | PASS | Slot1/Slot2 round-trip, clean `monitor-auth/*` namespace keys, and no absolute path serialization. |
| Existing single-account behavior | PASS | Existing window tests pass without persisting the working identity as a monitor account. |
| Existing usage semantics | PASS | Existing poller semantics tests remain green; no quota/reset/alert changes. |

#### Scope guard

No login UI, multi-account login/re-auth lifecycle, polling fan-out, account switching, alert changes, quota/reset changes, or Phase 02 work was added.

**Decision:** `PASS` — Sol Phase 01 gate verdict. Phase 01 implementation acceptance evidence is complete at the checkpoint above. Phase 02 is now authorized; Phase 03 and later phases remain blocked.

### Phase 02 — Account Login & Lifecycle

**Status:** `complete/PASS — Class R + Class F/S proof closure`

Phase 02 is complete. The superseded two-account runtime walkthrough is not used. Current amended authority is docs HEAD `99be80cc3bcaea0bf185c5dd36d9611338fea6ec`; implementation checkpoint is `537b2bbad951ccbb43f04ba9067b55b304f4d232`.

#### Collection-driven account model

- `AccountRegistry` is a deterministic collection with policy `MAX_RETAINED_ACCOUNTS = 4`; capacity is not encoded as account-position variants.
- `MonitorAuthHandle` is a typed dynamic positive owner index. New serialized values are `owner-N`; physical production namespaces remain the continuity-preserving `monitor-auth/slot-N` owner paths used by re-auth, rollback, and remove cleanup. Legacy `slot-N` input is accepted and resolves to the same typed owner. No `auth-spike` namespace or absolute path is serialized or resolved.
- `auth_handle` is optional. Automatic working-account discovery may retain an identity without an independent owner and marks it `ReauthRequired` when it becomes inactive; no working access/refresh token is copied.
- Startup and existing normal polling observations project the current working Codex identity into runtime state. A known identity is updated without duplication; a new identity is auto-retained when capacity exists; at capacity it is current-only and retained accounts are not evicted.
- Active status is derived from the runtime working identity and is absent from `AccountRegistryMetadata`/settings.
- Explicit removal of the current retained identity records a runtime suppression so repeated observation leaves it current-only until the working identity changes; removal never logs normal Codex out.

#### Manual lifecycle and menu routing

- Manual `Add monitor account...` preflights lifecycle state and retained count before `LoginOperation::start()`. At four retained accounts it is disabled and no OAuth server starts.
- Manual login resolves through `reconcile_manual_identity()`: a new identity is added with the dynamically allocated owner; a known ownerless identity is attached; a known already-owned identity does not create a second row and the newly-created owner is rolled back.
- Account menu construction iterates the collection and creates direct per-account submenus. The nested `Manage accounts >` menu and all per-slot action constants are removed. The menu has disabled `Account` and `Manage account` section labels, direct account entries, `Re-authenticate`, `Remove from monitor`, and the amended Add label.
- Re-auth keeps the expected stable identity, can start with a missing owner, attaches a newly allocated owner when needed, and leaves the existing identity unchanged on mismatch.
- Existing transaction safety from `ba7721d` remains: pre-login owner state is captured in memory; every post-server non-commit path restores or clears the selected owner; rollback failure is explicit; commit drops the snapshot only after registry acceptance.
- Successful Add/Re-auth starts one bounded account-scoped initial usage read using the approved monitor owner and direct usage endpoint. No Phase 03 polling fan-out is introduced.

#### Automated acceptance evidence

- `cargo fmt --check`: PASS.
- `cargo test --locked`: PASS — 70 passed, 0 failed.
- `cargo clippy --all-targets --locked`: PASS exit; existing warnings remain in legacy `poller.rs`/`window.rs` code, with no amended account-model warning introduced.
- `$env:CARGO_TARGET_DIR="$env:TEMP\codex-usage-phase02-proof-closure-release-target"; cargo build --release`: PASS — optimized release build. Binary: `C:\Users\SIDIKN~1\AppData\Local\Temp\codex-usage-phase02-proof-closure-release-target\release\codex-usage.exe`.
- `git diff --check`: PASS before source checkpoint.

#### Focused regression coverage

The 70-test suite includes the following focused Class F/S proof closure tests:

- `account::tests::collection_policy_supports_four_accounts_without_fixed_owner_variants` — Class F: four retained identities and policy capacity without fixed variants;
- `account::tests::full_capacity_unknown_identity_is_current_only_without_eviction_or_owner_theft` — Class F: unknown E is current-only and A/B/C/D metadata plus owners remain unchanged;
- `account::tests::freeing_capacity_allows_later_current_identity_retention` — Class F: capacity release permits later reconciliation;
- `window::tests::fifth_manual_add_is_rejected_before_login_preflight` — Class F/S: retained count four blocks Add before login dispatch;
- `account::tests::initial_usage_success_is_recorded_against_selected_account_only` — Class S: success is stored against the selected stable identity only;
- `account::tests::initial_usage_failure_is_scoped_to_selected_account` — Class S: failure becomes selected-account `Unavailable` without affecting another account;
- existing collection/identity/owner, active-role, current-display, direct-menu, legacy-owner, and transaction tests — Class F/S: collection routing, no persisted active role, owner continuity, and transactional auth safety.

#### Runtime proof boundary

The amended walkthrough was restarted against the corrected binary `C:\Users\SIDIKN~1\AppData\Local\Temp\codex-usage-phase02-menu-routing-target\release\codex-usage.exe`. The prior routing defect on `99097750...` is not reused as acceptance evidence.

Owner-observed runtime results, recorded with account labels only:

| Scenario | Proof class | Result | Owner-observed evidence |
|---|---|---|---|
| Startup with normal Codex account A | Class R | PASS | A was auto-discovered without manual Add and appeared as `A · Active`. |
| Re-authenticate A | Class R | PASS | Browser login completed; no duplicate row was created. |
| Normal Codex switch A → B | Class R | PASS | After Refresh, B appeared active, A remained retained, and no duplicate row appeared. Transitional display followed current B usage rather than retained A. |
| Manual Add for already-known B while A remained current | Class R | PASS | The Add flow completed with two rows only; normal Codex remained A; B was reconciled/attached rather than duplicated. |
| Restart after A+B | Class R | PASS | Both retained rows survived restart and active role was recomputed from current normal Codex. No persisted active field was used. |
| Re-authenticate inactive B | Class R | PASS bounded | Browser re-auth completed; rows remained deduplicated and A/current normal Codex stayed unchanged in the owner-observed UI. Per-owner byte/hash proof is not claimed here. |
| Remove inactive B | Class R | PASS | B was removed; A remained active; normal Codex stayed logged in. |
| Remove current B | Class R | PASS | B remained visible only as current-only while normal Codex was still B; retained Remove was unavailable and no immediate re-retain occurred. After switching normal Codex back to A and Refresh, B disappeared. |
| Direct account menu actions | Class R | PASS | Re-authenticate and Remove dispatched to the selected stable account after menu tracking; this is the corrected route-lifetime behavior. |
| Bounded initial usage read | Class S | PASS | Independently closed by deterministic selected-account success/error recording tests; transitional UI has no separate completion marker. |
| Four retained accounts / fifth Add disabled | Class F/S | PASS | Deterministic fixture tests prove four coexist and the fifth Add preflight rejects before login dispatch; no synthetic credentials were used. |
| Unknown current-only E at full capacity | Class F | PASS | Deterministic fixture proves E is current-only, A/B/C/D and owners are unchanged, and no eviction occurs. |
| Capacity release/reconciliation | Class F | PASS | Deterministic fixture proves freeing one retained entry permits later retention of the current identity. |

Working Codex safety result: `Class R — PASS bounded` for the owner-observed lifecycle state—normal Codex remained logged in after monitor removal and monitor actions did not visibly switch/logout the working account. `NOT PROVEN` for hash-identical normal `auth.json`/workspace/session state across this walkthrough because Sidik intentionally switched the normal Codex account during the test; that owner activity changes normal auth state and must not be attributed to the monitor. The prior controlled Phase 00 quiet interval remains the separate accepted proof for stable normal-state markers. No raw token, OAuth code, email, or account ID was recorded in this evidence.

Runtime/proof disposition: `Class R PASS` for the available two-account credential-sensitive behavior plus `Class F/S PASS` for the required deterministic cardinality and initial-usage closure. No third/fourth/fifth real account was required or fabricated. The prior A/B walkthrough is superseded except where the current proof contract explicitly preserves its Class R claims.

#### Scope guard

Phase 03 implementation starts after this transition commit. No Phase 03 source behavior, account switching, auto-switching, inference refresh, final collection taskbar rendering/tooltip/ring, or later-phase behavior is included in the Phase 02 checkpoint.

**Decision:** `PASS` — Sol Phase 02 gate. Production implementation checkpoint is `537b2bbad951ccbb43f04ba9067b55b304f4d232`; deterministic proof/test checkpoint is `2dd532525e48710dd03e4bea37819d052b117fc2`, with 70 tests passing. Class R two-account runtime evidence and Class F/S closure evidence are current. Phase 03 is authorized.

#### Sol Phase 02 gate verdict

- **Verdict:** `PASS`
- **Phase 02:** `complete/PASS`
- **Production implementation checkpoint:** `537b2bbad951ccbb43f04ba9067b55b304f4d232`
- **Deterministic proof/test checkpoint:** `2dd532525e48710dd03e4bea37819d052b117fc2`
- **Evidence checkpoint:** `214cccfa72a1ea0e51ed270762c8aa588e9333aa`
- **Approved proof contract:** Class R for available real-account credential-sensitive behavior; Class F/S for non-secret collection/capacity/state seams.
- **Next authorized action:** Phase 03 multi-account polling only.

### Phase 03 — Multi-Account Polling

**Status:** `complete/PASS — Class R + Class F/S`

Implementation checkpoint: `acb870d204d83bcba71534aeab5910784e74b1c1`.
This is the exact source/test checkpoint for the collection-driven polling
implementation. Phase 04 was not started in this checkpoint and is now active after the Sol Phase 03 PASS transition.

#### Class S — source/state-machine proof

- `poller::account_poll_targets()` is the single source-selection seam. It
  iterates `AccountRegistry::accounts()` and selects `MonitorOwner(handle)`,
  `WorkingCodex`, or `Unavailable` by stable account identity. It has no
  account-position branches and treats four as a registry policy, not a data
  shape.
- `poller::poll_account_collection()` publishes every result with the stable
  account id that was selected before I/O. Retained accounts use their own
  `MonitorAuthHandle`; an ownerless current account uses a direct read of the
  working Codex credential; an inactive ownerless account is explicitly
  unavailable. A current-only identity at full capacity is appended without
  mutating retained metadata or owners.
- Monitor-owner reads use `AuthManager::auth()` followed by
  `CodexAuth::get_account_id()`/`get_token()` and the direct usage endpoint.
  The collection path does not call the legacy `cli_refresh_codex_token()` or
  `codex exec .` fallback. Working current-only reads are also read-only and
  do not refresh or switch the normal Codex account.
- `apply_account_poll_result()` updates only the result's stable account id.
  Auth/credential failures become account-scoped `ReauthRequired`; transient
  failures become account-scoped `Unavailable`. Existing usage is retained
  only with `usage_stale = true`; successful data clears that marker. Missing
  quota windows remain `None` and never become synthetic zero.
- Non-Codex providers are polled separately in collection mode, so an
  account-scoped Codex failure does not widen into a global failure when
  another account/provider succeeds. Reset scheduling also considers every
  retained account's session and weekly reset independently.
- Account alert keys use an opaque hash of the stable account id plus
  provider, window, threshold, and reset identity. Default UI/evidence data
  does not expose account ids, email, or credential material.

#### Class F — deterministic fixture proof

The fixture/state tests use only fake stable identities and deterministic
usage/state values. They do not create OAuth, access tokens, refresh tokens,
keyring credentials, or synthetic files in production monitor namespaces.

| Focused test | Proof |
|---|---|
| `poller::tests::collection_poll_plan_uses_owner_and_working_sources_by_identity` | Four retained identities are planned by collection identity and independent monitor owners; no fixed A/B or position routing. |
| `poller::tests::collection_poll_plan_marks_inactive_ownerless_account_unavailable` | An inactive ownerless account is unavailable instead of borrowing the active account's source. |
| `poller::tests::full_capacity_current_identity_is_working_only_without_eviction_or_owner_theft` | A/B/C/D remain unchanged while current-only E is assigned only the working read source at full capacity. |
| `poller::tests::account_poll_result_application_is_scoped_to_stable_identity` | A healthy result and B auth/network failure update only their matching stable rows. |
| `account::tests::poll_failure_marks_only_one_account_stale_and_preserves_other_usage` | One account is explicitly stale/unavailable while the other remains connected and usable. |
| `window::tests::account_quota_alerts_are_opaque_and_deduplicated_per_identity` | Same-window alerts deduplicate independently for two stable identities without raw identity in the key. |
| Existing `account::tests::initial_usage_success_is_recorded_against_selected_account_only` and `initial_usage_failure_is_scoped_to_selected_account` | Initial usage success/error remains scoped to the selected stable account; other account state is unaffected. |
| Existing `account::tests::freeing_capacity_allows_later_current_identity_retention` | Reconciliation can retain a current identity after an explicit capacity release. |

#### Class R — real-account runtime boundary

Owner-observed walkthrough by Sidik against the Phase 03 implementation
checkpoint is complete. This is credential-sensitive runtime evidence for the
two available real accounts; no third/fourth/fifth real account is required by
the current proof contract.

| Scenario | Result | Owner-observed proof |
|---|---|---|
| Account A active and healthy | PASS | A polled successfully as the current active account. |
| Normal Codex switch A → B without restart | PASS | B became current without losing A's retained state. |
| Distinct A/B usage attribution | PASS | A and B displayed distinct usage values attached to the correct identities. |
| Switch B → A | PASS | A became current again and attribution remained correct. |
| Remove inactive B while A remains current | PASS | B retention/owner state was removed while A remained healthy. |
| Switch to removed B | PASS | B was auto-discovered as current-only/ownerless; no retained owner was borrowed. |
| Switch back to A | PASS | A remained healthy and B stayed retained only as the ownerless current-derived state observed in the walkthrough. |
| B recovery affordance | PASS | `Re-authenticate` remained available for B; no duplicate identity row appeared. |
| Credential ownership / eviction safety | PASS bounded | No evidence of credential borrowing, account eviction, or cross-account corruption was observed. |

The current-only/ownerless B state showed a partial or unavailable quota
window. This is an intentional safe result: missing or unsafe data was not
converted into synthetic `0%` remaining. No separate alert-balloon runtime
event or deliberately forced token-expiry event was performed; those specific
runtime events are not claimed.

The Phase 03 owner-refresh/auth-isolation and zero-inference closure is
attributed to the combined proof boundary: the historical Phase 00 Class R
real-owner refresh/deletion isolation proof using the same approved pinned
`codex-login` mechanism, plus the current Phase 03 Class S/F production-path
source and state-machine regression proof. The current Phase 03 Class R
walkthrough adds the observed A/B polling, attribution, active-role, removal,
and current-only safety proof.

#### Verification

- `cargo fmt --check`: PASS.
- `cargo test --locked`: PASS — 76 passed, 0 failed.
- `cargo clippy --all-targets --locked`: PASS exit; only the repository's
  existing clippy warnings remain, with no new Phase 03 warning.
- `cargo build --release`: PASS — optimized binary at
  `F:\PROJECT\Webapps\codex-usage-monitor\target\release\codex-usage.exe`.
- Alternate target build was attempted at
  `%TEMP%\codex-usage-phase03-polling-release-target`; it stopped with
  `No space left on device` after compiling dependencies. This is an
  environment limitation, not a source/build error; the default release build
  completed successfully.
- `git diff --check`: PASS.

#### Scope guard and disposition

No login UI, account switching, polling fan-out beyond the collection polling
seam, final multi-account taskbar renderer, tooltip/ring UI, or Phase 04/05
behavior was added. The accepted Phase 02 auth mechanism remains direct pinned
`codex-login` with `Keyring + Secrets`, isolated `monitor-auth/slot-*` owners,
no normal Codex workspace ownership, and no token copying.

**Decision:** `PASS` — Class R owner runtime PASS bounded, plus Class F/S
implementation and deterministic proof PASS, all against
`acb870d204d83bcba71534aeab5910784e74b1c1`. The separate alert-balloon and
deliberately forced token-expiry runtime events remain unclaimed. Sol has
accepted this Phase 03 evidence and authorized Phase 04.

#### Sol Phase 03 final gate verdict

- **Verdict:** `PASS`
- **Phase 03:** `complete/PASS`
- **Implementation checkpoint:** `acb870d204d83bcba71534aeab5910784e74b1c1`
- **Evidence checkpoint:** `a5fff27afbc3fbe2af10fa34d4264a88f4172705`
- **Next authorized action:** Phase 04 taskbar UI only.

### Phase 04 — Taskbar UI

**Status:** `ready-for-sol-final-gate`

Implementation checkpoint: `1cda2b33c7ed30166f364b26687a8876d347865d`.
Phase 04 source and deterministic visual/state evidence are complete. Sidik's
owner-runtime follow-up is accepted as bounded Class R evidence; Phase 04
remains ready for Sol final-gate reconciliation.

#### Class S — source/UI state proof

- Renderer input is a runtime snapshot derived from `AccountRegistry` plus the
  current working identity. It renders retained accounts in collection order
  and appends an unknown current identity as current-only without persisting or
  evicting retained accounts.
- Bar and Circle both render one two-row Codex block per account. Width is
  calculated from collection length and grows horizontally; taskbar height
  remains `WIDGET_HEIGHT = 46` at the tested 100% DPI baseline.
- Account Bar/Circle fill receives remaining percentage explicitly at the
  presentation boundary; canonical used values are not passed directly to the
  visual fill.
- Account collection blocks reserve an additional 8 px in the usage text
  column so two-digit reset countdowns such as `29d` are not clipped.
- Active state is derived by stable identity matching. The active identity chip
  uses a blue outline/ring; inactive chips use the neutral outline. No active
  role is persisted and no account switching control was added.
- The account identity chip is 24 px in the compact taskbar layout, with a 26
  px identity slot preserving spacing before the quota rows; widget height
  remains unchanged.
- Unavailable or stale account usage renders as `--`; missing windows remain
  unavailable and are never converted to synthetic `0%`.
- Tooltip reset values use humanized local date/time plus relative countdown,
  such as `Today, 13:17 · in 3h` or `Sep 7, 14:22 · in 6d`. Quota rows are
  drawn using explicit label/percentage/reset X columns, not proportional-font
  space padding.
- Tooltip status distinguishes an active ownerless account (`Connected via
  Codex` with a re-authentication note), an inactive ownerless account
  (`Re-authentication required`), and transient read failures (`Unavailable`).
- The hover hit-test covers the full account block across the complete widget
  height. A delayed, non-activating native Win32 tooltip reports display name,
  `ACTIVE` and current/monitored role, both remaining values, exact reset
  values, and connection status without email or opaque account ID.
- Existing direct collection-driven account menus and `Widget Style > Bar |
  Circle` persistence remain in place; this phase does not change their
  auth/polling behavior.
- Tooltip painting now uses the existing rounded window region and palette
  fill without drawing a visible border/stroke.
- Native context menus remain classic `HMENU`/`TrackPopupMenu`. Before menu
  construction, the app dynamically resolves UxTheme ordinals 135
  (`SetPreferredAppMode`) and 136 (`FlushMenuThemes`), selects `ForceDark` or
  `ForceLight` so the popup follows the Windows theme, and flushes menu themes. If the DLL or
  either export is unavailable, the resolver returns `Unsupported` and leaves
  standard native menu behavior unchanged; no owner-draw menu path was added.
- On Windows 11, the tooltip applies documented DWM `DWMWA_BORDER_COLOR` with
  `DWMWA_COLOR_NONE` after creation. This suppresses the system frame while
  retaining the rounded region; older Windows versions safely fall back when
  the attribute is unsupported.

#### Class F — deterministic visual/state proof

The UI fixtures use only fake identity/state values and do not create OAuth,
access tokens, refresh tokens, keyring credentials, or production auth files.

| Focused test/artifact | Proof |
|---|---|
| `window::tests::collection_render_data_keeps_retained_accounts_and_adds_current_only_overflow` | Four retained identities remain in order while current-only E is appended with active state and current usage, without owner mutation. |
| `window::tests::collection_width_grows_horizontally_without_changing_taskbar_height` | Collection width grows for 1–4 accounts while the taskbar widget height remains 46 px. |
| `window::tests::hover_hit_test_covers_the_entire_account_block` | Initial, quota rows, bars/circles, reset areas, and the full vertical block resolve to the same account hit region. |
| `window::tests::account_tooltip_uses_role_usage_and_status_without_identity_secrets` | Tooltip hierarchy includes role, active marker, remaining 5h/weekly values, and status without stable ID. |
| `window::tests::account_bar_and_circle_use_remaining_percentage_for_fill` | Account Bar/Circle fill and text use 81 used → 19 remaining, 55 used → 45 remaining, with 0/100 boundaries. |
| `window::tests::account_layout_reserves_width_for_two_digit_reset_countdown` | Account collection usage text reserves enough width for a two-digit countdown such as `52% · 29d`. |
| `window::tests::tooltip_reset_uses_human_date_and_relative_countdown` | Same-day and later-date reset labels use human-readable local dates and relative countdowns. |
| `window::tests::tooltip_columns_use_fixed_positions_without_monospace_padding` | Label, percentage, and reset columns have deterministic X positions. |
| `window::tests::tooltip_distinguishes_ownerless_active_from_inactive_and_transient_states` | Active ownerless, inactive ownerless, and transient failure states map to their locked tooltip semantics. |
| `window::tests::monitor_owner_stays_attached_when_active_role_moves_between_accounts` | Independent monitor owners remain attached while runtime active role moves to another identity. |
| Existing `window::tests::widget_style_serializes_and_defaults_to_bar` | Bar remains the default and Circle remains persisted through settings serialization. |
| Existing menu/routing tests | Direct per-account submenu, dynamic stable-identity routing, max-four Add disablement, and no fixed-slot command path remain green. |
| `theme::tests::native_popup_theme_uses_feature_detected_uxtheme_exports` | Pins the reviewed ordinal/mode contract used for the dynamic native popup-menu opt-in. |
| `theme::tests::native_popup_theme_missing_export_uses_native_fallback` | Deterministically proves any missing UxTheme export selects explicit `Unsupported` fallback while the complete export pair selects `Applied`. |
| `theme::tests::native_popup_theme_follows_the_system_palette` | Proves dark Windows selects UxTheme `ForceDark` and light Windows selects `ForceLight`. |
| `window::tests::account_initial_chip_is_larger_without_collapsing_text_spacing` | Proves the 24 px initial chip remains centered within the 46 px widget and retains a larger identity slot than the chip diameter. |

#### Visual artifacts

The following release-binary captures are fixture-only layout evidence. The
environment used the non-embedded fallback popup because no taskbar handle was
available; all captures retain the 46 px widget height.

| Artifact | Size | Class |
|---|---:|---|
| [phase04-bar-single.png](evidence/phase04-bar-single.png) | 165 × 46 | Class F/S |
| [phase04-circle-single.png](evidence/phase04-circle-single.png) | 120 × 46 | Class F/S |
| [phase04-bar-four-account-fixture.png](evidence/phase04-bar-four-account-fixture.png) | 827 × 46 | Class F |

The visual smoke environment could not reach the usage endpoint, so these
captures intentionally show unavailable values rather than fabricated quota
numbers. They are not real-account usage proof.

#### Class R — owner runtime proof

Sidik's owner-observed Phase 04 follow-up is accepted as bounded Class R
evidence. Existing Bar/Circle, single-account, A/B switch, menu lifecycle,
re-auth, and remove observations remain applicable because the
`673d97` to `82d3b54` tooltip/status delta did not alter those mechanisms.
The current checkpoint specifically reconfirms the changed tooltip/status
behavior.

| Scenario | Result | Class R observation |
|---|---|---|
| Real two-account taskbar rendering and attribution | PASS | A and B rendered as distinct taskbar account blocks with independently attributed quota values. |
| Active blue ring | PASS | The blue ring followed the normal Codex active identity during A/B switching. |
| Retained account with independent owner while another account is active | PASS | The inactive independently-owned account remained live and its owner stayed attached. |
| Current ownerless account | PASS | Working-Codex reading remained usable while active and tooltip status showed `Connected via Codex`. |
| Current ownerless tooltip guidance | PASS | Tooltip communicated `Re-authenticate to keep monitoring when inactive`. |
| Inactive ownerless account | PASS | The production state was `Re-authentication required`; no credential borrowing occurred. |
| Humanized reset and fixed tooltip columns | PASS | Reset labels were accepted in humanized local date/relative form and columns aligned visually. |
| Account-scoped tooltip/privacy and direct menu | PASS | Current and monitored tooltips remained account-scoped and privacy-safe; direct Accounts menu remained correct. |
| Representative real taskbar interaction | PASS | Owner accepted the representative taskbar interaction on Windows. |
| Tooltip border removal and final tooltip polish | PASS | Sidik accepted the current build's rounded, borderless tooltip, spacing, typography, and column layout. |
| Native menu theme and final menu polish | PASS bounded | Sidik confirmed the current build's tooltip and native menu behavior under the light Windows theme. Bar/background theme behavior is not claimed because it is outside this plan and non-blocking. |
| Larger account initial chip | PASS with non-blocking preference | Sidik accepted the 24 px chip visually, while noting it still feels slightly small. |
| Real three-account taskbar smoke | PASS | A third account was discovered by becoming current in normal Codex; the three account blocks rendered as a real taskbar smoke state. |
| Third account ownerless inactive transition | PASS | After normal Codex returned from account 3 to account 1, account 3 remained retained and correctly entered `Re-authentication required` without an independent monitor owner; no credential borrowing occurred. |

#### Theme runtime smoke

The earlier alternate release binary reached the native context-menu path on
the current Windows dark-theme environment and emitted the safe diagnostic
`native popup menu theme result=Applied system_dark=true`; the corrected
ForceDark/ForceLight implementation is included in the new release below. The
earlier smoke proves the feature-detected runtime path was reachable without
owner-draw code or a second menu implementation. It is a runtime smoke result,
not a visual owner acceptance claim: the existing 2S instance held the
single-instance mutex when the rebuilt corrected binary was launched, and the
shell capture helper could not capture desktop pixels. The light-theme path is
covered by the system-following mode contract and explicit missing-export
fallback tests. Sidik has now confirmed the light-theme tooltip and native
menu behavior. No separate owner acceptance is claimed for the bar/background
theme, which is outside the current plan and is non-blocking.

No new runtime claim is made for Explorer/taskbar restart, four-account real
cardinality, same-initial real accounts, or a deliberately exercised
multi-DPI walkthrough. Those remain Class F/S or pending rows below and must
not be treated as Phase 05 closure.

#### Verification

- `cargo fmt --check`: PASS.
- `cargo test --locked`: PASS — 91 passed, 0 failed.
- `cargo clippy --all-targets --locked`: PASS exit; existing repository
  warnings remain, with no new compile error.
- `CARGO_TARGET_DIR=<temp> cargo build --release --locked`: PASS — alternate
  optimized release build at
  `%TEMP%\codex-usage-phase04-theme-target\release\codex-usage.exe` after the
  tooltip/theme correction. SHA-256:
  `7464C3A36484406D2F49491D64E9B734C769E57DB8158C0718DABE6EB4DD692A`.
- `git diff --check`: PASS before checkpoint.

#### Scope guard and disposition

No changes were made to `poller.rs`, auth ownership, login lifecycle, quota
semantics, reset calculations, account switching, or multi-account polling.
Phase 05 remains blocked.

**Decision:** `READY FOR SOL FINAL GATE` — Class R owner-runtime PASS bounded
plus Class F/S UI implementation and deterministic evidence are complete
at `1cda2b33c7ed30166f364b26687a8876d347865d`. The new tooltip-frame,
native-menu theme, and larger identity-chip behavior
still have a focused owner visual check open; Phase 05 remains blocked.

### Phase 05 — Resilience & Acceptance

**Status:** `blocked-by-phase-04`

Evidence pending.

## Final audit

**Sol verdict:** `Phase 03 PASS; Phase 04 ready for Sol final gate`

Do not change this to PASS until the final implementation checkpoint has current TEST-MATRIX evidence and Sidik runtime acceptance.
