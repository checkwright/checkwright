# SPEC amendment: peer-request-channel

Pairs `peer-request-channel`. A project adopting the kits has no channel for asking this one for something. Each ask arrives as a hand-carried file, nothing records whether it was taken up, and the asker learns a requested feature shipped only by reading release notes. This amendment adds a **peer request channel** as lifecycle-kit mechanism. It has five parts: a request record in four fields, a reply record carrying the request's state, a filing arm a peer runs from any directory, the receiving project's scope intake and close stamps, and a read on the peer's side that reports a shipped request until the peer runs the release that carries it.

**What the tree does today**, read from `lifecycle-kit/SPEC.md`, the scope and close templates, and `native/src/emit/`:

- **No intake surface reads a request from outside the tree.** The committed gap inbox and the consult inbox are both tracked files inside the consumer's tree, filed by a session working in it (§The committed gap inbox, §The consult inbox). A request body written there would be published. No arm reads a directory outside the tree for intake (probe: `git grep -n -i "peer" -- lifecycle-kit/SPEC.md lifecycle-kit/templates native/src/emit`, which finds none).
- **Scope's intake takes carried gap bullets only.** The scope template's second step dispositions the bullets the entry reports, each exactly once. No step reads an outside request.
- **Close stamps a release, and nothing tells an asker.** Close step 10 stamps `<iteration> release <version>` into the consumer's disposition evidence. Nothing joins a tag to the asks it answers.
- **The binary knows its own release.** The build bakes the release stamp, `CHECKWRIGHT_RELEASE`, with its exactness, and the provenance block reads it (gate-sdk/SPEC.md §run-gates, *Where the release stamp comes from*). That is a version read with no network, no lock file and no repository.

## What changes

### (1) The channel, its two records and its knob {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a request and reply record in the four fields an adopter asked for, on an absolute untracked channel path}

A new section, `## The peer request channel`, placed after §The consult inbox. **Not yet applied:**

> A **peer** is another project asking this one for a change: a bug fix, a feature, a calibration. Its ask needs a place to land that never publishes the body, a state the peer can read without asking again, and an end that tells the peer when the change can be adopted. The gap inbox and the consult inbox cannot carry it. Both are tracked in the consumer's tree, and a request body may hold the peer's private context.
>
> **The channel** is a directory, `LIFECYCLE_KIT_PEER_CHANNEL` (§Layout and configuration). The receiving project and each peer filing to it bind the same absolute path, each in its own untracked configuration: the environment, or a local knob overlay. The value is refused at exit 2, naming the knob and its value, when it is relative, missing, not a directory, or inside a git work tree, which git answers by `rev-parse --is-inside-work-tree` run in it. A channel inside a work tree could be committed with the rest of that tree, and request bodies would be published. Unset, the reading arms print nothing and exit 0, and the writing arms refuse at exit 2.
>
> **One file per request, one per reply, each with one writer.** The peer writes `<id>.md` once and never edits it. The receiving project appends to `<id>.reply` and never edits a request. Neither writer touches the other's file, so no lock is needed. Every other file in the directory is ignored.
>
> **The request**, `<id>.md`:
>
> - line 1, `# peer-request v1`;
> - `id: <id>`, `from: <peer>` and `filed: <YYYY-MM-DDTHH:MM:SSZ>` on the next three lines, the time in UTC;
> - four sections in this order, each a `## ` heading over a body that must not be blank: `## need`, `## repro`, `## wanted`, `## evidence`.
>
> A `## ` line inside fenced code is body text. A fifth section, a missing or reordered one, or a blank body makes the file malformed. `<peer>` is one token from `[a-z0-9][a-z0-9-]*`. It names the peer to the receiver and lives only in the channel. `<id>` is `r<YYYYMMDD>-<h>`: the filing date, then the first 8 hex digits of the SHA-256 of the file's bytes with the id line read as `id: `. It is opaque, and it names neither the peer nor the ask, so a tracked surface may cite it.
>
> **The reply**, `<id>.reply`: line 1 `# peer-reply v1`, then one line per state change, `<YYYY-MM-DD> <state>`, appended and never rewritten. The states:
>
> - `filed <slug>`, where the ask became, or joined, the queue entry `<slug>`;
> - `refused <reason>`, where the ask is declined, the reason one line of prose;
> - `landed`, where the entry `<slug>` reached Done;
> - `shipped <tag>`, where a release carrying it was tagged.
>
> A request with no reply file, or none past its header, is **received**. The legal moves are received to `filed` or `refused`, `filed` to `filed` (a new slug, when the entry is renamed or split), `filed` to `landed`, and `landed` to `shipped`. `refused` and `shipped` are final. The writing arm refuses every other move at exit 2, naming the request, its state and the move asked. A request's state is its last reply line.
>
> **A queue entry cites the id, never the body.** The receiving session cites `peer request <id>` in the entry's prose. The id is the only trace of the request in the tracked tree. The reply is written outside it, so no commit carries a state change, and the entry's own motion is the tracked record.
>
> **Honest limits.** The channel is a local directory, so a peer reaches it only on the same machine or through a directory the two sync. No gate reads it, since it lies outside every tree a battery runs on. A malformed record is caught by the arms that parse it, as below. Nothing enforces one writer per file beyond the arms' own behaviour, so a hand edit to a reply bypasses the transition check. A channel in a directory git cannot see as a work tree, such as a tree checked out with a separate git directory, is admitted.

- **Why a reply beside the request and not a section inside it.** The asker wanted a reply beside each request. Beyond that, one writer per file is what makes the channel safe without a lock. A peer may re-read its request while the receiver writes, and neither write can tear the other's file.
- **Why `received` is derived and not written.** A request is received as soon as it exists in the channel. A written `received` line would cost a write per request and say nothing more.
- **Why the id hashes the whole file.** The filing time is inside the hashed bytes, so the same ask filed twice gets two ids, and two different asks filed in the same second get different ids. The writing arm creates the file exclusively, so a collision refuses rather than overwrites.
- **Why the format is the contract, not the arm.** A peer that does not run the binary can write a request by hand in the grammar above. A peer whose own intake uses the same four fields can file through one writer serving both.

### (2) The arms {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a filing arm a peer runs outside a checkwright tree, a listing arm, a reply arm, and a peer-side ready read}

New paragraphs in the new section. **Not yet applied:**

> **Four `--emit-` arms read and write the channel**, each declaring `LIFECYCLE_KIT_PEER_CHANNEL` alone. Each exits 0 on success and 2 on a refusal: a bad argument, a bad channel, a malformed record or an illegal move.
>
> - **`--emit peer-file <peer>`**, the peer's arm. It reads the four sections from standard input, in the request's section grammar, and refuses a malformed body, naming the section. It stamps the header lines, computes the id, creates `<id>.md` exclusively through a temporary file and a rename, and prints the id on one line.
> - **`--emit peer-list`**, the receiver's arm. It prints one tab-separated row per request, sorted by id: `<id>`, `<peer>`, the date of its last state change (the filing date for a received request), its state with its operand, and the first non-blank line of its `## need`. It takes no argument.
> - **`--emit peer-reply`**, the receiver's arm, in three forms. `peer-reply <id> filed <slug>`, `peer-reply <id> refused <reason>` and `peer-reply <id> landed` append one move to one request. `peer-reply --landed <slug>` moves every request whose state is `filed <slug>` to `landed`. `peer-reply --shipped <tag>` moves every `landed` request to `shipped <tag>`. It prints one line per move, `<id> <state>`. A bulk form that matches no request prints nothing and exits 0.
> - **`--emit peer-ready <peer>`**, the peer's notice. It prints one line per request filed by `<peer>` whose state is `shipped <tag>`, where `<tag>` is a later release than the running binary's release stamp: `peer request <id> shipped in <tag>, not in this binary (<stamp>): <need line>`. Versions compare as `MAJOR.MINOR.PATCH`, a leading `v` dropped. A stamp with a describe distance suffix compares by its base release. Once the peer runs a binary carrying the release, the line stops. With no stamp, or a tag that does not parse, the line prints with `<stamp>` as `unknown`, because it cannot be shown adopted.
>
> **The two peer arms run from any directory.** A peer may file from its own tree or from none, so neither arm asks for this kit's tree. Outside a repository the binary is reached directly, the front-end refusing a non-repository directory (gate-sdk/SPEC.md §Layout and configuration). The channel knob then comes from the environment or an absolute knob file locator, under the tree-free mark's rule (gate-sdk/SPEC.md §The knob file, *With no repository*). The receiver's two arms run in the receiving tree through the front-end.
>
> **The notice is wired where a session starts.** A peer running context-kit adds `--emit peer-ready <peer>` to its `CONTEXT_KIT_BRIEF_COMMANDS` (context-kit/SPEC.md §The session-context hook). Any other peer runs the binary from its harness's session-start hook. The arm prints nothing while no shipped request is unadopted, so it costs a session no line.

- **Why four sibling arms and not one arm with modes.** The consult inbox's rule applies: one arm with two target surfaces and two output grammars is the shape queue-kit/SPEC.md §The queue-counts arm refuses (§The consult inbox, *A sibling arm*). The filing arm writes a request and prints an id. The list prints rows, the reply appends to a reply file, and the notice prints prose. The two `peer-reply` bulk flags share the single-move form's output and target, so they ride it.
- **Why the notice compares against the running binary and not an installed lock.** The release stamp is baked into the binary the peer actually runs, so it is right under every install shape: a repository install, a user install, or a copied binary. A lock records what an install wrote, which is a second fact and can be stale. So this unit's order does not depend on `user-install-update-notice`, whose subject is `user.lock`. That entry stays deferred on its own grounds.
- **Why `landed` and `shipped` are separate.** A landed change is on the receiver's default branch. A shipped change is in a release the peer can install. A consumer releasing every few iterations holds landed requests across several closes, and the bulk `--shipped` move takes every one of them at the tag that carries them.

### (3) Scope's intake lane and close's two stamps {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: scope's intake lane and close's shipped stamp}

New text in the scope template, a third step after the second. **Not yet applied:**

> **Third step — take the peer requests, if the consumer binds a channel.** Run `--emit peer-list` on the gate binary `GATE_SDK_NATIVE_BIN` names. Every **received** row gets exactly one disposition. One the set takes up, or one that joins a live entry, is **filed**: the entry's prose cites `peer request <id>`, and `--emit peer-reply <id> filed <slug>` records it after the queue commit lands. One declined with cause is **refused** with `--emit peer-reply <id> refused <reason>`. A request that asks the operator for a direction rides this session's escalation, and stays received until the operator answers. The request body never enters a tracked surface: the entry restates the ask in its own words. An empty channel, or none bound, is a step with nothing to do (lifecycle-kit/SPEC.md §The peer request channel).

New text in the close template's step 5, before *Then clear Done*. **Not yet applied:**

> Before clearing it, run `--emit peer-reply --landed <slug>` once per slug in the done section, so every request filed under a landed entry reads landed (lifecycle-kit/SPEC.md §The peer request channel).

New text in the close template's step 10, after the tag is applied. **Not yet applied:**

> Where the procedure applied a tag, run `--emit peer-reply --shipped <tag>` once it is applied, so every landed request reads shipped in the release that carries it (lifecycle-kit/SPEC.md §The peer request channel).

- **Why scope and not any session.** A request is mid-iteration intake, and scope-gated intake places it at scope (doctrine-kit/DOCTRINE.md, Scope-gated intake). Scope is also the one session that writes queue entries directly, which a `filed <slug>` reply needs.
- **Why the filed reply follows the queue commit.** A reply naming a slug that never committed would point the peer at nothing.
- **Why the template steps need no consumer slot.** Every arm reads the knob, and an unbound channel prints nothing. A consumer without the channel runs the steps to an empty result.

## Producers and consumers

- **`LIFECYCLE_KIT_PEER_CHANNEL`** (new scalar knob) — producer: each project's untracked configuration, default empty. Consumers: the four arms.
- **The request record** (new file grammar) — producers: `--emit peer-file`, or a peer writing by hand. Consumers: `--emit peer-list`, `--emit peer-reply` (for the request's existence), and `--emit peer-ready` (for `from:` and the need line).
- **The reply record** (new file grammar) — producer: `--emit peer-reply`. Consumers: `--emit peer-list`, `--emit peer-reply` (the transition check), and `--emit peer-ready`.
- **The four arms** (new `ARMS` rows in `native/src/emit/mod.rs`) — named callers, as gate-sdk/SPEC.md §The non-gate arm requires: `peer-file`, a peer session or its operator; `peer-list` and the single-move `peer-reply`, the scope template's third step; the bulk `peer-reply` forms, close steps 5 and 10; `peer-ready`, a peer's session brief or session-start hook. Each owes a unit test in its module.
- **The opaque id in a queue entry** (new citation) — producer: scope. Consumer: a reader of the entry. No gate reads it, because the channel lies outside the tree.
- **The release stamp** (existing) — producer: the build. New consumer: `--emit peer-ready`.
- **Point 5 (narrowing).** Nothing narrows. A consumer binding no channel sees no output and no refusal from the template steps.
- **Point 6 (every member).** The members are every step that takes intake into the queue, and every close step that moves an entry past Done or applies a tag. Intake: the scope template's second step, the only step taking work from outside the queue (probe: `git grep -n "take the carried gap bullets" -- lifecycle-kit/templates`). Moving past Done: close step 5, the one step clearing Done (probe: `git grep -n "clear-done" -- lifecycle-kit/templates`). Applying a tag: close step 10 (probe: `git grep -n "the tag applied" -- lifecycle-kit/templates`). The release-sweep template applies no tag of its own (probe: `git grep -n -w -i "tag" -- lifecycle-kit/templates/release-sweep.md`, which finds none), and a consumer's close binding applies its tag inside step 10's procedure, so step 10 reaches it.

## Existing sections updated

Roster from `git grep -n -e "consult-count" -e "file-consult" -e "take the carried gap bullets" -e "clear-done" -e "the tag applied"` over the tracked tree, the posts, the queue and the generated mirrors excluded. The consult arms are the analog, and their sites are where a new lifecycle arm lands.

- `lifecycle-kit/SPEC.md` §Layout and configuration gains the knob bullet. The new §The peer request channel lands after §The consult inbox. §The scope template and §The close template each gain a sentence citing it (all deltas).
- `lifecycle-kit/templates/stages/scope.md` gains the third step, and `lifecycle-kit/templates/stages/close.md` gains the step 5 and step 10 sentences (delta 3).
- `lifecycle-kit/README.md` — the arm lines beside the consult arms gain the four peer arms (delta 2).
- `native/src/emit/` gains a peer-channel module, or one per arm on the consult arms' split. Each arm is a row in `ARMS` with its `KNOBS` and its unit tests. `native/src/knobs/` gains the knob row, empty by default (deltas 1 and 2).
- `gate-sdk/SPEC.md` §The knob file, *With no repository*, gains the two peer arms beside the tree-free mark's members as the readers it serves (delta 2). §The non-gate arm's *Fixed by the implementation* roster gains the four arms if they spawn `date` off unix, as `--emit-file-consult` does (delta 2).
- `lifecycle-kit/gate-tests/peer-channel.test.sh` — new. In a scratch channel outside any repository, it files a request, lists it, walks every legal move and one illegal one, runs both bulk forms, and runs `peer-ready` against the running binary's stamp. It also refuses a channel inside a work tree, a relative one, and a malformed body (deltas 1 and 2).
- `lifecycle-kit/smoke/install.sh` — one leg filing, listing and replying in a scratch channel, so a vendored install proves the arms answer (delta 2).
- Generated projections the new arms and knob move: `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md`, `docs/gate-sdk/SPEC.md`, `docs/llms-full.txt`, and any arm or knob roster a freshness gate derives. Each is regenerated by the command its gate prints on red (docs/generated-projections.md) (all deltas).
- `.workflow/release-declarations.md` — *Knob changes* gains the knob, and *Behavior changes* gains the arms and the template steps (all deltas).
- This repo binds the channel in its untracked overlay, `scripts/lifecycle-config.local.knobs`, which the operator writes. No tracked knob file changes, since the path is local to a machine (delta 1).

## Retired spellings

- None — the channel, its records, its arms and its knob are additions, and the template steps add text without renaming any.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knob, the two records, the four arms, the id citation and the template steps.
- [ ] **Instruction surfaces: instruction only** — the scope and close template text states steps and cites §The peer request channel for the grounds.
- [ ] **Merged with no information lost** — the new section lands whole after §The consult inbox and the template steps cite it; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `peer-request-channel` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local: the unit tests, the gate test, the smoke and the battery. No remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
