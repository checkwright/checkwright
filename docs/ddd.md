# Domain-driven design: enforcing the ubiquitous language

Domain-driven design asks a team to speak one **ubiquitous language** — a
single agreed word for each concept, used identically in conversation, code,
and specification. The discipline is famous and famously hard to hold: a
synonym creeps in one plausible word at a time, and by the time anyone notices,
the model and the vocabulary have quietly diverged.

That divergence is exactly the failure Checkwright exists to stop. A ubiquitous
language is the kind of discipline prose alone cannot hold when stateless agent
sessions do the writing — a fresh session was not in the room when the team
chose the canonical term, so it reaches for whichever synonym reads well. But
vocabulary consistency is mechanically decidable: whether a banned word appears
in the tree is a question a program can answer. So it belongs to a gate, not to
a reviewer's memory.

This page positions Checkwright as the enforcement layer for a ubiquitous
language. It owns no contract — each mechanism below is owned by the kit that
enforces it, cited downward so the invariant stays in one place.

## The mechanisms

- **Banned-synonym enforcement.** A pattern file lists the synonyms that may
  never be committed; a gate scans every tracked file and blocks the commit
  when one appears. The canonical term wins because its rivals cannot land.
  Owned by `gate-sdk/SPEC.md §check-tree-terms`.
- **Comment and naming directives.** The language reaches comments and
  identifiers too, where a restated definition drifts from the one it copied.
  Comment discipline — a comment is a directive or it is deleted — is owned by
  `canon-kit/SPEC.md §check-comment-tier`.
- **One home per definition.** A term is defined once and cited everywhere
  else, so there is a single place to change when the model moves. The
  content-tiering doctrine that makes each surface point rather than restate is
  owned by `canon-kit/SPEC.md §Content tiering — the star topology`.
- **Vocabulary as consumer config.** Every banned-word list here takes the same
  shape: optional consumer configuration a project supplies, following the
  pattern `gate-sdk/SPEC.md §check-graph` establishes for rule content that
  lives outside the kit. This is the honesty clause. The kits ship generic
  mechanism and stay domain-neutral: no kit carries a domain word, and the
  coupling to any one domain — including the example below — lives in the
  consumer's config, never in Checkwright.

## An example

Take a fictional cargo-shipping domain — the classic DDD teaching example — in
which `cargo` is the canonical aggregate and `parcel` and `shipment` are
synonyms the team has agreed to retire. The consumer writes a pattern file
naming the retired words as extended-regex lines:

```
# cargo-terms.list — retired synonyms for the "cargo" aggregate.
# One grep -E pattern per line; '#' and blank lines are ignored.
\bparcel\b
\bshipment\b
```

and points the tree scan at it — the pattern-file argument stands in for the
standing `GATE_SDK_MSG_PATTERN_FILES` configuration:

```bash
bash gate-sdk/checks/check-tree-terms.sh . cargo-terms.list
```

From then on a session that writes `parcel` where the model says `cargo` is
stopped at the commit, with the offending file and line named. Nothing in
Checkwright knows what a `cargo` is; the pattern file is the consumer's, and the
gate is the enforcement.

## Where to go next

- [Why Checkwright](methodology.md) — the delivery-methodology essay behind the
  mechanism.
- [The kits](index.md#the-kits) — one page per kit, in reading order.
