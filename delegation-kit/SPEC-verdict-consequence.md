# SPEC amendment: verdict-consequence

Rules the two-contradictory-STALE-steers defect filed as
`usage-verdict-stale-steer-contradiction`. The queue entry left the ruling open
between **(a)** converging the strings and **(b)** declaring the split
intentional behind a reader-tagging convention. **This amendment rules (a)**,
and rules it structurally: the two strings do not become two synchronized
copies of one sentence, they become **one string with one producer**.

## The ruling, and why (b) is ruled out

The defect is not a disagreement between two readers. It is **one incomplete
sentence**.

A verdict line carries two separable things: the **epistemic status** of the
reading (how far to trust the number) and the **decision consequence** (what
the reading gates). `usage-verdict`'s own STALE line states the status and
omits the consequence. The hook path's line states the consequence. Neither is
wrong about its own half; the direct-run line is simply missing a half, and a
reader supplies the missing half by inference — inferring "do not trust the
number" into "do not act", i.e. do not dispatch. That inference is the attested
failure, and it is why the corrective is structurally unreachable: the hook
fires *on* dispatch, which is exactly the act a session hesitating on STALE has
not yet performed.

**(b) is ruled out on record.** A reader-tagging convention presumes a reader
for whom "STALE gates delegation" is the correct read. There is none. A human
running the bin directly wants to know how far to trust the number — advice
about the *number*, never about whether to delegate; a dispatching agent wants
the dispatch consequence. One complete sentence serves both. Tagging would add
a governed convention to carry a distinction that does not exist, and would
leave the attested failure reachable — a session reading an agent-tagged string
still needs the consequence spelled out to not infer the wrong one. The split
is an omission, not an intention.

## What changes

**1. The verdict-string contract** (new, owned by `delegation-kit/SPEC.md`
§usage-verdict). Every line `usage-verdict` emits carries three parts: the
**reading** (the measured fields), the **epistemic status**, and the
**decision consequence** for delegation. A verdict string that states status
without consequence is incomplete by contract — the reader completes it by
inference, and the inference is unconstrained. This is the invariant the rest
of the amendment applies; it is stated once here and lands in §usage-verdict at
merge.

**2. Every STALE emission site gains the consequence clause.** `STALE` is
emitted at five sites in `delegation-kit/bin/usage-verdict.sh` — the unreadable
file, the missing-key parse failure, the non-numeric percentage, the age
threshold, and the login-window reroute. Today only the last two carry any
trailing prose at all; the three fail-closed sites carry none. All five gain the
consequence, so no path to STALE can print a status without its consequence:

> `STALE (… ; never blocks delegation — re-read or refresh before trusting the
> number)`

The status half stays site-specific (each site already names *why* it is
stale); the consequence half is uniform, because the consequence *is* uniform —
STALE is budget-unknown at every site.

The clause appends to the tail, *after* the ` -> STALE` arrow. This is what
keeps it compatible with the `width=<n>` invariant §usage-verdict already
asserts — that every emitted verdict line carries `width=<n>` immediately
*before* the arrow. Nothing lands between the width field and the arrow, so the
two contracts occupy disjoint halves of the line and neither constrains the
other.

**3. The guard's advisory tail is deleted, not synchronized.** Once the verdict
line carries its own consequence, the `2)` case arm in
`delegation-kit/templates/agent-budget-guard.sh` (and this repo's consumer copy
`scripts/agent-budget-guard.sh`) restates the payload it is relaying. It
collapses into the existing `*)` arm, leaving **one** advise arm that relays the
verdict line and adds nothing. The arm's "needs no override" clause dies with
it: it is implied by "never blocks", and the override instruction that matters
already lives on the PAUSE arm, which is the only arm that blocks.

**The two guard copies are not byte-identical today, and must not be made so.**
They differ on line 2 — each carries its own `# spec:` header, the template's
naming the settings-registration matcher the consumer copy has no need to
restate. That divergence is deliberate and predates this unit. Build edits the
two case blocks by hand in both files and leaves line 2 alone; "sync the
copies" is the wrong instruction, "apply the same body edit twice" is the right
one. No gate holds them together, so the hand-edit is the only mechanism.

This is the enforcement-first ruling. Two strings that must agree is a drift
class needing a gate; **one string with one producer is not a drift class at
all** — removing the duplication outranks gating it (CLAUDE.md §Delivery
doctrine). No new gate ships for this unit; the duplication it would have
policed ceases to exist. `delegation-kit/SPEC.md` already asserts the guard
"relays the verdict line verbatim" (§usage-verdict, inside the `width=<n>`
paragraph — the sentence is embedded there, not free-standing, which is where
build will find it). Precisely: what the change makes true is that the guard no
longer *restates the payload's content*. Both advise arms still prefix the
relayed line with `budget verdict (agent-budget-guard):`, and the PAUSE arm
still appends its corrective — "verbatim" governs the verdict line's own text,
never a claim that the guard emits it unadorned. It is the surviving contract,
and it survives unweakened.

**4. The two SPEC sections converge on one owner.** §usage-verdict owns the
verdict-string contract, including the consequence clause. §The delegation
model stops restating "STALE never blocks" as an independent assertion and
cites §usage-verdict for it, keeping only what is genuinely its own: the
*routing* rule (which exit code reaches `guard_block` versus `guard_advise`)
and why a consumer with no snapshot producer must route to advice rather than
out of delegation. Same-file prose-vs-prose drift is the failure mode being
closed; two sections asserting one fact is the shape that produced it.

## Producers and consumers

No new persistent state, no new key on the `usage.txt` contract, no new knob.
The changed interface is the **text of the verdict line** — the sole interface
§The delegation model already names.

**Producer** — the five STALE emission sites in
`delegation-kit/bin/usage-verdict.sh`. Reachable on every code path that
reaches exit 2, with no enabling config: the three fail-closed sites need no
configuration at all, the age site is governed by `DELEGATION_KIT_STALE_AGE`
and the reroute by `DELEGATION_KIT_LOGIN_WINDOW`, both of which carry kit
defaults and are therefore live in every consumer, not test-only.

**Consumers**, surveyed across the whole component set (every tracked `.sh`,
`.md`, `.txt` and `.json` in the tree, unsilenced):

- **The dispatching agent**, at the dispatch-decision transition, reading the
  line off `additionalContext` as relayed by `agent-budget-guard.sh` — the kit
  template and this repo's consumer copy both. This is the consumer the attested
  failure belongs to, and the one the consequence clause is added for.
- **A human or agent running the bin directly**, the invocation
  `delegation-kit/README.md` documents in its verdict line. This is the reader
  option (b) would have split off; it consumes the same completed sentence.
- **`delegation-kit/bin/run-usage-tests.sh`**, which matches the substring
  `-> STALE` and the exit code. Verified unaffected: the change appends to the
  parenthetical tail and moves neither the token nor the code.
- **`delegation-kit/bin/run-budget-guard-tests.sh`**, which asserts the
  block/advise routing and that the relayed text contains `->`. Verified
  unaffected by the arm collapse: the `2)` and `*)` arms are both `advise`, so
  the decision table's expected outcomes are unchanged and the `->` assertion
  still holds on the relayed line.
- **The trend log** (`append_sample`), which records the verdict as the bare
  token `verdict=STALE`, never the prose. Unaffected by construction.
- **`delegation-kit/smoke/install.sh`**, which drives `usage-verdict` twice.
  Unaffected here — its own defect is the separate unit, `SPEC-assertion-strength.md`.

Every field on the changed line has a named reader: the reading is read by the
human and by the agent as the live percentage that displaces a memory-quoted
one; the status is read by both as trust calibration on that number; the
consequence is read by the dispatching agent at the dispatch-decision
transition, which is precisely the reader it is being added for.

## Existing sections updated

- `delegation-kit/SPEC.md` §usage-verdict — gains the verdict-string contract;
  its stale-reading item stops reading `re-read before trusting` alone and
  carries the consequence; the exit-code line is unchanged.
- `delegation-kit/SPEC.md` §The delegation model — its STALE/OK routing bullet
  drops the independent "STALE never blocks" assertion in favour of a citation
  to §usage-verdict, keeping the routing rule and the no-producer rationale.
- `delegation-kit/SPEC.md`'s statement that the guard relays the verdict line
  verbatim — retained and strengthened; it is the contract that replaces the
  deleted arm.
- Generated projections regenerate with the SPEC edit: the `docs/` kit-SPEC
  mirror (`gen-docs-mirror.sh`, gated by `check-docs-mirror-fresh`). No `# graph:`
  manifest and no gate tier changes in this unit, so the check-graph artifact and
  the enforcement map are untouched by it.

## Seam

All generic delegation-kit mechanism. The verdict vocabulary is delegation-kit's
own published contract, not private rule content; no term list, no product
constant, and nothing consumer-specific is introduced, so nothing here becomes
consumer config. The net change is one deletion and one completed sentence.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
