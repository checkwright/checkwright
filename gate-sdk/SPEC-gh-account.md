# SPEC amendment: gh-account

A **third expectation kind** for `check-identity` — `gh-account <login>` —
asserting that this clone's machine-global GitHub CLI account is the one the
consumer's manifest names. It delivers `gh-account-identity-expectation`, and it
rides `check-identity`'s port: the operator ruled fork (3) **paired with the
port** on 2026-08-19 in the lead session through the harness's question
mechanism, so the kind lands native and is not built twice.

## The envelope

Stated first because it is what a build session needs before it writes a line,
and because it bounds what may move without re-opening design.

**Asserted.** One new manifest key, `gh-account <login>`, on the existing
optional manifest; one new actual read from the CLI's own persisted config by a
**local file read**; the same clean/violation/fail-closed grading the gate's two
existing kinds already carry; and the login staying consumer config in every
direction.

**Settled here, and these were the open forks the queue entry handed this
stage.** Fork (1) — the oracle is a local read, and delta 2 rules its path
resolution, its parse and its version tolerance. Fork (2) — the posture on an
absent or unreadable CLI config, ruled **graded** rather than binary in delta 4.
Fork (3) was the operator's and is recorded above.

**Not asserted, and the queue entry's scope fence is why** (operator-ruled
2026-08-18, on the entry): nothing here restores a switched account, prevents a
switch persisting, or repairs the local release runbook's missing restore step —
this kind *detects* a left-switched account at the next run. And this repo's own
`scripts/identity.conf` gains **no** `gh-account` line, so checkwright itself
stays undetected and ships mechanism a consumer opts into. Delta 8 carries what
that costs the evidence.

**Out of envelope — escalate rather than absorb.** Changing the manifest's
field-count contract or its fail-closed-on-unknown-key rule; a fourth
expectation kind; any behavior that *performs* an identity mapping rather than
asserting one applied, which is `check-identity`'s standing scope fence
(§check-identity); and anything about the release runbook, which the fence puts
out.

## What changes

### 1. gate-sdk/SPEC.md §check-identity gains the third kind

`gh-account <login>` joins `email` and `remote-host` in the manifest's key
vocabulary: exactly **two** fields, matched against the CLI's persisted active
account for the configured host by exact string. *design-bearing.*

The existing contracts reach it unchanged and that is the whole argument for
putting the kind here rather than in a new gate: an optional manifest behind
`GATE_SDK_IDENTITY_FILE`, a wrong field count or an unknown key as **exit 2**, a
mismatch as **exit 1**, and the CI clean-skip ahead of every manifest read. A
wrong-field-count `gh-account` line is fail-closed by the arm that already
exists, not by a new one. *mechanical.*

### 2. The oracle is a local read of the CLI's persisted config — fork (1), ruled

**Why not the CLI's status subcommand**, which is the obvious oracle and is
wrong: it validates the token over the network. At `tier=precommit` that makes
an offline commit red and puts a network round-trip in the pre-commit path, so
it is refused. *design-bearing.*

**The read.** The CLI keeps a hosts file mapping each host to that host's
credentials and to the **active** account. The gate resolves the path through a
new knob `GATE_SDK_GH_HOSTS_FILE`, whose default derivation honours the CLI's
own config-dir variable first and the XDG config home second — a consumer that
has relocated its CLI config has already said so once, and re-asking it through
a kit knob would be a second source for one fact. *design-bearing.*

**The parse, and the collision that decides it.** Inside the block introduced by
the configured host, the active account is the value of the key spelled exactly
`user`. The sibling key `users` — the map enumerating the accounts *available*
on this machine — is a **prefix of it**, so a substring or startswith match on
`user` hits the map header first and reads a structural key as a login. The
match is therefore on the exact key token, never a prefix. Recorded as a delta
rather than left to the implementation because it is the defect a reasonable
implementation walks straight into, and the fixture pair cannot fail on a case
nobody thought to write. *design-bearing.*

**The version tolerance, which is what makes binding to a tool's internal config
format defensible.** The gate binds to `user` and deliberately does **not** read
the `users` map: the map answers *which accounts exist here*, a different
question from *which one is active*, so a design reading it would answer the
wrong question even where it parsed cleanly. A file whose shape yields no `user`
key for the configured host is **exit 2** — an unrecognized shape is fail-closed,
never a clean, which is the one posture that keeps a format change from silently
retiring the assertion. *design-bearing.*

### 3. The host is a knob, not a manifest field

`GATE_SDK_GH_HOST`, defaulting to the public host, names the host whose block is
read — config-via-env, and the CLI's own host variable is the shape it mirrors.
*design-bearing.*

**The alternative is refused with cause**, because it is the one a later author
will reach for: a three-field `gh-account <host> <login>` would let one manifest
pin two hosts, and it would move the field count the queue entry's grammar fixes
and the fail-closed arm keys on. A consumer pinning two hosts from one clone is
not in evidence; a consumer on a single enterprise host is served by the knob.
*design-bearing.*

### 4. The posture on an absent or unreadable CLI config — fork (2), ruled graded

Three conditions, three verdicts, and the grading is the ruling rather than a
hedge between the two the entry named:

- **The hosts file is absent** — clean, with the fail-open caveat named *in the
  clean line*. The ground is the entry's own: a clone with no CLI cannot push
  through it, so the hazard this kind guards cannot arise there. The precedent is
  context-kit/SPEC.md §check-memory-off, which is clean where its surface is
  absent and states the caveat in its own output, and this kind takes that shape
  verbatim. *design-bearing.*
- **The hosts file is present and cannot be read or parsed** — exit 2. The
  surface exists and the gate cannot say what it holds; a clean there is a false
  clean on the one condition the kind exists to catch. *design-bearing.*
- **The file is present and carries no block for the configured host** — a
  **violation**, exit 1. The manifest says the account should be one thing and
  this machine is not logged into that host at all. *design-bearing.*

**The distinguishing principle, stated so the third bullet does not read as
inconsistent with the first.** `check-identity` already reds when a
manifest-named remote is absent from the clone, and that is not in tension with
clean-on-absent-CLI: a remote is **repo-local state a manifest may demand**,
while the CLI is **machine state outside the repo** that no repo-local manifest
can require to be installed. Absence of the tool is outside the manifest's
authority; absence of a login *within* a configured tool is inside it.
*design-bearing.*

### 5. The subject is the persisted account, not the effective one

The assertion is worded over the CLI's **persisted** active account, and that
wording is load-bearing rather than incidental: a token environment variable
makes the CLI authenticate as that token's account without touching the hosts
file, so an assertion worded over "the account the CLI would use right now"
would be false wherever one is set. *design-bearing.*

**The honest limit rides with the ruling, and it is small on purpose.** A
per-process token override is not detected. It is also not the hazard: the
recurrence the entry records is a *persistent* switch left behind by a sanctioned
release action, which damages every other clone on the box; a token in one
process's environment persists nothing and reaches no sibling. Scoping the
assertion to the persisted state is therefore narrowing it onto the thing that
actually recurs, not conceding a gap. *design-bearing.*

### 6. Sequencing: the port first, the kind second, in two commits

`check-identity` is a member of the fifth budget batch
(`gate-sdk/SPEC-fifth-batch.md`), so its port lands there. The kind lands
**after** it, in its own commit, in the crate only. *design-bearing.*

Both halves of that order are forced. Landing the kind shell-side first builds it
twice, which is exactly what the operator's paired ruling refuses. Landing it
*before* the port's cross-substrate parity run is worse: parity is taken while
both implementations exist, so a kind added to one side first leaves the port
with nothing to be equal to, and the batch's own oracle would be comparing two
different rules. *design-bearing.*

### 7. The injection point is a knob, because the port deletes the fixture arm

`check-identity`'s `--fixture <dir>` arm and its `[manifest]` positional do not
survive the port; `gate-sdk/SPEC-fifth-batch.md` delta 4 owns that ruling and the
actual-source knobs it mints. This kind therefore reaches its actual through
`GATE_SDK_GH_HOSTS_FILE` rather than through a fixture-dir file. *mechanical.*

Which is the better shape here anyway, and the reason is worth one sentence: that
knob is the one member of the minted family with a genuine **live** use — a
relocated CLI config — so the kind's test path and its production path are the
same path, which is the property the fixture arm's deletion exists to buy.
*design-bearing.*

### 8. The fixture pair gains the kind, and it is the kind's only exercise

Both cases gain a hosts file and a `gh-account` manifest line — the good case
matching, the bad case mismatching — reached through the knob. *mechanical.*

**What that leaves uncovered is stated rather than discovered.** The scope fence
keeps this tree's own `identity.conf` free of the kind, so **no live run in this
repo ever executes it**; the pair is the whole oracle. Two consequences a build
session owes: the pair must reach the graded postures of delta 4 and not only the
match/mismatch axis, since nothing else will; and the sibling `*.test.sh` is
where the postures the one-pair harness cannot hold belong. *design-bearing.*

### 9. The `# graph:` manifest is unchanged, and the reason is already recorded

The manifest keeps `couples=scripts/identity.conf`. The hosts file is
machine-global, outside the tree, and never stages, so no coupling could name it
— which is the situation §check-identity already answers for `git config
user.email`: a mapping change is not diff-visible, so the whole-tree battery is
the real backstop, with the `install-hooks.sh` rung covering the opt-in moment.
That answer extends to this kind with nothing added. *mechanical.*

### 10. The seam: the kit ships the kind, never an account

A login is one project's — one *person's* — vocabulary, and a crate constant
holding one would publish it as everyone's mechanism (CLAUDE.md §The provenance
seam). The manifest is already optional consumer config on the `graph-vocab.sh`
pattern, so the kind inherits the discharge: the crate carries the key name, the
parse and the comparison; the consumer's manifest carries the login; the host is
a knob with a generic default. Nothing in this amendment, in the crate, or in the
fixture pair names a real account. *design-bearing.*

## Producers and consumers

**New interface: the `gh-account` manifest key.**
*Producer* — the consumer's identity manifest, resolved by
`GATE_SDK_IDENTITY_FILE`, which is live config a consumer already writes (this
repo's own copy is at `scripts/identity.conf` and, per the scope fence, gains no
such line). *Consumer* — `check-identity`'s per-line dispatch, at the `case` arm
selecting on the first field; the arm that today falls through to *malformed*.
*Field readers* — `<login>`, the kind's only field, is read by the comparison in
that same arm and nowhere else.

**New state: the parsed active account.**
*Producer* — the GitHub CLI, writing its hosts file whenever an account is added,
switched or removed; the enabling configuration is the consumer having
authenticated at all, which is what makes the producer reachable rather than
test-only. *Consumer* — the gate's hosts-file reader, at the same per-line
dispatch, once per `gh-account` line.

**New knobs, each with a named reader.** `GATE_SDK_GH_HOSTS_FILE` is read by the
hosts-file reader and by the fixture pair; `GATE_SDK_GH_HOST` is read by the
block selector. Both cross the dispatch seam as **bridged knobs**
(gate-sdk/SPEC.md §The port-candidate criteria, criterion 6), so each must be
resolvable on a name in `gate-sdk/lib/gate.sh` rather than defaulted inline —
the bridge's undeclared-knob refusal is what a knob defaulted at its use site
trips, and §The fourth budget batch paid for that lesson six times.

**Existing readers of this gate's output, enumerated because the kind changes
what it prints.** `gate-sdk/bin/install-hooks.sh` reads the **exit code** only
and is unaffected by the count in the clean line. `check-gate-output` reads the
machine-keyable success line, whose shape is unchanged — the kind adds to the
counted expectations, never to the line's grammar.

**The narrowing question, answered in both directions**
(canon-kit/SPEC.md §The causal-completeness check, point 5). Adding a key
**narrows** one corpus: the set of manifest lines the unknown-key arm rejects
loses `gh-account`. That arm's red condition is *an unrecognized first field*, so
the narrowing can only remove violations from it — monotone, and clearable by
inspection. In the other direction the kind **adds** a violation class, a
mismatched or absent account, which is the point of the unit. No reader's verdict
is cleared here by the "a narrower corpus can only remove violations" argument
that point 5 exists to refuse: the malformed arm is the only narrowed corpus, and
it was checked rather than assumed.

**No probe silences stderr.** A `2>/dev/null` on the hosts-file read would turn
an unreadable file into an absent one and collapse delta 4's second verdict into
its first — the exact false negative the grading exists to keep apart.

## Existing sections updated

- **gate-sdk/SPEC.md §check-identity** — the third kind, its local-read oracle,
  its parse and version tolerance, the graded absence posture, the
  persisted-versus-effective wording, and the two new knobs. Owned by deltas
  1–5 and 10.
- **gate-sdk/SPEC.md §Layout and configuration** — `GATE_SDK_GH_HOSTS_FILE` and
  `GATE_SDK_GH_HOST` join the knob roster with their defaults, on the kit rule
  that each SPEC owns its own knob roster and values. Owned by deltas 2 and 3.
- **gate-sdk/SPEC.md §The fifth budget batch** — the kind's sequencing behind
  `check-identity`'s port is a fact about the batch as much as about the kind,
  and is stated once there. Owned by delta 6.
- **`TASK-QUEUE.md`'s `gh-account-identity-expectation` entry** — forks (1) and
  (2) are settled by this amendment and their open-fork text retires with them.
  Owned by deltas 2 and 4.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
