# SPEC amendment: account-swap

**`usage-verdict` learns which account its snapshot speaks for.** A consumer may swap the harness credential out from under in-flight agents to spread burn across accounts, and the budget verdict models one account. Three readings go wrong under a swap. The login reroute fires only when the credentials file's mtime moves, so a swap that leaves that file alone keeps trusting the prior account's snapshot. The roll witness compares the snapshot against the newest history sample whatever its account. And no surface shows the accounts side by side, so rotation can push every account toward its ceiling while each one reads individually under threshold.

**The entry's four components, re-verified against the tree at authoring.**

- **(a) Detection** and **(d) the account-keyed settle** are one change to one check, the post-login reroute, so they land as one delta (delta 1). Both turn on comparing the snapshot's `account` with the live account identity. That identity is the account config's `oauthAccount.accountUuid`, the field both shipped producers already stamp as `account` (`native/src/hook/poll.rs`, `native/src/hook/statusline.rs`).
- **(b) Evidence** is mostly in the tree already. §Trend reporter segments on `account` and `tier`, so a swap reads as a segment boundary and never as a spurious drop, and the boundary is derivable from the samples, so no marker line is added. The one unpartitioned reader left is the roll witness, which reads the log's last line whatever its account (`native/src/hook/verdict.rs`, `previous_boundary`). Delta 2 partitions it.
- **(c) Safety** is an aggregate view rather than a block (delta 3). A combined ceiling has no single threshold to enforce: the accounts' windows are independent and may differ in tier. So the shipped remedy makes the combined position visible where the weekly headroom is already read, and leaves the pause decision per account.

**Refused: an aggregate PAUSE.** Summing percentages across accounts sums different denominators wherever tiers differ, and a rotating consumer's intent is to spend across accounts. A block on the sum would refuse the use the swap exists for.

**Refused: a second identity source read from the token.** The poller's usage request carries the credentials file's token, and a swap that changes neither file is invisible to every reader here. Reading an environment-supplied token would name a harness variable no contract states, so that case stays an honest limit.

## What changes

### (1) The login reroute is keyed on the account where both identities are readable

§usage-verdict's failure mode 3 and its two sub-paragraphs gain an account-keyed arm, and `DELEGATION_KIT_LOGIN_SETTLE` joins the table {design-bearing} {user-facing: the entry's deliverable (a) and (d), selected by operator direction 2026-10-03}. **Not yet applied.**

The verdict reads the **live identity**, the account config's `oauthAccount.accountUuid`, through the reader's existing empty-means-derive fill of `DELEGATION_KIT_ACCOUNT_CONFIG`. It compares that identity with the snapshot's `account`. Where both are present:

- **A different account** routes a would-be OK to STALE with the reason `snapshot predates an account switch`. The demand-driven refresh ignores `DELEGATION_KIT_REFRESH_MIN_AGE` on that reading, so a configured poller re-polls at once. The reroute stays asymmetric: an at-or-over reading still pauses.
- **The same account** replaces the blanket window with a settle floor: the reroute fires only while the snapshot's `updated_at` is less than `DELEGATION_KIT_LOGIN_SETTLE` seconds past the credentials file's mtime. A routine token rotation therefore blinds the verdict for the settle floor, not for the whole window.

Where either identity is absent or unreadable, the blanket `DELEGATION_KIT_LOGIN_WINDOW` reroute runs as it does today. The roll witnesses disarm the reroute on both arms, as now.

Failure mode 3's replacement text:

> 3. **Post-login lag** — a fresh login starts a new window but the server-fed percentage lags it, and the file-write age check cannot see that. The verdict compares the snapshot's `account` with the **live identity**, the account config's `oauthAccount.accountUuid`. A different account routes a would-be OK to STALE and forces the demand-driven refresh past its short-circuit. The same account trusts a snapshot taken at least `DELEGATION_KIT_LOGIN_SETTLE` seconds after the credentials file's mtime, the auth event. Where either identity is missing, the verdict falls back to the blanket reroute: within `DELEGATION_KIT_LOGIN_WINDOW` of the mtime, a would-be OK routes to STALE. Letting the lag print OK would emit a fresh-looking chimera, the new account's id beside the dead login's percentage and `resets_at`.

The mtime-proxy paragraph's closing **Honest limit** sentence is replaced:

> **Honest limit:** where the snapshot carries no `account` or the account config is unreadable, a rotation with no roll still blinds the blanket window, because no witness contradicts it. A swap that changes neither the credentials file nor the account config is invisible to every arm.

The check order's `login-STALE` step reads the identity first, so the order becomes "parse → RESET-OK → age-STALE → pause axes → account-STALE or login-STALE → OK". The **Eleven declared knobs** paragraph becomes twelve, gaining `_LOGIN_SETTLE`. Its clause "so `_ACCOUNT_CONFIG` is declared although the verdict never opens that file" is replaced by "and the verdict opens `_ACCOUNT_CONFIG` for the live identity". §Layout and configuration gains, after `DELEGATION_KIT_LOGIN_WINDOW`:

> - `DELEGATION_KIT_LOGIN_SETTLE` — the account-keyed settle floor (§usage-verdict); default `90` (seconds), validated a non-negative integer by the table validator. The server-fed percentage lags a login by about a minute, so the default leaves that lag a margin. `DELEGATION_KIT_LOGIN_WINDOW` stays the fallback for a snapshot or host whose identity cannot be read.

§Layout and configuration's `agent-budget-guard` paragraph, "its declared knob slice is `--usage-verdict`'s eleven", becomes twelve.

### (2) The roll witness reads the snapshot's own account

§usage-verdict's roll-witness paragraph: the first witness reads the newest `DELEGATION_KIT_USAGE_HISTORY` sample **whose `account` equals the snapshot's**, and the newest sample of all only where the snapshot carries no `account` {mechanical} {user-facing: the entry's deliverable (b), selected by operator direction 2026-10-03}. **Not yet applied.** The witness bullet's replacement text:

> - the snapshot's `five_hour_resets_at` differs from the `resets_at` of the newest sample for the snapshot's account (the boundary moved); a snapshot carrying no `account` reads the newest sample of all;

The fall-open list gains "no sample for the snapshot's account", so a first reading after a swap falls open to the unrefuted reroute rather than comparing against another account's boundary.

### (3) The trend reporter closes with a cross-account view

§Trend reporter's step 3 gains a closing block {design-bearing} {user-facing: the entry's deliverable (c), selected by operator direction 2026-10-03}. **Not yet applied.** Where the log carries two or more distinct `account` values:

> 4. **Combine** the accounts after the per-segment report: one line per account, its newest weekly segment's last smoothed pct and its headroom against `DELEGATION_KIT_PAUSE_PCT_7D`, then `accounts: <n>, at or over the weekly ceiling: <k>`. A rotating operator reads the combined position there. The view is advisory and sums nothing: accounts may differ in tier, so a sum would add different denominators, and the pause decision stays per account. An account with no weekly segment prints `-` for both values.

A log with one account, or none, prints no block. The declared knobs are unchanged, since the ceiling is already `DELEGATION_KIT_PAUSE_PCT_7D`.

### (4) Tests

The verdict's case table and the trend reporter's module tests gain the rows deltas 1 to 3 need {mechanical}. **Not yet applied.**

- **Delta 1.** `delegation-kit/usage-tests/cases.tsv` gains two columns, the snapshot's `account` and the live identity the case writes into a fixture account config, `-` omitting each. Its rows then gain:
  - a mismatched account inside and outside the window, routing OK to STALE;
  - a mismatched account at or over threshold, still PAUSE;
  - a matched account past the settle floor inside the old window, reading OK;
  - a matched account within the settle floor, reading STALE;
  - a snapshot without `account`, keeping the blanket window.
- **Delta 1's refresh.** A crate test stubs `DELEGATION_KIT_REFRESH_CMD` and asserts it runs on a mismatch inside `DELEGATION_KIT_REFRESH_MIN_AGE`.
- **Delta 2.** The roll-witness unit test gains a history whose last line is another account's.
- **Delta 3.** `delegation-kit/usage-tests/trend-history.log` gains a second account. The trend assertions gain the block, and its absence for a one-account log.

§Testing names the new rows.

## Producers and consumers

- **The live identity.** Producer: the harness, which writes the account config. The verdict reads it through the existing fill, the third reader of `oauthAccount.accountUuid` beside the two snapshot producers. Its one consumer is the comparison in delta 1, at the verdict.
- **The account-switch STALE.** Producer: the verdict, on a mismatch. Consumers: `agent-budget-guard`, which relays it at exit 2 as advice; the session brief and any session reading `--usage-verdict`; and the history log, which samples it as `verdict=STALE`. It carries no account id, so no identity reaches a session's context.
- **`DELEGATION_KIT_LOGIN_SETTLE`.** Producer: delegation-kit's table, default `90`; this repo sets no override. Consumers: the verdict, plus the roster-holding readers of a knob name: the table and its validator in `native/src/knobs/delegation_kit.rs`, `--emit knob-roster`, `check-knob-citation`, `check-knob-default-coupling` (the `90` in §Layout and configuration) and the verdict's declared-knob list (`native/src/hook/verdict.rs`'s `KNOBS`), which `check-reads-couples` and `check-gate-substrate-parity` read.
- **The forced refresh.** Producer: delta 1's mismatch arm. Consumer: `DELEGATION_KIT_REFRESH_CMD`'s argv, which this repo points at the poll producer. A consumer with no refresh command gets the STALE and no re-poll.
- **The partitioned roll witness.** Consumer: the reroute, at the verdict, as now.
- **The combine block.** Producer: `--emit usage-trend`. Consumers: the operator planning a rotation, and a lead reading the weekly headroom before sizing a batch. Each field has that reader: the per-account pct and headroom for the one account, and the count at or over the ceiling for the rotation as a whole.
- **Enabling config.** This repo sets `DELEGATION_KIT_USAGE_HISTORY` and `DELEGATION_KIT_REFRESH_CMD` (`scripts/delegation-config.knobs`), so deltas 1 and 2 run on every dispatch here. Delta 3's block needs a second account in the log, which only a rotating operator produces. The fixture log is its tested producer, and the trend reporter is demand-gated on the entry's own terms.
- **Corpus narrowing (point 5).** Delta 2 narrows the witness's corpus to one account's samples. Its reader, the reroute, fails open on an empty corpus to the unrefuted reroute, which is a STALE, the non-blocking outcome. It asserts no count and holds no floor.

## Existing sections updated

- `delegation-kit/SPEC.md` §usage-verdict: failure mode 3, the mtime-proxy honest limit, the check order and the declared-knob paragraph (delta 1); the roll witnesses (delta 2).
- `delegation-kit/SPEC.md` §Layout and configuration: the new knob bullet and the `agent-budget-guard` paragraph's count (delta 1).
- `delegation-kit/SPEC.md` §Trend reporter (delta 3) and §Testing (delta 4).
- `native/src/hook/verdict.rs` (deltas 1 and 2), `native/src/knobs/delegation_kit.rs` (delta 1), `native/src/emit/usage_trend.rs` (delta 3).
- `native/src/usage_tests.rs`, `delegation-kit/usage-tests/cases.tsv` and `delegation-kit/usage-tests/trend-history.log` (delta 4).
- `docs/delegation-kit/SPEC.md`, the generated mirror (all deltas).

The roster came from `git grep -n 'LOGIN_WINDOW\|ACCOUNT_CONFIG' -- ':!docs/' ':!TASK-QUEUE.md'` for the reroute's knob readers, which names no line in `delegation-kit/templates/delegation-config.knobs`, `git grep -n 'accountUuid'` for the identity's readers, and `git grep -n 'previous_boundary'` for the witness.

## Retired spellings

- None — the amendment adds a knob and an arm and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the live identity, the account-switch STALE, the knob, the forced refresh and the combine block.
- [ ] **Instruction surfaces: instruction only** — no template takes the grounds.
- [ ] **Merged with no information lost** — §usage-verdict reads as one document, its asymmetric-reroute ground intact for both arms.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **Done move** — `background-credential-swap-support` moves to Done in the landing commit, a stage before the drain stage, with `--queue done`.
