# SPEC amendment: couples-caution

Queue entry: `couples-caution-unreachable-by-pointer`. It rides unit set `config-seam-second-cut`
beside `config-seam-static-format`'s cut 2 under **operator direction, 2026-09-14, lead-relayed**.

## The question

gate-sdk/SPEC.md §The `# graph:` manifest states that the `couples=` field has two matchers: the
coverage reader matches segment-wise, and the trigger readers use bash string matching, where `*`
spans `/`. It calls reading the narrow rule as universal a live error. That caution sits inside the
`couples=` bullet of the grammar list, after the prefix-token and `knob:` rules, with no heading
and no anchor. The only citation reaching it is CLAUDE.md's pointer to the whole section, which
gets a reader to the right page and leaves them to find the paragraph in one long list item.

The entry offers three surfaces: a sub-heading makes the caution citable, a pointer on a surface
a gate author opens makes it findable, and a second `.gate` `# spec:` pointer makes it reachable
from the artifact. Restating the paragraph anywhere is refused on the entry itself.

**Probed at this stage.** One premise needs correcting. The entry says no gate-authoring template
exists, but `gate-sdk/templates/check-skeleton.sh` does, and it carries the field as a placeholder.
It is the shell skeleton a vendoring adopter without a crate starts from, and its header block is
read by `# spec:`, `# graph:` and `# no-port:` readers. Adding a line there is costlier than the
entry's framing suggests, and delta 3 explains why it is not taken. The trigger reach also has an
executable oracle the caution never names: `run-gates.sh --for <path>`, defined as identical to the
generated hook's matching (gate-sdk/SPEC.md §run-gates). So the fix can point a reader at a command
instead of at a shell function's body.

## The seam

Kit mechanism and kit prose only: a gate-sdk SPEC heading, a gate-sdk README pointer and one
gate's help line. This repo's CLAUDE.md pointer is the consumer side. No consumer config and no
private rule content.

## What changes

### (1) The caution becomes its own sub-section {design-bearing}

gate-sdk/SPEC.md §The `# graph:` manifest. The paragraph that begins **That is the coverage
reader's rule and not every reader's** and ends with *while the trigger's reach is wider.* moves
out of the `couples=` bullet into a new `###` sub-section. It sits directly after the grammar list
(after the `gen=manual` bullet) and before §The install disposition. It is **moved, not copied**:
the bullet keeps the authoring rule it states (*globs never cross `/`*) and gains one pointer
sentence in place of the moved text. **Not yet applied:**

In the bullet, replacing the moved text:

> That rule is the coverage reader's, and the trigger's reach differs from it: §Reading a
> `couples=` field's reach.

The new sub-section, with the moved paragraph's content, and its remedy sentence re-pointed from
the shell function to the oracle:

> ### Reading a `couples=` field's reach
>
> **The field has two matchers, and its text cannot tell a reader which one is asking.**
> `check-reads-couples` matches segment-wise, and the authoring rule above is written for it: a
> glob never crosses `/`. The **trigger** readers do not match that way. The generated hook's
> `staged_matches` is spliced from `gate_staged_matches` in `lib/gate.sh`, whose
> `[[ "$f" == $pat ]]` leaves the pattern operand unquoted under a standing
> `# shellcheck disable=SC2053`. That is bash *string* matching, in which `*` spans `/`, and
> `run-gates --for` reaches the same matcher. So `native/src/*.rs` **does** fire the hook on
> `native/src/emit/mod.rs`, while **not** covering a read there. Reading the narrow rule as
> universal is a live, attested error: applied to a trigger, it says a gate will not run when it
> will.
>
> **Read the reach through the oracle for the question asked, never off the field.** Run
> `run-gates.sh --for <path>` to learn what a path triggers, and `check-reads-couples` to learn
> what a gate's couples cover. **Do not "fix" the unquoting**: it is declared intent, and quoting it
> would break every trigger in the tree at once. Which semantics the field *should* have is not
> settled here and is filed as its own deliverable. What is settled is that the authoring rule
> stays conservative for the coverage reader while the trigger's reach is wider.

The closing sentences of the bullet (`check-graph` verifies couples→hook parity,
`check-reads-couples` mechanizes reads⊆couples, and the author's remaining duty) stay in the bullet,
since they belong to the authoring rule and not to the caution.

### (2) The always-loaded pointer targets the sub-section {mechanical}

CLAUDE.md, the sentence beginning **Never read a `couples=` field's reach off the field**. Its
citation `gate-sdk/SPEC.md §The \`# graph:\` manifest` becomes `gate-sdk/SPEC.md §Reading a
\`couples=\` field's reach`. The line keeps its length class, so the always-loaded budget does not
move. **Not yet applied.**

### (3) The kit README points a gate author at it {mechanical}

gate-sdk/README.md, the `bin/gen-pre-commit.sh` bullet that introduces per-gate `# graph:` coupling
manifests. It gains one clause, a pointer and not a restatement. **Not yet applied:**

> …so hook membership cannot drift. A manifest's trigger reach is read with `--for`, never off
> the field ([SPEC.md](SPEC.md) §Reading a `couples=` field's reach).

This is the surface a gate author opens first for the kit: the README introduces both the manifest
and the skeleton. The skeleton itself gains nothing. Its header block is read field by field, and
a comment inserted there would be copied into every adopter gate born from it, where the skeleton's
own `# no-port:` line already has to tell the adopter to delete a line.

### (4) The coverage gate's help names the trigger difference {mechanical}

`native/src/gates/reads_couples.rs`, the red help line. It currently says *globs never cross '/'*,
which is exactly the sentence the attested misreadings generalized to the trigger. It gains a
trailing pointer. **Not yet applied:**

> …Never widen a glob to cross '/' to pass a near-miss. That is this gate's matcher, not the hook's:
> gate-sdk/SPEC.md §Reading a `couples=` field's reach.

This is the moment the misreading happens in practice. An author meets this red, edits `couples=`,
and reasons about what the edit triggers. If the gate's fixture pair asserts the help text, its
expected output follows.

**Refused, with grounds (they stay here and in git history):**

- **A second `# spec:` pointer on every `.gate` descriptor.** The descriptor's field roster is
  closed, and each field has one named reader. `# spec:` is `check-spec-pointer`'s, and
  §check-gate-substrate-parity assertion H opens the section it names. A second pointer is either a
  new field with no reader or a second value under one field name that both readers would have to
  learn to skip. In both cases every descriptor in the tree would carry one identical line, which is
  a copied pointer rather than a findable one.
- **A new gate-authoring template for descriptors.** It would be a new surface with no reader, and
  it would carry the grammar a second time next to the SPEC that owns it.

## Producers and consumers

- **The sub-section heading** (new interface: a citable anchor). *Producer:* delta 1.
  *Consumers:* CLAUDE.md's pointer (delta 2), gate-sdk/README.md (delta 3) and `check-reads-couples`'
  help (delta 4), all resolved by `check-spec-pointer` and canon-kit's section-citation readers,
  and the generated mirror `docs/gate-sdk/SPEC.md`.
- **Existing citations to §The `# graph:` manifest** stay valid, because the parent heading is
  unchanged. Probed with `git grep` at this stage: CLAUDE.md is the only citation that means the
  caution specifically, and delta 2 moves it.
- **Red conditions (point 5).** No corpus narrows. `check-spec-pointer` and the section-citation
  gates red on an unresolved `§` target, and the new heading adds a target, so they are monotone.
  `check-brevity` and the always-loaded ratchet measure CLAUDE.md, and delta 2 keeps the line's
  length class. `check-reads-couples`' fixture pair reds if its expected output pins the old help
  line. Build runs the pair.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The `# graph:` manifest, and the new §Reading a `couples=` field's reach
  (delta 1).
- `CLAUDE.md` — the `couples=` pointer (delta 2).
- `gate-sdk/README.md` — the `bin/gen-pre-commit.sh` bullet (delta 3).
- `native/src/gates/reads_couples.rs` — the red help line (delta 4).
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md` — generated mirrors, regenerated (all deltas).

## Retired spellings

- None — no delta retires a name; delta 1 adds a heading and every citation to the parent section
  still resolves.

## Definition of Done

- [ ] **Causal completeness** — the heading has its three pointers, and each one resolves.
- [ ] **Instruction surfaces: instruction only** — the help line and the README clause point; the
      grounds stay in the SPEC.
- [ ] **Merged with no information lost** — the moved paragraph lands whole in the sub-section.
- [ ] **Amendment deleted**, and the entry moves to Done.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — none expected.
