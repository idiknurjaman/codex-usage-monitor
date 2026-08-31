# 2S Multi-Account Monitoring

**Status:** `in-progress`
**Current phase:** `02-account-login`
**Implementation branch:** `feat/2s-multi-account-monitoring`
**Plan authored from checkpoint:** `f5c090c58d45e12eed4c9f564733bf7a974a9ac1`
**Current product amendment baseline:** `ba7721d2bde2659e18e34db45757a4023dd4ecc8`

## Goal

Monitor Codex usage through a collection-driven account model that distinguishes the account currently active in the user's normal Codex runtime from retained monitor accounts, while leaving the user's normal Codex workspace, sessions, history, config, and active credential untouched.

The account capability must not be structurally limited to two accounts. Product policy for this iteration is:

- up to **four retained accounts**;
- automatic discovery of the current working Codex identity;
- manual `Add monitor account...` as a secondary path;
- active account determined at runtime from the working Codex identity;
- inactive retained accounts remain in the monitor until the user explicitly removes them;
- no Codex account switching by 2S.

`2S` remains the project codename. It is not a two-slot capacity contract.

## Product authority and execution

- **Sidik:** final product authority and runtime acceptance.
- **Luna:** sole implementation writer/executor for implementation work.
- **Sol:** architecture review, plan authoring/amendment, phase-gate audit, and final closure audit.

Luna may proceed from this plan without asking Sidik to choose details already resolved here. Any unresolved credential/auth contradiction is a hard stop and must be surfaced rather than guessed through.

## Canonical account model

Three concepts must remain separate:

```text
Account identity
  stable identity for one Codex account

Active role
  runtime-only: identity currently authenticated in normal Codex

Monitor credential owner
  optional isolated credential owner used to keep an account readable
  when it is not the current working Codex account
```

A single account record may move between active and inactive roles. Active status is never a persisted account property.

The persistent registry is a collection of known/retained account identities, not `account_a/account_b` fields. Account actions, polling, alerts, menu routing, and rendering must iterate the collection rather than hard-code account positions.

### Automatic discovery

On startup and when the working Codex identity changes:

1. Read the current stable Codex identity through the existing adapter boundary.
2. If that identity is already known, mark it active at runtime; do not create a duplicate.
3. If it is new and retained capacity is available, add the identity to the known-account registry automatically.
4. When Codex later switches from A to B, B becomes active and A remains retained until explicit removal.

Automatic discovery does **not** authorize token copying. An account is only considered independently live-monitorable after it has an approved independent monitor credential owner. If an auto-discovered account lacks such an owner after it becomes inactive, keep the identity visible with an explicit unavailable/re-auth state until safe monitor ownership is established. Do not fake live monitoring or duplicate the working refresh token.

### Manual add

`Accounts > Add monitor account...` remains available as a secondary path for an account the user wants to monitor without first making it active in Codex.

If manual login resolves to an identity already present in the registry, reconcile/attach the monitor credential to that identity rather than create a duplicate account row.

### Runtime capacity policy

The implementation capability must be collection/N-capable. The current product policy is:

```text
MAX_RETAINED_ACCOUNTS = 4
```

The fifth **manual** add action must be disabled before OAuth/login starts.

If four accounts are retained and the user independently switches normal Codex to an unknown fifth identity, 2S must still recognize the current identity because it cannot hide or veto the user's Codex login. That identity is current-only and is not silently persisted or used to evict an existing retained account. The UI must make capacity state understandable and allow the user to remove a retained account before retaining the new identity.

No silent eviction is allowed.

## Credential ownership rules

- The normal Codex workspace remains the source of truth for normal Codex sessions/history/config and current active credential.
- Monitor credentials are isolated credential owners only; they are never second Codex workspaces.
- Never copy or duplicate one refresh token into independently refreshing owners.
- Prefer the approved direct pinned `codex-login` mechanism with `Keyring + Secrets`.
- A monitor account may have no independent owner yet; that is an explicit capability/state, not synthetic success.
- A quota poll must never make an inference request.
- Raw credentials/tokens must never enter settings, logs, diagnostics, screenshots, tooltips, or evidence.

## Locked account UX

### Taskbar active indication

- Widget account identity stays compact: one-character initial/name chip as already established.
- The current Codex account is indicated only by a subtle **blue outline/ring** around its identity chip.
- Inactive monitored accounts keep the neutral border.
- Active styling is derived from runtime identity matching, not persisted.

### Whole-account hover tooltip

Hovering anywhere over an account block — initial, 5h row, weekly row, bars, percentage, or reset text — opens the same compact account tooltip.

Tooltip content should include:

```text
Account display name                     ACTIVE   (only when current)
Current Codex account / Monitored account

Usage remaining
5h       <remaining %>       <exact reset time>
Weekly   <remaining %>       <exact reset date/time>

<Connected | Re-auth required | Unavailable>
```

Do not expose email or opaque account IDs by default. Tooltip is descriptive only; account management actions stay in the context menu.

### Accounts context menu

Remove the nested `Manage accounts >` layer.

Target information architecture:

```text
Accounts >
  Account                         (disabled section label)
  Sidik · Active              >
  Sol                         >
  ...
  ─────────────────────────────
  Manage account                  (disabled section label)
  Add monitor account...
```

Each account directly owns its submenu:

```text
Sidik · Active >
  Re-authenticate
  Remove from monitor
```

Rules:

- use display name when available; initial is fallback only;
- `· Active` may appear in the menu for the runtime current account;
- `Remove from monitor` never logs normal Codex out;
- removing the current account removes retained/monitor ownership, but the account remains visible while it is still the current Codex account;
- `Add monitor account...` is disabled at four retained accounts and while conflicting lifecycle work is active;
- do not create per-slot menu constants such as `REAUTH_SLOT1/2` or `REMOVE_SLOT1/2`; routing must be collection-driven.

## In scope

- Collection-driven account registry and auth-owner allocation.
- Product limit of four retained accounts.
- Automatic working-Codex identity discovery and runtime active-role attribution.
- Manual monitor-account add.
- Safe per-account monitor credential ownership, re-authentication, and removal.
- Independent 5h/7d usage polling and account-scoped failure/alert state.
- Bar/Circle rendering for the account collection.
- Active blue identity ring and whole-account informational tooltip.
- Direct per-account context-menu actions.

## Out of scope

- Switching the active Codex account.
- Auto-switching when quota is low.
- Routing Codex inference requests between accounts.
- Changing `~/.codex` sessions, history, project state, or config.
- Copying monitor credentials into or out of the active Codex credential.
- Proxying inference traffic.
- Unlimited retained accounts in this iteration; runtime policy remains four.

## Non-negotiable architecture rules

1. **Monitoring and account switching are separate domains.**
2. Working Codex state remains source of truth for current active identity and normal workspace state.
3. Active role is runtime-derived and never persisted as ownership.
4. Monitor auth roots own credentials only, never sessions/history/config/workspace state.
5. Never duplicate a refresh token into multiple independently refreshing owners.
6. Prefer Codex-managed authentication/refresh over custom OAuth refresh logic.
7. A quota poll must not make inference traffic or intentionally consume quota.
8. Credentials and raw tokens never appear in settings/logs/diagnostics/screenshots/evidence/UI.
9. One account failing must not invalidate, hide, or pause another healthy account.
10. Unknown/missing quota windows remain unavailable, never synthetic `0%`.
11. Canonical model semantics remain `used_percentage: Option<f64>`; remaining conversion occurs at presentation/alert boundaries.
12. Account/auth/menu/poll/render capability must be collection-driven; runtime policy, not type shape, enforces the four-account limit.
13. Duplicate identity is reconciled, never represented as two account rows.
14. Capacity handling never silently evicts a retained identity.

## Phase order

| # | Phase | Gate |
|---|---|---|
| 00 | [Auth spike](./phases/00-auth-spike.md) | Prove isolated, zero-inference monitor credential ownership without mutating working Codex state |
| 01 | [Account registry](./phases/01-account-registry.md) | Stable identity/ownership boundary; historical two-slot policy is superseded by the current amendment |
| 02 | [Account login & lifecycle](./phases/02-account-login.md) | Reconcile the model to collection-driven max-four, add auto-discovery/manual add, active attribution, transaction-safe auth lifecycle |
| 03 | [Multi-account polling](./phases/03-multi-account-polling.md) | Independent collection polling, failure isolation, account-scoped alerts |
| 04 | [Taskbar UI](./phases/04-taskbar-ui.md) | Collection rendering, active indication, tooltip, direct account menu, no regressions |
| 05 | [Resilience & acceptance](./phases/05-resilience-acceptance.md) | Full Definition of Done and runtime acceptance |

Phases 00 and 01 have passed their original Sol gates. The current product amendment supersedes their two-account capacity assumptions without reopening the proven identity/credential-ownership boundaries. Phase 02 is authorized to perform the required account-domain reconciliation. Phases 03–05 remain blocked.

## Historical evidence disposition

Evidence produced before this amendment remains valid only for the exact mechanism/checkpoint it proved. In particular, Phase 00 isolation proof and Phase 01 identity/owner separation remain authoritative; prior max-two, two-slot, Add-A/Add-B menu, and two-account runtime expectations are historical and do not prove the amended product behavior.

The Phase 02 runtime walkthrough requested before this amendment is **paused/superseded**. Do not continue it until Phase 02 implementation is reconciled to this document and current evidence is recorded.

See [`PRODUCT-AMENDMENT-2026-08-31.md`](./PRODUCT-AMENDMENT-2026-08-31.md) for the concise owner-approved amendment record.

## Hard stops

Stop implementation and report to Sol/Sidik if any of these occur:

- safe per-account credential isolation requires splitting normal Codex sessions/history/config;
- seamless auto-discovery would require copying the working refresh token into a monitor owner;
- the refresh/poll method requires inference traffic;
- account attribution cannot reliably distinguish the runtime active identity from retained identities;
- a monitor lifecycle operation can mutate/replace normal working Codex auth;
- two monitor owners would race on the same refresh token;
- a required auth mechanism relies on undocumented behavior that cannot be proven in a focused runtime test;
- the implementation requires account-position hard-coding rather than collection-driven routing.

## Definition of Done

The plan is complete only when all current rows in [`TEST-MATRIX.md`](./TEST-MATRIX.md) are PASS, [`EVIDENCE.md`](./EVIDENCE.md) contains current evidence for the final implementation checkpoint, Sidik accepts runtime UI/behavior, and Sol completes final audit.

Required final command proof:

```powershell
cargo fmt --check
cargo test
cargo clippy --all-targets
cargo build --release
git diff --check
```

Existing unrelated Clippy warnings may be recorded, but the plan must introduce no new warning caused by its changes.

## Next authorized action

Execute **Phase 02 reconciliation only** from the current branch. Do not run the superseded two-account walkthrough and do not begin recurring polling fan-out, final multi-account rendering, account switching, or later phases until Phase 02 passes Sol review.
