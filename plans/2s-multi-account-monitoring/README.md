# 2S Multi-Account Monitoring

**Status:** `in-progress`
**Current phase:** `04-taskbar-ui`
**Implementation branch:** `feat/2s-multi-account-monitoring`
**Plan authored from checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1`
**Current product amendment baseline:** `ba7721d2bde2659e18e34db45757a4023dd4ecc8`
**Current Phase 02 implementation checkpoint:** `537b2bbad951ccbb43f04ba9067b55b304f4d232`

## Mandatory authority order

Before executing, reviewing, or gating this plan, read in this order:

1. root `README.md`;
2. `plans/README.md`;
3. this file;
4. [`PRODUCT-AMENDMENT-2026-08-31.md`](./PRODUCT-AMENDMENT-2026-08-31.md);
5. [`PROOF-CONTRACT-AMENDMENT-2026-09-01.md`](./PROOF-CONTRACT-AMENDMENT-2026-09-01.md);
6. the current phase document;
7. [`TEST-MATRIX.md`](./TEST-MATRIX.md);
8. [`EVIDENCE.md`](./EVIDENCE.md).

The proof-contract amendment is authoritative for **proof modality** from Phase 02 onward. Where older phase/test text literally requires three, four, or five distinct real accounts solely for count/capacity/render cardinality, the proof-contract amendment supersedes that requirement. The product behavior itself is unchanged.

## Goal

Monitor Codex usage through a collection-driven account model that distinguishes the account currently active in the user's normal Codex runtime from retained monitor accounts, while leaving normal Codex workspace, sessions, history, config, and active credential ownership untouched.

Current product policy:

- capability is structurally collection/N-capable;
- maximum **four retained accounts** in this iteration;
- current normal-Codex identity is auto-discovered;
- runtime active role is derived, never persisted as ownership;
- manual `Add monitor account...` is a secondary path;
- previously active identities remain retained until explicit removal;
- no account switching by 2S;
- fifth manual Add is disabled before OAuth starts;
- four retained + unknown external current identity produces current-only overflow without silent eviction.

`2S` is the codename, not a two-slot contract.

## Product authority and execution

- **Sidik:** final product authority and runtime acceptance.
- **Luna:** sole implementation writer/executor.
- **Sol:** architecture review, plan authoring/amendment, phase-gate audit, and final closure audit.

Luna may proceed without asking Sidik to choose details already resolved by current authority. Any unresolved auth/credential contradiction is a hard stop.

## Canonical account model

Keep these concepts separate:

```text
Account identity
  stable identity for one Codex account

Active role
  runtime-only: identity currently authenticated in normal Codex

Monitor credential owner
  optional isolated credential owner used to keep an account readable
  when it is not the current working account
```

The persistent registry is a collection of retained identities, not `account_a/account_b` fields. Active status is not persisted. Account actions, polling, alerts, menu routing, and rendering iterate the collection rather than hard-code account positions.

### Automatic discovery

On startup and when the working Codex identity changes:

1. read the current stable identity through the existing adapter boundary;
2. if already known, mark it active at runtime;
3. if new and retained capacity is available, retain it automatically;
4. when A → B, B becomes active and A remains retained until removal;
5. never duplicate an identity discovered through both automatic and manual paths.

Automatic discovery is identity discovery only. It never authorizes copying the working access/refresh token into a monitor owner.

If an inactive retained account has no independent monitor owner, keep it visible with explicit re-auth/unavailable state instead of faking live monitoring.

### Manual Add

`Accounts > Add monitor account...` may add a new identity or reconcile/attach an isolated monitor owner to an existing identity. It must never replace normal working Codex auth.

### Capacity

```text
MAX_RETAINED_ACCOUNTS = 4
```

Four is policy, not type shape. Do not encode the capability as `Slot1|Slot2|Slot3|Slot4`, account-specific fields, fixed poll functions, or per-slot action constants.

At four retained accounts:

- manual Add is disabled before OAuth/login dispatch;
- an externally selected unknown current Codex identity is still recognized current-only;
- no retained identity is silently evicted;
- capacity must be freed explicitly before the new identity can be retained.

## Credential boundary

- Normal Codex remains source of truth for the user's current credential and workspace/session/history/config state.
- Monitor owners contain auth credentials only, not a second Codex workspace.
- Never copy/duplicate a refresh token into independently refreshing owners.
- Approved monitor auth mechanism remains direct pinned `codex-login`, `Keyring + Secrets`, isolated monitor-auth namespace, zero inference.
- Raw credentials/tokens never enter settings, logs, diagnostics, screenshots, tooltips, or evidence.
- One account failure must remain account-scoped.
- Missing usage windows remain unavailable, never synthetic `0%`.

## Locked account UX

### Taskbar

- compact identity initial/name chip;
- current account gets a subtle blue outline/ring;
- inactive accounts use neutral outline;
- no permanent `ACTIVE` word in compact taskbar.

### Whole-account tooltip

Hovering anywhere over one account block opens the same account-scoped tooltip with display name, current/monitored role, 5h and weekly remaining + exact reset, and connection state. Do not expose email or opaque account ID by default.

### Context menu

```text
Accounts >
  Account
  Sidik · Active >
  Sol            >
  ...
  ─────────────────────
  Manage account
  Add monitor account...
```

`Account` and `Manage account` are disabled section labels. Each account submenu directly contains:

```text
Re-authenticate
Remove from monitor
```

No nested `Manage accounts >` layer. Routing is dynamic by stable identity. `Remove from monitor` never logs normal Codex out.

## Proof model

The current proof contract deliberately separates **credential truth** from **higher-cardinality state truth**.

### Real-account proof

Use available real accounts for OAuth/credential-sensitive behavior: auto-discovery, A↔B active movement, manual Add/reconcile, restart, re-auth, remove, real usage/refresh, and working-Codex safety.

Two real accounts are sufficient for these claims unless a future behavior genuinely depends on more than two simultaneous real credential owners.

### Deterministic fixture proof

Use non-secret identity/state/data fixtures for four/five-account count/capacity/orchestration/rendering behavior. Fixtures must not fabricate OAuth success or credential isolation. Full rules are in `PROOF-CONTRACT-AMENDMENT-2026-09-01.md`.

This means the max-four requirement remains mandatory, but Sidik is not required to create extra real Codex accounts solely to exercise cardinality.

## Phase order

| # | Phase | Gate |
|---|---|---|
| 00 | [Auth spike](./phases/00-auth-spike.md) | Isolated zero-inference monitor credential ownership |
| 01 | [Account registry](./phases/01-account-registry.md) | Stable identity/ownership boundary; historical two-slot policy superseded |
| 02 | [Account login & lifecycle](./phases/02-account-login.md) | Collection-driven max-four lifecycle, auto-discovery, manual Add, active attribution, safe auth lifecycle |
| 03 | [Multi-account polling](./phases/03-multi-account-polling.md) | Independent collection polling, failure isolation, scoped alerts |
| 04 | [Taskbar UI](./phases/04-taskbar-ui.md) | Collection rendering, active indication, tooltip, direct account menu |
| 05 | [Resilience & acceptance](./phases/05-resilience-acceptance.md) | Full Definition of Done and runtime acceptance |

Phases 00 and 01 have passed their historical Sol gates. Phase 02 has passed the amended Sol gate. Phase 03 has passed its Sol final gate. Phase 04 is active; Phase 05 remains blocked until Phase 04 passes.

## Historical evidence disposition

Evidence is authoritative only for the exact checkpoint/mechanism and proof class it actually exercised.

- Phase 00 isolation proof and Phase 01 identity/owner separation remain valid.
- Historical max-two/two-slot assertions are not current product requirements.
- The owner-observed two-account amended Phase 02 walkthrough at HEAD `5de898eca809dcc5011fae7587340e8dbbdc3a3a` remains valid real-account evidence for implementation checkpoint `537b2bbad951ccbb43f04ba9067b55b304f4d232`.
- Its prior max-four/fifth-account `NOT PROVEN` entries are now fixture-proof requirements, not product failures, under the proof-contract amendment.

## Non-negotiable architecture rules

1. Monitoring and switching are separate domains.
2. Working Codex remains source of truth for current active identity/workspace state.
3. Active role is runtime-derived, never persisted as ownership.
4. Monitor auth roots own credentials only.
5. Never duplicate a refresh token across independently refreshing owners.
6. Prefer Codex-managed auth/refresh over custom refresh logic.
7. Polling makes no intentional inference request.
8. No credentials/raw tokens in persisted or user-visible evidence surfaces.
9. One account failure is isolated.
10. Missing windows are unavailable, never synthetic zero.
11. Canonical semantics remain `used_percentage: Option<f64>` with remaining conversion at presentation/alert boundaries.
12. Account/auth/menu/poll/render capability is collection-driven; four is policy only.
13. Duplicate identity reconciles into one row.
14. Capacity handling never silently evicts.

## Hard stops

Stop and report instead of guessing if:

- safe ownership would require splitting normal Codex workspace state;
- auto-discovery would require copying working refresh token;
- refresh/poll requires inference traffic;
- account attribution cannot distinguish active identity from retained identities;
- monitor lifecycle can mutate/replace normal working Codex auth;
- two owners would race on the same refresh token;
- the implementation requires account-position hard-coding;
- a proposed deterministic fixture would need to fake OAuth/credential ownership rather than exercise a pure state seam.

## Definition of Done

The plan is complete only when:

- every current `TEST-MATRIX.md` requirement is proven using its authorized proof mode from the proof-contract amendment;
- `EVIDENCE.md` is current to the final implementation checkpoint and labels real runtime vs fixture/state-machine evidence accurately;
- required command verification passes;
- Sidik accepts representative real-account UI/runtime behavior;
- Sol completes final audit with no open blocker/high-severity finding.

Required final command proof:

```powershell
cargo fmt --check
cargo test
cargo clippy --all-targets
cargo build --release
git diff --check
```

## Current Phase 04 status and next authorized action

Phase 02 implementation checkpoint remains `537b2bbad951ccbb43f04ba9067b55b304f4d232`; its deterministic proof/test checkpoint is `2dd532525e48710dd03e4bea37819d052b117fc2`, and Sol has issued a PASS gate.

Phase 03 collection-driven polling implementation and owner-observed runtime evidence passed Sol's final gate. Phase 04 is authorized for taskbar collection rendering only. The accepted Phase 00–03 auth mechanism, owner isolation, active-role boundary, and no-switching/no-inference constraints remain in force.

Luna may execute **Phase 04 taskbar UI only** and stop at its Sol final-gate review. Do not begin Phase 05 until Sol issues Phase 04 PASS.
