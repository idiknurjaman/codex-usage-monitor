# Phase 02 — Account Login & Lifecycle

**Status:** `in-progress — amended proof closure`
**Goal:** reconcile the account model to collection-driven max-four retention, automatically discover the current working Codex identity, and provide safe manual add/re-auth/remove flows without changing the user's active Codex account.

## Current authority

This phase is governed by both:

- [`../PRODUCT-AMENDMENT-2026-08-31.md`](../PRODUCT-AMENDMENT-2026-08-31.md) for product/account behavior;
- [`../PROOF-CONTRACT-AMENDMENT-2026-09-01.md`](../PROOF-CONTRACT-AMENDMENT-2026-09-01.md) for accepted proof modality.

Where older Phase 02 wording required three, four, or five distinct real accounts solely to prove collection cardinality/capacity, the proof-contract amendment supersedes that proof requirement. The max-four product behavior itself is unchanged.

## Account lifecycle model

2S owns a collection of retained identities plus a runtime projection of which identity is currently active in normal Codex.

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

Required behavior:

- validated collection-friendly logical owner handle/index or equivalent;
- no `Slot1|Slot2|Slot3|Slot4` capacity type;
- no per-slot lifecycle/action routing;
- one stable identity ↔ at most one retained row;
- one monitor credential owner ↔ at most one account;
- deterministic ordering and non-secret persistence.

## Automatic working-account discovery

On startup and whenever working Codex identity changes:

1. read the current stable identity through the existing account adapter;
2. if already known, mark it active at runtime;
3. if new and retained capacity is available, retain it automatically;
4. when A → B, B becomes active and A remains retained until explicit removal;
5. never duplicate an identity discovered through automatic and manual paths.

Automatic discovery is identity-only. It must not copy working access/refresh tokens into a monitor owner.

If an auto-discovered identity lacks an independent monitor owner after it becomes inactive, retain the identity but show explicit re-auth/unavailable state until safe ownership exists.

## Manual Add monitor account

Context-menu action: `Add monitor account...`.

Flow:

1. preflight retained capacity and lifecycle availability;
2. at four retained accounts, disable/reject before OAuth/login dispatch;
3. allocate unused logical monitor owner dynamically;
4. start approved direct `codex-login` flow;
5. complete/cancel/timeout transactionally;
6. read stable identity;
7. add new identity or reconcile existing identity;
8. never create a duplicate row;
9. run bounded initial usage read;
10. refresh account/menu/widget state.

Manual Add must never replace normal working Codex credentials.

## Credential transaction requirements

Before Add/Re-auth, capture owner state as `Absent` or restorable previous credential snapshot when available.

- committed success → keep new owner state;
- any non-commit exit after possible owner mutation → restore previous owner or clear newly-created owner;
- rollback failure → explicit degraded error;
- missing previous owner must not block true re-auth;
- re-auth resolving to another stable identity must not silently replace the selected account.

## Re-authenticate

Each account directly exposes `Re-authenticate`.

- scope to the selected identity/monitor owner;
- other accounts remain intact;
- current active account without monitor owner may provision one without changing normal Codex active auth;
- identity mismatch is explicit failure/reconciliation, never silent replacement.

## Remove from monitor

Each account directly exposes `Remove from monitor`.

- remove only retained state and selected monitor-owned credential;
- never call normal Codex logout;
- inactive remove removes the retained row;
- current-account remove removes retention/monitor ownership but leaves current identity visible current-only while normal Codex remains logged in;
- repeated observation of the same current identity must not immediately re-retain it during that active session.

## Full-capacity active identity

When A/B/C/D are retained and normal Codex independently changes to unknown E:

- recognize E current-only;
- do not persist E automatically;
- do not evict A/B/C/D;
- manual Add remains disabled;
- after capacity is explicitly freed, E may be retained through normal discovery/reconciliation.

## Accounts context menu

Target:

```text
Accounts >
  Account
  Sidik · Active >
  Sol            >
  ...
  ─────────────────────────────
  Manage account
  Add monitor account...
```

`Account` and `Manage account` are disabled labels. Each account submenu contains:

```text
Re-authenticate
Remove from monitor
```

Routing is dynamic by stable account identity. No nested `Manage accounts >` layer and no fixed per-slot command IDs.

## Implementation status

Current implementation checkpoint under review:

```text
537b2bbad951ccbb43f04ba9067b55b304f4d232
```

At this checkpoint the source/automated gate has already established:

- dynamic monitor-owner index with legacy physical owner continuity;
- max-four policy;
- optional monitor owner for auto-discovered identities;
- automatic current-account discovery and runtime active role;
- current-only overflow state model;
- duplicate manual identity reconciliation;
- direct collection-driven account menu routing;
- fifth Add preflight policy;
- active/inactive remove semantics;
- transaction-safe Add/Re-auth rollback;
- bounded initial usage operation;
- corrected dynamic Win32 menu-route lifetime.

## Real-account runtime evidence already accepted for this checkpoint

Owner-observed runtime with the available two real accounts has proven:

- startup auto-discovery of A;
- A → B switch: B active, A retained, no duplicate;
- manual Add resolving to known B reconciles/attaches rather than duplicating;
- restart preserves retained rows and re-derives active role;
- Re-authenticate routes and completes for the selected real account;
- remove inactive B;
- remove current B leaves normal Codex logged in and B current-only until identity changes;
- direct Re-auth/Remove menu routing works after the route-lifetime correction;
- captured evidence contains no raw credential/token/OAuth code/email/account ID.

This is Class R evidence under the proof-contract amendment and does not need to be rerun unless implementation changes invalidate it.

## Remaining Phase 02 proof closure

No extra real Codex accounts are required.

Luna must add/confirm deterministic Class F/S proof for:

- A/B/C/D can coexist as four retained identities without position-specific branches;
- fifth manual Add is disabled/rejected before OAuth/login dispatch;
- at full retained capacity, unknown E remains current-only;
- A/B/C/D are not silently evicted or assigned E's ownership;
- after capacity is freed, normal retention/reconciliation can proceed;
- bounded initial usage result/error is consumed into the selected account state;
- fixture/state-machine evidence contains no synthetic auth token/credential claims.

Fixtures must be identity/state/data fixtures only. Do not fabricate OAuth credentials, refresh tokens, or fake real login success.

## Acceptance criteria

Phase 02 PASS requires the combined proof set:

### Class R — real-account runtime

- first launch auto-discovers A;
- A → B moves runtime active role and retains A;
- manual Add/reconcile works without changing normal Codex active auth;
- restart re-derives active role;
- Re-auth is account-scoped;
- remove inactive/current semantics are correct and never log normal Codex out;
- real menu routing works;
- evidence is secret-safe.

### Class F/S — deterministic

- four retained identities supported under policy;
- fifth Add unavailable before login dispatch;
- full-capacity unknown E current-only with no eviction/owner theft;
- capacity release/reconciliation transition correct;
- rollback/mismatch/missing-owner deterministic paths remain safe;
- one-shot initial usage completion/error recording is proven at the correct production seam;
- architecture remains collection-driven and max-four is policy, not shape.

No third, fourth, or fifth real account is required solely for these higher-cardinality claims.

## Hard stops

- no Make Active/Switch/Use-this-account behavior;
- no working-Codex auth file as Add/Re-auth scratch;
- no copying working refresh token into monitor owner;
- no fixed four-account type/branch design;
- no synthetic credential fixture presented as auth proof;
- no fake success for inactive accounts without safe monitor ownership.

## Evidence

Before Phase 02 can pass:

1. retain the already-recorded two-account Class R runtime evidence;
2. add exact deterministic fixture/test names and results for the Class F/S closure items;
3. label each evidence item as real runtime, deterministic fixture, or source/state-machine proof;
4. run current verification commands;
5. record the exact implementation checkpoint used by those tests;
6. update `../EVIDENCE.md` accordingly;
7. stop at `Phase 02 — READY FOR SOL FINAL GATE`.

Phase 03 remains blocked until Sol reviews the fixture proof and issues Phase 02 PASS.
