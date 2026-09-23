# SPEC amendment: ps1-ascii

installer/SPEC.md §Requirements rules that a shipped PowerShell file's code stays ASCII outside its comments. Windows PowerShell 5.1 reads a BOM-less script in the host's ANSI code page, so a UTF-8 em-dash inside a string literal decodes to a closing double quote and the parse collapses around it; a BOM would break the `#!/usr/bin/env pwsh` line npm's shim reads. Nothing local holds the rule. The one enforcement is the `install-smoke-pwsh-windows` leg, so a regression costs a watched push to find.

**The ruling: an ASCII arm in `check-portability-floor`, beside its roster and not in it.** The gate already owns the install-path corpus (`GATE_SDK_PORTABILITY_PATHS`) and its tracked-only enumeration, and the corpus is exactly what an adopter's host executes. The roster cannot carry the rule: it is one POSIX ERE per line with no comment awareness and no vocabulary for a non-ASCII byte. The arm reads bytes, not the lossy-decoded text, and it runs over the corpus members whose path ends in `.ps1`.

**What counts as a comment is the honest, decidable slice.** A line whose first non-blank character is `#` is a comment, which covers the `#!` line too. Anything else is code. A trailing `# …` after code is not exempted, because a `#` can sit inside a string and telling the two apart needs a PowerShell lexer the crate does not carry; that over-refuses a non-ASCII trailing comment, whose remedy is to move it onto its own line, and never under-refuses code. A `<# … #>` block comment is not recognized either, on the same over-refusal side. A here-string is tracked, because a `#`-leading line inside one is string content and exempting it would under-refuse: a line ending in `@"` or `@'` opens one, and a line opening with `"@` or `'@` closes it. A BOM is three non-ASCII bytes on line 1 before the `#`, so it reds with no clause of its own, which matches the installer's BOM ruling.

**No valve.** The `# portability-declared:` valve admits a construct a supported host tolerates behind a declaration. A non-ASCII byte in 5.1-read code is a parse failure on the host the rule exists for, so there is nothing to declare; the remedy is the one `gate-sdk/bin/run-gates.ps1` already takes, spelling the character by code point (`[char]0x2014`).

**This repo's corpus widens to the PowerShell front end.** `gate-sdk/bin/run-gates.ps1` runs under Windows PowerShell 5.1 on the same ground as the bootstrap (§run-gates), ships in the payload, and is held today only by `--run-front-end-parity` at push. It sits outside this repo's binding of `GATE_SDK_PORTABILITY_PATHS`.

**Measured at authoring (2026-09-23).**

- `git ls-files '*.ps1'` lists two shipped files (`installer/bin/checkwright.ps1`, `gate-sdk/bin/run-gates.ps1`) and six fixture copies. `git grep -n -P '[^\x00-\x7F]' -- '*.ps1'` finds 24 lines, 12 in each shipped file and none in a fixture, and every one is a full-line comment, two of them indented. No tracked `.ps1` carries a BOM, a here-string or a block comment. So the arm lands green on both shipped files.
- With `GATE_SDK_PORTABILITY_PATHS` extended by `gate-sdk/bin/run-gates.ps1`, the gate prints `PORTABILITY-FLOOR: clean (5 install-path file(s) scanned under 4 configured pathspec(s), 0 binary member(s) skipped; none uses one of the 23 banned construct(s) undeclared)`, so the shell-construct roster does not misfire on the PowerShell file.

## What changes

### (1) The ASCII arm {design-bearing}

**Not yet applied.** `native/src/gates/portability_floor.rs` gains a second arm over the enumerated corpus: for each member whose path ends in `.ps1` and which is not skipped as binary, every line carrying a byte above `0x7F` is a finding unless the line is a full-line comment outside a here-string, by the rules above. It reads the member's raw bytes. The finding reports `path:lineno:line` followed by `    non-ASCII outside a comment`, and the help block names the Windows PowerShell 5.1 ANSI-code-page ground and the code-point remedy.

The arm needs no roster, so an empty pattern set disables the roster arm alone. The early return at the empty-pattern branch becomes a skip of the roster loop, and the enumeration runs whenever the corpus is non-empty. An empty corpus still disables both arms and says so, which keeps the member's `# armed-by: GATE_SDK_PORTABILITY_PATHS` declaration true of both arms. The clean line gains the count of `.ps1` members checked, so a corpus holding none reads differently from one whose PowerShell members passed. With an empty roster it names the roster as unchecked and the ASCII arm as run.

No knob, no `couples=` change and no install-disposition change: the arm reads the corpus knob the member already declares, and `trigger=*` already fires it on every commit.

### (2) The fixture pair and the bespoke test hold the arm {design-bearing}

**Not yet applied.** `good/tree/` gains a `.ps1` whose non-ASCII sits only in full-line comments, one of them indented, beside ASCII code, and `good/expect.txt`'s pinned clean line moves with the file count and the new count. `bad/tree/` gains a `.ps1` carrying an em-dash inside a string literal, a second in a trailing comment after code, which pins the over-refusal, and a `#`-leading line inside a here-string that carries one; `bad/expect.txt` gains their records. `check-portability-floor.test.sh`'s record-order list gains the new paths, and its empty-roster case asserts that the ASCII arm still runs.

### (3) This repo's corpus binds the PowerShell front end {mechanical}

**Not yet applied.** In `scripts/gate-sdk-config.knobs`, `GATE_SDK_PORTABILITY_PATHS` gains `gate-sdk/bin/run-gates.ps1`.

### (4) §check-portability-floor states the arm {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-portability-floor:

The invariant sentence "Invariant: no file on the configured **install-path corpus** matches a configured banned-construct pattern, except at a site carrying a `# portability-declared: <reason>` valve." becomes:

> Invariant: no file on the configured **install-path corpus** matches a configured banned-construct pattern, except at a site carrying a `# portability-declared: <reason>` valve, and no PowerShell member of it (`*.ps1`) carries a non-ASCII byte outside a full-line comment.

After the paragraph opening "**What a green run buys**", add:

> **The ASCII arm is a language fact, not a roster.** Windows PowerShell 5.1 reads a BOM-less script in the host's ANSI code page, so a non-ASCII byte in code changes what the script says and, inside a string, where the string ends (installer/SPEC.md §Requirements). The arm reads each `.ps1` member's raw bytes and reds a line carrying one unless the line is a full-line `#` comment outside a here-string. A trailing comment after code and a `<# … #>` block are read as code, since telling a `#` in a comment from one in a string needs a lexer the crate does not carry; that over-refuses, and never under-refuses. A BOM reds on line 1. The arm takes no valve: the byte is a parse failure on the host the rule exists for, and the remedy is spelling the character by code point.

In the paragraph opening "**Both the vocabulary and the corpus are consumer config**", after "an absent pattern file or an empty corpus makes the gate assert nothing and exit clean", insert: "— an absent or empty roster disables the roster arm alone, since the ASCII arm needs none, while an empty corpus disables both —".

In the paragraph opening "**This is the provenance-seam ruling for the member**", "**Kit mechanism** is the scanner, the valve grammar, the resolution and the two knobs" becomes "**Kit mechanism** is the scanner, the ASCII arm, the valve grammar, the resolution and the two knobs".

In the fixture paragraph, `good/` gains "a PowerShell member whose non-ASCII sits only in full-line comments", `bad/` gains "a PowerShell member carrying non-ASCII in a string, in a trailing comment and inside a here-string", and the bespoke test's list gains "the ASCII arm running under an empty roster".

### (5) The installer's rule points at its gate {mechanical}

**Not yet applied.** In installer/SPEC.md §Requirements, **The install blocks** bullet, after "…and a BOM would break the `#!/usr/bin/env pwsh` line npm's shim reads.", add: "`check-portability-floor`'s ASCII arm holds it at commit over every `.ps1` on the configured install-path corpus (gate-sdk/SPEC.md §check-portability-floor)."

## Producers and consumers

- **The ASCII finding** (delta 1). Producer: the gate's corpus enumeration, whose enabling configuration (`GATE_SDK_PORTABILITY_PATHS`) this repo sets and delta 3 widens, so the arm is live here and not only in fixtures. Consumers: the committing session through the output contract, on the generated pre-commit hook, `run-gates.sh` and CI, and `--run-gate-tests` through the fixture pair and the bespoke test.
- **The clean line's new count** (delta 1). Readers: `good/expect.txt` and `check-portability-floor.test.sh`'s substring assertions (delta 2). `git grep -n "PORTABILITY-FLOOR"` finds no other reader of the line.
- **Point 5.** Not reached: the corpus does not narrow. The empty-roster branch widens from asserting nothing to running the ASCII arm, so a consumer with an empty roster and a corpus holding a `.ps1` can newly red. That is the release bullet below.
- **Point 6.** The corpus's `.ps1` members on this tree: `installer/bin/checkwright.ps1` and, after delta 3, `gate-sdk/bin/run-gates.ps1`. Each one's satisfying value is its current text, which is non-ASCII only in full-line comments (measured above).

## Existing sections updated

Roster from `git grep -n "portability" gate-sdk/SPEC.md installer/SPEC.md`, `git grep -n "stays ASCII" -- '*.md'` and `git grep -n "GATE_SDK_PORTABILITY_PATHS" -- scripts`, run 2026-09-23.

- `gate-sdk/SPEC.md` §check-portability-floor, the invariant, the ASCII-arm paragraph, the disablement paragraph, the provenance ruling and the fixture paragraph (delta 4).
- `installer/SPEC.md` §Requirements, **The install blocks** (delta 5).
- `native/src/gates/portability_floor.rs`, the arm and the restructured empty-roster branch (delta 1).
- `gate-sdk/gate-tests/check-portability-floor/` both cases, and `gate-sdk/gate-tests/check-portability-floor.test.sh` (delta 2).
- `scripts/gate-sdk-config.knobs`, `GATE_SDK_PORTABILITY_PATHS` (delta 3).
- `.workflow/release-declarations.md`, a Tightened gates bullet: `check-portability-floor` now reds a non-ASCII byte outside a full-line comment in any `.ps1` on your configured install-path corpus, even with an empty construct roster (delta 1).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`.

## Retired spellings

- None — no delta retires a spelling; the empty-roster clean line's wording is rewritten in the module and its fixtures together.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the new arm and the new count.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC addition re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `ps1-ascii-unheld-at-commit` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
