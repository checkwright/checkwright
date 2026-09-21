# SPEC amendment: fence-run

`check-fence-command-head` shows that a fence's command head can run, and it
shows nothing else: a wrong operand or an unassigned variable passes. Its own
section says so: *"Executing a fence needs a sandbox and a way for a doc to mark
which fences are hermetic, and it is not built here"* (canon-kit/SPEC.md
§check-fence-command-head, *What it does not attempt*). This amendment builds
both. It adds a per-fence marker, which defaults to not runnable, and a gate that
executes every marked fence. The gate runs each fence in a scratch copy of the
tracked tree, with no network, under a declared set of gate-binary arms.

**Facts the design rests on, each probed.**

- **No sandbox exists to reuse.** `grep -rln "unshare\|firejail\|sandbox-exec\|network namespace" native/src gate-sdk` returns nothing. In this tree, "hermetic" has only ever meant config-isolated: gate-sdk/lib/test-hermetic.sh pins knob files and nothing more. There is no portable process sandbox on all three adopter operating systems (gate-sdk/SPEC.md §The adopter constraints), so this amendment gets "no network" by construction instead of from a kernel facility (delta 3).
- **The crate's network reach is two arms.** `grep -rn "programs::\(CURL\|GH\|NPM\|NPX\)" native/src` hits two files. `native/src/hook/poll.rs` spawns `curl` for `--usage-poll`, and `native/src/emit/pack_installer.rs` spawns `npm` for `--pack-installer`. Both are `Arm::Run` rows. `grep -rn '"clone"\|"fetch"\|"push"\|"ls-remote"\|TcpStream' native/src` finds one spawn, `upgrade_smoke.rs:524`'s local `git clone --shared` (the `ls-remote` hit is a word list in `scan_prompts.rs`, which spawns nothing). So none of the arms the declared set of delta 2 admits reaches the network.
- **Corpus.** The governed doc set's shell fences number 36. All of them are `bash`, and no `console` or `powershell` fence exists in it. The dominant idiom is the kit READMEs' and index pages' `. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"`, followed by `"$gates" <arm> …` lines. The fences that are not candidates are the ones that fetch: `docs/install.md`'s `curl` release download and the home page's `npx` lines.
- **No per-fence marker exists.** The one valve is the info string (`text`, `console`), which removes a fence from scanning altogether.

It is a root-level amendment because it spans canon-kit (the marker and the gate),
gate-sdk (the declared arm set beside the arm table, and the program roster) and
context-kit (the tool-floor audience of `bash`).

## What changes

**Batching.** All four deltas land together. The gate reads the marker and the
declared set, and the program-roster change is what makes its `bash` spawn lawful.

### (1) The runnable-fence marker {design-bearing}

**Not yet applied.** A doc marks a shell fence runnable with a
`<!-- fence-runnable -->` line **immediately above** the fence's opening line.
Optionally it declares an expected exit status, `<!-- fence-runnable: exit=<n> -->`,
for a fence that shows a gate reddening. The default is 0. With no marker, a fence
is not runnable, which leaves every fence in the tree today as it is.

The marker is judged strictly, because a misspelled marker that silently
disarms is the failure a marker exists to prevent. Each of the following is red:

- a marker whose next line is not a `bash`, `sh` or `shell` fence opener;
- a marker operand other than `exit=<decimal>`;
- a line that looks like the marker but is misspelled, meaning one that matches
  `<!--\s*fence-run` and not the grammar.

The corpus is `check-md-refs`' governed doc set, the one both fence gates already
read.

### (2) The fence-safe arm set, declared beside the arm table {design-bearing}

**Not yet applied.** gate-sdk declares `FENCE_SAFE_ARMS` in the crate, beside the
arm table. `FAIL_OPEN_ARMS` is the precedent for a set declared beside the table:
it is one declaration of a property of arms, not a knob. An arm belongs in the
set when it meets two conditions: it spawns **no program that reaches the
network**, and it **writes nowhere but its working tree and stdout**.

The set is seeded with:

- `--help` and `--list`;
- the battery, `--run`;
- `--run-gate-tests`;
- every `--emit` family member;
- every registered gate name.

Membership is checked by the two probes above. Each seed arm's spawns stay inside
the tree it runs in, and the only network spawners are `--usage-poll` and
`--pack-installer`, neither of which is admitted. A crate unit test holds the set
and the two network-spawning arms disjoint. So a new network-spawning arm
admitted to the set reds the build. It is never discovered in an adopter's
scratch.

The set is gate-sdk's and not canon-kit's, because it is a property of the
binary's arms and is read by a gate that happens to live in canon-kit. A consumer
that needs a further program in a runnable fence names it in delta 3's knob. It
never widens this set.

### (3) `check-fence-run` executes each marked fence in a scratch copy of the tracked tree {design-bearing}

**Not yet applied.** This is a new canon-kit gate on the native substrate, with a
`good/`+`bad/` fixture pair and `tier=align-only`. It runs in the full battery and
stays out of the generated pre-commit hook, the class
gate-sdk/SPEC.md §check-graph reserves for gates that pre-commit cannot afford.

**Static pass first, before anything runs.** Every command in a marked fence
must have a head admitted by `check-fence-command-head`'s rules, narrowed:

- a **configured program** must be a member of the new knob
  `CANON_KIT_FENCE_RUN_PROGRAMS`, whose default is `git`, not of
  `CANON_KIT_FENCE_PROGRAMS`;
- an **expansion** head (`"$gates"`), or a tracked path naming the gate binary,
  has its first operand checked against `FENCE_SAFE_ARMS`. A `-`-led operand
  must be a member. A bare word must be a registered gate name.

Any other head reds, statically, and the fence is not executed. The static pass
is what makes "no network" hold for the shipped defaults: no admitted head can
dial out.

**Then the run.**

1. **One scratch per doc.** The scratch sits under the scratch base
   §Consumer smoke's builder uses (`DEMO_TMP_DIR`, then the platform temp dir).
   The gate copies the index's tracked set into it with `std::fs`, then runs
   `git init` there and a seed commit with a fixed identity, so a fence that
   reads git sees a repository. This copy is one in-crate helper, a
   tracked-tree scratch that gate-sdk owns beside §Consumer smoke's builder.
   SPEC-projection-witness.md's witness needs the same helper, so whichever unit
   lands first writes it and the other calls it.
2. **The binary is placed.** The running binary goes at the path
   `GATE_SDK_NATIVE_BIN` resolves to inside the scratch, through the
   scratch-consumer builder's own placement, which a source clone's untracked
   build output needs.
3. **Marked fences run in document order**, each as its own
   `bash -euo pipefail` process with the scratch root as its working directory.
   Files carry over from one fence to the next and shell state does not. A reader
   who pastes a later fence into a fresh shell likewise has no variable an earlier
   fence set, and `-u` is what turns that unassigned variable into a red.
4. **The environment is fixed and not inherited.** It sets `PATH` and `LC_ALL=C`,
   points `HOME` at a directory inside the scratch, and sets
   `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL` at an empty file in the scratch,
   and `GIT_ALLOW_PROTOCOL=file`. It also sets `http_proxy`, `https_proxy` and
   `all_proxy`, in both cases, to an unroutable loopback port, and sets
   `GATE_SDK_NATIVE_BIN`. **Honest limit:** the environment is defense in depth
   for a consumer-added program that honors proxies. It is not a sandbox. A
   program named in `CANON_KIT_FENCE_RUN_PROGRAMS` that ignores proxy variables
   can reach the network, and naming it is the consumer's act.
5. **The scratch is removed on every exit path.**

**Point 5, the red condition.** A fence reds when its exit status differs from the
declared one. The finding names:

- the doc and the fence's opening line;
- the declared and actual status;
- the last twenty lines of the fence's merged output.

A static-pass red names the doc, the line and the head, together with the rule it
failed. The clean line counts docs, marked fences and executed commands. So a
tree with no marked fence prints `0 marked fences`, which reads as "nothing is
executed" and is not a quiet pass. A failed copy, a missing `bash` or a
failed `git init` exits 2, under the fail-closed contract.

**On a host with no `bash`, and on Windows.** The gate spawns `bash` only when at
least one marked fence exists. With zero markers it spawns nothing and is clean on
every host. So an adopter who never marks a fence takes on no interpreter. Marking
one is the act that takes the dependency, and the fence is a `bash` fence by its
own info string. Delta 4 records the spawn.

### (4) The spawn is rostered, and the static gate's section points here {mechanical}

**Not yet applied.**

- `native/src/programs.rs`: canon-kit's gate joins `BASH`'s spawners. The
  tool-floor audience for `bash` (context-kit/SPEC.md's roster,
  `native/src/toolfloor.rs`) gains canon-kit **conditionally**, conditional on a
  marked fence existing, and in the conditional form the roster already carries.
  gate-sdk/SPEC.md §The program roster's per-member record for arms gains the
  gate's spawn.
- canon-kit/SPEC.md §check-fence-command-head, *What it does not attempt*: the
  last sentence is rewritten to point at `check-fence-run`, the gate that
  executes marked fences.
- canon-kit/SPEC.md §Layout and configuration gains `CANON_KIT_FENCE_RUN_PROGRAMS`
  (default `git`). canon-kit's knob table in `native/src/knobs/canon_kit.rs` and
  `canon-kit/templates/canon-config.knobs` gain the knob.
- **The dogfood marking.** Build marks every fence in this tree's governed doc set
  that passes delta 3's static pass and exits 0 in the scratch. The probe is the
  gate run with every fence marked, in a scratch branch. The result must be at
  least one marked fence, so this repo's own gate is never the vacuous
  `0 marked fences`. The count marked is recorded in the commit message.

Mechanical: every value is fixed by deltas 1 to 3.

## Producers and consumers

- **The marker (delta 1).** Producer: a doc author. Consumer: `check-fence-run`'s
  scan, the only reader. `check-fence-command-head` keeps treating a marked fence
  exactly as it treats any other, since a comment line above a fence is outside
  its fence body.
- **`FENCE_SAFE_ARMS` (delta 2).** Producer: the crate declaration. Consumers:
  `check-fence-run`'s static pass, and the disjointness unit test. It is not a
  knob, so it joins no knob roster.
- **`CANON_KIT_FENCE_RUN_PROGRAMS` (delta 3).** Producer: canon-kit's static knob
  table default, and a consumer's config seam. Consumer: the static pass. It is a
  vocabulary, not a walk filter, so it takes no `knob:` couples token, on
  `CANON_KIT_FENCE_PROGRAMS`' own ground (canon-kit/SPEC.md
  §check-fence-command-head). Roster-holding readers:
  - `check-knob-citation`, satisfied by delta 4's SPEC row;
  - the knob-file template's derivation, satisfied by delta 4's template line.
- **`check-fence-run` (delta 3).** Producer: `canon-kit/checks/check-fence-run.gate`,
  registered in `scripts/gates.list`, with an `# install:` disposition that
  registers it for every profile carrying canon-kit. With no markers it is clean,
  so no arming knob is owed. Consumers:
  - the full battery;
  - `check-graph`'s manifest reader, since the gate is `tier=align-only`;
  - the fixture runner;
  - `canon-kit/README.md`'s gate roster, a roster-holding reader
    (`check-readme-roster`), which gains the row.
- **The fixed environment and the scratch (delta 3).** Internal to the gate. They
  have no field that another component reads.
- **Point 6.** The marked corpus is opt-in, so no member of an enumerable corpus
  is obliged. Delta 4's dogfood marking obliges only the fences its named probe
  finds passing.

## Existing sections updated

Rosters produced by the three probes in the preamble, by reading canon-kit/SPEC.md
§check-fence-command-head and §check-docs-cmd, gate-sdk/SPEC.md §The program
roster, §Consumer smoke and §check-graph's tier field, and by reading
`native/src/emit/mod.rs`'s arm table in full.

- `native/src/gates/` gains a new gate module, and `native/src/gates/mod.rs`'s
  `REGISTRY` gains its row. The gate also gets `canon-kit/checks/check-fence-run.gate`,
  a fixture pair under `canon-kit/gate-tests/check-fence-run/`, and a row in
  `scripts/gates.list` (deltas 1 and 3). The tracked-tree scratch helper is
  shared with SPEC-projection-witness.md (delta 3).
- The crate's arm-table module gains `FENCE_SAFE_ARMS` and its disjointness test
  (delta 2).
- canon-kit/SPEC.md: a new §check-fence-run, and §check-fence-command-head's
  closing sentence. §Layout and configuration's knob roster also changes, along
  with `native/src/knobs/canon_kit.rs` and `canon-kit/templates/canon-config.knobs`
  (deltas 3 and 4).
- gate-sdk/SPEC.md §The program roster, `native/src/programs.rs`,
  `native/src/toolfloor.rs` and context-kit/SPEC.md's tool-floor roster row for
  `bash` (delta 4).
- `canon-kit/README.md`'s gate roster (delta 3).
- The docs marked by delta 4's probe (delta 4).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md`, `docs/gate-sdk/SPEC.md`,
  `docs/context-kit/SPEC.md`, `docs/enforcement.md`, `docs/check-graph.html`.

## Retired spellings

- None — no name is removed, and `check-fence-command-head`'s sentence is reworded,
  not renamed.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The marker's one-line mention
      in any README carries no grounds.
- [ ] **Merged with no information lost.** Each addition re-phrases the text it
      refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `fence-execution-gate` moves to Done in the merge commit,
      at a stage before the drain stage.
- [ ] **The gate fails closed.** Its `bad/` fixture reds on each of these:
      - a fence exiting other than declared;
      - an unassigned variable;
      - a head outside the admitted set;
      - an orphan or misspelled marker.
- [ ] **Not vacuous here.** This repo's run prints at least one marked fence.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
