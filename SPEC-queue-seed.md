# SPEC amendment: install-queue-template-unreachable

`queue-kit/templates/TASK-QUEUE.md` is unreachable at install by construction, so
no adopter has ever received it. `recipe_seed`'s queue arm runs once per kit and
writes on the first kit reached for which `recipe_needs_queue` holds and the queue
file is still absent; that kit's own `templates/TASK-QUEUE.md` is used when it
ships one, else a minimal inline skeleton is written. A kit that ships the
template can therefore never be reached if an earlier one does not.

**Re-verified across every profile that exists, not only `full`.** `full` reaches
canon-kit first (payload order is alphabetical, from
`profile_payload_kits`' glob) and canon-kit ships no such template.
`delegation` carries no canon-kit, but `profile_kits` re-sorts a roster's rows into
payload order, so it reaches lifecycle-kit before queue-kit — and lifecycle-kit
ships no such template either. `starter` carries none of the three, so it seeds no
queue at all. There is no profile in which queue-kit is reached first, and the
unreachability is a structural fact of the tree rather than a hypothetical.

## The ruling

The entry framed two honest closes: an explicit owner declaration replacing the
sort accident, or deleting the unreachable template. **Both are refused, and the
evidence that refuses them is the same fact neither branch anticipated: the inline
skeleton the adopter actually receives is not conformant to queue-kit's own
default section contract.**

`QUEUE_KIT_REQUIRED_SECTIONS` defaults to a six-heading set including
`Lessons Learned`. The inline skeleton writes five headings and omits it.
`check-queue-sections` is the gate that would say so, and it is the one queue-kit
gate declared `on-surface` — so it is not in the registry `init` writes, and
nothing catches the miss. That is not a cosmetic gap: §check-queue-sections
states that this gate is the fail-closed floor under every section-scoped scanner,
and `check-amendment-queue`, `check-task-names` and `check-task-conservation` —
all `zero-config`, all registered on that tree — **pass open** on a dropped
heading rather than reddening. So `init` writes a queue file that silently
under-gates itself, and the adopter's first red arrives later, on the file `init`
wrote, the day they register the floor.

Against that, the two filed branches fail:

- **Deleting the template is now clearly worse, not "not obviously worse".** It
  would fix the unreachability by making the impoverished skeleton the only
  outcome forever, discard the worked example of every entry-grammar shape that
  queue-kit/SPEC.md §templates/ specifies as the artifact's purpose, and redden
  `queue-kit/smoke/install.sh`, whose unconditional `cp` of the template runs
  under `set -euo pipefail`.
- **An explicit owner declaration is new mechanism with no precedent and a
  standing prohibition against the nearest one.** No "which kit owns X"
  convention exists in the tree; `# install:` is the closest shape and its
  vocabulary is barred from naming a path or a surface
  (gate-sdk/SPEC.md §The install disposition). Declaring an owner would also be a
  maintained roster where a derivation is available.

**The arm should not select by kit at all.** A kit that ships
`templates/TASK-QUEUE.md` has already declared itself the format's owner by
shipping it; that is the derivation-first answer and it needs no second surface to
state the same fact. The defect is structural rather than a bad tie-break: the
selection is made *inside* a per-kit call that cannot see the other kits, so the
first kit reached decides before any kit that ships a template gets a turn.

## What changes

- **(D1) Hoist the queue seed out of the per-kit loop.** `init` resolves the queue
  file once per install, over the profile's whole resolved kit set: take the
  template shipped by the first kit in that set that ships one, and write the
  inline skeleton only when **no** kit in the set ships one. Payload order stops
  deciding anything here — the outcome is the same for any ordering of a given kit
  set, which is what makes the rule a derivation rather than a better accident.
  **design-bearing** — it moves a write across a loop boundary that two other
  mechanisms are keyed to (below), and getting the plan/perform split wrong is
  silent.

- **(D2) Make the inline fallback conformant.** The skeleton gains the
  `## Lessons Learned` heading, so the file `init` writes satisfies
  `QUEUE_KIT_REQUIRED_SECTIONS` at its default. **mechanical** — one heading, and
  the oracle is registering `check-queue-sections` against a scratch consumer.

- **(D3) Assert the reachability the smoke does not.** `installer/consumer-smoke/run-smoke.sh`
  never mentions the queue file; it is the one harness that drives the real
  `recipe_seed` path end to end and it asserts nothing about which arm wrote the
  queue or what the file contains. Add, per profile that seeds a queue: the file
  exists, and `check-queue-sections` is green against it. This is the
  enforcement-first half — the defect and the gate that catches it land in one
  unit — and it is what stops D1 and D2 from silently regressing.
  **design-bearing** — the assertion has to hold for both outcomes of D1 (a
  kit-shipped template and the fallback) without asserting which one a profile
  got, since that is a property of the profile's kit set rather than of the
  installer.

- **(D4) State the selection rule in `installer/README.md` §What init seeds.**
  One sentence beside the existing derived-roster and derived-config-seam
  paragraphs, which it joins as a third derivation. **mechanical** — the rule is
  D1's; this is its prose tier.

## The fallback stays reachable, and this iteration is why

Before D1, the inline skeleton is what every profile receives. After D1 it is
reached only by a profile whose kit set contains a `recipe_needs_queue` kit and no
kit shipping the template — which is empty today and **will not be after the
`prose-profile` amendment lands in this same iteration**: a prose profile carrying
canon-kit without queue-kit is exactly that shape. So the fallback is not dead
code kept for symmetry, and D2 is not hygiene: it is the path a profile shipping
this iteration will take. The two amendments touch one file for one reason.

## Producers and consumers

The change introduces no new message or field. It relocates one producer and adds
one assertion; the causal-completeness obligation is that every mechanism keyed to
the producer's old position is named and moved with it.

- **Producer (relocated)** — the queue-file write. Today: `recipe_seed`'s queue
  arm, `installer/lib/common/recipe.sh`, called per kit from `init.sh`'s per-kit
  loop. After D1: a once-per-install resolution over `KITS[]`, the array
  `init.sh` already builds from `profile_kits`. Enabling config: none; it runs on
  every non-dry `init` whose resolved kit set contains a `recipe_needs_queue` kit.
- **Consumer of the emitted path** — `init.sh`'s `claim "$p" && record "$p"`
  channel, which puts the seeded file on the manifest's `files[]`. **The hoisted
  step must feed this same channel**, and it must run in the parent shell, not
  inside a process substitution: `init.sh`'s own directive there records that a
  producer inside the substitution has its `record()` and `CHANGED` appends
  discarded. A hoist that lands the write outside that channel produces a file
  `init` wrote and did not record — which reads as "never installed" on the next
  run and, per §The consumer smoke, breaks the tree-object equality the uninstall
  arm asserts.
- **Second consumer, and the one most easily missed** — the `--dry-run`
  prediction. `init.sh` predicts the queue path in a **separate** `DRY` branch
  that re-spells `recipe_needs_queue "$kit" && [[ ! -f … ]]`. That prediction is
  keyed to the per-kit loop too, and D1 must move it with the producer or the dry
  plan silently stops matching the run it predicts. The file's own directive
  already warns against exactly this class ("the plan needs no dry variant and
  cannot drift from the run it predicts"), so the fix is to bring the queue arm
  under that discipline rather than to keep a second spelling in step.
- **Consumer of the seeded file's content** — queue-kit's `zero-config` gates,
  registered on that tree by `recipe_gates`; and `check-queue-sections` once the
  adopter registers it. D2's heading exists for the second of those, which is the
  named reader D3 then makes standing.
- **Unaffected, checked rather than assumed** — `plan_gates`' gate-name dedup is
  the one other site keyed on payload order ("a member registered by more than one
  kit lands once, under the first of them"). D1 changes no ordering it reads, and
  the two collisions are different in kind: gate names against template files.
  `profile_order` and the smoke's monotonicity assertions are set-based and
  order-independent.

## Existing sections updated

- `installer/README.md` §What init seeds — D4's sentence; the section already
  owns the two-disciplines rule the hoisted step must keep obeying (seed when
  absent, never claim-and-copy).
- `installer/README.md` §The consumer smoke — D3's per-profile assertion joins
  the ordered list of post-conditions that section states.
- `queue-kit/SPEC.md` §templates/ — the template's contract paragraph is
  **correct and stays**, but it describes an artifact that was undeliverable; a
  clause recording that the installer now selects it by its existence belongs
  beside it, so the kit's own doc stops implying a delivery it did not get.
- `installer/lib/common/recipe.sh` and `installer/lib/init.sh` — the `spec:`
  directives on the moved code cite D4's section.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
