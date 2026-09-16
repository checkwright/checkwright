# SPEC amendment: payload withholding

Pairs the queue entry `payload-withholds-kit-specs`. The customer payload
withholds each kit's `SPEC.md` and its `smoke/`, and a shipped gate's `# spec:`
pointer resolves to the site's SPEC mirror instead of to a file the payload
carries.

Sited at the repository root: the unit spans `native/`, `gate-sdk/`,
`installer/`, `docs/` and four kit READMEs, so no single component owns it.

**The seam this unit crosses, ruled.** Kit mechanism: the *shape* of a
withholding applied per packed root, the *shape* of a resolved pointer, and the
two knobs that carry them — nothing here names a kit, a section or a host.
Consumer config: both values. `GATE_SDK_PAYLOAD_WITHHOLD` carries the kit
convention's own member names as its default because they are gate-sdk's names,
and stays steerable for a consumer whose spec file is named otherwise;
`GATE_SDK_SPEC_BASE_URL` is **empty** by default and its value is the
publisher's, set in this repository's own knob file beside the host
`GATE_SDK_GRAPH_EXTERNAL_REFS` already carries. Private rule content: none is
created or moved — and the withholding *reduces* the class, since a kit SPEC is
the surface most likely to carry its publisher's rule content and it stops
travelling in customer trees. Provenance: the grounds below stand undated in the
merged kit SPEC, as engineering grounds; no dated stamp, ledger pointer or queue
slug goes with them.

**Ordering.** `SPEC-installer-front-door.md` relocates five of the sections this
amendment edits — §The packer, §The gate binary, §The manifest, §What init seeds
and §The consumer smoke — out of `installer/README.md` and into
`installer/SPEC.md`. Landing that amendment **first** costs one uniform citation
sweep; landing it second costs authoring those five passages twice. Every
`installer/README.md §<section>` named below therefore reads as *that section,
in whichever of the two files currently holds it*. That is the only coupling
between the two amendments.

Deltas 1, 2, 3, 7, 8, 9, 10 and 11 are the queue entry's five deliverables.
Deltas 4, 5, 6 and 12 are not additions to that scope — they are what the
deliverables cost once measured: delta 4 repairs a silent breakage delta 1
causes, deltas 5 and 6 are what make the word *resolvable* in deliverable (b)
true rather than asserted, and delta 12 stops a shipped arm from refusing
without a cause.

## What changes

### (1) The payload withholds a declared shape, applied per kit root

`pack_tracked` takes the withheld shape as an explicit argument and applies it as
`git archive` pathspec exclusions; the pack loop's kit-root call site passes the
declared shape and the `installer/` call site passes none. {design-bearing}

The shape is one knob, not a per-kit list: `GATE_SDK_PAYLOAD_WITHHOLD`,
`Shape::Scalar`, default `SPEC.md smoke` — whitespace-separated root-relative
members, each contributing `:(exclude)<root>/<member>` and, for a member that
names a directory on disk, `:(exclude)<root>/<member>/*`. One shape reaching
every root the loop yields is what keeps the shipped set derived from the
governed set; a roster naming kits would be the maintained copy
derivation-first refuses, and would let a kit fall out of the withholding by
being forgotten.

The exclusion rides `git archive`'s pathspec rather than `tar --exclude`. The
pipeline's fragile half is `tar`'s reproduction of what it is handed — the
ground §Consumer payload's symlink paragraph already states — so the half that
decides *what* is handed over is the half to narrow, and a pathspec needs
nothing of `tar` at all.

**Grounds, measured over the tracked tree at the head this amendment was
authored against.** The eleven kit `SPEC.md` files are 2,804,400 bytes of the
kit roots' 4,224,401 tracked bytes and the eleven `smoke/` trees a further
142,980 across 21 files, so the withheld set is 69.8% of what the payload
carries out of those roots. Neither is load-bearing for a consumer: canon-kit's
finders prune a vendored kit root by default, opt-out only through
`CANON_KIT_SCAN_KIT_ROOTS`, so no consumer battery ever resolves a vendored
gate's pointer; and a kit's `smoke/install.sh` is the copy-vendoring path's
installer, which the installer's own `init` recipe replaces — `init` derives a
kit's consumer config from `templates/*-config.sh` and `templates/*-config.knobs`
and its queue seed from `templates/TASK-QUEUE.md`, reaching `smoke/` for nothing.
Beyond size, a kit's SPEC is the surface most likely to carry its publisher's own
rule content, and withholding it keeps a publication decision from riding on the
completeness of a sweep.

**Refused, with its reason on record rather than left to be re-derived:** a
generated extract carrying only the sections shipped pointers cite. It buys
offline reading for a reader the publication already serves, at the standing cost
of a projection and a freshness gate, and it leaves the publisher's prose in the
payload for exactly the sections most likely to carry it.

### (2) The archive's symlink pre-flight narrows to the set actually packed

The `git ls-files -s` pre-flight takes the same pathspec the archive takes, so it
refuses on a tracked symlink that will ship and not on one inside a path the
payload now withholds. {design-bearing}

**The reader's red condition, named rather than inferred from the narrowing.**
The pre-flight's verdict is *refuse iff the packed set contains a `120000`
entry*, which is monotone in the packed set: narrowing can only remove
refusals, and every refusal it removes names a path the payload no longer
carries. The fail-closed guarantee is therefore unchanged in reach — it still
covers precisely what is packed, which is what §Consumer payload claims for it —
while the class of refusals with no available remedy is emptied. Leaving the
pre-flight wide would refuse a pack over a symlink inside a withheld `smoke/`
tree, a refusal an operator could satisfy only by deleting content the payload
was never going to carry.

### (3) The clean-tree footprint stays wide, and its second ground is stated

§The packer's footprint keeps each kit root whole — no member is dropped because
it is now withheld — and the passage gains the ground delta 1 creates for it.
{mechanical}

A withheld `SPEC.md` is not an unread file. It is the file the on-site mirror is
generated from, and the mirror is where delta 6 sends a shipped pointer. A pack
taken over a dirty SPEC stamps a commit whose published statement may not be the
one a red gate sends its adopter to read. Narrowing the footprint to the packed
set here would trade a real precondition for nothing: the footprint's subject is
the tree under test, not the tarball's manifest.

### (4) A vendored tree's kit-root set is declared, not inferred

`init` writes `GATE_SDK_KIT_DIRS` into `<gates-dir>/gate-sdk-config.knobs` from
the payload's own kit set, and that seam file's creation stops being conditional
on the payload carrying an artifact. {design-bearing}

**The cause is measured, not anticipated.** The derived resolver admits a
sibling directory as a kit root by testing for `checks/` or `smoke/` on disk
(`native/src/walk.rs`, `roots_from`). Probed over the tracked tree, two kits
carry no `checks/` at all — drift-kit (`README.md`, `SPEC.md`, `smoke/`,
`templates/`) and guard-kit (`README.md`, `SPEC.md`, `gate-tests/`,
`guard-tests/`, `lib/`, `smoke/`, `templates/`). Delta 1 therefore makes both
unrecognizable as kit roots in a vendored tree, silently and with no red: a root
that drops out of the set contributes no gate, and a battery over a smaller set
is still a green battery.

**No disk marker repairs it, and the elimination is the ruling's grounds.** The
only member every kit still ships once `SPEC.md` and `smoke/` are withheld is
`README.md`, which at a consumer's repository root names nothing in particular.
`gate-tests/` repairs guard-kit and not drift-kit. `templates/` is the only
marker both carry, and it is a directory name a consumer's own repository root
commonly holds — admitting it would read a consumer's own tree as a kit root,
widening every kit-root sweep onto content no kit owns. A predicate that cannot
be made both sufficient and specific is the wrong instrument.

**So the resolver is right where it runs and wrong where it does not.**
Inference belongs to the repository that owns the kits, where the markers are
present and the siblings are known. A tree that *received* the kits has a better
source: the set is a fact the payload already carries, and `GATE_SDK_KIT_DIRS` is
the override that already replaces the derived set when it is non-empty. The
declaration therefore mints no name and adds no mechanism; it uses the one that
exists for it.

**The seam file gains a second owned line and loses its condition.** Today the
file is claimed and written only inside the branch that has selected an artifact
target, so a verb reasoning about the surfaces `init` rewrites must not assume it
is present. After this delta `GATE_SDK_KIT_DIRS` is written on every install and
`GATE_SDK_NATIVE_BIN` on an install that placed an artifact, as now; the file is
therefore always present and that caveat retires. Every other line is preserved,
on the rewrite rule §The install location has one owner already states, so a
consumer who adds their own kit to the value keeps it — and `init` reports the
edited seam rather than overwriting it, which is the existing discipline for a
surface whose whole purpose is to be edited.

### (5) A red gate presents its own invariant at the point of the block

The runner prints the failing gate's descriptor `# spec:` line beneath its
`FAIL:` line, on every red, in every tree. {design-bearing}

**The gap this closes is presentation, not reach, and the distinction is
load-bearing.** The invariant is *not* unreachable and never becomes so: the
`.gate` descriptor ships, `checks/` is not withheld by delta 1, and the
descriptor's `# spec:` line carries the pointer and a one-line statement of the
invariant after its em-dash. What is missing — today, in this repository, and
independently of anything delta 1 does — is that nothing **presents** it at the
moment a gate blocks a commit; it is reachable only by a detour the blocked
author has to think to take. So delta 1 does not create this gap and does not
break the second shipped member's reason. What delta 1 changes is the **cost of
the detour**: the file at the end of it stops being in the tree. An amendment
claiming otherwise would hand the next reader a premise one `head` of a `.gate`
file refutes, so the claim is stated at the width it actually holds.

It is worth closing at that narrower width because an unactionable block is how a
blocking gate turns into a bypassed one, and this repository's standing rule is
that a red gate is fixed and never bypassed. A remedy that arrives with the block
is what keeps that rule cheap to follow.

**The reach is wide — every red, every tree — and that is a recorded lead
decision of this iteration rather than a ruling or an operator direction; a later
scope or spec may revise it.** With the premise corrected, the narrow form
(printing only where the SPEC is withheld) buys nothing: it would condition an
output line on a payload property, leaving this repository's own battery output
and a vendored tree's gratuitously different, and it would make the one surface
that proves the behaviour — this tree's battery — the one surface that never
exercises it.

**Three coupled readers, enumerated by probe, so build does not meet them at
validate.** The producer is the runner's `tail` field, formatted at each of its
three `FAIL:` sites; the new line attaches there. Probe for the readers:
`grep -rn 'FAIL: ' --include=*.rs --include=*.sh` over `native/src`,
`gate-sdk/lib`, `gate-sdk/bin` and `installer`. Each reader's condition differs,
and reading them as one rule is how two of the three get missed:

- **The gates-log adapter** keys on the exact prefixes `"  PASS: "` and
  `"  FAIL: "` — two leading spaces — and takes the gate name as the line's
  second whitespace-delimited token. Its own fixture asserts that a line indented
  otherwise maps to nothing. So its condition is on the **prefix**, and is not
  the weaker *use a separate line*: a separate line beginning with those two
  spaces would emit a scenario for a gate that never ran. Satisfied by a line
  that misses both prefixes.
- **The overhead meter** holds a `contains`-based marker table in which
  `FAIL: check-` classifies a transcript line as gate output. Its condition is
  therefore on **content, not position**: an extra line carrying that substring
  would inflate a measurement wherever it appeared. Satisfied because the new
  line carries the descriptor's `# spec:` body and not a verdict.
- **The demo excerpt** quotes a reddened gate's block from its
  `===== <gate> =====` header and **terminates on a line containing
  `FAIL: <gate>`** — its stated purpose being that the reader sees the finding
  and the help line rather than being told they exist. Its condition is on
  **order**, and it is the one reader this delta cannot satisfy by staying out of
  the way: printed beneath the verdict, where a reader wants it, the new line
  falls outside the excerpt and the demo silently stops showing the remedy it
  exists to show. **So this reader changes**: the excerpt's terminator moves past
  the invariant line, and its fixture moves with it. That is the delta's one
  non-trivial coupling, and it is named rather than discovered.

What this does **not** claim: the descriptor's one line is not the SPEC section.
It is the actionable statement of what the gate holds, which is what an adopter
at a blocked commit needs; the section behind it is where the reasoning lives and
is reached by delta 6.

### (6) The payload carries the rule that resolves a pointer

A new knob `GATE_SDK_SPEC_BASE_URL`, `Shape::Scalar`, default empty, names where
a pointer's SPEC is published; where it is set, a pointer `<dir>/SPEC.md
§<heading>` resolves to `<base>/<dir>/SPEC#<anchor>`, the anchor being the
heading's own auto-generated id. {design-bearing}

Empty means *resolve in the tree*, which is today's behaviour and remains correct
for this repository and for any consumer who vendored by hand from the public
repository. The value is the publisher's own and never a kit literal: this
repository sets it in `scripts/gate-sdk-config.knobs`, beside
`GATE_SDK_GRAPH_EXTERNAL_REFS`, which already carries the same host for the same
reason. The packer writes the resolved value into the payload stamp beside the
version and the commit, and `init` writes it into the consumer's knob seam with
delta 4's line.

**The pointer grammar is untouched, and that is the ruling rather than an
omission.** A directive's target is a repo-relative path with an optional
`§<heading>` fragment and nothing else; a URL written into a pointer is read as a
path, fails the tracked-file test, and is reported as a missing target. Adding a
URL form would mint a second grammar for one fact, rewrite 387 shipped directive
lines, and put the publisher's host inside kit source — three costs for a
resolution one knob already carries.

**The honest bound, stated wherever the resolution is stated.** A published
mirror serves the *current* statement of an invariant, not the statement at the
commit a tree was vendored from. What resolves that exactly is the manifest's
`commit`, against the public repository. Neither surface may present the mirror
as version-pinned.

### (7) §Consumer payload's second shipped member is restated

The disclosure section says what now ships and what is now resolved, in place of
the sentence promising the SPEC section itself. {design-bearing}

**Not yet applied.** Replacement for the *payload withholds the predicate*
paragraph:

> **The payload withholds the predicate and the engineering record.** A gate on
> the binary substrate reaches a consumer as its `.gate` descriptor, its
> `# spec:` pointer, the one-line invariant that pointer's directive carries,
> its `good/`+`bad/` fixture pair, and a prebuilt, digest-verified binary.
> **Its implementation source does not ship, and neither does the SPEC file the
> pointer names.** A consumer receives everything needed to run a gate, act on
> its verdict, and verify it behaves as specified. The rule's text is withheld;
> the specification's prose is published rather than packed.

**Not yet applied.** Replacement for the second bullet of *What opacity does not
extend to*:

> - **The `# spec:` pointer, resolved rather than accompanied**, because a gate
>   that goes red without an explicable invariant is an unactionable block, and
>   an unactionable block is how a blocking gate turns into a bypassed one. What
>   that ground needs is that the invariant be *reachable at the block*, not that
>   its file be *present in the tree*: the descriptor's own one-line invariant
>   ships and is printed with the failure, and the section behind the pointer is
>   published at `GATE_SDK_SPEC_BASE_URL` (§Layout and configuration) — with the
>   manifest's `commit` resolving the exact text a tree was vendored from. The
>   SPEC file itself is the majority of the packed bytes and is read by no
>   consumer battery, a vendored kit root being pruned from every finder by
>   default.

**Not yet applied.** Addition to the `pack_tracked` paragraph, after the
symlink sentence:

> The same helper is where the payload's withheld shape is applied, and it is
> applied to the kit roots alone: `installer/` goes through this helper too and
> is packed whole, its own non-shipping content being decided by the package
> roster instead. Because the shape is one declared value rather than a per-kit
> roster, the withheld set cannot drift from the governed one; and because the
> symlink pre-flight reads the same pathspec the archive does, the fail-closed
> guarantee still covers precisely what is packed.

The `<!-- payload-discloses: predicate-withheld -->` marker is **unchanged**, and
so is the disclosure class it declares. Nothing is newly withheld *from* a
consumer here: the SPEC section is still disclosed, at a published location
instead of inside the tarball. A change of transport is not a change of
disclosure, and editing the marker would assert one.

### (8) The installer smoke witnesses both halves

`installer/consumer-smoke/run-smoke.sh` asserts that the vendored tree carries no
kit `SPEC.md` and no kit `smoke/`, and that the kit-root set the battery resolves
equals the manifest's `kits`. {design-bearing}

The second assertion is not symmetry. The suite's existing assertion is that the
battery is **green**, and a kit root that silently drops out of the resolved set
yields a smaller battery that is still green — so green cannot witness delta 4.
An assertion that *counts* is what the withholding needs, and the manifest's
`kits` key is the count already recorded. The first assertion is the withholding's
own oracle: it fires on a packer that stopped excluding, which is the regression
delta 1 can suffer silently.

### (9) Each kit README says where its SPEC is published, and its links stay relative

Every kit README gains one sentence naming the publication; the markdown links
to its own `SPEC.md` are **unchanged**, and no README names a host.
{design-bearing}

**The link target was re-derived and the shape was re-ruled.** The corpus is 18
links across all eleven kit READMEs, not the nine across four this delta was
authored with — the probe is an anchored grep for `[...](...SPEC.md...)` over
`*/README.md` restricted to the README's own kit, and it yields
`gate-sdk` 3, `lifecycle-kit` 4, `doctrine-kit` 3 and one each for the remaining
eight. Re-targeting them to the site path was **refused**, on three grounds that
point the same way:

- It is a **kit literal carrying the publisher's host**, exactly the class the
  `<KIT>_<KNOB>` convention exists to prevent, and exactly the cost delta 6
  already counts when it refuses a URL form for the pointer grammar. This
  amendment's own seam ruling says nothing here names a host; a rewrite that put
  one in eleven kit files would contradict it.
- The site mirror is a byte projection of these same READMEs, and
  canon-kit/SPEC.md §The reference-link grammar rules that the mirror
  **preserves the documents' cross-citation topology one-to-one**. An absolute
  URL in the source turns the site's own relative kit-page topology into
  absolute self-links.
- A root-relative site path is read by `check-md-refs` as a repo-relative path
  and reds.

Shipping the URL now and filing the debt was put up and refused too: it lands
the seam crossing and defers only the cleanup.

**The residual cost, accepted knowingly rather than missed.** A vendored
adopter's kit README still carries a relative link to a file their tree no
longer holds. This delta **narrows** that break — the README now says where the
document is published, so a reader meeting the dead link has the answer on the
same page — and it does not close it. A later unit that wanted to close it
would resolve a README's link target from `GATE_SDK_SPEC_BASE_URL` at pack
time, the way a shipped pointer already resolves; that is a gap to file, not
work this delta starts.

### (10) The disclosure restatements are corrected

`docs/install.md` §What a gate discloses and `installer/README.md` §What this
package is are rewritten to the delta 7 statement, and §Vendoring the kits gains
the one sentence the two paths now differ by. {design-bearing}

**Not yet applied.** Replacement for `docs/install.md` §What a gate discloses,
second paragraph-opening sentence onward:

> A gate whose implementation is a compiled binary ships four things, withholds
> one, and publishes one. It ships its declaration, which carries a one-line
> statement of the invariant the gate holds and is printed when the gate blocks
> you. It ships its `# spec:` pointer. It ships its `good/`+`bad/` fixture pair.
> It ships the binary itself, verified against a published digest before
> anything is written to your tree. What it does not ship is the implementation
> source. What it publishes rather than ships is the specification section the
> pointer names: it is read on this site, and the `commit` your
> `checkwright.lock` records resolves the exact text your tree was vendored
> from.

**Not yet applied.** Addition to `docs/install.md` §Vendoring the kits, after
step 1:

> Copying a kit directory out of this repository gives you its `SPEC.md` and its
> `smoke/` as well, which the installer's payload withholds; the manual path is
> the same kits with more of the engineering record attached, never fewer of the
> gates.

**Not yet applied.** Replacement for `installer/README.md` §What this package is,
second sentence onward of the first paragraph:

> What governs your tree afterwards is committed and auditable: every gate
> arrives with its declaration and the one-line invariant that declaration
> carries, its `# spec:` pointer, and its `good/`+`bad/` fixture pair, and a gate
> whose implementation is compiled arrives as a digest-verified binary rather
> than as source. The specification section behind each pointer is published
> rather than packed (gate-sdk/SPEC.md §Consumer payload, which rules that and
> bounds it).

### (11) The front nav leads with install and the per-kit READMEs

The generated-sibling nav suffix keeps `readme` and drops `spec`. {mechanical}

The mirror stays reachable — from each kit's own `index.md`, and from a pointer
delta 6 resolves — and stops being offered as a browsing destination. It is a
reference tier: a surface read when something sends you to it, not one a reader
is invited into from every page.

### (12) The kit-smoke harness names its cause in a vendored tree

`--run-consumer-smoke`'s refusal on a kit root shipping no `smoke/install.sh`
gains the cause, so a vendored tree reads the refusal as a boundary rather than
as a broken install. {mechanical}

The arm runs each kit's `smoke/install.sh` and refuses on a root that ships none.
After delta 1 that is every root in a vendored tree, on every kit. The arm's
subject is *do the kits work when vendored by copy* — a publisher's question,
asked of a tree that has the kit sources — so refusing there is correct; refusing
there without saying why is not.

## Producers and consumers

**`GATE_SDK_PAYLOAD_WITHHOLD` (delta 1).** Producer: the knob table row, read by
`pack_tracked` through the pack loop's kit-root call site — a default-valued row,
so it is set on every invocation of the `--pack-installer` arm rather than only
under a configuration someone remembers to apply. Consumers: `pack_tracked`'s
pathspec construction, and `pack_tracked`'s symlink pre-flight (delta 2), which
reads the identical pathspec. Roster-holding readers of the surface the name
lands on: `pack_installer::KNOBS`, whose declared rows must each be a name a kit
library defines — an undefined name there fail-closes the arm on every run, so
the table row lands in the same change as the declaration; gate-sdk/SPEC.md
§Layout and configuration's knob roster; and the knob-default coupling gate,
which reds on a table default the roster does not state. **Where the definition
goes, measured rather than read off the declaration's own comment:** the table is
`native/src/knobs/gate_sdk.rs`, and `gate-sdk/lib/gate.sh` defines only the
pre-binary subset — `GATE_SDK_KIT_DIRS` is declared in `KNOBS` today while
being defined in the Rust table alone, which settles that a table row suffices.

**`GATE_SDK_SPEC_BASE_URL` (delta 6).** Producer: the knob table row at an empty
default, set for this repository in `scripts/gate-sdk-config.knobs` and for a
consumer by `init` writing the payload stamp's value into the knob seam. The
empty default is the live configuration for this tree and for every hand-vendored
tree, so neither branch is test-only. Consumers: the runner's failure reporter
(delta 5's line gains the resolved location when the value is non-empty), the
packer's stamp writer, and `init`'s seam writer. Roster-holding readers: the same
three as above — `pack_installer::KNOBS`, the SPEC's knob roster, and the
knob-default coupling gate.

**The withheld paths (delta 1).** Producer: `git archive`'s pathspec, once per
packed root. Consumers, named as the readers that would notice their absence: the
kit-root resolver (delta 4 answers it), `check-spec-pointer` (prunes vendored kit
roots by default, so it never reads a withheld SPEC — the pruning is the reason
the withholding is safe and is itself a knob, `CANON_KIT_SCAN_KIT_ROOTS`, whose
opt-out value a consumer who sets it would meet as dangling pointers; stated so
the opt-out is not discovered as a defect), `--run-consumer-smoke` (delta 12), and
the installer smoke's new assertions (delta 8).

**Two more readers of those paths, found by running the installer smoke rather
than by reading, and added because this roster is what a later reader trusts.**
Both are gates the payload installs into a consumer's starting battery, and both
red there once the paths are withheld:

- **`check-gate-assertions`** builds its corpus from `<gates-dir>/SPEC.md` plus
  each kit root's `SPEC.md`; withholding empties it, and an empty corpus is its
  fail-closed exit 2. It passed in a vendored tree only because the kit SPECs
  filled the set. **Its disposition becomes `on-surface`** — in a consumer tree
  its subject is the adopter's own SPEC and their own gates, which is
  gate-sdk/SPEC.md §The install disposition's definition of that value verbatim.
- **`check-install-disposition` assertion B** reds on every `zero-config` member
  of every vendored kit, no kit shipping a `smoke/install.sh` any more. **A kit
  shipping none is skipped and reported on the clean line** — verbatim the
  disposition that section already gives sibling assertion C for the same tree
  class, and what B's own ground ("the smoke's tree is a superset of the tree
  `init` makes") already presupposes.

**Neither repair weakens a gate to make a battery pass, and the test is stated
because that is what the shape resembles.** Neither is a path or glob exemption.
`on-surface` touches install-time arming alone, leaves the rule and the exit-2
wrong-directory guard intact, and leaves this repository's own `gates.list`
registering the gate. Assertion B's skip still fires wherever a kit does ship a
`smoke/install.sh`, which is every kit in the publishing tree. Local coverage is
unchanged by both; what changes is only which tree arms them.

**The kit-root declaration (delta 4).** Producer: `init`, on every install,
writing the payload's own kit set. Consumer: `walk::roots_from`, which returns
the declared set whenever the knob is non-empty and never reaches the disk
predicate there. Every field has a named reader: the value is the same
whitespace-separated root list the knob already takes, so no new field exists.
**Red condition for the narrowing this causes:** the disk predicate stops being
reached in a vendored tree, and the assertion that would otherwise go unwitnessed
— the resolved set matching the manifest — is delta 8's second assertion, which
reds on a count rather than on a verdict.

**The failure line (delta 5).** Producer: the runner, on each `FAIL:`, in every
tree — the reach delta 5 records, so the producer is unconditional and this
repository's own battery is the surface that exercises it. Consumers: the adopter
at a blocked commit, and the installer smoke's value arm, which already records
the battery's verdict on one crafted defect and therefore reads the new line. The
descriptor's `# spec:` line is present on every shipped gate descriptor by the
self-lint contract, so the producer has a value to print for every member; a
descriptor carrying none is already a red before this delta.

**The roster-holding readers of the surface that line lands on, each with its own
red condition.** Three readers parse the run log's verdict lines, and the delta
body enumerates them with the probe that found them. Their conditions are on
three different properties — prefix, content and order — which is why the roster
is stated as three rather than as one rule about separate lines. Two are
satisfied by the new line's shape and **one changes**: the demo excerpt's
terminator, whose red condition is that a reddened gate's quoted block no longer
ends at the remedy. That reader's fixture is the oracle for the change, and it is
listed as an update target below.

**Enumerable corpus, each member's satisfying value (delta 9).** The corpus is
the **kit READMEs**, all eleven, and each member's satisfying value is the same:
one sentence naming where that kit's SPEC is published, and its own-SPEC links
left as they are. No member lacks one. The link corpus those READMEs carry — 18
markdown links, re-derived by the probe delta 9 states — is the corpus this
delta deliberately **does not** change, and the reasons are on the delta.

**Enumerable corpus, each member's satisfying value (delta 1's kit set).** The
corpus is the kit roots the pack loop yields. Probe: `--emit kit-roots` in this
tree, cross-checked against the directories holding a `SPEC.md`. It yields
eleven, and each member's satisfying value under the declared shape is the same:
its `SPEC.md` and its `smoke/` excluded, everything else packed. Two members —
drift-kit and guard-kit — take delta 4 in addition, and no member takes a
per-kit exception.

## Existing sections updated

Each bullet names the delta that owns it. The rosters below name the probe that
produced them and are floors the merging session re-derives, not completeness
claims.

- `gate-sdk/SPEC.md` §Consumer payload — the disclosure paragraph, the second
  *what opacity does not extend to* bullet, and the `pack_tracked` paragraph
  (delta 7); the footprint's second ground is `installer/README.md`'s, below
  (delta 3). Probe: the section's own heading span.
- `gate-sdk/SPEC.md` §Layout and configuration — two knob rows,
  `GATE_SDK_PAYLOAD_WITHHOLD` and `GATE_SDK_SPEC_BASE_URL`, with their defaults
  (deltas 1 and 6).
- `gate-sdk/SPEC.md` §The non-gate arm — the `--pack-installer` declared-names
  paragraph, which states three names and their definition requirement (deltas 1
  and 6).
- `gate-sdk/SPEC.md` §Consumer smoke — the refusal on a root shipping no
  `smoke/install.sh` gains its vendored-tree cause (delta 12).
- `gate-sdk/SPEC.md` §run-gates — the failure line's content, and the verdict
  line's prefix stated as **reserved**, three readers now parsing it (delta 5).
- `gate-sdk/SPEC.md` §Consumer smoke — the demo excerpt's terminator, which moves
  past the invariant line so a quoted block still ends at the remedy, and the
  fixture asserting the excerpt's shape (delta 5). This is the delta's one reader
  that changes; the other two are satisfied by the new line's shape and are
  recorded in the delta body rather than here, having nothing to update.
- `installer/README.md` §The packer — the withheld shape, the narrowed
  pre-flight, and the footprint's second ground (deltas 1, 2 and 3).
- `installer/README.md` §The gate binary — the knob seam's unconditional
  creation and its two owned lines (deltas 4 and 6).
- `installer/README.md` §The manifest — the stamp's new value and the `kits`
  key's second reader (deltas 4 and 6).
- `installer/README.md` §What init seeds — the seam-writing discipline, now
  unconditional (delta 4).
- `installer/README.md` §The consumer smoke — the two new assertions (delta 8).
- `installer/README.md` §What this package is — the disclosure restatement
  (delta 10).
- `docs/install.md` §What a gate discloses and §Vendoring the kits (delta 10).
- `docs/site-architecture.md` §Generated projections and their freshness gates —
  the nav suffix's members (delta 11).
- `canon-kit/SPEC.md` §check-spec-pointer — the vendored-root pruning gains the
  sentence saying what the pruning now protects, its opt-out having acquired a
  consequence (delta 1). Probe: `grep -n 'CANON_KIT_SCAN_KIT_ROOTS'` over the
  tracked tree.
- Every kit `README.md` — one sentence naming the publication, links unchanged
  (delta 9). Probe: as stated on the delta.
- `gate-sdk/checks/check-gate-assertions.gate` — its `# install:` disposition,
  and `gate-sdk/SPEC.md` §check-install-disposition — assertion B's skip for a
  kit shipping no `smoke/install.sh` (delta 1's two further readers, above).
- The on-site SPEC mirror — `docs/<kit>/SPEC.md` and `docs/<kit>/README.md` are a
  byte-gated projection of every source above that this amendment edits, so the
  regeneration is owed with them (all deltas). Probe:
  `docs/site-architecture.md` §Generated projections and their freshness gates.
- `TASK-QUEUE.md` — the entry `payload-withholds-kit-specs` carries
  `[spec: SPEC-payload-withholding.md]` and leaves the design-pending set (all
  deltas). Its terminal move discharges the standing ruling this amendment
  implements; the discharge oracle is the one recorded on that ruling and is not
  restated here.

## Retired spellings

- None — this amendment mints two knob names, `GATE_SDK_PAYLOAD_WITHHOLD` and
  `GATE_SDK_SPEC_BASE_URL`, and retires no spelling: every existing name it
  touches keeps it, and the one thing it removes outright — the clause saying the
  knob seam is absent on an artifact-free payload — is a passage rather than a
  name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through the gap inbox.
- [ ] **The withholding is witnessed** — the installer smoke's two new
      assertions run and pass against a freshly packed payload, and the
      resolved kit-root set in that scratch consumer names all eleven kits.
- [ ] **The verdict line's three readers are re-probed, not assumed** — the
      delta 5 grep is re-run at merge and yields the same three; the demo
      excerpt quotes a reddened gate's block through its remedy; and
      `--emit parse-gates-log` over a log carrying the new line emits one
      scenario per gate that actually ran.
- [ ] **The provenance seam holds in the merged text** — no dated stamp, no
      ledger pointer and no queue slug reaches a kit SPEC; the withholding's
      grounds stand undated as engineering grounds.
