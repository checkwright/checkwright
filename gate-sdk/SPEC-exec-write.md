# SPEC amendment: exec-write

Pairs `exec-write-guard`. A Rust program that writes a file with execute bits is a security site: it is where an attacker who controls the bytes gets them run. A consumer that routes every such write through one reviewed helper wants a gate that reds an executable-mode write anywhere else. This amendment adds check-exec-write and the Rust code reader it needs to tell a call from a string or a comment that spells one.

**What the tree does today**, each bullet with the probe that settled it:

- check-exec-bit holds tracked files' git index modes (gate-sdk/SPEC.md §check-exec-bit); no gate reads source for a mode a program sets at run time.
- No reader in the crate separates Rust code from Rust string and comment text: the comment readers take comments by line, and `grep -rn 'r#' native/src/gates/comment_tier.rs` finds no raw-string handling.
- This repo sets execute modes from 14 Rust files, production and test code both (`git ls-files 'native/src/*.rs' | xargs grep -ln 'set_permissions\|from_mode'`), with no helper.

## What changes

### (1) The Rust code reader {design-bearing} {user-facing: the adopter input on `exec-write-guard` — calls outside strings and comments}

**Not yet applied**: a new section, §The Rust code reader, before §check-exec-write:

> `native/src/rust_code.rs` gives a Rust file's **code view**: the text with every comment (`//` to the line end, `/* */` nested) and every string, byte-string and character literal (`"…"`, `b"…"`, `r#"…"#` at any hash count, `br#"…"#`, `'…'` told from a lifetime) blanked to spaces, line breaks kept, so a position in the view is a position in the file. Integer literals stay. It gives each **call** in the view: the callee path as written (`a::b::c`, `.m`), and its argument list split at depth-zero commas, each argument's source text kept. An unterminated literal, an unterminated block comment or an unbalanced call is reported as `<path>:<line>: [LINT-PARSE] unparseable span` and makes the reading gate exit 3.

- **Why a view and not a parse.** The gate's rules are callee paths and an argument's text; a token-level view gives both, and a full Rust parser is a dependency the floor rules out (§The adopter constraints).
- **Honest limit.** Calls are read as written: a callee renamed on import (`use std::fs::copy as cp`), or built inside a macro, is invisible.

### (2) check-exec-write {design-bearing} {user-facing: the adopter input on `exec-write-guard`}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_EXEC_WRITE_PATHS` (default empty: pathspecs over the tracked tree; empty disables the gate) and `GATE_SDK_EXEC_WRITE_HELPERS` (default empty: repo-relative helper files). A new section, §check-exec-write:

> `checks/check-exec-write.gate` (`precommit`, binary-dispatched). Invariant: no Rust file but a declared helper writes a file with an execute bit. Its corpus is the tracked `.rs` files under `GATE_SDK_EXEC_WRITE_PATHS`, a path with a component in the shared prune dirs or opening with `.` skipped, minus the helper files: each helper is exempt for its own content alone, never for a file it is called from. A helper path holding `..`, absolute, or naming no file refuses at exit 2. With the paths knob empty it prints `not applicable: no Rust path configured`; it declares `# install: on-surface` and `# armed-by: GATE_SDK_EXEC_WRITE_PATHS`.
>
> **The mode test.** An argument passes when it is an integer literal (decimal, `0o`, `0x` or `0b`, `_` and a type suffix allowed) whose value has no bit of `0o111`; any other argument, a name or an expression, fails, since the gate cannot know its value.
>
> **The sites**, each read in the code view (§The Rust code reader), the mode argument named:
>
> - `from_mode(M)` and `from_raw_mode(M)`; `.mode(M)` and `<Path>::mode(…, M)`; `set_mode(…, M)` and `.set_mode(M)` — each hits when `M` fails the mode test.
> - `set_permissions(P, Q)` — hits when `Q` holds none of the calls above, a copied or computed permission set; when it holds one, that call is judged instead.
> - `chmod`, `lchmod` and `fchmod` (last argument) and `fchmodat` (third), under a `libc`, `nix` or `rustix` path; `open` (third) and `openat` (fourth) under `libc` when that argument is given, and `creat` (second) — each hits when the mode fails the mode test.
> - `fs::copy(…)` — always hits, since it copies the source's permissions.
> - `Command::new(L)` where `L` is the literal `"chmod"` or `"install"` — always hits.
> - A mode constant: an identifier `S_IXUSR`, `S_IXGRP`, `S_IXOTH`, `S_IRWXU`, `S_IRWXG`, `S_IRWXO`, `ACCESSPERMS` or `ALLPERMS` anywhere in the view — hits.
>
> **The record** is `<path>:<line>: [EXEC-WRITE] <call>`, `<call>` the callee's last path segment or the constant, one per site in file then line order. It takes no waiver: a site moves into a helper.
>
> The pair: `bad/` holds each site kind once, a non-literal mode, and a helper's call made from another file; `good/` holds a `0o644` mode, each site spelled inside a string, a raw string and a comment, and the helper's own content. `check-exec-write.test.sh` holds a `..` helper at 2, an absent helper at 2, an unterminated raw string at 3 and an unbalanced call at 3.

- **Why every non-literal mode reds.** A mode computed from input is the case the rule exists for, and a reviewed exception has a home: the helper.
- **Why the record names the call and not the argument.** An argument can be any expression; the call is what the reader fixes.
- **Honest limit.** A file made executable by a child process the gate does not name (a shell running `chmod`, say) is outside it; so is a write through a crate the site list omits.

## Producers and consumers

- **The Rust code reader.** Producer: `native/src/rust_code.rs`, with its unit tests. Consumer: check-exec-write (delta 2).
- **check-exec-write.** Producer: the battery, armed where `GATE_SDK_EXEC_WRITE_PATHS` is set. Deployed: the orchestrator adopter, over its crates with one helper *(inferred: its configuration is out of tree, lead-relayed)*. This repo binds no path: its 14 setting files would each red, and routing them through one helper is a refactor this unit does not make, so the gate is registered and not applicable here, and `doctor` names it disarmed. Consumers: the committing session and the SARIF log.
- **Roster readers of the new member:** `gates::REGISTRY`, `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity`, `scripts/gates.list`, and the knob rows in `native/src/knobs/gate_sdk.rs`.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration, rule rosters — the two knobs (delta 2).
- gate-sdk/SPEC.md §The Rust code reader, new (delta 1).
- gate-sdk/SPEC.md §check-exec-write, new (delta 2).
- `native/src/rust_code.rs` (delta 1); `native/src/gates/exec_write.rs`, `checks/check-exec-write.gate`, its pair and test, `native/src/knobs/gate_sdk.rs`, `gates::REGISTRY`, gate-sdk/README.md's roster block and `scripts/gates.list` (delta 2).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — a new gate, a new reader and two knobs; nothing existing is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
