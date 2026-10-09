# SPEC amendment: tracked-listing

Git's line-form listing quotes a member whose name carries a non-ASCII byte, a double quote or a backslash, and most of the crate's listing readers take the quoted spelling as a path. Run over a scratch repository tracking two such names, `check-docs-collapsible`, `check-docs-render-fidelity` and `check-docs-liquid-parse` each reported one page of two and exited clean, and `check-root-tiering` reported a top-level entry named `"docs`. One reader, the one `check-portability-floor` and `check-harness-literal` share, lists NUL-terminated. This amendment makes that the crate's one listing rule.

## What changes

### (1) One listing rule and one shared reader

The NUL-form rule moves from gate-sdk/SPEC.md §check-portability-floor to §Fail-closed contract as the crate's rule for every tracked-set listing, and the reader behind it moves out of that member's module into shared crate code {design-bearing} {user-facing: the entry's Deliverable, every listing reader on the NUL-terminated form}.

- **The rule.** A listing whose output is read is taken NUL-terminated, since git's line form quotes a name carrying a non-ASCII byte, a double quote or a backslash, and a quoted spelling opens no file, matches no glob and compares equal to no walked name. The failure is silent in the common shape: the member drops out of the corpus and the verdict reads clean.
- **One reader where the caller's shape allows.** A caller listing tracked names under pathspecs, with or without a repository directory, calls the shared reader. A caller the reader cannot serve keeps its own argv on the NUL form: stage records, the untracked set, a top-relative listing.
- **A probe is outside the rule.** An `--error-unmatch` call reads a status and no output.
- **A caller keeps its failure direction.** The reader reports a listing git could not produce, and each caller decides as it did whether that refuses or degrades. The move changes how a name is read, never what a failed listing means to its caller.
- **The reader is a module of its own beside `walk.rs`**, never a gate's module, so a member's derived couple names it and no member imports another member's file for it.

§check-portability-floor's sentence becomes a pointer. **Not yet applied.**

> The listing is the crate's NUL-terminated one (§Fail-closed contract).

§check-harness-literal's sentence on what the module imports drops the corpus enumeration, which is no longer that member's. **Not yet applied.**

> The module imports that member's valve window and binary skip, and lists its corpus through the shared reader.

### (2) The sites the shared reader serves move onto it

Each site below calls the shared reader in place of its own line-form argv and line split {mechanical}. Probe: a census of every quoted `ls-files` argv token in `native/src` and `native/build.rs`, each site then read for how its output is consumed. Every one splits git's line output as it comes and unquotes nothing; the four members named at the head were also run.

- **A listed name is opened**: `emit/always_loaded.rs`, `gates/amendment_retired_spelling.rs`, `gates/consumer_value_literal.rs`, `gates/docs_cname_parity.rs`, `gates/docs_collapsible.rs`, `gates/docs_highlight_coverage.rs` (one call per glob, kept), `gates/docs_liquid_parse.rs` (both listings, which are compared with each other), `gates/docs_render_fidelity.rs`, `gates/door_binding.rs`, `gates/guard_registration.rs`, `gates/kit_ref_liveness.rs`, `gates/tree_terms.rs`, and `walk.rs`' `tracked_matching`, whose callers read the shell tree and the suites.
- **A listed name is matched or printed**: `emit/enum_sets.rs`, `gates/root_tiering.rs`, `gates/template_registry_parity.rs`, `spec.rs`' `workflow_tier`.

### (3) The sites the shared reader cannot serve take the NUL form in place

Each site below keeps its own argv and gains the NUL flag and a NUL split {design-bearing}.

- **Stage records.** `gates/exec_bit.rs` and `emit/pack_installer.rs` read `-s` records, the name being the text after the tab. `check-exec-bit`'s argument mode still reads a canned dump as lines, a fixture format (§check-exec-bit), so its record reader takes either terminator.
- **A shape of its own.** `queue.rs` lists top-relative under `--full-name`; `gates/reads_couples.rs` hands its listing as text to a function its tests also feed canned text; `emit/kpi/amendment_age.rs` lists through a helper it shares with its history reads.
- **Emptiness alone is read.** `gates/kit_enum.rs`, `gates/kit_registration.rs`, `gates/md_refs.rs`' directory probe, `gates/tracking_claim.rs`, `gates/workflow_tiering.rs`, `walk.rs`' `authoring_tree` and `gates/crate_arms.rs`' untracked-set read. No name is used, so no verdict moves; they take the flag so the holder needs no class for them.

### (4) A crate test holds the form

A crate unit test reds a quoted `ls-files` argv token in the crate's source whose argv carries neither the NUL flag nor `--error-unmatch`, outside a roster the test itself carries, each row a file and its reason {design-bearing}. The roster's rows:

- **The source stamp**, `fresh.rs` and `native/build.rs`. It is one derivation in four holders, two of them shell (§check-gate-binary-fresh), and each writes a path into the stamp as listed, so moving two holders alone would break their equality on the names the move exists for. Its corpus is the crate's own files, and git's hash read refuses a quoted path (`could not open`, exit 128), so on such a name every holder refuses rather than stamps.
- **The hook-environment test** in `emit/git_hook.rs`, which lists in line form to reproduce a hook's own read.
- **The read-only subcommand roster** in `emit/scan_prompts.rs`, a word list and no argv.

**Honest limit:** the test reads source text, so an argv assembled across functions, or a listing reached through a helper that hides the token, passes it. It is a unit test rather than a gate because its subject is the crate, which §check-crate-arms' test arm already runs on every commit that touches it.

### (5) A crate case lists the names the rule exists for

A crate test builds a scratch repository tracking a name with a non-ASCII character on every platform, and on Unix one with a double quote and one with a backslash, lists it through the shared reader and asserts each name comes back as written {mechanical}. The two bespoke suites' case for a member under a name git quotes stays (§check-portability-floor, §check-harness-literal).

### (6) A name that is not UTF-8 fails closed

The shared reader refuses a listing holding a member whose name is not valid UTF-8, and §Fail-closed contract's listing rule states it {design-bearing} {user-facing: the lead's decision recorded on `tracked-name-lossy-decode`, a non-UTF-8 name fails closed}.

- **The refusal.** Exit 2 through the calling member, naming the member in its lossy spelling and saying that its name is not UTF-8. Today the reader decodes lossily, so the member is opened under a replacement-character spelling: `check-harness-literal` exits 2 naming a path that does not exist, and would scan another file where that spelling happens to name one.
- **Carrying the name's bytes is refused.** The crate's directory lister already refuses such a name, so a member listing by git and walking the same tree would read the name on one path and refuse it on the other; `check-portability-floor` and `check-md-refs` exit 2 there today. One answer for a tree holding such a name is worth more than a reader that opens what the walk beside it will not.
- **It is the reader's own class, never a failed listing.** Delta 1 leaves a caller its direction on a listing git could not produce. This refusal is not that: a caller that degrades a failed listing to an empty corpus would turn it into a clean verdict over members it could not name. So the three callers of delta 2 with a degrading direction, `walk.rs`' `tracked_matching`, `emit/enum_sets.rs` and `spec.rs`' `workflow_tier`, gain a refusing path for this class alone.
- **Honest limit.** A reader that keeps an argv of its own, delta 3's and the ones already on the NUL form outside the shared reader, keeps its lossy decode.
- **The case** is a crate test compiled for Linux alone, a host whose filesystem admits such a name: it tracks one in a scratch repository and asserts the refusal and its text. The battery's host and the Linux legs run it.

§check-portability-floor's pointer sentence of delta 1 then reads. **Not yet applied.**

> The listing is the crate's NUL-terminated one, which refuses a member whose name is not UTF-8 (§Fail-closed contract).

## Producers and consumers

- **The shared reader.** Producer of a name list: one `git ls-files -z` spawn through the crate's spawn site. Consumers: the sites of delta 2, each at the point it enumerates its corpus, and the two members that already call it.
- **Verdicts that move, by red condition (point 5 is not binding, since the corpus widens: a member dropped before is now read).** A gate that opens listed names can newly red on a finding inside a page it never read: the site-kit docs members (site-kit/SPEC.md §check-docs-render-fidelity, §check-docs-liquid-parse, §check-docs-collapsible and their siblings in delta 2), `check-tree-terms`, `check-kit-ref-liveness`, `check-guard-registration` (guard-kit/SPEC.md §check-guard-registration), `check-amendment-retired-spelling` (canon-kit/SPEC.md §check-amendment-retired-spelling) and the always-loaded meter (context-kit/SPEC.md §The always-loaded meter). `check-root-tiering` stops redding a quoted first component. Each of those sections says its corpus is the tracked files, which the move makes true, so none is reworded.
- **Readers of a count.** A clean line's page or file count rises by the members now read. Probe for a reader holding such a count: `git grep -n "tracked markdown page" -- '*expect.txt'`; a fixture tree tracks no quoted name, so no expectation moves.
- **Every member's satisfying value (point 6).** The obliged corpus is the census's thirty-four argv tokens outside the twelve already on the NUL form and the six probes. Deltas 2 and 3 name each site's value and delta 4 the four tokens that keep the line form with their reasons.
- **The non-UTF-8 refusal.** Producer: the shared reader, reached wherever a tree tracks such a name under a listed pathspec. Consumer: the calling member's exit 2, read by the committing session and the battery. Every member of delta 2 gains it, where git's octal-quoted line form had it drop the member unread.
- **The release declaration.** Producer: the landing session. §upgrade-smoke holds an upgraded consumer's red set to the gates-section declaration, so a gates-section bullet names the members of delta 2, each of which can red on a page it never read, or exit 2 on a name that is not UTF-8, in an adopter's tree tracking one.

## Existing sections updated

- `gate-sdk/SPEC.md` §Fail-closed contract — the rule and its refusal (deltas 1 and 6).
- `gate-sdk/SPEC.md` §check-portability-floor, §check-harness-literal — the pointer and the import sentence (deltas 1 and 6).
- `gate-sdk/SPEC.md` §check-exec-bit — the live read's form beside the dump's (delta 3).
- `native/src/gates/portability_floor.rs`, `native/src/gates/harness_literal.rs` — the reader leaves the first and the second imports the shared one (delta 1).
- the crate modules deltas 2 and 3 name (deltas 2 and 3).
- `.workflow/release-declarations.md` — the gates-section bullet (deltas 1 and 6).
- `docs/gate-sdk/SPEC.md` — generated, stale once delta 1 lands (all deltas).

## Retired spellings

- None — the reader moves modules and no governed name is removed or renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Every reached suite re-run** — the fixture suite of every kit a moved module backs, which the crate-arms fixture arm runs.
- [ ] **The probe re-run** — the four members named at the head read both pages, and no entry named `"docs`, over a scratch repository tracking such names.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Entries moved** — `listing-line-form-readers` and `tracked-name-lossy-decode` move to Done in the commit merging the last of this amendment's deltas, which lands before the drain stage.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
