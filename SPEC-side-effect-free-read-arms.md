# SPEC amendment: side-effect-free-read-arms

**A read an agent spells with a utility that can also write or execute has a write path the guard judges per call.** `sed` carries `w` and `e` commands, `awk` carries `system()` and `print >`, and `find` carries `-exec` and `-delete`. The guard already steers some of these reads to a tool with no such path: `sed -i` goes to `--rewrite`, a `sed` or `awk` line range on a file goes to the Read tool, and an `awk` heading range on a markdown file goes to `--emit md-section` (guard-kit rule `sed_file`). Two read shapes still reach the out-of-band decision although a replacement exists that the guard already grants. An isolated child's read set trusts the roster's declarations, which is guard-kit rule `worktree_confinement`'s stated honest limit: "Only a tool whose effect is its command line would close that."

**The shape is an operator direction, relayed by the lead on 2026-09-30 (answered through AskUserQuestion, not a /consult ruling):**

- **Block only a form that can write or execute, and only where a side-effect-free replacement exists, steering each to it.** A form with no replacement keeps today's disposition. Blocking `awk` or `sed` wholesale would take away reads that have no replacement: a column sum, a computed report.
- **Declare a read-only arm set in the crate, hold it with a crate test, and have rule `worktree_confinement` admit it.** Build no new general-purpose read arm.
- **Add no allowlist entry.** The recommended `Bash(@GATE_SDK_NATIVE_BIN@ *)` grant already reaches every arm (guard-kit/SPEC.md §The recommended allowlist), and what an isolated child gains is the confinement rule's admission, which no settings entry can express.

**The entry's seed was corrected at spec.** `FENCE_SAFE_ARMS` (`native/src/emit/mod.rs`) is not a read-only set. gate-sdk/SPEC.md §The non-gate arm admits an arm there that "writes nowhere but its working tree and stdout". So `--run`, `--run-gate-tests`, every `--emit-` arm (the `--write` modes included) and every gate are members. The read-only set is a new, narrower declaration beside it, and a subset of it.

**Measured at authoring, on this checkout.**

- **Demand.** `.workflow/prompt-friction.log` held 313 fall-through lines, mostly compounds. A segment was led by `awk` 16 times, by `sed` 3, and by `find`, `tee` or `xargs` never. The `awk` programs were counts, `NR` line ranges and markdown heading reads, and one piped `sed -n '40,200p'`.
- **The shell guard's verdicts today** (payloads fed to `--hook shell-guard` with `GUARD_KIT_LOG` pointed at scratch):
  - `grep -n door guard-kit/SPEC.md | sed -n '5,10p'` fell through, and its replacement `grep -n door guard-kit/SPEC.md | head -n 10 | tail -n +5` was auto-allowed.
  - `find guard-kit -name '*.md' -exec grep -l door '{}' +` and its `\;` twin fell through. Their replacement `find guard-kit -name '*.md' -print0 | xargs -0 grep -l door` was auto-allowed.
  - The file-operand `sed -n '5,10p' <file>` and `awk 'NR>=5 && NR<=10' <file>` are already blocked to the Read tool.
  - `find … -delete` falls through and has no read replacement.
- **Arm modules.** A scan of each candidate module's rule text (cut at its test module, comment lines dropped) for the write and spawn APIs found none in `md_section.rs` or `md_index.rs`, and one `proc::run` of `git ls-files` in `enum_sets.rs`. `md_index.rs` reaches the `walk` module, whose `make_scratch` writes, through helpers it does not call.

**What stays as it is.** `GUARD_KIT_RO_BINS`, `GUARD_KIT_RO_FORMS` and rule `ro_pipeline`'s declared-forms test, which stays the admitted read's roster test for shell utilities. `FENCE_SAFE_ARMS` and `check-fence-run`. The recommended allowlist. Every write-or-execute form with no read replacement: `find -delete`, `tee` to a file, an `awk` program calling `system()`, and a `sed` substitution filter.

## What changes

### (1) The crate declares a read-only arm set

`native/src/emit/mod.rs` gains `READ_ONLY_ARMS: &[&str] = &["--emit-md-section", "--emit-md-index"]` beside `FENCE_SAFE_ARMS`, and a crate test holds it {design-bearing}. **Not yet applied.** gate-sdk/SPEC.md §The non-gate arm gains, after the fence-safe paragraph:

> **The read-only arm set is declared beside it and is a subset of it.** `READ_ONLY_ARMS` names the arms a read may be steered to and an isolated child may run against the main checkout (guard-kit/SPEC.md §The rule roster, rules `sed_file` and `worktree_confinement`). An arm belongs when its effect is its command line: it writes nothing but stdout and stderr, spawns no program, and reaches no network. The set holds reads, not every arm that happens to meet the contract. A crate unit test holds four things: each member names an arm-table row, each is fence-safe, each member's module rule text carries no filesystem write, removal, rename, copy, permission or link call and no program spawn, and no network-spawning arm is a member. **Honest limit:** the scan reads the member's own module, so a write reached through a helper in another module passes it. An arm therefore joins only after its call graph has been read once, and that reading is recorded where the member is added.

The two seed members are the binary's markdown reads. `--emit md-section` is the target rule `sed_file` already steers an `awk` heading range to. `--emit md-index` prints the heading outline with each section's first sentence, which the friction log's heading-listing `awk` programs were assembling by hand. No steer is added toward it, since those programs vary too widely for a decidable shape. Both are admitted from a worktree by delta 4. `--emit enum-sets` spawns `git` from its own module and stays out.

### (2) Rule `sed_file` steers a piped range print to `head` and `tail`

Rule `sed_file` gains a stream arm. A segment fed by a pipe and led by `sed -n` whose whole program is a range print (`A,Bp` or `Ap`), or by `awk` whose program is arm (i)'s `NR`-comparison pattern with no action or print-all, is blocked. The steer is `head -n <B> | tail -n +<A>` (`head -n <A> | tail -n 1` for a single line), which rule `ro_pipeline` grants {design-bearing}. **Not yet applied.** In guard-kit/SPEC.md §The rule roster, rule `sed_file`'s sentence "A `sed` fed by a pipe is a text filter with no tool equivalent and is untouched, so the discriminator is the operand, not the binary" becomes: "A `sed` fed by a pipe is a text filter and is untouched, except a bare range print, whose equivalent is `head` and `tail`; so the discriminator is the operand and the program's shape, not the binary." The `awk` arm list gains:

> - **(iii) A piped line-range read.** The (i) pattern with the segment fed by a pipe and no file operand. The steer names `head -n <B> | tail -n +<A>`, the read-only roster's spelling of the same lines, which rule `ro_pipeline` grants where it grants the pipeline's other segments.

**The ground is the replacement, not the frequency.** A range print has an exact equivalent in two roster members with no write or execute form. A substitution, a transform or a filter has none, and passes. The arm reads the program on the dequoted view and declines where that view does, as arms (i) and (ii) do. It is placed where rule `sed_file` already sits, ahead of the auto-allow band.

### (3) Rule `find_exec` steers a roster command under `-exec` to `xargs`

A new generic rule is placed directly after rule `find_glob` {design-bearing}. **Not yet applied.** Its roster item:

> - **A read-only command run by `find -exec`** (`find_exec`) — Declares `sq dq hd` and `dequoted`. A segment led by `find` whose only execute form is one `-exec … '{}' +` or `-exec … '{}' \;` running a `GUARD_KIT_RO_BINS` member, in none of that member's declared write and execute forms, is **blocked**. The steer is `find <predicates> -print0 | xargs -0 <command>` (`xargs -0 -n 1` for the `\;` form), which rule `ro_pipeline` grants through its `xargs` discriminator. `-exec` is one of `find`'s declared execute forms, so the read falls through today although its pipeline spelling is granted. The replacement runs the same command over the same files, and its effect is readable from its command line. **Declines** on any other execute or write form in the segment (`-execdir`, `-ok`, `-delete`, `-fprint`), on an executed command outside the roster or in a declared write form, on more than one `-exec`, and on an unquoted `{}`, which rule `brace_glyph` blocks first. Placed before the auto-allow band on rule `find_glob`'s reasoning, and independent of `GUARD_KIT_SEARCH_TOOLS`, since its steer names no harness tool.

The crate half is a row in `native/src/guard/rules/mod.rs`'s table after `find_glob`, and a rule function beside it. Its declared views are whatever the item says (`check-guard-registration` arms B and C).

### (4) Rule `worktree_confinement` admits the read-only arm set

The admitted read gains a second form: a segment whose command word is the gate binary or the front end, and whose arm is a `READ_ONLY_ARMS` member {design-bearing}. **Not yet applied.** The binary counts as the steer door's spelling of `GATE_SDK_NATIVE_BIN`, or that value resolved against the main checkout. The front end counts as `gate-sdk/bin/run-gates.sh` under `bash`. The arm's `--emit <name>` spelling is normalized to `--emit-<name>` as the binary normalizes it. In guard-kit/SPEC.md, the rule's first admitted-read bullet becomes: "Every segment passes rule `ro_pipeline`'s roster test … or runs a `READ_ONLY_ARMS` member on the gate binary or the front end (gate-sdk/SPEC.md §The non-gate arm)." The honest-limit bullet "The admitted read is exactly as safe as the roster's declarations … Only a tool whose effect is its command line would close that." becomes: "A roster-led read is exactly as safe as the roster's declarations, which is rule `ro_pipeline`'s own limit: a member gaining a write option is a hole until it is declared. A read-only arm closes that for the reads it replaces." The corrective's advertisement list gains the arm set, interpolated from the crate's declaration like the roster, and printed with the steer door. The PowerShell exclusion stays: the admitted read is still not offered there.

### (5) delegation-kit's isolated-child read set names the arms

delegation-kit/SPEC.md's statement that rule `worktree_confinement` "admits a read-only pipeline over the main checkout, in the one form unable to write there", and isolation cost (3) in delegation-kit/templates/agent-execution.md ("searches under it only with a read-only pipeline of the shell guard's read-only roster"), each name the read-only arm set beside the pipeline {mechanical}. **Not yet applied.** The template clause becomes: "…searches under it only with a read-only pipeline of the shell guard's read-only roster, or a read-only arm of the gate binary."

### (6) The queue entry's seed wording is corrected

In the promoting commit, the entry's "the seed is gate-sdk's fence-safe arm set, `FENCE_SAFE_ARMS` (gate-sdk/SPEC.md, arms that reach no network and write nowhere)" is corrected to say that the fence-safe set admits working-tree writes, and that the seed is a read-only subset declared beside it {mechanical}. Applied with this file.

## Producers and consumers

- **`READ_ONLY_ARMS`.** Producer: the crate declaration. Consumers: rule `worktree_confinement`'s admission and corrective (delta 4), and the crate test (delta 1). Rule `sed_file`'s markdown steer already names `--emit md-section` through the steer door and does not read the set.
- **The stream arm and rule `find_exec`.** Producer: the shell guard's rule table, on every `Bash` payload in every consumer wiring the guard. Consumer: the agent, through the block. Readers of the new rule name: `check-guard-registration` (arms A to D), the decision table, and the generated `docs/guard-kit/SPEC.md` mirror.
- **The admission.** Producer: rule `worktree_confinement`, selected by `GUARD_KIT_WORKTREE_READS` at `read-only`, the default, so every consumer that dispatches an isolated child reaches it. Consumer: the isolated child, which runs the arm.
- **Decision-table rows** (`guard-kit/guard-tests/cases.tsv`), each rule's firing and non-firing pair:
  - Firing, `block`: `grep -n x tracked.md | sed -n '5,10p'`, `grep -n x tracked.md | awk 'NR>=5 && NR<=10'` and `find . -name '*.md' -exec grep -l x '{}' +`.
  - Non-firing: `grep -n x tracked.md | sed 's/a/b/'`, which falls through; `grep -n x tracked.md | head -n 10 | tail -n +5` and `find . -name '*.md' -print0 | xargs -0 grep -l x`, which are allowed; and `find . -name '*.tsv' -delete`, which falls through.
  - Each existing row carrying a piped `sed -n`, a piped `awk` or a `find -exec` is re-derived under §Testing's non-monotone rule.
- **The admission's cases** join `guard-kit/gate-tests/worktree-confinement.test.sh`, the harness for rows a table cannot vary. From a scratch linked worktree, `<door> --emit md-section <main>/x.md "H"` is admitted, and `<door> --emit md-unwrap --write <main>/x.md` is still blocked.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The non-gate arm, the read-only set's paragraph (delta 1).
- `native/src/emit/mod.rs` — the declaration and its test (delta 1).
- `guard-kit/SPEC.md` — §The rule roster: rule `sed_file`'s pipe sentence and arm (iii) (delta 2), the rule `find_exec` item (delta 3), and rule `worktree_confinement`'s admitted read, corrective and honest limit (delta 4).
- `native/src/guard/rules/` — the stream arm, the new rule and the admission (deltas 2, 3 and 4).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/gate-tests/worktree-confinement.test.sh` — the cases (deltas 2, 3 and 4).
- `delegation-kit/SPEC.md` and `delegation-kit/templates/agent-execution.md` — the isolated-child read set (delta 5).
- `TASK-QUEUE.md` — the entry's seed wording (delta 6).
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/SPEC.md` and `docs/delegation-kit/SPEC.md` — the generated mirrors (all deltas).

The roster came from `git grep -n 'worktree_confinement\|FENCE_SAFE\|read-only roster\|admitted read'` over the tracked tree, and `git grep -n 'sed_file\|find_glob'` for the readers of a neighbouring rule name.

## Retired spellings

- None — no delta retires a spelling, and the re-phrased sentences in rules `sed_file` and `worktree_confinement` are quoted by no other surface.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the set, the stream arm, rule `find_exec` and the admission.
- [ ] **Instruction surfaces: instruction only** — the template clause carries no grounds.
- [ ] **Merged with no information lost** — the grounds for blocking only the replaced forms, and for adding no allowlist entry, land in guard-kit/SPEC.md as rule grounds, undated and unattributed (the provenance seam).
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `side-effect-free-read-arms` moves to Done in the landing commit, a stage before the drain stage.
