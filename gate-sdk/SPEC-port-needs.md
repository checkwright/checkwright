# SPEC amendment: port-needs

Closes `port-criterion-7-roster-understated`. §The port-candidate criteria's
criterion 7 — *"its rule invokes no external program the payload does not
carry"* — names exactly one gate and one program: `check-action-run-shell` and
`shellcheck`. Criterion 7 is what sequences a gate **last** in the port, so an
understated roster makes the remaining port work look smaller than it is and
mis-sequences any cohort selection that trusts it.

## What the probe found, and why it settles the derive-vs-list question

Probed at HEAD across `*/checks/check-*.sh` and `scripts/check-*.sh`:

| program | gates that require it |
|---|---|
| `shellcheck` | `check-action-run-shell` (gate-sdk/checks/check-action-run-shell.sh:17,193), `check-shellcheck` (gate-sdk/checks/check-shellcheck.sh:27,46) |
| `jq` | `check-installer-no-deps` (scripts/check-installer-no-deps.sh:16,23), `check-settings-pins` (context-kit/checks/check-settings-pins.sh:61,73), `check-memory-off` (context-kit/checks/check-memory-off.sh:82,88) |
| `ruby` | `check-docs-render-fidelity`, via `SITE_KIT_RENDERER`'s default (site-kit/lib/site.sh:32), invoked at site-kit/checks/check-docs-render-fidelity.sh:42,194 |

Six gates, three programs — against the SPEC's one and one.

**The derivation is noisy in both directions, and both directions are attested
rather than predicted.** This is what the queue entry anticipated when it offered
"or, if derivation proves noisy, state the full roster once at the SPEC with a
freshness gate", and the shape of the noise is what decides against that fallback.

*False positives* a text match produces: `ruby` appears at
site-kit/checks/check-docs-render-fidelity.sh:71 as the **HTML element name** in
an awk list of every HTML tag, and at :45 inside a `help:` string; `shellcheck`
appears at canon-kit/checks/check-comment-tier.sh:58 in
`shell_word=(shellcheck assertion)`, a directive-token list. None is an
invocation.

*The false negative* is the load-bearing one: the real `ruby` dependency is never
spelled `ruby` in command position at all. It is `"${SITE_KIT_RENDERER[@]}"`, an
**array knob** whose default's first element is the program. A text match over
the gate's own source finds nothing.

**That false negative is why a stated roster cannot be made correct, and it is
the ruling.** `SITE_KIT_RENDERER` is consumer config. A consumer who repoints it
at a different renderer changes which external program that gate requires — so
**no literal roster in a kit SPEC is true for every consumer**, and a
freshness-gated copy would be freshness-gated against *this* repo's config while
reading as a general claim. Deriving is not merely "preferred" as the entry had
it; listing is unavailable. Recorded because the fallback the entry offered looks
cheaper right up until this is noticed.

## What changes

**(1) `gate-sdk/bin/port-blockers.sh` — the derivation.** [design-bearing] A
report over `gates.list`: for each registered member, resolve its declaration
path, and derive the external programs its rule requires. Emits one line per
member with a requirement, plus a trailing count line naming how many members
were scanned and how many roots were **undecidable**.

Three derivation inputs, in order of confidence:

- **The `command -v <prog>` guard**, which is already the tree's convention for
  exactly this dependency (five of the six gates above announce themselves this
  way). A guarded program is a requirement with no inference.
- **Command position** — a word at the head of a simple command. This is what
  excludes the attested false positives: an element of an array literal, a word
  inside a string, and an awk-internal token are all non-command-position, so
  positional analysis removes them without a per-case exception list.
- **Knob resolution for a command-position expansion.** A command-position
  `"${KNOB[@]}"` or `"$KNOB"` resolves to the knob's default assignment where that
  default is statically readable in the owning kit's `lib/`, taking the first word
  as the program. `SITE_KIT_RENDERER` resolves to `ruby` this way.

**Undecidable is reported, never guessed** — a command-position expansion whose
default the tool cannot statically resolve prints `?` and increments the
undecidable count, adopting §check-reads-couples' precedent exactly: *"a gate
whose author cannot bound a root declares `?` rather than guessing, and the
reader counts it as undecidable instead of trusting it as empty."* A tool that
silently reported nothing for an unresolvable knob would reproduce the very false
negative this amendment exists to close.

**(2) Criterion 7 stops carrying a roster and cites the derivation.**
[design-bearing] §The port-candidate criteria criterion 7 keeps its whole
argument — the dependency floor, the toolchain-free adopter, `git` as the one
sanctioned exception, and the 2026-08-09 ruling that this is *the largest named
piece of port work* rather than a permitted exclusion. What it loses is the
sentence that reads as an enumeration. In its place: the invariant, plus
`bash gate-sdk/bin/port-blockers.sh` as the roster's one home and the recorded
reason a literal cannot be correct (the consumer-config dependence above).

`check-action-run-shell` stays named — not as the roster, but as the *worked
example*, which is what that paragraph was actually written for ("every
mechanical screen puts that gate *in*, and the fact that stops it is one none of
the six criteria sees").

**(3) No gate, and the reason is doctrinal rather than an omission.**
[design-bearing] Enforcement-first ranks *removing* a duplication above gating
it, and delta (2) removes it: with no roster stated, there is no copy to go
stale and nothing for a freshness gate to assert. A gate over the derivation's
own output would have to compare it against a stored expectation — which is the
maintained roster re-entering through the back door, wrong for every consumer
whose config differs. The report is dispatched by a porting session choosing a
cohort, which is the transition where the answer is actually needed.

**(4) The floor set is config, not a kit literal.** [mechanical]
`GATE_SDK_PROGRAM_FLOOR` — array of programs the payload is entitled to assume,
so a command-position word in it is not a requirement. Kit default: the POSIX/
coreutils set the battery already rests on plus `git`, which §The port-candidate
criteria already rules "the one sanctioned exception, because it is the floor".
Config-via-env per the `<KIT>_<KNOB>` convention. A consumer shipping a different
floor repoints it rather than patching the tool.

**(5) The substrate-neutral half is sequenced, not built.** [design-bearing]
§The `.gate` descriptor rules its **field roster closed**, and it explicitly
*refuses* a `# reads:` declaration on the ground that nothing would hold it to the
implementation. A `# needs:` field would earn the identical refusal, so this
amendment adds no descriptor field. The contract generalizes the one already
stated there — *"shell answers by parse, the binary answers by `--reads`"* —
to: **shell answers by parse, the binary answers by `--needs`**, a fifth
top-level flag beside `--list` / `--reads` / `--knobs` / `--source-stamp`, backed
by a fifth registry-tuple element held to behavior by a crate unit test in the
shape of unit test B.

It is **not built here**, and that is a decision with a reason: no ported member
requires an external program today, so the arm would ship with no named reader —
which the amendment contract removes rather than reserves. What lands instead is
the sequencing: the first port of a member carrying an external requirement
builds `--needs` with it. Until then `port-blockers.sh` reports a `.gate`-declared
member as undecidable rather than as clean, so the hole is **counted, not
silent** — the same distinction delta (1) turns on.

**(6) Registration and fixtures.** [mechanical] `bin/port-blockers.sh` is a
report, not a `gates.list` member, so it carries no fixture pair and no
`# graph:` manifest; it is rostered in gate-sdk/README.md beside the kit's other
`bin/` tools, and any command it documents is reachable by `check-docs-cmd`.

## Producers and consumers

**New interface: `bin/port-blockers.sh` stdout report.**
- *Producer* — a session selecting or sequencing a port cohort, invoking it by
  hand. Enabling config: none; it reads `gates.list` and the resolve dirs that
  every gate-sdk tool already reads, so it works in a fresh clone with no knob
  set. `GATE_SDK_PROGRAM_FLOOR` has a kit default, so the tool is never dark for
  want of configuration.
- *Consumer* — the porting session, and gate-sdk/SPEC.md criterion 7, which cites
  the command in place of its former list. There is no machine consumer, stated
  rather than left implicit: nothing parses this output, which is why delta (3)
  ships no gate to read it.
- *Fields, each with a named reader at a named transition* — the **member name**,
  read when the session maps a blocker back to a gate it is sequencing; the
  **program**, read when the session decides whether the payload can carry it or
  the rule must be redesigned; the **evidence** `file:line`, read when the session
  verifies the finding rather than trusting it (probe-before-assertion); the
  **undecidable count**, read as the honest bound on the report's completeness. No
  field is carried that no transition reads.

**New state: `GATE_SDK_PROGRAM_FLOOR`.**
- *Producer* — the kit default in `gate-sdk/lib/gate.sh`, overridden by a
  consumer's `scripts/gate-config.sh`.
- *Consumer* — `bin/port-blockers.sh`, at the transition where a command-position
  word is classified as a requirement or discarded.

**Existing interface whose contract this changes: `SITE_KIT_RENDERER`.** No code
change — but the knob acquires a **second reader** in prose: it is now the
worked example of a knob whose value *is* an external-program dependency, read by
`port-blockers.sh`'s knob-resolution step. site-kit's own SPEC section for it is
updated rather than left to drift, so the knob's owner records that repointing it
moves a port blocker.

**Red conditions of the readers this change touches** (§The causal-completeness
check, point 5 — delta (2) *narrows* a prose corpus by deleting an enumeration,
which is exactly the shape that argument warns about, so each reader is named by
what makes it **red**, not by its subject):
- `check-prose-enum` reds on a prose enumeration that has drifted from its
  derived source — deleting the enumeration removes the pairing it asserts over.
  This is the non-monotone case the point warns about: verify it does not hold a
  **minimum** over criterion 7's paragraph before the deletion lands.
- `check-spec-pointer` reds on a `# spec:` pointer naming a heading that does not
  exist — criterion 7's heading is unchanged, so it stays green, but any pointer
  citing the deleted sentence's sub-anchor would red.
- `check-md-refs` reds on an unresolvable reference — the new
  `bin/port-blockers.sh` path must exist before the SPEC cites it.
- `check-docs-cmd` reds on a doc fencing a command that does not run — this is a
  **zero-count** red condition and it is why delta (6) requires the tool to be
  executable and runnable in a fresh clone before the citation lands.
- `check-knob-citation` and `check-knob-default-coupling` red on a knob mentioned
  without its citation, and on a default spelled in more than one place —
  `GATE_SDK_PROGRAM_FLOOR`'s default is written once, in `lib/gate.sh`.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 7** — owned by
  deltas (2) and (3). The enumeration goes; the invariant, the citation, and the
  recorded reason a literal cannot be correct arrive.
- **gate-sdk/SPEC.md §The `.gate` descriptor** — owned by delta (5). Its "field
  roster is closed" paragraph and its recorded refusal of `# reads:` gain
  `# needs:` as a second field refused on the same ground, so a later author does
  not re-propose it. Its substrate-neutral sentence gains the `--needs`
  generalization as *sequenced*, marked so it is not read as shipped.
- **gate-sdk/SPEC.md §Layout and configuration** — owned by delta (4). Adds
  `GATE_SDK_PROGRAM_FLOOR` to the knob roster with its default.
- **gate-sdk/SPEC.md §lib/gate.sh** — owned by delta (4), the home of the default.
- **gate-sdk/README.md** — `bin/` tool roster, owned by delta (6).
- **site-kit/SPEC.md §lib/site.sh** — owned by delta (1): `SITE_KIT_RENDERER`'s
  section records that its value is a port blocker, so the coupling is written
  where the knob lives rather than only where it is derived.

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
