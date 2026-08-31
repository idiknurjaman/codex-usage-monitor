# Phase 04 — Taskbar UI

**Status:** `blocked-by-phase-03`
**Goal:** render the account collection cleanly in the existing native taskbar widget, distinguish the runtime current Codex account without clutter, and provide compact account detail through whole-block hover.

## Preconditions

Phase 03 must have a Sol PASS. Renderer input must already be stable-account collection state; Phase 04 must not invent account ownership or polling behavior.

## Locked visual direction

Each account keeps the accepted compact two-row quota hierarchy in Bar/Circle style.

Example account block:

```text
[L]  5h     ─────── 52% · 2h
     7d     ─────── 93% · 6d
```

The account currently active in normal Codex receives only a subtle **blue outline/ring** on the identity chip. Inactive monitored accounts use the neutral identity outline.

Do not add a permanent `ACTIVE` word inside the compact taskbar widget.

## Identity rules

- Display one uppercase initial in the compact widget; presentation only, never identity key.
- Same-initial accounts remain valid.
- Active outline is runtime-derived by stable identity matching and is never persisted.
- Display name is preferred in menu/tooltip; initial is fallback when name is unavailable.
- If identity presentation is unavailable, quota layout remains valid without a fake identity.

## Whole-account tooltip

Hover target is the **entire account block**, not only the identity badge.

Hovering the initial, either quota row, bar/circle, percentage, or reset text opens the same compact informational tooltip.

Required information hierarchy:

```text
<Display name>                              ACTIVE   (current only)
<Current Codex account | Monitored account>

Usage remaining
5h       <remaining %>       <exact local reset time>
Weekly   <remaining %>       <exact local reset date/time>

<Connected | Re-auth required | Unavailable>
```

Requirements:

- no raw email, stable account ID, or credential material by default;
- exact reset information may be more detailed than the compact taskbar countdown;
- tooltip is informational; account actions remain in the context menu;
- use a small hover delay/stability behavior appropriate to a native taskbar utility so moving within the same account block does not flicker the tooltip;
- tooltip remains account-scoped when multiple account blocks are adjacent.

## Accounts context-menu UI

The final menu structure is direct-account routing, not nested `Manage accounts >`.

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

Each account submenu:

```text
Re-authenticate
Remove from monitor
```

Requirements:

- account name when available, initial fallback;
- `· Active` may mark the current runtime account in the menu;
- no fixed per-slot command IDs/branches; action dispatch maps dynamically to account identity/owner;
- `Add monitor account...` is disabled at four retained accounts or during conflicting lifecycle operations;
- no separate `Manage accounts >` submenu.

## Collection and capacity layout

- Renderer must consume a collection, not `account_a/account_b` fields.
- Product policy supports up to four retained accounts.
- One-account layout stays as compact as the accepted baseline.
- Multi-account layout may widen horizontally, but must remain taskbar-height compatible and avoid unnecessary vertical growth.
- Three/four-account layouts require explicit runtime acceptance; do not hide account identity, reset data, or quota windows merely to fit.
- If four retained accounts exist and a distinct current-only active identity is present, the current identity must remain understandable and no retained identity may be silently evicted. If a special overflow layout is necessary, obtain Sidik acceptance rather than dropping data silently.
- Missing quota windows render as unavailable, never zero.
- One unavailable/re-auth account must not obscure healthy account data.

## Style behavior

`Widget Style > Bar | Circle` remains persisted and applies consistently to the full account collection. Do not create per-account style settings.

Bar direction remains: dark/black taskbar-local background, thin continuous bars, soft-white fill, subtle grey track, percentage primary, reset secondary.

Circle direction remains compact and monotonic with remaining quota.

## Tasks

- [ ] Refactor renderer input to collection-driven account sections.
- [ ] Render one account without regression.
- [ ] Render two accounts in Bar and Circle.
- [ ] Render three/four retained accounts without account-position hard-coding.
- [ ] Add runtime active blue identity outline.
- [ ] Add whole-account compact tooltip with name/role/usage/reset/status.
- [ ] Implement direct account context-menu hierarchy and remove nested Manage accounts layer.
- [ ] Disable Add monitor account at retained capacity.
- [ ] Handle same-initial accounts.
- [ ] Handle unavailable/re-auth states independently.
- [ ] Handle current-only active overflow state at retained capacity without silent eviction.
- [ ] Recalculate taskbar width/DPI behavior at 100%, 125%, 150%, and 200%.
- [ ] Preserve drag/taskbar embedding and multi-monitor behavior.
- [ ] Capture required screenshots/runtime artifacts.

## Acceptance criteria

- Single-account Bar/Circle remains visually acceptable to Sidik.
- Active account is recognizable by the blue identity outline without adding clutter.
- Tooltip appears from the whole account block and clearly explains name, active/monitor role, both quota values, exact resets, and connection status.
- Direct account submenu structure matches the locked information architecture.
- Two-, three-, and four-account states remain readable with no identity/action routing collision.
- Healthy accounts remain readable when another is unavailable.
- Missing data never appears as `0% remaining`.
- No material clipping in exercised DPI cases.
- Widget positioning/multi-monitor behavior does not regress.

## Hard stops

- Do not put full usernames/emails into the compact taskbar widget.
- Do not expose email/account IDs in hover by default.
- Do not solve width pressure by silently hiding reset time or quota labels without product approval.
- Do not introduce switching controls into account chips/menu.
- Do not implement renderer/menu logic as four copy-pasted account branches.

## Evidence

Add current screenshot/runtime evidence for:

- one-account Bar and Circle;
- two-account Bar and Circle;
- four-retained-account representative layout;
- active blue-ring state;
- whole-account tooltip;
- direct account submenu structure;
- one healthy + one unavailable/re-auth state;
- DPI/layout verification.

Sidik runtime acceptance is required before Phase 04 passes.
