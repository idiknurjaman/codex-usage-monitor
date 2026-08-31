# Phase 02 — Account Login & Lifecycle

**Status:** `in-progress — amended`
**Goal:** reconcile the account model to collection-driven max-four retention, automatically discover the current working Codex identity, and provide safe manual add/re-auth/remove flows without changing the user's active Codex account.

## Current authority

This phase is governed by [`../PRODUCT-AMENDMENT-2026-08-31.md`](../PRODUCT-AMENDMENT-2026-08-31.md).

The pre-amendment two-account runtime walkthrough is superseded. Do not continue it until the implementation below is complete and ready for new evidence.

## Account lifecycle model

2S owns a collection of known/retained identities plus a runtime projection of which identity is currently active in normal Codex.

```text
Known account identity
  persistent, deduplicated by stable id

Active role
  runtime-only, derived from current normal Codex identity

Monitor credential owner
  optional isolated owner used for independent monitoring
```

Active status must never be persisted as an account type or credential owner.

## Capacity and collection capability

The implementation must be structurally collection/N-capable. Current runtime policy:

```text
MAX_RETAINED_ACCOUNTS = 4
```

Required reconciliation:

- replace fixed `Slot1/Slot2` capacity design with a validated collection-friendly logical owner handle/index or equivalent;
- do not merely add `Slot3/Slot4` enum variants;
- remove per-slot account/action assumptions from lifecycle routing;
- preserve one stable identity ↔ at most one retained account row;
- preserve one monitor credential owner ↔ at most one account;
- preserve deterministic ordering and non-secret persistence.

## Automatic working-account discovery

On startup and whenever the working Codex credential identity changes:

1. read the current stable identity through the existing account adapter;
2. if already known, mark that identity active at runtime;
3. if new and retained capacity is available, persist it as a known account;
4. when the current identity changes A → B, B becomes active and A remains retained until explicit removal;
5. never create a duplicate row for an identity discovered through both automatic and manual paths.

Automatic discovery is identity discovery only. It must not copy the working access/refresh token into a monitor owner.

If an auto-discovered identity has no independent monitor owner when it later becomes inactive, retain the identity but show an explicit unavailable/re-auth-required state until safe monitor ownership exists.

## Manual Add monitor account

Context-menu action: `Add monitor account...`.

Flow:

1. Preflight retained capacity and lifecycle availability.
2. If four retained accounts already exist, the menu/action is disabled and OAuth/login must not start.
3. Allocate an unused logical monitor owner dynamically.
4. Start the Phase 00 approved direct `codex-login` flow.
5. Complete/cancel/timeout transactionally.
6. Read stable identity.
7. If identity is new, add it to the registry subject to current policy.
8. If identity already exists, reconcile/attach/repair that account's monitor owner instead of creating a duplicate row.
9. Run bounded initial usage read.
10. Refresh account/menu/widget state.

Manual add must never replace the normal working Codex credential.

## Credential transaction requirements

The credential-owner transaction must remain safe across success, duplicate, identity mismatch, cancellation, timeout, and failure.

Before a login/re-auth attempt, capture owner state as `Absent` or a restorable previous credential snapshot when available.

- committed success → keep the new owner state;
- any non-commit exit after possible owner mutation → restore previous owner or clear newly-created owner;
- rollback failure → explicit degraded error; never claim previous ownership is safe when it may not be;
- re-auth may start even if the previous credential is missing;
- re-auth resolving to another stable identity must never silently replace the account identity.

## Re-authenticate

Each account entry directly exposes `Re-authenticate`.

- Applies only to the selected identity/monitor owner.
- Other account state remains intact.
- For current active account with no monitor owner, re-auth may provision a monitor owner without changing normal Codex active auth.
- Identity mismatch requires explicit failure/reconciliation; no silent replace.

## Remove from monitor

Each account entry directly exposes `Remove from monitor`.

- Remove only retained registry state and monitor-owned credential for that identity.
- Never call normal Codex logout or mutate working `~/.codex` state.
- Removing an inactive account removes it from the retained collection.
- Removing the currently active account removes retained/monitor ownership, but the current account remains visible as current-only while normal Codex is still logged into it.
- Do not immediately re-retain the same current identity simply because periodic identity observation repeats; removal must remain effective for the current active session.

## Full-capacity active identity

When four identities are retained and normal Codex independently changes to a new fifth identity:

- recognize and display the new identity as current-only;
- do not persist it automatically;
- do not evict any retained account;
- manual add remains disabled;
- after the user removes a retained account, the current identity may be retained through the normal discovery/reconciliation path.

## Accounts context menu

Remove the nested `Manage accounts >` submenu.

Target:

```text
Accounts >
  Account                         (disabled label)
  Sidik · Active              >
  Sol                         >
  ...
  ─────────────────────────────
  Manage account                  (disabled label)
  Add monitor account...
```

Each account submenu:

```text
Re-authenticate
Remove from monitor
```

Requirements:

- display name preferred; initial fallback;
- runtime current account may show `· Active`;
- menu actions route dynamically to account identity/owner, not fixed slot constants;
- `Add monitor account...` disabled at four retained accounts or while conflicting lifecycle work is active;
- `Cancel login` may appear only while an interactive login is active.

## Tasks

- [ ] Generalize account-owner handle/capacity model without fixed Slot1/Slot2 variants.
- [ ] Set runtime retained-account policy to four.
- [ ] Preserve duplicate identity and unique-owner invariants.
- [ ] Add automatic current-account discovery on startup and working-identity change.
- [ ] Add runtime active-role matching separate from persistence.
- [ ] Preserve/finish transaction-safe Add/Re-auth rollback behavior.
- [ ] Reconcile manual duplicate login into the existing identity rather than a duplicate row.
- [ ] Add direct per-account context-menu actions; remove nested Manage accounts layer.
- [ ] Disable fifth manual add before browser login.
- [ ] Implement current-only overflow behavior when normal Codex changes identity at full retained capacity.
- [ ] Implement remove semantics for active and inactive accounts without normal Codex logout.
- [ ] Run bounded initial usage read after successful owner provisioning.
- [ ] Add deterministic tests for all lifecycle transitions.

## Acceptance criteria

- First launch with working Codex Account A shows A automatically without requiring `Add monitor account...`.
- Switching normal Codex A → B makes B active and keeps A retained; no duplicate identity row appears.
- Manual Add B while A is active can create/reconcile B without changing normal Codex active auth.
- A manual login resolving to an already-known identity attaches/reconciles ownership rather than creating a duplicate row.
- Up to four retained identities are supported by policy without account-position hard-coding.
- Fifth manual add is disabled before OAuth/login starts.
- At full retained capacity, switching normal Codex to unknown E recognizes E current-only and silently evicts nothing.
- Cancel/timeout/failure leaves registry and credential ownership transactionally consistent.
- Re-auth for one account cannot damage another.
- Remove deletes only selected monitor ownership/retention and never logs normal Codex out.
- Restart reconstructs retained metadata without exposing secrets.
- No app-server/TUI/`codex exec`/inference/account-switching path is introduced for monitor auth.

## Hard stops

- No `Make Active`, `Switch`, `Use this account`, or automatic account switching.
- Do not use the working Codex auth file as an Add/Re-auth scratch target.
- Do not copy the working refresh token to make auto-discovered accounts independently monitorable.
- Do not encode the four-account policy as four enum variants or four account-specific lifecycle branches.
- If an inactive auto-discovered account cannot remain live without unsafe credential copying, preserve explicit unavailable/re-auth state and report the limitation; do not fake success.

## Evidence

Before Phase 02 can pass, update `../EVIDENCE.md` to the exact reconciled implementation checkpoint and record:

- automated lifecycle/generalization tests;
- current command verification;
- a new owner-approved runtime walkthrough covering automatic discovery, A→B switch retention, manual add, duplicate reconciliation, restart, re-auth, remove, and max-four/fifth-add-disabled behavior;
- working-Codex before/after safety proof without raw credentials.

Then stop at `READY FOR SOL REVIEW`. Phase 03 remains blocked until Sol PASS.
