# 2S Proof Contract Amendment — 2026-09-01

**Authority:** Sidik product/runtime clarification, authored by Sol
**Baseline HEAD:** `5de898eca809dcc5011fae7587340e8dbbdc3a3a`
**Implementation checkpoint under current Phase 02 review:** `537b2bbad951ccbb43f04ba9067b55b304f4d232`
**Applies to:** Phase 02 onward, `TEST-MATRIX.md`, required runtime artifacts, and Definition-of-Done proof modality

## Decision

The product requirements are unchanged:

- capability remains collection-driven/N-capable;
- runtime product policy remains **maximum four retained accounts**;
- a fifth manual add must be blocked before OAuth starts;
- an unknown fifth normal-Codex identity at full retained capacity must be recognized as current-only without silent eviction;
- credential ownership, active role, polling, menu routing, and rendering remain collection-driven and identity-scoped.

What changes is the **required proof modality**.

Sidik has two real Codex accounts available for owner-observed runtime verification. The plan must not require creating extra real accounts solely to satisfy count/capacity test fixtures when the underlying behavior can be proven deterministically without exercising additional credential owners.

Therefore:

1. **Credential-sensitive behavior requires real-account proof with the available real accounts.**
2. **Pure collection/count/capacity behavior may use deterministic non-secret fixtures.**
3. **Deterministic fixtures must never fabricate, copy, or stand in for OAuth credentials, refresh tokens, or credential-owner isolation proof.**
4. A test may combine real-account proof for auth semantics with deterministic fixture proof for higher-cardinality collection behavior.

This amendment supersedes any older Phase 02–05 wording that requires three, four, or five distinct real accounts when the scenario is only proving collection cardinality, capacity policy, menu gating, state routing, polling orchestration shape, or renderer layout.

## Proof classes

### Class R — real-account runtime proof

Use owner-approved real accounts for behavior whose correctness depends on real Codex authentication, credential ownership, or working-Codex lifecycle interaction.

Examples:

- first-launch auto-discovery;
- working Codex A → B active-role movement;
- manual Add against a real identity;
- duplicate identity reconciliation after real login;
- isolated monitor-owner provisioning;
- re-authentication;
- restart restoring monitor ownership;
- remove inactive/current account without logging normal Codex out;
- real usage read through the approved credential source;
- refresh behavior and working-Codex safety boundaries.

Two real accounts are sufficient for this class unless a later product change introduces a behavior whose auth semantics genuinely depend on more than two simultaneously authenticated owners.

### Class F — deterministic fixture proof

Use synthetic **identity/state/data fixtures only**, with no real or fake secrets, to prove behavior that depends on collection state rather than OAuth authenticity.

Approved fixture examples:

- retained identities A/B/C/D;
- current-only identity E;
- deterministic `UsageData` with distinct 5h/weekly values and reset times;
- deterministic connection states such as Connected/ReauthRequired/Unavailable;
- stable fake account IDs used only inside tests;
- deterministic menu command routing and renderer input.

Class F may prove:

- four retained identities coexist under policy;
- the fifth manual Add is unavailable before login dispatch;
- full-capacity unknown E remains current-only;
- no retained identity is silently evicted;
- freeing capacity permits later retention/reconciliation;
- collection iteration works for 1/2/4 accounts without fixed branches;
- failure isolation across A/B/C/D state;
- four-account menu generation and action routing;
- three/four-account taskbar layout;
- full-capacity current-only overflow rendering;
- same-initial identity handling;
- account-scoped alert-key behavior at higher cardinality.

### Class S — source/automated state-machine proof

Focused unit/integration tests may prove deterministic lifecycle transitions that are not reliably observable through the transitional UI, provided the production path is exercised at the correct seam.

Examples:

- transaction commit/rollback state machine;
- identity mismatch disposition;
- missing-owner re-auth preflight;
- initial one-shot usage-read completion/error recording when no user-facing completion marker exists;
- legacy owner-handle compatibility;
- active-role persistence prohibition.

Class S is not a substitute for Class R where real credential ownership is the thing being claimed.

## Fixture safety rules

A deterministic fixture is acceptable only when all of the following are true:

- it contains no access token, refresh token, OAuth code, password, or copied working-Codex credential;
- it does not write a fake credential into a real monitor keyring namespace;
- it does not claim real OAuth/login/refresh success;
- it enters through an explicit domain/test seam such as account identity, registry state, usage data, connection state, polling adapter, menu model, or renderer input;
- it preserves stable-identity semantics and does not rely on list position as identity;
- results are labeled as fixture/automated evidence in `EVIDENCE.md` and are never described as owner-observed real-account runtime proof.

If a scenario cannot be isolated from credential behavior without faking auth, it must remain Class R and cannot be waived by this amendment.

## Phase 02 proof contract

Phase 02 may PASS with the following combination:

### Required Class R

Owner-observed runtime proof with the available real accounts must cover:

- auto-discovery of A;
- A → B active-role switch with A retained and no duplicate;
- manual Add/reconcile of a known real identity without changing normal Codex active auth;
- restart with retained metadata/owner state and runtime active-role recomputation;
- re-authentication scoped to the selected real account;
- remove inactive account;
- remove current account while normal Codex remains logged in/current-only until identity changes;
- direct per-account Re-auth/Remove menu routing;
- no credential/token/email/account-ID leakage in captured evidence.

### Required Class F/S

Deterministic proof must cover:

- four retained identities supported by policy without fixed account branches;
- fifth manual Add disabled/rejected before OAuth/login dispatch;
- unknown E at full retained capacity becomes current-only;
- A/B/C/D remain retained with no silent eviction or owner theft;
- capacity release permits later normal retention/reconciliation;
- bounded initial usage-read result/error is consumed into the selected account state;
- capacity/overflow behavior remains identity-based and collection-driven.

No third, fourth, or fifth real Codex account is required for Phase 02 closure.

## Phase 03 proof contract

Phase 03 must still prove the real multi-owner polling path with the available real accounts.

Required Class R:

- at least two real accounts can produce independently attributed usage in one process;
- refresh/auth failure of one real monitor owner does not corrupt the other;
- active-role switching does not move quota ownership;
- zero-inference monitor polling/refresh behavior remains true.

Required Class F/S:

- orchestration over 1/2/4 retained accounts;
- mixed A/B/C/D success/failure isolation;
- distinct deterministic reset times/window availability;
- full-capacity current-only E source-selection behavior;
- account-scoped alerts/dedup across higher cardinality.

Four simultaneously authenticated real accounts are not required solely to prove iteration cardinality.

## Phase 04 proof contract

Sidik runtime acceptance remains required for the actual taskbar UI with the available real accounts.

Required Class R:

- one-account and two-account representative Bar/Circle behavior;
- active blue-ring movement on a real A ↔ B switch;
- whole-account tooltip behavior and privacy;
- real direct context-menu actions and normal taskbar interaction.

Required Class F:

- three/four retained-account renderer layout;
- four-account Bar/Circle representative screenshots;
- full-capacity A/B/C/D + current-only E overflow layout;
- same-initial accounts;
- one healthy + one unavailable/re-auth state where a deterministic state fixture is sufficient;
- DPI/layout checks for higher-cardinality states.

Fixture-rendered screenshots are acceptable evidence when clearly labeled as deterministic fixture output. They do not claim additional real credential owners.

## Phase 05 / Definition of Done

The final test matrix may be satisfied by the proof class appropriate to each behavior.

A row is PASS when its required result is proven by an authorized proof mode under this amendment. `N/A` is not needed merely because only two real accounts exist; higher-cardinality collection behaviors should be proven with Class F instead.

Final closure still requires:

- real-account evidence for credential-sensitive behavior;
- deterministic higher-cardinality evidence for max-four/full-capacity behavior;
- current build/lint/test/diff proof;
- current evidence at the final implementation checkpoint;
- Sidik UI/runtime acceptance for representative real-account behavior;
- Sol final audit with no open blocker/high-severity finding.

## TEST-MATRIX proof-mode mapping

The following rows explicitly accept Class F/S for the high-cardinality or deterministic portion of the scenario:

- `AUTH-08`, `AUTH-10`, `AUTH-11`, `AUTH-12`, `AUTH-13`, `AUTH-14`;
- `DATA-02`, `DATA-03`, `DATA-04`, `DATA-05` where deterministic data/state is the subject;
- `POLL-01` four-account cardinality portion, `POLL-02` higher-cardinality failure mixture, `POLL-05`;
- `ALERT-01`, `ALERT-02` higher-cardinality/dedup logic;
- `LIFE-04`, `LIFE-09`, `LIFE-10`;
- `UI-08`, `UI-11`, `UI-12`, `UI-13`, `UI-14`, and higher-cardinality portions of `UI-15`;
- `ARCH-01`, `ARCH-02`, `ARCH-03`, `MIG-01`, `MIG-02`, `SEC-03`.

Rows whose central claim is real credential ownership/auth interaction remain Class R unless historical approved Phase 00 evidence already proves the exact mechanism and the current phase only needs regression/source confirmation.

## Evidence disposition for current Phase 02

The owner-observed two-account walkthrough already recorded at HEAD `5de898eca809dcc5011fae7587340e8dbbdc3a3a` remains current Class R evidence for implementation checkpoint `537b2bbad951ccbb43f04ba9067b55b304f4d232`.

Its prior `NOT PROVEN` entries for max-four/fifth-account behavior are not failures. Under this amendment they become **required deterministic fixture proof** before Phase 02 PASS.

The initial one-shot usage-read item may be closed through Class S focused proof because the transitional Phase 02 UI has no independent completion marker.

## Next authorized action

Luna may perform **Phase 02 proof closure only**:

1. rehydrate this amendment and current Phase 02 authority;
2. add or strengthen deterministic fixtures/tests only where needed for the Class F/S requirements above;
3. do not add synthetic OAuth/token credentials;
4. run the full current verification suite;
5. update `EVIDENCE.md` with exact test names/results and classify evidence as Class R, F, or S;
6. preserve implementation checkpoint semantics: if production source is unchanged, keep `537b2bbad951ccbb43f04ba9067b55b304f4d232`; if source/test code changes, record the new implementation checkpoint;
7. stop at `Phase 02 — READY FOR SOL FINAL GATE`.

Phase 03 remains blocked until Sol reviews the fixture proof and issues Phase 02 PASS.
