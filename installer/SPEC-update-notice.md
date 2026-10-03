# SPEC amendment: update-notice

**An installed tree learns when a newer release exists.** A new arm of the gate binary, `--emit update-notice`, compares the release the lock records with the newest release tag of the upstream. It probes at most once per consumer-chosen interval, with `git ls-remote --tags`. It prints one line when the upstream is ahead, and nothing on any failure. Bare `doctor` renders the same reading, and context-kit's session-context template gains a step that runs the arm. `init` states, on every run, that the check is on, which host it contacts, and how to turn it off.

The entry handed the lead's proposed shape to this stage to challenge. Three premises of that shape fail against the tree, and the design below answers each:

- **The session-context hook reaches only some installs.** It is context-kit's, and only the `delegation` profile and the derived `full` carry context-kit (`grep -v '^#' installer/profiles.list`). Its template is copied by hand (context-kit/README.md step 2), and `init` never seeds it. So `starter` and `prose` installs, and every consumer copy made before this lands, have no hook. Bare `doctor` is the reading every install reaches, which is why the notice lives in one arm that both call.
- **A cache in gitignored scratch lasts a day, not a week.** Session-context step 6 sweeps `GATE_SDK_TMP_DIR` entries older than a day (context-kit/SPEC.md §The session-context hook). The cache lives in the repository's git directory instead: per clone, never tracked, never swept.
- **An installed tree does not record the upstream.** No lock field and no knob names it. Only `init` sees `installer/package.json`'s `repository`, which `native/src/installer/workflow.rs` already parses. So `init`'s placement op writes the upstream into a knob from that field, and the crate carries no project URL.

## What changes

### (1) The probe and its knobs {design-bearing} {user-facing: operator direction 2026-09-29 recorded on update-availability-notice — add a newer-release notice, seeded default weekly, its interval a consumer-selectable set with off among it, and init stating that it is on and how to turn it off because the probe shows the host the adopter's IP address}

**Three new static knobs** in `native/src/knobs/gate_sdk.rs`:

- `GATE_SDK_UPDATE_CHECK` — the interval, one of `off`, `daily` or `weekly`, default `weekly`. The table validator refuses any other value, as `GUARD_KIT_WORKTREE_READS`' does, so a misspelt `off` refuses rather than reading as the default.
- `GATE_SDK_UPDATE_UPSTREAM` — the git URL to probe, default empty. Empty means no check.
- `GATE_SDK_UPDATE_TIMEOUT` — the probe's bound in whole seconds, default `5`.

**`--emit update-notice`**, a non-gate arm (gate-sdk/SPEC.md §The non-gate arm) declaring those three knobs. It always exits 0, except that a usage error exits 2. It prints nothing unless every condition below holds:

1. The interval knob resolves to `daily` or `weekly`. An unresolvable value is read as `off`, so a refused knob never probes.
2. The upstream knob is non-empty.
3. A `checkwright.lock` at the repository root carries a `version`. A residue or a tree vendored without the installer has none.
4. The cache, or a probe made because the cache is stale, names a newest release above that version.

**The probe.** `git ls-remote --tags --refs <upstream>` runs under `GIT_TERMINAL_PROMPT=0`, bounded by the timeout through the proc module's bounded runner, which gains an environment argument for this call. The newest release is read from the refs it lists. Only `refs/tags/v<major>.<minor>.<patch>` refs count, so a prerelease or build suffix is never a candidate. They are ordered over their digit runs, as `toolfloor::floor_met` compares a recorded version, the comparison `init`'s downgrade refusal uses (§init). A run that expires, exits non-zero or yields no candidate writes nothing and prints nothing. The next read then finds the cache still stale and probes again.

**The cache** is one file, `checkwright-update-check`, in the directory `git rev-parse --git-common-dir` names, so every worktree of a clone shares it. It holds one line: the probe's Unix time and the newest version it found. A probe is due when the file is absent or unreadable, when its time is older than the interval (86400 seconds for `daily`, 604800 for `weekly`), or when its time lies in the future. A successful probe rewrites the file whole.

**The line.** It reads `checkwright v<newest> is available; this tree has v<installed>. Run update to upgrade, or set GATE_SDK_UPDATE_CHECK = off to stop this check.`

Unit tests:

- tag parsing: a suffixed tag and a non-`v` tag are skipped;
- choosing the newest;
- the staleness rule: absent, old, future and fresh files;
- the arm's silence on each failed condition.

A crate test probes a scratch bare repository by path, with tags `v0.1.0` and `v9.0.0`, for the printed line. It probes an unreachable path for silence and exit 0.

### (2) init writes the upstream and states the check {design-bearing} {user-facing: the same operator direction — init states it is on and how to turn it off}

`native/src/installer/init.rs`, and `native/src/install.rs`: the placement op and its `declared_lines`.

- **The upstream line.** `GATE_SDK_UPDATE_UPSTREAM` becomes the seam's third declared line, beside `GATE_SDK_KIT_DIRS` and `GATE_SDK_SPEC_BASE_URL`. `declared_lines` takes a third value, and `--place-artifact` takes it as an optional `upstream` key, so the in-process call and the wire op resolve one derivation (§The gate binary). `init` passes `package.json`'s `repository.url` with a leading `git+` dropped. The line is re-derived and written on every install, and its stale spelling is replaced rather than duplicated, as the other declared lines are. A package naming no repository passes an empty value. The knob is then omitted under the declared-line rule, and the notice stays off.
- **The statement.** Where the interval resolves to `daily` or `weekly` and the upstream is set, `init` prints one paragraph ahead of the `next:` block:

      update check: weekly, with git ls-remote against <url>, which shows that host your IP address; set GATE_SDK_UPDATE_CHECK = off in <gates-dir>/gate-sdk-config.knobs to turn it off.

  The interval word is the resolved one. The line is not indented, so the follow-up block's grammar (§init) is untouched.
- **What init does not do.** It never probes. Its `doctor` precondition renders no update reading, and the plan `--dry-run` prints is unchanged apart from the seam line it writes.

### (3) doctor renders the reading {design-bearing} {user-facing: the same operator direction — the lead's proposed shape names doctor as the second reader}

`native/src/installer/doctor.rs`, the installed block. On bare `doctor` only, a `latest` line follows `version`. Bare `doctor` is `doctor::run`, which passes the installed selection to `diagnose`, so the selection argument cannot tell the two callers apart. `diagnose` therefore gains an argument naming its caller, and `init`'s precondition call passes the value that renders no line. It reads the cache through delta 1's code and probes when the cache is stale. Its renderings:

- `latest       v0.32.0 available — run update`
- `latest       v0.31.0, current`
- `latest       not checked — GATE_SDK_UPDATE_CHECK is off`
- `latest       not checked — no upstream recorded`
- `latest       unknown — the last check could not reach the upstream`

The line reports without setting the exit status, on the artifact and disarmed lines' ground (§doctor). The verdict line is unchanged, since a newer release is neither a machine below contract nor a failed install.

### (4) The session-context template runs the arm {mechanical}

`context-kit/templates/session-context.sh`, and this repository's filled copy `scripts/session-context.sh`.

A step 10, **Update notice**, runs the gate binary with `--emit update-notice` under the same binary guard as step 5, and prints whatever it prints. Like step 5, it is not suppressed for a `lead` session. The arm bounds its own probe and is silent on failure, so the step can neither fail nor stall a session past the timeout. This repository records no lock, so its own copy prints nothing.

### (5) The owning sections state the notice {mechanical}

**Not yet applied.**

- **installer/SPEC.md, a new `### The update notice` under §update.** It states:
  - the arm's four conditions, the probe, the tag filter and its ordering;
  - the cache, its git-directory home and the reason it is not in scratch (step 6's sweep);
  - the line, and the arm's exit status.

  Two honest limits close it. The probe shows the upstream host the adopter's IP address at most once per interval, which is why the check is stated at install and has `off` among its values. A consumer copy of the session-context template made before this lands lacks step 10, and `starter` and `prose` installs carry no hook, so for them bare `doctor` is the reading.
- **installer/SPEC.md §The gate binary.** Three spots change:
  - the bullet *`--kits` and `--spec-base-url` become the seam's declared lines* adds `--upstream`;
  - the paragraph *The two declared-line keys are optional* becomes *The declared-line keys are optional*;
  - the paragraph *The owned set is the artifact path plus the caller's declared lines* names `GATE_SDK_UPDATE_UPSTREAM`, the upstream an installed tree probes (§The update notice), as the third declared line.
- **installer/SPEC.md §init**, the paragraph on the follow-up block: one sentence states the update-check paragraph printed ahead of the block.
- **installer/SPEC.md §doctor**, the bullet *Run inside a vendored repository*: it adds the `latest` line, rendered by the bare verb alone. The paragraph *The artifact, omitted and disarmed lines report without setting the exit status* names it beside them.
- **gate-sdk/SPEC.md §Layout and configuration** gains the three knobs with their defaults and value set, pointing at installer/SPEC.md §The update notice for the rule.
- **context-kit/SPEC.md §The session-context hook** gains step 10, in the list's form.
- **docs/install.md §Upgrading** gains two sentences. *An installed tree checks for a newer release once a week with `git ls-remote`, which shows the upstream host your IP address, and `doctor` and the session-context hook say when one exists. Set `GATE_SDK_UPDATE_CHECK = off` in your gates directory's `gate-sdk-config.knobs` to turn it off.*

### (6) The release declaration and the mirrors {mechanical}

- `.workflow/release-declarations.md` gains one Behavior changes bullet in its grammar. Its lead is **checkwright init, doctor, `--emit update-notice`**, followed by:
  - the new knobs `GATE_SDK_UPDATE_CHECK` (default `weekly`), `GATE_SDK_UPDATE_UPSTREAM` (written by `init`) and `GATE_SDK_UPDATE_TIMEOUT` (default `5`);
  - the probe and the IP disclosure;
  - nothing to do, or set the interval to `off`;
  - and that context-kit/templates/session-context.sh gains step 10, to be copied into an edited copy by hand.
- `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md` and `docs/context-kit/SPEC.md` are regenerated with the command their freshness gate prints, in the commit landing delta 5. Any other projection the battery reds on is regenerated with the command its own gate prints. The knob roster and the enforcement map are candidates.

## Producers and consumers

- **The upstream knob.**
  - Producer: the placement op's declared lines, fed by `init` on every install from a package naming a repository. The published package names one (`installer/package.json`'s `repository`), so every adopter install produces it.
  - Consumers: delta 1's arm, and doctor through it.
- **The interval and timeout knobs.**
  - Producer: their table defaults, or a consumer's knob-file line or environment.
  - Consumer: delta 1's arm.
- **Roster-holding readers of the three new names**, by `grep -rn "static_row\|fn rows" native/src/knobs` and `grep -n "GATE_SDK_TMP_DIR" gate-sdk/SPEC.md`:
  - the gate-sdk knob table and its validator;
  - gate-sdk/SPEC.md §Layout and configuration, which `check-knob-citation` and `check-knob-default-coupling` hold to the table;
  - `--emit knob-roster`, which derives its listing from the table;
  - the arm's declared-knob roster, which `check-reads-couples` holds to its reads.
- **The arm.**
  - Producer: the binary's emit table, `ARMS` in `native/src/emit/mod.rs`. That table is the pinned arm set `check-front-door-verbs` reads, and the arm is advertised on no front-door page.
  - Consumers: delta 3's doctor line, and delta 4's template step and this repository's copy.
- **The cache file.**
  - Producer: a successful probe.
  - Consumer: the next read. `uninstall` leaves it, as it leaves the scratch directory, since it is no `files` row and lives outside the worktree.
- **The statement.**
  - Producer: `init`, on every run where the check is on.
  - Consumer: the adopter. The consumer smoke's follow-up arm parses only the indented block after `next:`, which this unindented paragraph sits outside.
- **Fields:** none is added to the lock. Point 4 is vacuous.
- **Red conditions (point 5):** no corpus narrows. The knob validator refuses a value outside the set, at exit 2 where the knob file is read.

## Existing sections updated

Roster by `grep -n "declared line\|declared-line\|follow-up block\|Run inside a vendored\|report without setting" installer/SPEC.md`, `grep -rn "declared_lines" native/src`, `grep -n "^[0-9]\. \*\*" context-kit/SPEC.md`, `grep -n "## Upgrading" docs/install.md` and `grep -rn "fn diagnose\|placement" native/src/installer`, over the tracked tree.

- `native/src/knobs/gate_sdk.rs` — the three knobs and the interval validator (delta 1).
- `native/src/proc.rs` — the bounded runner's environment argument (delta 1).
- `native/src/emit/mod.rs` — the arm's registration (delta 1).
- `native/src/emit/update_notice.rs` — the arm, its cache and its tests (delta 1).
- `native/src/install.rs` — `declared_lines`' third value and `--place-artifact`'s `upstream` key (delta 2).
- `native/src/installer/init.rs` — the upstream value passed to the placement, and the statement (delta 2).
- `native/src/installer/doctor.rs` — the `latest` line (delta 3).
- `context-kit/templates/session-context.sh` — step 10 (delta 4).
- `scripts/session-context.sh` — step 10 (delta 4).
- `installer/SPEC.md` — §update's new subsection, §The gate binary's declared-line paragraphs, §init and §doctor (delta 5).
- `gate-sdk/SPEC.md` — §Layout and configuration (delta 5).
- `context-kit/SPEC.md` — §The session-context hook, step 10 (delta 5).
- `docs/install.md` — §Upgrading (delta 5).
- `.workflow/release-declarations.md` — the Behavior changes bullet (delta 6).
- `docs/installer/SPEC.md` — the regenerated mirror (delta 6).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 6).
- `docs/context-kit/SPEC.md` — the regenerated mirror (delta 6).

## Retired spellings

- None — the change adds names and retires none.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the template's step 10 carries no grounds; context-kit/SPEC.md and installer/SPEC.md hold them.
- [ ] **Merged with no information lost** — §The update notice states the arm whole, and the knob rows point at it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`), discharged at the iteration while sibling installer amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery, the gate-sdk and context-kit fixture suites, and the consumer smoke (`--run-consumer-smoke`), whose installs now write the upstream line.
- [ ] **The entry is done** — `update-availability-notice` moves to Done in the commit landing delta 5, before the drain stage.
