# SPEC amendment: smoke-registry

Each kit's `smoke/install.sh` writes that kit's slice of the scratch consumer's registry as a heredoc, and the heredoc carries `# unregistered:` declarations for the members it leaves out. `check-gate-substrate-parity` assertion I reads the assembled `scripts/gates.list` only inside the scratch consumer, so in the authoring repo nothing reads the heredocs. A kit-owned subcommand added to the binary without a line there is found three validate suites later, as an ACT 1 hook failure in `demo`, `consumer_smoke` and `agents_md_smoke`, which is what happened to `check-action-job-ref`. docs/site-architecture.md already rosters the site as hand-maintained, so this is enforcement of a rule already written.

**The ruling: a tenth assertion, J, in `check-gate-substrate-parity`, holding each kit's smoke registry to that kit's owned subcommands at commit.** The gate already owns the roster read (`gates::names_with_owners`), the `# unregistered:` reader and the pure comparison assertion I runs (`registration_parity`). J runs that comparison once per kit, with the kit's smoke heredoc standing in for the registration file and the roster narrowed to the kit's own members. `check-install-disposition` assertion B also reads each kit's smoke script, and it stays: it holds the `zero-config` members to the smoke by a whole-line match anywhere in the file, which is why an `on-surface` member such as `check-action-job-ref` passed it. J is the complete, owner-scoped form, and a `zero-config` member declared `# unregistered:` would still red B.

**Scoped to the publishing tree.** The payload withholds `smoke/` (§Consumer payload), so an adopter normally has no smoke script. One who vendored by copy would be asserting the kit author's file against a shared binary that may be skewed from it, a red only the kit author can clear. Assertion G's tree half is scoped the same way for the same reason, through the same predicate. Outside a publishing tree J reads nothing and says so on the clean line.

**Measured at authoring (2026-09-23).** A probe script run through `checkwright-gates --scratch-run` found the heredoc whose redirect target ends in `gates.list` in each `*/smoke/install.sh`, split its body into members (non-comment, non-blank lines) and `# unregistered:` names, and compared the union with `checkwright-gates --list`'s owner column:

| Kit | Heredocs | Owned | Registered | Declared | Missing | Foreign | Both |
|---|---|---|---|---|---|---|---|
| canon-kit | 1 | 31 | 30 | 1 | none | none | none |
| context-kit | 1 | 6 | 4 | 2 | none | none | none |
| delegation-kit | 1 | 3 | 3 | 0 | none | none | none |
| doctrine-kit | 1 | 1 | 1 | 0 | none | none | none |
| drift-kit | 0 | 0 | 0 | 0 | none | none | none |
| evidence-kit | 1 | 4 | 2 | 2 | none | none | none |
| gate-sdk | 1 | 42 | 38 | 4 | none | none | none |
| guard-kit | 1 | 1 | 1 | 0 | none | none | none |
| lifecycle-kit | 1 | 15 | 11 | 4 | none | none | none |
| queue-kit | 1 | 11 | 11 | 0 | none | none | none |
| site-kit | 1 | 3 | 3 | 0 | none | none | none |

So J lands green. Every registry heredoc opens `cat > scripts/gates.list <<'EOF'` (gate-sdk) or `cat >> scripts/gates.list <<'EOF'` (the rest) at column 0 and closes on `EOF` at column 0, with its `# unregistered:` lines inside the body. gate-sdk's other writes to a `gates.list` are `printf` redirects into scratch directories, not heredocs. The `# smoke-unregistered:` lines sit outside the heredoc as shell comments, a different roster that J does not read.

## What changes

### (1) Assertion J {design-bearing}

**Not yet applied.** In `native/src/gates/gate_substrate_parity.rs`, for each kit root K that `gate_kit_roots` yields and whose `smoke/install.sh` exists, in a publishing tree:

- **Find the registry.** A line opening, at column 0, with `cat` and a `>` or `>>` redirect whose target's basename is `gates.list`, followed by a `<<` heredoc operator with a quoted or bare delimiter, opens the registry body. The body runs to the line equal to the delimiter. A small reader for this lives beside `registry::members`. `guard.rs`' heredoc extents and `bashscan.rs` report no redirect target, so neither serves.
- **Read the body** with `registry::members` and `registry::unregistered_declarations`, both unchanged.
- **Compare** through `registration_parity`, with the kit's owned members of `gates::names_with_owners()` as the roster and `[K]` as the vendored-kit set. The forward arm, both stale-declaration shapes and the missing-field shape come unchanged. The reverse arm reds a body member K does not own, which §Consumer smoke's rule that no leg registers another kit's member already requires; its message names the smoke script rather than the registration file.
- **Red** when K owns at least one subcommand and its smoke script carries no registry heredoc, or carries two. A kit owning none with no heredoc is clean, which is drift-kit's case.

The clean line gains J's counts: kits read, owned members covered as registered and declared, and kits with no smoke script. Outside a publishing tree it reports J out of scope, a counted zero rather than silence.

No manifest change is owed. `couples=` already carries `kit:*.sh`, whose `*` crosses `/` and so covers `<kit>/smoke/install.sh`, and `native/*`, which covers a new registry row (§Reading a `couples=` field's reach).

### (2) J's configurations are held in the bespoke test and the unit tests {design-bearing}

**Not yet applied.** Neither fixture case is a publishing tree, so the pair cannot reach J. That is the reason assertion G's tree half is proved in `check-gate-substrate-parity.test.sh`, and J goes there on the same ground. The test's publishing sandbox gains a kit root with a smoke script covering its owned members, and variants that omit one, register a foreign member, carry a stale declaration, carry no heredoc, and carry two. The same tree with no tracked crate source shows J out of scope. The forward arm's whole-roster configurations also join the crate unit tests beside assertion I's, against the pure function.

### (3) §check-gate-substrate-parity states J {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-gate-substrate-parity, the usage paragraph's "Eight assertions: (A) … and (I) the crate's dispatch roster joined to the battery's registration." becomes "Ten assertions: (A) … (I) the crate's dispatch roster joined to the battery's registration; and (J) each kit's smoke registry joined to the kit's owned subcommands." The count was already wrong, since nine were listed.

After the assertion I bullet, add:

> - **assertion J — each kit's smoke registry joined to the kit's owned subcommands.** Assertion I reads a registration file, and in the authoring tree that file is this repo's own; the per-kit registries that assemble a scratch consumer's file are heredocs in each kit's `smoke/install.sh`, which I reads only second-hand, inside the scratch consumer, three validate suites from the edit that broke one. J reads each kit's heredoc directly — the one whose redirect target's basename is `gates.list` — and runs assertion I's comparison against that kit's owned members, so an owned subcommand the heredoc neither registers nor declares `# unregistered:` reds at commit. A heredoc member the kit does not own reds too, since no leg registers another kit's member (§Consumer smoke), and so do I's two stale-declaration shapes. A kit owning subcommands with no registry heredoc, or with two, is a finding; a kit owning none needs none. **Scoped to the publishing tree**, on assertion G's ground: the payload withholds `smoke/`, and a vendored copy would hold the kit author's file against a binary it may be skewed from. **Its coverage is the bespoke test and the unit tests**, for the reason G's tree half is: neither fixture case publishes. It costs no manifest edit, since `kit:*.sh` already covers the smoke scripts. `check-install-disposition` assertion B reads the same scripts for `zero-config` members only, and stays.

In the paragraph opening "Its coverage is split across three oracles", after the sentence ending "…for exactly the reason assertion B's matrix does.", add: "Assertion J is out of the pair's reach entirely, since neither case publishes, and lives in the bespoke test and the unit tests."

### (4) The declaration's readers and the roster row say J reads them {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §Layout and configuration, the `gates.list` bullet's "and §check-gate-substrate-parity's assertion I is its reader" becomes "and §check-gate-substrate-parity's assertions I and J are its readers — I in the consumer's registry, J in each kit's smoke registry heredoc".

In gate-sdk/SPEC.md §Consumer smoke, the `smoke/install.sh (required)` bullet, after "held so by `check-install-disposition` assertion B.", add: "Its registry heredoc is held to the kit's owned subcommands, each registered or declared `# unregistered:` inside the heredoc, by `check-gate-substrate-parity` assertion J."

In gate-sdk/SPEC.md §Consumer smoke, **The declaration valve** paragraph, "**This grammar now has a sibling on a second roster** — `# unregistered:` in the consumer's own `gates.list` (§Layout and configuration, read by §check-gate-substrate-parity's assertion I)" becomes "**This grammar now has a sibling on a second roster** — `# unregistered:` in the consumer's own `gates.list` (§Layout and configuration, read by §check-gate-substrate-parity's assertions I and J — I in the consumer's registry, J in each kit's smoke registry heredoc)". The two `# unregistered:` sites (the consumer's assembled file, and the kit's own heredoc that assembles it) are one grammar with two readers now, exactly as delta 4's `gates.list` bullet edit above states for the first site.

In docs/site-architecture.md, the new-gate fan-out row "the owning kit's `smoke/install.sh` expected-gate roster, hand-maintained, so a new gate is added there or carries a `smoke-unregistered:` declaration (`gate-sdk/SPEC.md` §Consumer smoke owns which);" becomes "the owning kit's `smoke/install.sh` registry heredoc, where a new gate is registered or declared `# unregistered:` (gated by `check-gate-substrate-parity` assertion J), and an omitted one may also owe a `# smoke-unregistered:` declaration (`gate-sdk/SPEC.md` §Consumer smoke owns which);". The row named the wrong grammar before: an owned member left out of the heredoc always needs `# unregistered:` inside it, and only sometimes the accounting valve.

## Producers and consumers

- **J's finding** (delta 1). Producer: the gate at every commit touching a kit `*.sh` or the crate, in a publishing tree, which this repo is (assertion F's publishing test reads its tracked crate source). Consumer: the committing session through the output contract.
- **J's clean-line counts** (delta 1). Readers: the bespoke test's substring assertions (delta 2). `good/expect.txt` pins a clean-line substring, and J's out-of-scope phrase joins what it can see; build moves it if the pinned substring spans the new phrase.
- **The heredoc reader** (delta 1). Its only caller is J. A second reader of the smoke script, `check-install-disposition` assertion B, keeps its own whole-line match; unifying the two is not asked.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged corpus is every kit-owned subcommand, enumerated by the probe above: 117 members across ten kits, each satisfied by its registered line or its `# unregistered:` declaration as the table shows. drift-kit owns none.

## Existing sections updated

Roster from `git grep -n "assertion I\|# unregistered:\|Eight assertions" gate-sdk/SPEC.md` and `git grep -n "smoke/install.sh\` expected-gate" docs/site-architecture.md`, run 2026-09-23.

- `gate-sdk/SPEC.md` §check-gate-substrate-parity, the usage paragraph, the J bullet and the coverage paragraph (delta 3).
- `gate-sdk/SPEC.md` §Layout and configuration, the `gates.list` bullet, and §Consumer smoke, the `smoke/install.sh` bullet and **The declaration valve** paragraph (delta 4). The third of these was reached by align's own re-run of this roster's stated probe, not by the authoring-time roster.
- `docs/site-architecture.md`, the new-gate fan-out row (delta 4).
- `native/src/gates/gate_substrate_parity.rs` and `native/src/registry.rs`, the assertion, the heredoc reader and the unit tests (deltas 1 and 2).
- `gate-sdk/gate-tests/check-gate-substrate-parity.test.sh` (delta 2).
- `.workflow/release-declarations.md`, a Tightened gates bullet: `check-gate-substrate-parity` gains assertion J, in a publishing tree only (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- `Eight assertions: (A)` — the stale count on the usage line (delta 3).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for assertion J.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC addition re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `smoke-registry-omission-blind` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
