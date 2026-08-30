# Phase 04 — Taskbar UI

**Status:** `blocked-by-phase-03`
**Goal:** render two monitored accounts cleanly in the existing native taskbar widget while preserving Bar/Circle styles and compact hierarchy.

## Locked visual direction

Circle style target:

```text
[S]  5h ○ 32% · 2h
     7d ○ 74% · 6d

[N]  5h ○ 81% · 4h
     7d ○ 43% · 3d
```

Bar style keeps the currently accepted compact visual language: dark/black taskbar-local background, thin continuous bars, white/soft-white progress, subtle grey tracks, percentage primary, reset secondary, and the account initial chip at the left.

## Identity rules

- Display one uppercase account initial.
- Initial is presentation only, never account identity/registry key.
- Two accounts may share the same initial without collision.
- Initial is vertically centered against that account's two quota rows.
- If identity cannot be displayed, layout remains valid without a fake initial.

## Layout requirements

- Keep the widget taskbar-height compatible.
- Do not introduce a large outer card.
- Preserve drag handle and multi-monitor placement.
- Two-account layout may widen horizontally; avoid unnecessary height growth.
- One-account layout must remain as compact as the current accepted version.
- Missing quota windows render as unavailable (`--` or equivalent), never as zero.
- Re-auth/error state must be understandable without replacing healthy account data.
- Existing native Segoe UI hierarchy remains.

## Style behavior

`Widget Style > Bar | Circle` remains persisted and applies consistently to all monitored accounts. Do not create per-account style settings in this iteration.

## Tasks

- [ ] Refactor renderer input from single Codex usage fields to account rows/sections.
- [ ] Render one account with no visual regression.
- [ ] Render two accounts in Bar style.
- [ ] Render two accounts in Circle style.
- [ ] Handle same-initial accounts.
- [ ] Handle one account unavailable/re-auth-required.
- [ ] Recalculate widget width for 100%, 125%, 150%, and 200% DPI.
- [ ] Preserve drag/taskbar embedding behavior.
- [ ] Keep context menu `Widget Style` behavior and persistence.
- [ ] Capture screenshots for required runtime states.

## Acceptance criteria

- Single-account Bar and Circle remain visually acceptable to Sidik.
- Two-account Circle matches the locked hierarchy above.
- Two-account Bar is compact and readable.
- No clipping at supported DPI cases exercised by tests/manual proof.
- Healthy account remains readable when the other is unavailable.
- Widget positioning/multi-monitor behavior does not regress.

## Hard stops

- Do not add full usernames or emails to the taskbar widget.
- Do not solve width pressure by hiding reset time or quota window labels without product approval.
- Do not introduce switching controls into account chips.

## Evidence

Add screenshots or local screenshot paths plus DPI/layout verification notes to `../EVIDENCE.md`. Sidik runtime acceptance is required before this phase is marked complete.
