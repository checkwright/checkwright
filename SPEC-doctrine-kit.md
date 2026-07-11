# SPEC amendment: doctrine-kit

## What changes

A new kit — **doctrine-kit**, the experience-packaging rung: the delivery
doctrine the other kits enforce piecemeal becomes one kit-shipped,
customer-deliverable rules file, installed by reference into the consumer's
always-loaded agent file and held there by a gate. Owning-kit ruling: a new
kit, not canon-kit — the doctrine spans kit remits (enforcement-first is
gate-sdk-flavored, load-trigger residency is context-kit-flavored, tiering is
canon-kit-flavored), and canon-kit's own out-of-scope boundary says a
consumer's tier contract is not canon-kit's to own.

- **`doctrine-kit/DOCTRINE.md`** — the deliverable. Referenced in place
  (vendored path), never copy-installed: point-never-restate applies to the
  mechanism itself, and re-vendoring the kit *is* the doctrine upgrade. Each
  rule lands as: the rule statement, why it holds under agent work, and a
  pointer to the enforcing mechanism (the kit gate or skill that holds it).
  Initial roster, relocated from this repo's CLAUDE.md §Single source of
  truth plus the residency rule ruled at convention-hardening align:
  1. **Content-tiering / SSOT** — one content tier per surface; point, never
     restate; the fix for a parallel copy is a pointer plus a gate that
     forbids the slab growing back.
  2. **Enforcement-first** — a fix and its enforcing mechanism land in one
     unit; a green instance fix is the stop signal to ask what check should
     have caught it.
  3. **De-literalization** — prose cites names; code or the owning SPEC owns
     values (knob defaults, shared constants, derivable rosters).
  4. **Always-loaded shape** — one-line rule per convention in the
     always-loaded file; mechanism and rosters live behind the pointer.
  5. **Load-trigger residency** — the always-loaded file earns a rule only
     when no stage, skill, or tool-call trigger exists to load it; anything
     triggered lives in its owned doc behind that trigger.
- **Roster extension — the corpus sweep (build worklist item).** Sweep the
  private companion consumer's instruction corpus (identity in the
  operator's local brief) for generic practices not yet banked. Candidate
  families identified at scope, from section titles only: rename/sweep
  verification craft for delegated agents, spec-invariant test naming,
  ubiquitous-language consistency in its generic (vocabulary-free) form,
  git-operation hygiene, attested working-style habits. Seam discipline:
  mechanism and generic rule formulations cross; rule content — term lists,
  vocabularies, glossary bodies, product constants — never does; a candidate
  that cannot be stated without its vocabulary becomes optional consumer
  config or is dropped. The supervisor rules each candidate in or out.
- **`doctrine-kit/bin/install-doctrine.sh`** — idempotent installer: inserts
  (or replaces, between fixed marker lines) a reference block in the
  consumer's always-loaded agent file. The block is the always-loaded shape
  applied to the doctrine itself: the one-line-per-rule digest plus a
  markdown link to the doctrine file. Manual insertion stays documented in
  the README for harness-less consumers.
- **`doctrine-kit/checks/check-doctrine-registration.sh`** — the
  check-kit-registration shape: asserts the configured agent file carries a
  markdown link to the configured doctrine file, fail-closed when the agent
  file is missing. Ships a `good/`+`bad/` fixture pair, `smoke/`, and
  registers in this repo's `scripts/gates.list`.
- **Knobs** (`<KIT>_<KNOB>` convention; kit SPEC owns the roster and
  defaults): `DOCTRINE_KIT_AGENT_FILE` — the always-loaded file the
  installer edits and the gate scans (default `CLAUDE.md`);
  `DOCTRINE_KIT_DOCTRINE_FILE` — the link target the installer writes and
  the gate asserts (default `doctrine-kit/DOCTRINE.md`). Optional consumer
  config file follows the `scripts/<kit>-config.sh` pattern.
- **Tier ruling.** DOCTRINE.md owns the cross-kit practice-rule
  *statements*; each kit's SPEC owns its mechanism and knob rosters, cited
  from the doctrine, never restated; the agent-file reference block is the
  sanctioned one-line digest tier. DOCTRINE.md joins the spec manifest
  (`scripts/canon-config.sh`) so its links and commands resolve under the
  canon-kit doc gates.
- **CLAUDE.md conversion.** This repo becomes consumer-by-reference:
  §Single source of truth is replaced by the installed reference block;
  repo-specific bindings stay in CLAUDE.md. Other CLAUDE.md sections keep
  their current homes — their conversion is not this unit.

**Ruled out.** Folding into canon-kit (above). Copy-install of the doctrine
file (a copied doctrine drifts; the reference is the mechanism). A standing
consultation step instead of packaging — already declined at
convention-hardening close as the always-loaded anti-pattern; this kit is
the load-triggered structural fix. Per-rule enable/disable knobs — the
doctrine is one document; a consumer that rejects a rule edits its digest
block, and the gate asserts only the link, not the block body.

## Producers and consumers

- **DOCTRINE.md** — producer: the kit, maintained here and re-vendored by
  consumers; consumers: any session that loads the agent file and follows
  the reference block's link, and the canon-kit doc gates via spec-manifest
  membership (they read every governed page's links/commands).
- **The reference block** — producer: `install-doctrine.sh` (or the
  documented manual insert); consumer: the session reading the always-loaded
  file at start; held present by `check-doctrine-registration` at every
  commit via `gates.list` registration.
- **`DOCTRINE_KIT_AGENT_FILE` / `DOCTRINE_KIT_DOCTRINE_FILE`** — producer:
  consumer config or env, else the kit defaults; readers:
  `install-doctrine.sh` (insert target; link target written into the block)
  and `check-doctrine-registration.sh` (scan target; link asserted).
- **`check-doctrine-registration` verdict** — producer: the gate; consumers:
  the generated pre-commit hook and the CI battery, both via the
  `gates.list` registration.

## Existing sections updated

- `CLAUDE.md` §Single source of truth — replaced by the installed reference
  block (the relocation is the conversion, not a copy).
- `README.md` §Kits — gains the doctrine-kit row (`check-kit-registration`
  assertion A holds it); `docs/index.md` gains the kit row
  (`check-docs-kit-parity`).
- `scripts/gates.list` + the regenerated pre-commit hook, CHECK-GRAPH
  artifact, and `docs/enforcement.md` (their freshness gates force the
  regen; the meta-paths ruling in this iteration's debt lane unblocks the
  same-commit landing).
- `scripts/canon-config.sh` — DOCTRINE.md joins the spec manifest.
- CLAUDE.md battery block, CI workflow, and `scripts/evidence-config.sh`
  pick the new kit up via the fixture-suite-derivation debt task riding
  this iteration — doctrine-kit is that mechanism's first green case.
- TASK-QUEUE.md deferred `delegation-doctrine-single-source` — its
  candidate-home note (context-kit's SPEC for the residency rule) is
  superseded: DOCTRINE.md is the residency rule's home.

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
