# SPEC amendment: overlay-dead-path

Queue entry: `settings-overlay-dead-path-unreported`, the dead-path unit of `guard-friction-reach`
(operator direction, 2026-09-15, lead-relayed). A root-level amendment because it spans three
components: guard-kit (the advisory and its triage step), context-kit (the predicate's owning section)
and `native/` (the one holder both readers call).

**The placement call the entry left open, ruled.** `compare-settings-allow` gains a third question over
the local overlay: which entries grant a script that no longer exists. It answers by calling the
extraction predicate `check-settings-paths` already holds, not a copy of it. Three alternatives were
weighed and refused:

- **Widening `check-settings-paths` to read the overlay** is refused by the placement ruling
  guard-kit/SPEC.md §compare-settings-allow already carries. The overlay is a gitignored per-machine
  file CI cannot see, so a gate over it reds on one clone and not another. The gate keeps the committed
  file.
- **A second extraction in guard-kit** is refused because it duplicates a predicate whose scoping was
  ruled in one place (the `*`-token skip, the `env`/interpreter walk, the `.sh` scope). An edit to one
  copy would silently leave the other reading differently.
- **Moving the predicate's ruling into guard-kit** is refused because the rationale lives with the gate
  that owns the committed settings file as a subject. Relocating it moves the record without changing
  which reader decides it.

So the predicate stays owned by context-kit/SPEC.md §check-settings-paths and held in
`native/src/gates/settings_paths.rs`. guard-kit's advisory calls it as a second reader, and both
sections name the other. An emit arm calling into a gate module has precedent in the crate:
`emit/enum_sets.rs` and `emit/entry_history.rs` both do.

**Measured, and the class is attested rather than live.** At `owed-port-tail`'s close triage, 24 of
108 local allow entries named deleted scripts, and they were pruned by hand. Re-probed at this stage
with `ls` over each literal `.sh` path in the current overlay's `Bash(…)` entries: all resolve. Two of
the overlay's entries carry an absolute path, the shape a per-machine grant takes and a committed file
does not. Delta 2 rules on that shape.

## The seam

- **Kit mechanism:** one public predicate function in the gate module, one report section in the
  advisory, one disposition line in the close-triage template, and the two SPEC sections.
- **Consumer config:** none. The advisory's four declared knobs are unchanged. The dead-path question
  needs no vocabulary, since existence is a filesystem fact, so unlike the breadth question it has no
  probe knob and no opt-in.
- **Private rule content:** none in reach.

## What changes

### (1) The extraction predicate becomes one callable holder {mechanical}

`native/src/gates/settings_paths.rs`. The per-entry extraction that `run` performs inline becomes a
public function, `literal_script_path(entry: &str) -> Option<&str>`. Its steps: strip `Bash(` and
`)`, split on ASCII whitespace, walk `command_token`, require a `.sh` suffix, and reject a token
containing `*`. `run` calls it and keeps its checked count and its resolution against `root`
unchanged. The existing unit tests stay, and one is added: the function returns `None` for a
non-`Bash` entry, a non-`.sh` token and a `*` token, and the token for the bare, `env`-prefixed and
trailing-flag shapes. `allow_entries` stays private to the gate, because the advisory already reads
its allow list through its own three-way `AllowRead`.

### (2) `compare-settings-allow` reports dead-path local entries {design-bearing}

`native/src/emit/compare_settings_allow.rs`. A function over the overlay's entries collects, for each
entry, `settings_paths::literal_script_path`'s candidate. A candidate starting with `/` resolves as
written. Any other candidate resolves against the working directory, which the front-end sets to the
repository root. It returns the entries whose candidate is not a file, plus the checked count. The
report gains a section **between** the redundancy section and the breadth section, since both prune
dispositions sit together:

- a header `=== settings allowlist dead paths (advisory — prune candidates) ===`;
- when none is dead: `no dead-path local entries (<n> literal .sh grant(s) in <local> resolve)`. The
  checked count is what tells a predicate that scoped to the array from one that matched nothing, as
  it does in the gate;
- otherwise: `<k> local allow entr(ies) grant a script that does not exist — safe to prune from <local>:`,
  each row `<entry> — no such file: <path>`, and a help line saying a script moved or deleted since the
  grant was taken strands it, so repoint or remove the entry.

The absolute arm is the one place the advisory's resolution differs from the gate's. A committed file
is shared across clones, so its literals are repo-relative and the gate joins them to its root. An
overlay entry is per-machine and routinely absolute, so joining it to the root would report a live
grant dead. The no-overlay path (absent or unparseable overlay) is unchanged and prints no dead-path
section. `--count` is **unchanged** at two integers. The ground is the one the section already gives
for refusing a declared count: no reader needs it, and the two-number line is a shape a consumer may
already parse. The dead-path set is read on the report, at the close step that prunes it.

### (3) The advisory's cases {mechanical}

- In-crate, in `compare_settings_allow.rs`: a fixture dir with one live script and a local list holding
  the live literal, a dead relative literal, a dead absolute literal, a `*` pattern and a bare command.
  The dead set is exactly the two dead literals, and the checked count is 3.
- `guard-kit/gate-tests/compare-settings-allow.test.sh`, through the front-end: an overlay holding
  `Bash(bash gate-sdk/bin/run-gates.sh)` and `Bash(bash scripts/no-such-script.sh)` reports the second
  and not the first. The existing `--count` assertions (`0 1`, `0 0`, `0 1`) stay as they are, which is
  what pins delta 2's unchanged count.

### (4) guard-kit/SPEC.md §compare-settings-allow: the third question {design-bearing}

**Not yet applied.** A paragraph after the placement ruling's **That ruling is about this subject**
paragraph:

> **A third question over the overlay: which entries grant a script that is gone.** A grant naming a
> deleted script breaks nothing, since no command reaches it, but every reader of the overlay must
> re-verify which lines still mean anything. And a dead grant beside a renamed tool reads as coverage.
> The report lists each local entry whose command token is a literal `.sh` path that does not resolve,
> with the checked count on its clean line. The token is taken by context-kit/SPEC.md
> §check-settings-paths' extraction predicate, **called rather than copied**: the scoping is ruled
> there, and a second extraction would drift from it the first time either was edited. The one
> difference is resolution. An absolute token resolves as written, because a per-machine grant is
> routinely absolute where a committed one is not. This is the same subject that section gates for the
> committed file, answered here as an advisory, because the placement ruling above is about the
> overlay and does not move. `--count` does not carry the set, on the declared count's ground below.

The sentence in the declaration paragraphs `every byte of output matches the tool as it behaved before
the declaration shipped` becomes `the breadth report matches, byte for byte, the tool as it behaved
before the declaration shipped`. The claim was always about the declaration's cost, and the dead-path
section now sits in every report.

The section's opening sentence (`lists local-overlay allow entries already granted by a glob in the
committed settings`) gains `, and those naming a script no longer in the tree`.

### (5) context-kit/SPEC.md §check-settings-paths names its predicate's second reader {mechanical}

**Not yet applied.** After the **The extraction predicate** paragraph:

> The predicate has a second reader: guard-kit's `compare-settings-allow` applies it to the local
> overlay as an advisory (guard-kit/SPEC.md §compare-settings-allow). It calls this gate's holder
> rather than restating it, so a change to the scoping above changes that report too. The gate's
> subject stays the committed file alone.

### (6) The close-triage step takes the third set {mechanical}

`guard-kit/templates/close-triage.md`, step 4. `it reports two sets` becomes `it reports three sets`,
and a bullet follows **Redundant**. **Not yet applied:**

> - **Dead path**: remove every listed entry — it grants a script that no longer exists.

guard-kit/SPEC.md §The close-stage triage step's prose (`take its **two** dispositions — prune the
listed redundant local entries, and for each entry …`) gains the dead-path prune as a third
disposition, in the same sentence.

`guard-kit/README.md`'s `--emit compare-settings-allow` usage comment gains `, and those naming a
script that no longer exists`.

## Producers and consumers

- **The dead-path section** — producer: `compare_settings_allow::emit`, reached through the front-end
  as `--emit compare-settings-allow`. That is step 4 of `templates/close-triage.md`, spliced into this
  repo's close binding, so the call runs at every close. Consumer: the close session, which prunes the
  listed entries. Pruning is low-impact under §compare-settings-allow's impact criterion (an edit that
  only narrows), so it is applied and reported.
- **`literal_script_path`** — producer: `settings_paths.rs`. Readers: `settings_paths::run` (the gate)
  and the advisory's dead-path function. It has no other caller and adds no field.
- **Each row's fields**: the entry (read by the pruning session to find the line), the unresolved path
  (read to decide whether to repoint or remove), and the checked count (read on the clean line to tell
  scoping from a vacuous match). All three have a named reader.
- **Point 5.** No corpus narrows. The gate's extraction moves behind a function and its verdicts stay
  as they are. Its fixture pair pins the **checked count**, a non-monotone exact-count reader, so the
  build clears it by running the pair, not by inspection. The advisory adds a section and changes no
  existing section's content or `--count` value.
- **What the advisory newly reads**: the filesystem, through `Path::is_file`. It spawns no program, so
  gate-sdk/SPEC.md §The non-gate arm's record of this arm as the class's first member with an empty
  spawned-program set stays true.

Derivation of the rosters here and below: `git grep -n "compare-settings-allow\|compare_settings_allow"`
and `git grep -n "settings_paths\|check-settings-paths"` over the tracked tree, excluding the queue;
a read of `guard-kit/templates/close-triage.md` and of this repo's close binding shim
(`.claude/commands/close.md`, which points at the template and restates no set); and a read of
gate-sdk/SPEC.md's empty-spawn-set paragraph. The build unit re-derives them.

## Existing sections updated

- `native/src/gates/settings_paths.rs` — `literal_script_path` and its test (delta 1).
- `native/src/emit/compare_settings_allow.rs` — the dead-path function, section and tests (deltas 2 and
  3).
- `guard-kit/gate-tests/compare-settings-allow.test.sh` — the front-end dead-path case (delta 3).
- `guard-kit/SPEC.md` — §compare-settings-allow, the third question, the byte-identity sentence and the
  opening sentence (delta 4), and §The close-stage triage step's disposition sentence (delta 6).
- `context-kit/SPEC.md` — §check-settings-paths, the second-reader paragraph (delta 5).
- `guard-kit/templates/close-triage.md` — step 4's third set (delta 6).
- `guard-kit/README.md` — the usage comment (delta 6).
- `.workflow/surface-ceiling.txt` — `guard-kit/templates/close-triage.md` is a ratcheted surface with a
  committed ceiling (context-kit/SPEC.md §The surface ratchet), so its growth is re-stamped with
  `--emit always-loaded --ceiling` in the growing commit (delta 6).
- `docs/guard-kit/SPEC.md` and `docs/context-kit/SPEC.md` — the on-site SPEC mirror, regenerated with
  `--emit docs-mirror --write` (deltas 4, 5 and 6).
- `docs/footprint.md` and `docs/value.md` — a `templates/` markdown edit moves the per-kit footprint
  and the value rollup (docs/site-architecture.md §Generated projections and their freshness gates)
  (delta 6).
- `.workflow/release-declarations.md` — one guard-kit bullet: `compare-settings-allow` gains a dead-path
  section and the close-triage template a third disposition, and a consumer re-splicing the template
  picks it up (deltas 2 and 6).

## Retired spellings

- None — no delta retires a spelling. Delta 4's sentence rewrite and delta 6's `two sets` → `three sets`
  are not names another surface cites.

## Definition of Done

- [ ] **Causal completeness** — the section has a deployed producer and a named consumer, and every row
      field has a reader.
- [ ] **Instruction surfaces: instruction only** — delta 6's template bullet carries no grounds, and
      delta 4 places them.
- [ ] **Merged with no information lost** — each SPEC addition integrated, not appended.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
