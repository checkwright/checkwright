# SPEC amendment: path-rooted

Tracked shell sites test a path for absoluteness by a leading `/` alone (`[[ "$x" == /* ]]`, or a `case` arm `/*)`). A caller-supplied Windows drive-letter path therefore reads as relative and is joined onto a root, `$PWD/C:/…`. The crate closed the same defect by giving absoluteness one owner, `walk::path_root` (gate-sdk/SPEC.md §The crate's crosser). The shell side has none, and `check-path-dialect`'s honest limit records the gap: a file composing one root with a caller's foreign absolute path passes, because there is "no shell counterpart of `walk::path_root` to route it through".

**The ruling: one shell predicate, `gate_path_rooted`, in `gate-sdk/lib/gate.sh`, answering exactly what `walk::path_root(p).is_some()` answers, and a shell half of the locality arm holding every other spelling out.** Rooted means a leading `/` or `\`, which covers `//srv` and `\\srv`, or an ASCII letter, `:`, then `/` or `\`. `C:` alone and `C:foo` are relative, as they are to the crate.

**This narrows a recorded refusal and does not reverse it.** gate-sdk/SPEC.md §The path-dialect contract refuses a shared shell *normalizer*, on the ground that the corpus's families source different libraries and some resolve their root before sourcing anything, so a helper buys a cross-kit dependency to save a one-line idiom. Neither ground reaches this predicate. It converts nothing, so it is not a normalizer and cannot double-normalize a value. And every site but one already sources `gate-sdk/lib/gate.sh`. The one that cannot is `installer/bin/checkwright.sh`, the POSIX bootstrap, which locates the payload by resolving its own `$0` and so cannot source a payload copy of anything (installer/SPEC.md §The install boundary). It carries a byte-identical twin, and a crate unit test holds both the twin and the library body to `walk::path_root` over one corpus. That is criterion 6's machine-held duplication (gate-sdk/SPEC.md §The port-candidate criteria), which admits a second holder only against a standing comparison. The one-line idiom the refusal protects, `cd … && pwd -P`, is untouched.

**Measured at authoring (2026-09-23).** The filed probe `git grep -n -E '== /\*|/\*\)' -- '*.sh'` is a coarse substring match and over-catches: re-run verbatim it returns sixteen lines, the eight below plus every prefix/containment arm the next sentence lists. **Verified at align (2026-09-23):** the eight real sites are the seven filed plus `gate-sdk/lib/gate.sh`'s `gate_native_bin_spelled`, which already takes a drive root but not a leading `\` — that eighth site is outside the coarse probe's own match set (its arm reads `[A-Za-z]:\\*)`, no `/*` substring) and was located by manual review, not the grep. Applying delta 6's own predicate (below), rather than the coarse probe, to the same corpus is what actually isolates the eight from the sixteen. Every other `/*`-shaped `case` arm or glob in the tracked shell tree is a prefix or containment test and stays green under the predicate: guard.sh's `*/*)` (both `_guard_unclean_word`'s check at line 372 and `guard_rule_grant_path_slot`'s at line 1952), `"$d"/*)`, `"$d"/* | "./$d"/*)`, `*"$d"/*)`, `*/../*)` and `/dev/*)`, and gate.sh's `https://*|http://*)`.

| Site | Input | Reachable by a drive path |
|---|---|---|
| `gate-sdk/lib/test-hermetic.sh` (`_th_bin`) | `gate_native_bin`, from a knob file value | yes |
| `gate-sdk/smoke/install.sh` (`pbt_bin`) | `gate_native_bin` in a subshell, from the caller's env or knob file | yes |
| `installer/bin/checkwright.sh` (`SELF`) | `readlink` output while resolving npm's bin link | unlikely on MSYS, but its PowerShell twin already tests with `IsPathRooted` |
| `installer/consumer-smoke/run-smoke.sh` (`ROSTER_FILE`) | the caller's `GATE_SDK_NATIVE_TARGETS_FILE`, or a scratch path under `INSTALLER_SMOKE_TMP_DIR` | yes: the Windows CI legs run `cygpath -u` first, a caller-side workaround for exactly this site |
| `lifecycle-kit/smoke/install.sh` (`smoke_bin`) | `gate_native_bin`, from the caller's env or knob file | yes |
| `guard-kit/lib/guard.sh` (`_guard_unclean_word`, rule 24) | the lead word of a path-slot capture in the agent's command | yes, and it is a guard hole: `bash C:/elsewhere/checks/check-x.sh` matches the `*/checks/check-*.sh` grant and its lead word reads clean |
| `guard-kit/gate-tests/guard-lib-parity.test.sh` (`BIN`) | `gate_native_bin` after the hermetic pin | yes, if the invoker pins a drive path |
| `gate-sdk/lib/gate.sh` (`gate_native_bin_spelled`) | the knob's value | a leading `\` reads as relative and gains `./` |

## What changes

### (1) The predicate and its library home {design-bearing}

**Not yet applied.** `gate-sdk/lib/gate.sh` gains:

```sh
gate_path_rooted() {  # <path> — 0 when rooted in either dialect, walk::path_root's predicate
    case "$1" in
        /* | \\* | [A-Za-z]:/* | [A-Za-z]:\\*) return 0 ;;
    esac
    return 1
}
```

It prints nothing. Its body is POSIX sh, so the bootstrap's twin (delta 2) can be the same bytes. `gate_native_bin_spelled` routes its rooted arms through it: a rooted value returns unchanged, and the `./* | ../*` arm stays a `case`. A value opening with `\` is therefore anchored where it gained `./` before.

**Inferred, cannot run before build:** `[A-Za-z]` matches only ASCII letters under bash's default `globasciiranges` and under dash, and the parity corpus in delta 3 carries a non-ASCII letter before `:` to hold it — if a shell under test reads the range by locale, the body spells the 52 letters out instead. The corpus runs only once delta 1 lands.

### (2) The bootstrap's byte twin {mechanical}

**Not yet applied.** `installer/bin/checkwright.sh` carries the same function, byte-identical from its definition line through its closing `}`, and its `readlink` arm becomes `gate_path_rooted "$SELF" || SELF="$LINK_DIR/$SELF"`.

### (3) One crate test holds both bodies to `path_root` {design-bearing}

**Not yet applied.** A unit test beside `walk::path_root` spawns `bash`, sources `gate-sdk/lib/gate.sh` and asks `gate_path_rooted` over a canned corpus: `/x`, `\x`, `//srv`, `\\srv`, `C:/x`, `c:\x`, `C:`, `C:x`, `1:/x`, `./x`, `x`, the empty string and a non-ASCII letter before `:/`. Each answer must equal `path_root(p).is_some()`. It then extracts the function body from `gate-sdk/lib/gate.sh` and from `installer/bin/checkwright.sh` by name, as the hook emitter's matcher extraction does, and requires the two byte-identical. The pre-binary accessor test in `native/src/knobs/gate_sdk.rs` is the precedent for a crate test sourcing the library.

### (4) Every other site routes through the predicate {mechanical}

**Not yet applied.**

- `gate-sdk/lib/test-hermetic.sh`: `gate_path_rooted "$_th_bin" || _th_bin="$_th_root/$_th_bin"`.
- `gate-sdk/smoke/install.sh`: the test runs inside the subshell that already sources the library and resolves `gate_native_bin`, so `pbt_bin` leaves it rooted.
- `installer/consumer-smoke/run-smoke.sh`: `native gate_path_rooted "$ROSTER_FILE" || ROSTER_FILE="$REPO/$ROSTER_FILE"`, through the script's existing `native` wrapper, which sources the library in a subshell.
- `lifecycle-kit/smoke/install.sh`: `gate_path_rooted "$smoke_bin" || export GATE_SDK_NATIVE_BIN="$PWD/$smoke_bin"`.
- `guard-kit/gate-tests/guard-lib-parity.test.sh`: `gate_path_rooted "$BIN" || BIN=…`, the rest of the line unchanged.

The `cygpath -u` conversions in `.github/workflows/gates.yml` before the consumer smoke stay. They convert the scratch base for every later consumer of it, not only for this test.

### (5) Guard rule 24 reads a drive root as absolute {mechanical}

**Not yet applied.** In `guard-kit/lib/guard.sh`, `_guard_unclean_word`'s `/*)` arm becomes `gate_path_rooted "$2"`, keeping the `'~'*` arm as a `case`. The library is sourced at the guard's load, and the guard exits on its advisory before any rule runs when it is not, so the predicate is always defined when rule 24 runs. `guard-kit/guard-tests/cases.tsv` gains `block	bash C:/elsewhere/checks/check-x.sh` beside the existing `bash /elsewhere/checks/check-x.sh` row. In guard-kit/SPEC.md §The generic ruleset, rule 24 step (3), "A clean word carries no `..` component and does not begin with `/` or `~`." becomes "A clean word carries no `..` component, is not rooted by gate-sdk's `gate_path_rooted` (a leading `/` or `\`, or a drive root), and does not begin with `~`."

### (6) The locality arm gains a shell half {design-bearing}

**Not yet applied.** `native/src/gates/path_dialect.rs` runs a shell locality scan inside its existing shell-corpus loop, over each line's code half from the shell split it already applies. A line is a finding when its code half spells a single-dialect absoluteness test:

- a `[[ ]]` glob test, `==`, `!=` or ` =`, then optional blanks, then `/*` (optionally quoted `"/"*`), followed by a blank, `]`, `;` or the end of the line;
- a `case` alternative that is exactly `/*` (optionally quoted), preceded by the start of the line, a blank, `(` or `|`, and followed by optional blanks and `)` or `|`.

Two clearances, the crate arm's pair spelled for shell. The line lies inside a function whose definition line is `gate_path_rooted() {`, through its closing `}`, matched by name so the bootstrap's twin and a fixture's copy clear the same way. Or the line, or the contiguous comment run above it, carries `# path-dialect-exempt: <reason>` with a non-empty reason. No `spec:` verdict clears it, on the crate arm's ground. The finding names the file, the line and the form, and the remedy `gate_path_rooted` (gate-sdk/lib/gate.sh).

The clean line gains its own phrase, `N shell absoluteness test(s) — M inside gate_path_rooted, K declared out of the filesystem namespace`, rather than folding into the crate primitive tally, so the existing `expect.txt` substrings survive.

The fixture pair: `bad/tree/` gains a `[[ … == /* ]]` file and a `case … /*)` file, and `bad/expect.txt`'s count rises from 11 to 13 with a line per finding. `good/tree/` gains a file defining `gate_path_rooted()` with both forms inside it, a site routed through it, a `# path-dialect-exempt:` shell line, and the prefix arms that must stay green (`*/*)`, `/dev/*)`, `"$d"/*)`), and `good/expect.txt` gains the new phrase's counts. The descriptor's `# spec:` summary names both halves.

### (7) The owning sections state the predicate and the arm {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md:

§The path-dialect contract, the paragraph opening "**No shared shell normalizer is introduced, and the refusal is load-bearing.**" becomes:

> **No shared shell normalizer is introduced, and the refusal is load-bearing.** No kit's `lib/` holds one, and a helper that *converts* a root would buy a cross-kit dependency to save a one-line idiom, since the corpus's families source different libraries and some resolve their root before sourcing anything — for no error class `check-path-dialect` does not already catch. The idiom needs no name: the installer's own bootstrap and `bin/build-native.sh` both write it unprompted. **Absoluteness is the exception, because it is a predicate and converts nothing.** `gate_path_rooted` (§lib/gate.sh) answers what `walk::path_root` answers, every site testing a caller's path already sources the library, and the bootstrap, which cannot source it, carries a byte twin a crate test holds.

§Porting to Rust does not retire dialect exposure, after "a module outside it reaches each through a named `walk` helper — … `child` — rather than re-spelling it.", add: "The shell side owns absoluteness the same way, through `gate_path_rooted`."

§lib/gate.sh: in the opening paragraph, "the one POSIX sh body in the library" becomes "one of the library's two POSIX sh bodies", and the list gains, after the `gate_native_bin_spelled` bullet:

> - **`gate_path_rooted` is the shell's one owner of absoluteness**, `walk::path_root`'s predicate: a leading `/` or `\`, or an ASCII letter, `:`, then `/` or `\`. It prints nothing and converts nothing. Its body is POSIX because `installer/bin/checkwright.sh` carries it byte for byte, the bootstrap being unable to source a payload file before it has located the payload. A crate unit test holds both bodies to `path_root` over one corpus and to each other, criterion 6's machine-held duplication. `gate_native_bin_spelled` asks it, and `check-path-dialect`'s shell locality arm reds any other spelling.

In the `gate_native_bin_spelled` bullet, "rooted, drive-rooted (`X:/…`, `X:\…`), or already `./` or `../`" becomes "rooted by `gate_path_rooted`, or already `./` or `../`".

§check-path-dialect: the invariant's clause "every text-level path primitive in the crate is spelled in the crate's speller" becomes "every text-level path primitive in the crate is spelled in the crate's speller and every shell absoluteness test in the tree asks `gate_path_rooted`". The paragraph opening "**A second arm asserts locality of the text-level primitives**" gains, after its bullet list's lead-in, the sentence "Its shell half holds the first primitive over the tracked shell tree: a `[[ ]]` glob test or a `case` alternative that is exactly `/*` is a finding outside `gate_path_rooted`'s own body, cleared by the same `path-dialect-exempt:` token spelled as a `#` comment." In the paragraph opening "**Its honest limit:**", the sentence "That shape turns on testing a caller-supplied value's absoluteness from its text — a different primitive, with no shell counterpart of `walk::path_root` to route it through, and this contract refuses a shared shell normalizer on its own stated grounds." becomes "The composition is out of this arm's reach, but its precondition is not: a caller's foreign absolute path now reads as rooted through `gate_path_rooted`, which the locality arm's shell half holds every absoluteness test to." The fixture sentence gains the shell half's cases.

In installer/SPEC.md §Implementation, the bullet "**The payload directory comes from `$0`.**" gains: "Whether `readlink`'s answer is already rooted is asked of `gate_path_rooted`, which the bootstrap carries as a byte twin of gate-sdk's library body, held to it by a crate unit test (gate-sdk/SPEC.md §lib/gate.sh)."

## Producers and consumers

- **`gate_path_rooted`** (deltas 1 and 2). Producer: the library, sourced by every site in delta 4 and by guard-kit at load, and the bootstrap's own copy. Consumers: the eight sites above, each reading only the exit status. Its roster-holding readers: the crate test (delta 3) and the locality arm's by-name clearance (delta 6), both keyed on the name. `git grep -n "gate_staged_matches() {"` finds the one other by-name extraction, which names a different function.
- **The shell locality finding** (delta 6). Producer: `check-path-dialect` at every commit (`trigger=*`). Consumer: the committing session through the output contract. It reds a consumer's own shell file spelling `== /*`, so the release bullet below owes it.
- **The new clean-line phrase** (delta 6). Readers: `good/expect.txt`. `git grep -n "PATH-DIALECT: clean"` finds no reader outside the fixtures and the module.
- **Point 5.** No corpus narrows. The shell half widens the gate's verdict over a corpus it already reads.
- **Point 6.** The corpus the arm obliges, enumerated by the probe above: eight sites, each satisfied by the routing in deltas 1, 2, 4 and 5, and `gate_path_rooted`'s own body, satisfied by the by-name clearance. `guard-lib-parity.test.sh` sits outside the arm's corpus (`walk::tracked_shell_tree` drops `*.test.sh`), so delta 4 routes it by review and the arm does not hold it.

## Existing sections updated

Roster from the probe above, `git grep -n "path_root\|shared shell normalizer\|one POSIX sh body\|drive-rooted" -- '*.md' ':!docs'` and `git grep -n "begin with \`/\` or" guard-kit/SPEC.md`, run 2026-09-23.

- `gate-sdk/SPEC.md` §The path-dialect contract, §Porting to Rust does not retire dialect exposure, §lib/gate.sh and §check-path-dialect (delta 7).
- `guard-kit/SPEC.md` §The generic ruleset, rule 24 step (3) (delta 5).
- `installer/SPEC.md` §Implementation (delta 7).
- `gate-sdk/lib/gate.sh`, the predicate and `gate_native_bin_spelled` (delta 1).
- `installer/bin/checkwright.sh`, the twin and the `readlink` arm (delta 2).
- `native/src/walk.rs`, the parity test (delta 3).
- `gate-sdk/lib/test-hermetic.sh`, `gate-sdk/smoke/install.sh`, `installer/consumer-smoke/run-smoke.sh`, `lifecycle-kit/smoke/install.sh` and `guard-kit/gate-tests/guard-lib-parity.test.sh` (delta 4).
- `guard-kit/lib/guard.sh` and `guard-kit/guard-tests/cases.tsv` (delta 5).
- `native/src/gates/path_dialect.rs`, `gate-sdk/checks/check-path-dialect.gate` and `gate-sdk/gate-tests/check-path-dialect/` (delta 6).
- `.workflow/release-declarations.md`: a Tightened gates bullet, `check-path-dialect` now reds a shell `[[ … == /* ]]` or `case … /*)` absoluteness test outside `gate_path_rooted` (delta 6); a Behavior changes bullet, `gate_native_bin_spelled` treats a value opening with `\` as anchored, and the new library function `gate_path_rooted` (delta 1).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/SPEC.md` and `docs/installer/SPEC.md`.

**Commit split, owed to `check-gate-tamper` assertion A.** A commit touching `gate-sdk/lib/gate.sh` may co-stage only this repo's meta paths, and `installer/` is not one. So delta 2's bootstrap edit and delta 4's `run-smoke.sh` line land in a commit of their own. Delta 3's byte-twin half reds until both bodies exist, so it lands in the later of the two commits.

## Retired spellings

- None — no spelling leaves the tree; each routed site keeps its variable and its join, and the single-dialect test forms are held by delta 6's arm rather than retired by name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the predicate and the arm.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `shell-textual-absoluteness-single-dialect` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
