# SPEC amendment: binary-strings

Pairs `binary-strings-scan`. A built artifact carries strings its source never spelled: the builder's home path, a dependency's registry path, a URL a macro expanded, a project name a build script embedded. A consumer shipping binaries wants each one checked against its private deny terms and a set of leak shapes before it ships, with a record that locates a hit and never prints it. This amendment adds check-binary-strings, an ELF reader for the sections it scans, and the shapes list.

**What the tree does today**, each bullet with the probe that settled it:

- `bin/build-native.sh` matches the gate binary it builds against the banned-pattern set and prints the matched text (gate-sdk/SPEC.md §build-native, "The artifact is verified"). It reads the whole file, covers this repo's own artifact only and is no registered gate.
- check-tree-terms reads a tracked binary's decoded text whole and prints its path alone (gate-sdk/SPEC.md §check-tree-terms); a build output is untracked, so it is outside that corpus.
- The crate has no ELF reader (`git grep -n -i 'elf64\|e_shoff\|SHT_NOBITS' -- native/src` finds none).

## What changes

### (1) The corpus and the reader {design-bearing} {user-facing: the adopter input on `binary-strings-scan`}

**Not yet applied**: §Layout and configuration's rule rosters gain:

> - `GATE_SDK_STRINGS_FILES` (default empty): space-separated build-output paths for check-binary-strings, each named exactly, never globbed or discovered. Empty disables the gate.
> - `GATE_SDK_STRINGS_SHAPES` (default `<gates-dir>/strings-shapes.list`): space-separated ERE shape lists for the same gate, each **optional**.

**Not yet applied**: a new section, §check-binary-strings, after §check-path-names, opens:

> `checks/check-binary-strings.gate` (`precommit`, binary-dispatched). Invariant: no named build output carries a deny term or a configured leak shape in a string it would ship. Its corpus is `GATE_SDK_STRINGS_FILES`, each path read from the repository root; a named path that is absent refuses at exit 2 and one present but unreadable is exit 3. With the knob empty it prints `not applicable: no build output named` and exits 0, and it declares `# install: on-surface` and `# armed-by: GATE_SDK_STRINGS_FILES`.
>
> **A string is a run of 4 or more printable ASCII bytes** (0x20 to 0x7E, and tab), the `strings(1)` default. **In a 64-bit little-endian ELF file** only the bytes of sections that occupy the file (any type but `SHT_NOBITS`) and do not carry `SHF_EXECINSTR` are read, each section's runs read alone; any other file is read whole. An ELF64 little-endian file whose header or section table does not parse, or points outside the file, is exit 3, unparseable input. The reader is `native/src/elf.rs`, section offsets and flags only, with a unit test over a constructed image.

- **Why sections and not the whole file.** Code bytes form printable runs by accident, and a run that is an instruction stream is no string the binary ships. Other formats fall back to the whole file, which over-reports and never under-reports.
- **Why the paths are named, never discovered.** Which outputs ship is the consumer's build layout, and a glob over a target directory reads intermediates that never ship.
- **Honest limit.** Mach-O and PE files are read whole, and a string shorter than 4 bytes or held in a code section is not read.

### (2) The two hit kinds {design-bearing} {user-facing: the adopter input on `binary-strings-scan`}

**Not yet applied**: §check-binary-strings continues:

> **A term hit.** Each string is split into tokens at every byte outside `[A-Za-z0-9._-]`, and a token equal to a deny term, ASCII case folded, hits (§The deny-term list, read through `GATE_SDK_DENY_TERMS`).
>
> **A shape hit.** Each live line of a shape list is a POSIX ERE (§The POSIX ERE matcher), compiled once, with `#` lines skipped and `# rule:` lines read as §The deny-term list states. A string matching one hits. The kit ships `templates/strings-shapes.list` as a starting roster, on `templates/msg-patterns.list`'s model: a home-shaped path (`/home/`, `/root/`, `/Users/`), a cargo-registry path, and any `<scheme>://` URL. A GNU escape in a shape list is refused at exit 2, as check-tree-terms refuses it.

- **Why the shapes are a list and not built in.** Which URLs a binary may carry is the consumer's call: a binary linking its own documentation site would red on a built-in URL shape with no way to narrow it (the policy-as-choice rule).
- **Why tokens equal rather than contain.** A term inside a longer identifier is a different word, and the token split keeps a path's or a symbol's parts apart.

### (3) The record {design-bearing} {user-facing: the entry's deliverable — a location and a rule id, never a term}

**Not yet applied**: §check-binary-strings continues:

> **The record is `<path>@<offset>: [<ID>] <list> <position>`**, one per hit. `<offset>` is the decimal file offset of the hit's first byte, `<ID>` the `# rule:` id in force above the term or shape, else `STRINGS-DENIED`, `<list>` is `term` or `shape`, and `<position>` the term's or shape's position (§The deny-term list). The record never prints the string, the term or the shape. Records are ordered by path in knob order, then offset.
>
> Its fixture pair carries small constructed files: `bad/` an ELF64 image with a term in a data section, a shape in a read-only section, and a non-ELF file with a term; `good/` the same terms inside a section flagged executable and inside a longer token, and a run of 3 bytes. `check-binary-strings.test.sh` holds the residue: an absent named file at 2, a truncated ELF header at 3, a GNU escape at 2, and the record order.

- **Why the offset.** A binary has no lines, and §run-gates places `<path>@<offset>:` at a byte region (gate-sdk/SPEC-rule-class.md delta 2). The offset sits in the location prefix, so a rebuild that moves it leaves the result's identity alone, and the position after the id tells two terms of one id apart.

## Producers and consumers

- **The verdict.** Producer: check-binary-strings in the battery, armed where `GATE_SDK_STRINGS_FILES` names an output. Deployed: the orchestrator adopter, over the binaries its build ships *(inferred: its configuration is out of tree, lead-relayed)*. This repo names no output, since `bin/build-native.sh` already verifies its one artifact at build time, so the gate is registered and not applicable here and `doctor` names it disarmed. Consumers: the committing session, the SARIF log through the byte-offset placement, and the outside engine diffing a rerun *(inferred, as above)*.
- **The shapes list.** Producer: the consumer's copy of the starter roster. Consumer: this gate alone. Its self-exemption needs no rule, since the gate reads named outputs, never tracked text; check-tree-terms reads a tracked shapes list as text, so the starter under `templates/` joins its self-exempt set the way `templates/msg-patterns.list` is.
- **Roster readers of a new member:** `gates::REGISTRY`, `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity`, and `scripts/gates.list`. A new template is read by the payload and `init`'s config-seam plan, which copies a kit template into a consumer's gates dir (`grep -n 'config_seam_plan' native/src/*.rs native/src/*/*.rs`).

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration, rule rosters — the two knobs (delta 1).
- gate-sdk/SPEC.md §check-binary-strings, new (deltas 1, 2 and 3).
- gate-sdk/SPEC.md §check-tree-terms — the shapes starter joins the self-exempt starter rosters (delta 2).
- `native/src/elf.rs`, `native/src/gates/binary_strings.rs`, `native/src/knobs/gate_sdk.rs`, `checks/check-binary-strings.gate`, its pair and test, `templates/strings-shapes.list`, `gates::REGISTRY`, gate-sdk/README.md's roster block and `scripts/gates.list` (deltas 1, 2 and 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — a new gate, two knobs and a template; nothing existing is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
