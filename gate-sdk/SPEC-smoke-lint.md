# SPEC amendment: smoke-lint

**A kit's smoke scripts join `check-shellcheck`'s derived set, so a smoke script bash cannot parse reds at commit.** Today no commit-time gate parses `smoke/install.sh` or `smoke/violation.sh`: `check-smoke-entry-guard` reads the guard by prefix and never parses, and `check-shellcheck` lints each kit root's `lib/`, `bin/`, `checks/` and `templates/` but not `smoke/`. An apostrophe in the entry-guard hint once left every kit's smoke script unparseable, and only `--run-consumer-smoke`, a validate suite, caught it. ShellCheck parses before it lints, so a parse error is its own finding at every severity: widening the lint set is the parse check, with no new gate and no new program.

**Measured at authoring** (ShellCheck 0.11.0 on this host):

- The attested defect class, an apostrophe inside `"${SMOKE_KIT_ROOT:?…}"` or the unquoted `${SMOKE_KIT_ROOT:?…}`, fails `shellcheck -S warning` with SC1073 and SC1072 (errors) and fails `bash -n` with exit 2, on a scratch script of each form.
- `shellcheck -S warning */smoke/*.sh` over the twenty kit-root smoke scripts (fixture copies under `gate-tests/` are no kit root's) reports four warnings and no error: SC2034 at `drift-kit/smoke/install.sh:413` and `:536` (an unused capture) and SC2120 at `lifecycle-kit/smoke/install.sh:320` and `:321` (a function reading arguments no caller passes).
- The derived set is `KIT_SUBDIRS` in `native/src/gates/shellcheck.rs:12`, and the member declares no walk root (`native/src/gates/mod.rs:2192-2202`): it lists each directory's direct `*.sh`, so no `check-reads-couples` declaration moves.
- `check-shellcheck.gate`'s `couples=` already carries `kit:*.sh`, which the `case` matcher reads across `/`, so a staged smoke script already fires the gate.
- The payload withholds `smoke/` (`GATE_SDK_PAYLOAD_WITHHOLD`), so a vendored tree has no smoke directory for the widened derivation to reach.

## What changes

### (1) The derived set gains each kit root's `smoke/`

`native/src/gates/shellcheck.rs`'s `KIT_SUBDIRS` gains `smoke`, after `templates` {mechanical}, and its comment's four-directory sentence names five. A kit root with no `smoke/` contributes nothing, as a kit with no `templates/` does today, so the empty-target-set refusal is unchanged.

The four warnings above are fixed at their sites in the same commit, never by a disable directive: each is a true finding, an unused capture or an argument list no caller fills. Where the pinned CI release of ShellCheck (§check-shellcheck, `.github/workflows/gates.yml`) reports a different set over the smoke scripts than the local run, the pinned release's set is the one fixed, per §check-shellcheck's verdict-reference rule.

### (2) §check-shellcheck, §Self-lint and §check-smoke-entry-guard state the widened set

gate-sdk/SPEC.md {mechanical}. **Not yet applied.**

In §check-shellcheck, the invariant's "each vendored kit's `lib/`, `bin/`, `checks/`, and `templates/`" becomes "each vendored kit's `lib/`, `bin/`, `checks/`, `templates/` and `smoke/`", and the paragraph after the invariant gains:

> **The parse is ShellCheck's, so the gate spawns no bash.** ShellCheck parses a script before linting it and reports a parse failure as an error, which `-S warning` keeps, so a smoke script bash cannot parse reds here at commit. A host without bash runs the gate unchanged, and a host without ShellCheck is refused as above. **Honest limit:** ShellCheck's parser is not bash's, so a construct only one of them rejects is caught by the other tier, the consumer smoke running each script under bash.

In §Self-lint, "(the consumer's gates and the kit's own `lib/`, `bin/`, `checks/`, `templates/`)" becomes "(the consumer's gates and the kit's own `lib/`, `bin/`, `checks/`, `templates/`, `smoke/`)".

In §check-smoke-entry-guard, the honest-limit paragraph gains: "Whether the script parses is §check-shellcheck's, which lints the smoke directory."

### (3) The site mirror follows

`docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 2 {mechanical}.

## Producers and consumers

- **The widened lint set.** Producer: `target_dirs` in `native/src/gates/shellcheck.rs`, at every run of `check-shellcheck` with no positional scope. Enabling configuration: a tree whose kit roots carry `smoke/`, which is every kit-authoring tree and this one, registering the gate (`scripts/gates.list:6`); no vendored tree has the directory. Consumer: the ShellCheck spawn, whose findings the committing session reads through the output contract; the generated pre-commit hook fires the gate on a staged smoke script through the existing `kit:*.sh` couple.
- **Readers whose verdict moves.** `check-shellcheck` reds on the four current warnings until delta 1's fixes land in the same commit. Its red condition is monotone in the target set (§check-shellcheck, the narrowing paragraph), and this change widens it, so no reader reds on finding none. `check-install-disposition` assertion B and `check-smoke-entry-guard` read the same scripts for other properties and are unchanged. The gate stays `on-surface`, so no adopter's install registration moves.
- **No new state, knob, event or interface.**

## Existing sections updated

Roster produced by `grep -rn 'lib/`, `bin/`, `checks/`' --include='*.md'` over the tracked tree, reading `native/src/gates/shellcheck.rs`, and the ShellCheck run above.

- `native/src/gates/shellcheck.rs` — `KIT_SUBDIRS` and its comment (delta 1).
- `drift-kit/smoke/install.sh` — the two SC2034 sites (delta 1).
- `lifecycle-kit/smoke/install.sh` — the two SC2120 sites (delta 1).
- `gate-sdk/SPEC.md` — §check-shellcheck, §Self-lint, §check-smoke-entry-guard (delta 2).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 3).

## Retired spellings

- None — the change adds a directory to a derived set; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
