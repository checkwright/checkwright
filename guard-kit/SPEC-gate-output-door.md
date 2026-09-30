# SPEC amendment: gate-output-door

**A gate's printed remedy is adopter-facing, and 28 of them name a door an adopter's tree may not run.** gate-sdk/SPEC.md counts "the gate's own output and help text" among adopter-facing surfaces, because the remedy line is the product. Yet the compiled gates under `native/src/gates/` print `bash gate-sdk/bin/run-gates.sh <arm>` as the command to run. A starter or prose install owes no bash, and the front end needs bash 4.3, which stock macOS lacks, so on such a host the printed remedy cannot run at all. `check-door-binding` reads kit READMEs, templates, `lib/` and `bin/`, and never a compiled gate's strings, so nothing reds.

**The roster**, from `git grep -n 'run-gates' native/src/gates` read line by line on 2026-09-30, the unit-test literals and the non-output constants set aside. That leaves 28 output sites in 23 modules. Every one of them names an arm the binary serves, so every one has a binary spelling:

- **Kit-shipped gates**, each `# install: zero-config` or `on-surface`:
  - `graph.rs` — 720 (the `REGEN` constant, `--emit git-hooks --write`), 760 and 765 (`--emit graph`)
  - `doctrine_registration.rs` — 237 and 436 (`--install-doctrine`)
  - `lifecycle_registration.rs` — 44 and 98 (`--install-lifecycle`)
  - `merge_attrs.rs` — 107 (`--install-lifecycle`)
  - `enforcement_fresh.rs` — 50 (`--emit enforcement-map`)
  - `footprint_fresh.rs` — 50 (`--emit footprint`)
  - `gap_inbox_neutrality.rs` — 158 (`--emit file-gap`)
  - `md_unwrapped.rs` — 351 (`--emit md-unwrap --write`)
  - `prose_bounds.rs` — 459 (`--only check-prose-bounds`, behind an environment prefix)
  - `queue_entry_budget.rs` — 282 (`--emit entry-history`)
  - `roadmap_fresh.rs` — 160 (`--emit roadmap --write`)
  - `scratch_citation.rs` — 222 (`--emit cite-survey`)
  - `smoke_entry_guard.rs` — 111 (`--run-consumer-smoke`, inside text a kit author pastes into a smoke script)
  - `stage_evidence.rs` — 71 (a multi-line remedy)
  - `surface_ratchet.rs` — 25 and 68 (`--emit always-loaded --ceiling`)
  - `survey_record.rs` — 409 (`--emit file-survey`)
  - `task_names.rs` — 240 (`--emit queue-migrate --write`)
- **This repository's own `scripts/` gates**, which carry no `# install:` line:
  - `docs_mirror_fresh.rs` — 97
  - `install_evidence_fresh.rs` — 59
  - `product_statement_fresh.rs` — 48
  - `support_table_fresh.rs` — 28
  - `trajectory_fresh.rs` — 50
  - `value_rollup_fresh.rs` — 63

The queue entry counted 16, which is the `println!`/`format!` lines whose own text carries the literal. The other twelve sit on continuation lines or in `graph.rs`'s constant.

**The crate already owns the spelling.** `native/src/installer/init.rs`'s `command_token` spells a binary path as a command, prefixing `./` to a relative one. It is gate-sdk's `gate_native_bin_spelled` rule, which the shell guard's steer door also follows (guard-kit/SPEC.md §The shell guard). No gate calls it today.

**What stays as it is.** Assertions A and B of `check-door-binding`, and C's markdown reading, declaration token, scopes and default-deny. The fail-open arms, which keep naming the front end by construction wherever they sit. The output strings of hook members and emit arms (`native/src/hook/`, `native/src/emit/`), which are outside this unit and are filed as a gap rather than swept here.

## What changes

### (1) A gate prints its remedy through one door helper

`native/src/gates/mod.rs` gains `door_command(args)`: the `GATE_SDK_NATIVE_BIN` value spelled by `command_token`, a space, then `args` {mechanical}. **Not yet applied.** Every roster site above except `smoke_entry_guard.rs` prints its remedy through it, so an adopter reads `./scripts/checkwright-gates --emit graph` where this repository reads `./native/target/release/checkwright-gates --emit graph`. `graph.rs`'s `REGEN` constant becomes a call. `prose_bounds.rs` keeps its environment prefix in front of the door. Each re-pointed gate's registry row in `native/src/gates/mod.rs` declares `GATE_SDK_NATIVE_BIN`, since `check-gate-substrate-parity` holds what a member reads against what it declares (gate-sdk/SPEC.md §lib/gate.sh). A knob read that fails is the gate's existing exit-2 answer, not a remedy printed without a door.

The six `scripts/` gates re-point too, rather than declaring themselves contributor-facing. Their readers are contributors, and a contributor holds the binary. One door spelling across every gate leaves a declaration to mark a real exception and not a habit.

### (2) The smoke-entry advice names the arm and no door

`smoke_entry_guard.rs`'s help line tells a kit author to paste `: "${SMOKE_KIT_ROOT:?…}"` into a smoke script, and today its message reads `run via run-gates.sh --run-consumer-smoke` {design-bearing}. **Not yet applied.** That text ships inside a kit's `smoke/` script. A resolved binary path there would publish one consumer's install location in a kit file, which is the provenance seam `check-door-binding`'s assertion B exists for, and the stub spelling is the adopter door this sweep retires. So the pasted message names the arm on the binary without a path: `run via the gate binary's --run-consumer-smoke`. `check-smoke-entry-guard` recognizes the guard by its `${SMOKE_KIT_ROOT:?` prefix alone (the module's `GUARD` constant), whatever message follows, so no smoke script already carrying the old message goes red.

### (3) Assertion C reads a Rust member's output strings

`check-door-binding`'s assertion C gains a reading for a configured member whose name ends `.rs` {design-bearing}. **Not yet applied.** Such a member is read up to its test module, the first `#[cfg(test)]` line followed by `mod tests {`, which is the cut the crate's `check-reads-couples` reader already makes for a module's rule text. A test literal is no output string. Within that region:

- **A line whose first non-blank characters are `//` is never a door site.** A comment is not output, and the crate's comments cite doors in prose.
- **A declaration is `door-contributor: <reason>` inside a `//` comment**, found by the existing substring reading, with site scope alone: the line itself or the line above. The span scopes do not apply. A Rust file has no fence, and its `#[…]` attribute lines would read as the whole-file scope's first `#` heading.
- **Every other line is read by the markdown reading's predicate unchanged**, `is_door`, `token_span` and the fail-open arm exemption alike. A string literal carrying `bash gate-sdk/bin/run-gates.sh --emit x` is a door because an interpreter word precedes it, and so is `run-gates.sh --emit x` because an arm follows it.

The honest limit is the continuation line. A literal ending in the stub name, with its arm supplied through a `{}` argument, carries neither the interpreter word nor a following arm on its line. That is the shape the roster's multi-line sites take today, so delta 1 removes every present instance, and the reading catches a new one only where the door and its arm share a line.

**The act is the same under each of the sibling debt `guard-kit-value-audit`'s three verdicts on this gate**, and that entry lands first in build order. The reading rides C's corpus knob, which is empty by default, so it is inert in every tree that does not list a Rust source:

- **Kept, or kept out of the payload and the customer-OS legs**: this delta lands as written.
- **Made generic by changing C's corpus**: the Rust reading attaches to whatever corpus C then reads, keyed on the `.rs` suffix as here.

### (4) This repository lists its gate modules in C's corpus

`scripts/guard-config.knobs` gains `GUARD_KIT_DOOR_ROOTS[] = native/src/gates` beside its `README.md` and `docs` entries {mechanical}. **Not yet applied.** The directory yields its tracked members under A's prune, so a gate module added later is swept with no edit. The descriptor's `knob:GUARD_KIT_DOOR_ROOTS` couple expands the new entry into the gate's trigger the way it expands `docs` today (`--emit graph` renders that entry as `*docs`), so the regenerated hook and graph carry it. After delta 1 the corpus carries no undeclared door. Any site build judges contributor-facing takes a `// door-contributor: <reason>` declaration and is named in the landing commit.

**Inferred, cannot run before build:** after deltas 1 to 3 land, `check-door-binding` is clean over `native/src/gates` with this repository's knob set, and its third clean-line count rises by the tracked module count — the Rust reading does not exist until build lands it.

### (5) The fixture pair gains a Rust member

`guard-kit/gate-tests/check-door-binding/` gains, in `bad/`, a `.rs` member carrying an undeclared door in a string literal, and in `good/`, a `.rs` member carrying the same door behind a `// door-contributor: <reason>` declaration, a door in a `//` comment, and a door inside a trailing `#[cfg(test)]` `mod tests {` block {mechanical}. **Not yet applied.** Each fixture's knob file lists the member in `GUARD_KIT_DOOR_ROOTS`, and `expect.txt` carries the new finding and count. Without the good half, the test-module cut and the comment rule would ship with no executable statement.

## Producers and consumers

- **`door_command`.** Producer: each roster gate, at its red, on every run: the generated pre-commit hook, the battery, CI, and an adopter's installed battery, since the kit-shipped gates are installed `zero-config` or `on-surface`. Consumer: the person or session reading the red. The one knob it reads, `GATE_SDK_NATIVE_BIN`, is set in every deployed configuration, since the binary is the battery's substrate.
- **C's Rust reading.** Producer: `check-door-binding`, over `GUARD_KIT_DOOR_ROOTS` members ending `.rs`, which this repository sets in delta 4. Consumer: the committing session, through the output contract. The clean line's third count gains the Rust members and stays a report, with no floor.
- **The declaration token in a `//` comment.** Reader: C. `check-comment-tier` reads full-line crate comments too, and its directive roster in `native/src/gates/comment_tier.rs` already carries `door-contributor:`, so a declaration there is a directive and needs no new row.

No new knob or name is minted beyond `door_command`, a crate-internal helper no other surface cites.

## Existing sections updated

- `guard-kit/SPEC.md` — §check-door-binding: the corpus paragraph and the discriminator paragraph gain the Rust reading, and the red and clean paragraphs are unchanged (delta 3). §Layout and configuration's `GUARD_KIT_DOOR_ROOTS` bullet says a `.rs` entry is read as Rust source (delta 3).
- `gate-sdk/SPEC.md` — §run-gates, the paragraph "The front-end serves a harness shim and a pre-build clone rather than an adopter door": its list "every kit README, template, knob header and stage procedure" gains "and every compiled gate's printed remedy", held there by the helper and by C's Rust reading (deltas 1 and 3).
- `native/src/gates/mod.rs` — the helper and the registry rows (delta 1).
- The 23 roster modules — their output lines (deltas 1 and 2).
- `native/src/gates/door_binding.rs` — the Rust reading (delta 3).
- `scripts/guard-config.knobs` — the corpus entry (delta 4).
- The generated graph and pre-commit hook — regenerated by the commands `check-graph` prints, since the knob's expansion feeds both (delta 4).
- `guard-kit/gate-tests/check-door-binding/` — the fixture pair (delta 5).
- `docs/guard-kit/SPEC.md` and `docs/gate-sdk/SPEC.md` — the generated mirrors (all deltas).

The module roster came from the `git grep` above, and the section roster from `git grep -n 'door' gate-sdk/SPEC.md guard-kit/SPEC.md`.

## Retired spellings

<!-- retired-spelling-exempt: the stub's spelling stays lawful in the fail-open hook wiring, in contributor surfaces and in every SPEC that describes the front end, so a survivor scan would name every one of them -->
- `bash gate-sdk/bin/run-gates.sh` — retired from compiled gate output (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the helper, the Rust reading and the declaration token.
- [ ] **Every roster site has its value** — each of the 28 sites prints through `door_command` or, for `smoke_entry_guard.rs`, names the arm alone, and `git grep -n 'run-gates' native/src/gates` returns only test literals and non-output constants.
- [ ] **Merged with no information lost** — §check-door-binding reads as one gate with one corpus rule per member kind.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the retired spelling is gone from compiled gate output.
- [ ] **Gaps filed** — the hook-member and emit-arm output strings naming the stub, filed with `--emit file-gap` at authoring.
- [ ] **The entry moves** — `gate-output-contributor-door` moves to Done in the landing commit, a stage before the drain stage.
