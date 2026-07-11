# SPEC amendment: doctrine-rule-lockstep

`check-doctrine-registration` asserts only that the always-loaded file links
the doctrine file; nothing holds the one-line-per-rule digest and the
doctrine's methodology-maintenance rule set in lockstep, so a rule added on
either side without its counterpart passes. Extend the gate to per-rule
bidirectional coverage.

Gate-economy weighing (the queue entry's mandated question): the gate is
owed. The doctrine upgrades by re-vendor — a re-vendored DOCTRINE.md that
adds or renames a methodology-maintenance rule leaves every consumer's
digest silently stale *by construction*, on the exact path the kit
advertises as its upgrade story. The digest is a surface that must exist
(the always-loaded-shape rule requires it), so this is the cheap-insurance
case, not a removable surface; the check is two name-set extractions and a
set compare.

## What changes

- **check-doctrine-registration gains digest-coverage assertions** (the
  existing link assertion becomes assertion A; extending the existing gate
  rather than adding one keeps the gate count flat and the coupling is
  identical — the `# graph:` manifest already couples the agent file and
  the doctrine file):
  - **Assertion B (doctrine → digest):** every rule name under the
    doctrine's methodology-maintenance section — the bold text of each
    numbered rule, trailing period dropped (e.g. `Content-tiering / SSOT`)
    — appears as a bold digest lead-in (`- **<name>**`) in the agent
    file's digest section. Engineering-craft rules are exempt: they live
    behind the link by the doctrine's own two-register design.
  - **Assertion C (digest → doctrine):** every bold bullet lead-in in the
    agent file's digest section matches a methodology-maintenance rule
    name — a digest line with no owning rule is a rule stated nowhere the
    doctrine governs.
- **Digest-section resolution** — new knob `DOCTRINE_KIT_DIGEST_SECTION`
  (lib/doctrine.sh; the consumer's agent-file heading whose bullet list is
  the digest; this repo's value is its CLAUDE.md "Delivery doctrine"
  heading). Resolution fails closed like `check-brevity`'s section knob: a
  configured section matching no heading exits 2 — a renamed digest
  section must not disarm the gate into passing an empty set. The
  doctrine-side section heading (`Methodology-maintenance rules`) is kit
  mechanism, not config — the kit ships DOCTRINE.md, so it owns that name.
- **Fixture pair extended**: `bad/` gains a digest-missing-a-rule case and
  a digest-extra-line case; `good/` carries a matched pair of files.

## Producers and consumers

- Assertion inputs: producer of the rule-name set is DOCTRINE.md (kit
  ships it; re-vendor updates it); producer of the digest set is the
  consumer's agent file (installed once by `install-doctrine.sh`,
  maintained by the consumer). The consumer of both sets is the gate at
  its single scan transition; a mismatch surfaces at the next commit or
  battery run after either side changes — which is exactly the re-vendor
  moment.
- `DOCTRINE_KIT_DIGEST_SECTION`'s producer is the consumer config
  (`scripts/doctrine-config.sh` pattern via lib/doctrine.sh defaults); its
  reader is the gate's section resolver. This repo's config sets it
  explicitly only if its heading differs from the shipped default —
  decide the shipped default at build against install-doctrine.sh's
  installed block heading so the zero-config consumer is green out of the
  box (the installer and the gate must agree on the default; that
  agreement is part of this amendment's contract).
- `install-doctrine.sh` (the digest block's installer) is the producer of
  a fresh consumer's digest; verify the block it installs passes
  assertions B and C — installer and gate landing incoherent is the
  causal gap this section exists to close.

## Existing sections updated

- doctrine-kit/SPEC.md §check-doctrine-registration — the invariant grows
  the two assertions and the knob; the "honest limit" prose (link ≠ read)
  stays.
- doctrine-kit/SPEC.md §Layout and configuration — the knob row.
- doctrine-kit/README.md — only if it states the single-assertion
  behavior.
- DOCTRINE.md preamble — no change expected; it already names the digest
  contract ("form the always-loaded digest a consumer installs").

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls doctrine-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
