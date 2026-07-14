# SPEC amendment: commit-msg-session-trailer-screen

## What changes

The banned-pattern defaults gain a **session-reference class**: patterns that
catch a harness-injected trailer referencing an internal session before it
lands in public history. No new gate and no new knob — `check-commit-msg`'s
mechanism is untouched; this is pattern content plus its spec prose.

Two patterns join both `gate-sdk/templates/msg-patterns.list` (the shipped
generic defaults) and this repo's `scripts/msg-patterns.list`:

1. `claude\.ai/` — the session-share URL host. A harness-injected session
   link is the primary attested form; the host is public software's public
   domain, so the pattern is generic mechanism, not private vocabulary.
2. `^[A-Za-z][A-Za-z-]*: .*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}`
   — a trailer-shaped line (`Key: …`) carrying a full UUID, the shape of a
   session-id trailer regardless of the key the harness uses.

**Seam ruling (premise correction).** The queue entry filed this class into
the gitignored local list "so the public repo ships no session-shaped
literal". Scope re-verified: neither pattern carries a private term — one is
a public host, the other a shape — and a local-only screen leaves every
consumer's fresh clone unguarded against the exact leak this unit exists to
stop. The class therefore ships tracked; `msg-patterns.local.list` remains
the home for private sharpenings only.

**Shared-source constraint (why the UUID half is trailer-anchored).**
`gate_msg_pattern_files` feeds both `check-commit-msg` and
`check-tree-terms`, and the tree legitimately carries synthetic session
UUIDs in `lifecycle-kit/smoke/install.sh` (`smoke/` is not in the shared
prune set). A bare-UUID pattern would red the tree scan; the `^Key: ` anchor
matches no current tracked line (verified against the whole tree) while
still matching any trailer-shaped injection. The `claude\.ai/` half is
shared-safe as-is: the tracked tree has zero occurrences. The
`Co-Authored-By` trailer stays allowed — it is a footer convention, not a
leak (existing doctrine, unchanged).

## Producers and consumers

- **Producer:** the generated `commit-msg` hook invoking `check-commit-msg`
  with the prospective message file; the pattern set resolves through
  `gate_msg_pattern_files` defaults (`scripts/msg-patterns.list` here; the
  template ships to consumers at vendor time) — the enabling config is the
  existing default resolution, emitted everywhere the hook is installed.
- **Consumers:** `check-commit-msg` (message surface) and `check-tree-terms`
  (tracked-tree surface) — both read every pattern line via the shared
  resolver; the tree consumer is why the anchoring constraint above exists
  and was verified.
- **Fields:** pattern lines in an existing file format; reader named above.

## Existing sections updated

- `gate-sdk/SPEC.md §check-commit-msg` — the shipped-defaults sentence
  ("absolute home paths, the Claude Code promo URL") extends to name the
  session-reference class and its trailer-anchoring rationale (one line,
  citing the shared-source constraint).
- Fixture: `gate-sdk/gate-tests/check-commit-msg/bad/` extends to cover a
  session-trailer message (its per-case `patterns.list` gains the class;
  `msg.txt` a trailer-shaped line); `good/` keeps a `Co-Authored-By` trailer
  passing.

**Build heads-up:** once the patterns land, a commit message (or tracked
prose) spelling the raw host unescaped trips its own screen — amendment and
spec prose use the escaped regex form only, and build-session commit
messages say "the session-URL host", never the literal.

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
