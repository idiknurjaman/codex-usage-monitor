# 2S Product Amendment — 2026-08-31

**Authority:** Sidik product clarification, authored by Sol
**Baseline HEAD:** `ba7721d2bde2659e18e34db45757a4023dd4ecc8`
**Applies to:** Phase 02 onward and any Phase 01 implementation details that encode obsolete two-slot capacity assumptions

## Decision

The plan is no longer a two-slot product contract.

- Capability must be collection-driven and structurally N-account capable.
- Runtime product policy is **maximum four retained accounts**.
- The identity currently authenticated in normal Codex is the runtime **active** account.
- Active status is not persisted and does not imply credential ownership.
- Working Codex login/switch automatically discovers account identities.
- A previously active identity remains retained after switching until the user removes it.
- Manual `Add monitor account...` remains available as a secondary path.
- Duplicate identity from auto/manual discovery is reconciled into one account record.
- The fifth manual add action is disabled before login starts.
- If normal Codex itself changes to an unknown identity while four accounts are retained, 2S recognizes the current identity without silent eviction; it stays current-only until capacity is freed.

## Credential boundary

Automatic discovery does not authorize copying the working credential or refresh token.

An inactive account is independently live-monitorable only when an approved isolated monitor credential owner exists. If an auto-discovered account lacks one, retain the identity and show an explicit unavailable/re-auth state after it becomes inactive until safe ownership is established.

The accepted auth mechanism remains direct pinned `codex-login` at SHA `94cbbddafc1776d5e377bca1b05932c697e82238`, `Keyring + Secrets`, isolated monitor-auth namespace, zero inference, and no normal Codex workspace ownership.

## Locked UI behavior

### Taskbar

- Active account identity chip uses a subtle blue outline/ring.
- Inactive monitored accounts use neutral outline.
- No persistent `ACTIVE` word is required in the compact taskbar widget.

### Tooltip

Hovering anywhere over the full account block opens one compact informational tooltip containing:

- display name;
- `ACTIVE` badge only for current Codex account;
- current-vs-monitored account description;
- 5h remaining percentage + exact reset time;
- weekly remaining percentage + exact reset date/time;
- connection state.

Do not expose email or opaque account ID by default.

### Context menu

Remove the nested `Manage accounts >` level.

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

`Account` and `Manage account` are disabled section labels.

Each account submenu directly contains:

```text
Re-authenticate
Remove from monitor
```

Use display name when available; initial is fallback only.

`Add monitor account...` is disabled when four retained accounts exist or a conflicting lifecycle operation is active.

## Implementation consequence

Remove account-position coupling such as:

- `Slot1/Slot2` enum-only capacity design;
- `account_a/account_b` state;
- per-slot polling functions;
- per-slot re-auth/remove menu constants.

Use validated logical slot/index or equivalent opaque owner handle, collection iteration, and dynamic action routing. The runtime limit of four is policy, not type shape.

## Evidence consequence

The pre-amendment Phase 02 runtime walkthrough is superseded and must not be used as acceptance evidence for the amended product model.

Historical Phase 00 isolation evidence and Phase 01 identity/credential-owner separation remain valid. Historical max-two/two-slot assertions remain evidence only for their old checkpoints and are not current acceptance requirements.

Phase 03 remains blocked until Phase 02 has reconciled implementation and evidence to this amendment and receives a new Sol PASS.
