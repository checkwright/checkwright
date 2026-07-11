DISPATCH CHECKLISTS — a reach-through from [agent-execution.md](agent-execution.md),
loaded only when a dispatch is a deletion, a rename, or a heavy cross-spec audit.
It adds no new protocol: the safety, budget, resume-journal, and
validate-after-commit rules stay in `agent-execution.md` and still apply in full.
This file is the mechanical pre-flight for those dispatch shapes — the steps an
agent skips by reflex and the supervisor pays for later. Owned by
delegation-kit/SPEC.md §One template, a resident pointer.

## Before a deletion dispatch

- **Importer survey.** Grep every importer of the symbol or file across the tree
  before dispatching the deletion; where the toolchain ships a consumer-scan tool
  (an LSP find-references, a build-graph query), use it rather than a bare text
  grep. No importers is the safe-to-delete signal. An importer outside the unit's
  declared scope does not license the deletion — it rescopes the unit from delete
  to migrate-then-keep, surfaced to the supervisor before the dispatch proceeds.
- **Ownership of a shared removal.** When both sides of a boundary import the
  surface being removed, the dispatch that owns the removal is the layer each side
  can import without a cycle — the common dependency, not necessarily the domain
  owner. Placing the removal at the domain owner when the other side cannot reach
  it without a cycle strands a caller.

## Before a rename dispatch

- **Target-collision check.** Grep the target namespace for the new name before
  the rename amendment is declared build-ready — a scope-stage step, not a
  build-time discovery. A collision found at scope reshapes the amendment; found
  at build it strands a half-done sweep.
- **Mention-versus-use brief.** A rename dispatch brief names where the old token
  is legitimately *mentioned* — a historical `Old→New` migration arrow, a
  changelog line, a deprecation record — separately from where it is *used*. A
  naive sweep that rewrites both collapses the historical record; the brief tells
  the agent which occurrences to leave standing.

## Deriving the dispatch set

- **Re-derive from the amendment body.** Derive the set of components a dispatch
  touches from the amendment body itself, never from the queue entry's
  enumeration — the queue line is a summary written before the work and drifts
  from the amendment's real footprint. The amendment's own surface list is the
  authoritative one.

## After any bulk edit

- **Sweep verification.** Re-grep in a fresh tool call after any bulk edit; the
  edit's own report is not the check. Match the gate's matcher class, not a looser
  one — a whole-word gate over-reports under a substring grep and under-reports
  under a looser regex. A value that crosses a language boundary needs both an
  emit-side and a consume-side grep; prefer an LSP find-references sweep for a
  compiled symbol. A term gate carries structural blind spots — allowlisted words,
  string and variant spellings, modeling errors — so a by-eye pass covers what the
  matcher cannot see.

## Heavy cross-spec audit

- **Mechanize, then read.** A heavy cross-spec audit's real output is a gate
  backlog, not a one-time fix list: mechanize the decidable classes into checks
  first, then run a light semantic pass over what no gate can decide. The fan-out
  width bound and the resume-journal discipline are the core protocol's
  (`agent-execution.md`), applied here unchanged rather than restated.
