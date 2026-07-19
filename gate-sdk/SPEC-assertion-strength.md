# SPEC amendment: assertion-strength

Rules the defect filed as `smoke-exit-code-assertion-honesty` and, per
enforcement-first, the gate for its class. `delegation-kit/smoke/install.sh`
guards a `usage-verdict` call with a bare `if`, which discriminates only zero
from non-zero and therefore accepts PAUSE (exit 1) and STALE (exit 2) alike,
under a failure message asserting specifically that the call "did not PAUSE".
A STALE regression passes that smoke silently. The one-line fix and the scanner
for the class land in one unit.

## Component ownership — a deviation worth reading

The unit was filed as delegation-kit's, because that is where the defect lives.
**The fix is delegation-kit's; the gate is gate-sdk's.** The scanner is generic
testing discipline over smoke and gate-test scripts — it knows nothing about
budgets or delegation — and its natural sibling is `check-test-hermetic`, which
already scans smoke scripts, keys on the own-kit-bin convention, and was itself
motivated by this very smoke script. Widest-true-tier placement decides it: a
consumer vendoring gate-sdk without delegation-kit still writes smoke
assertions and still needs the gate, whereas one vendoring delegation-kit is
already a gate-sdk consumer. The registry resolves gates consumer-first
regardless of vendoring kit, so nothing about registration argues the other way.

## What changes

### The class being gated

**An assertion is at least as strong as its own failure message.** The failure
is a guard that discriminates only success-from-failure while its message names
a specific one of several failure modes. The message is a claim about *which*
outcome occurred; the guard never established it. The result is not a false
green today but a **masked regression** — a different failure mode arrives and
reports itself under the wrong name, which is worse than an honest silence
because it sends the reader to the wrong place.

### check-assertion-strength (new gate, gate-sdk)

Invariant: in a smoke or gate-test script, a guard that discriminates only
zero-from-non-zero must not carry a failure message naming a verdict token that
the invoked script's **declared exit contract** binds to one specific non-zero
exit code.

**The declared exit contract.** A script may already declare its exit codes in
its header comment block — a comment line whose first word is `exit:`, followed
by codes with the uppercase tokens they name. Both live declarations sit inside
a wider `# usage:`-style block and are written `#   exit: …` with leading
whitespace, so the parser keys on `^#[[:space:]]*exit:`, not a bare `# exit:`
prefix. This amendment makes that existing header **machine-read** and gives it
a grammar: each uppercase token binds to the nearest preceding integer on the
line, yielding a token→code map. Tokens admit internal hyphens, so `RESET-OK`
reads as one token rather than two. A token bound to more than one code, or to
code 0, is not discriminable and is skipped.

This is the seam move, and it is the same one `check-test-hermetic` makes: that
gate keys on the own-kit-bin convention *so that no kit's credential-consuming
bin roster is spelled out in gate code*. Here, the verdict vocabulary
(`PAUSE`, `STALE`, and any other kit's tokens) is derived from the callee's own
declaration and **never appears as a literal in gate code**. A gate shipping a
vocabulary would publish it; a gate deriving one cannot.

**Reach is opt-in and the gate never widens it.** A callee that declares no
`# exit:` header is simply out of reach — the gate does not demand the header
of anyone. This keeps the unit from imposing a new obligation on every script
in the tree; the header stays a thing a script chooses to offer, and offering
it buys the assertion check.

**Detection.** For each guard construct in the scanned scripts that invokes a
declaring script through the own-kit-bin convention and whose discrimination is
truthiness only, the gate reads the guard body's failure text within a bounded
window and reds when that text names a discriminable token while the guard
compares no explicit exit status to that token's code.

**Valve.** A `# assertion-strength-exempt: <reason>` marker line, the same
valve shape and placement as `# hermetic-exempt:` — for a guard that
establishes the outcome by other means.

**Honest limit.** The gate reads guard shape, not semantics. An assertion
weakened without a token-naming message is out of reach, as is any callee that
declares no exit contract; a message naming a token bound to several codes is
skipped rather than guessed. What it catches is precisely the attested shape: a
message that is more specific than the guard behind it. Like its siblings, a
false positive is loud — a forced reword or an explicit exemption — never a
silent miss.

### The fix

`delegation-kit/smoke/install.sh`'s 95%-reading guard captures the exit status
and compares it to the PAUSE code, and its failure message reports the status it
actually observed rather than asserting an outcome it did not establish. The
script runs under `set -e`, so the capture uses the status-preserving idiom
rather than a bare call. Its sibling guards in the same file are already honest
and are unchanged — but for a sharper reason than "they name no token". The
poller-snapshot guard *does* name one: its message reads `did not verdict OK`.
It is honest because `OK` binds to code 0, and a truthiness guard discriminates
code 0 exactly — the message claims no more than the guard established. That is
the whole work the grammar's skip-code-0 rule does, and it is what keeps the
gate off an honest neighbour four lines from its own fixture. The remaining
guards in the file invoke `templates/usage-poller.sh`, outside the own-kit-bin
convention and declaring no exit contract, so they are out of reach twice over.
The fixed guard and the current one are the gate's fixture pair.

## Producers and consumers

**The `# exit:` header, as a machine-read contract.** Producer: the script
author's declaration, already live in the tree on `delegation-kit/bin/usage-verdict.sh`
and `delegation-kit/bin/usage-trend.sh` — real declarations under no test-only
config, so the gate has a live producer on day one rather than a fixture-only
one. Stated honestly, though, the *effective* day-one reach is narrower than
that count suggests: `usage-trend.sh`'s declaration names its codes in prose
and carries no uppercase token at all, so it yields an empty token→code map and
contributes nothing to discrimination. The whole live vocabulary is
`usage-verdict.sh`'s `PAUSE`→1 and `STALE`→2 (`OK` and `RESET-OK` bind to 0 and
are skipped as indiscriminable). One declaring script, two usable tokens — thin
by construction, and the amendment claims no more. An iteration about verdict
honesty does not get to overstate its own gate's reach. Consumer: `check-assertion-strength`'s parser, at the guard-scan
transition. Named reader per field: the **code** is read to compare against the
guard's explicit status comparison; the **token** is read to match against the
failure message text. There is no third field, and neither field is populated
at any other transition.

**The gate name.** Producer: registration in this repo's `scripts/gates.list`,
which the pre-commit generator reads to emit the hook. Consumer:
`run-gates.sh` and the generated pre-commit hook, each invoking it by name; the
enforcement map reads its tier.

**The exemption marker.** Producer: a script author writing it with cause.
Consumer: the gate's own scan, which skips a guard carrying it — and that is
the *only* consumer. `check-gate-exemption-tasks` does **not** reach it, and
this is deliberate rather than a gap: §check-gate-exemption-tasks scopes itself
to `# exception-list:`-tagged arrays inside `check-*.sh` gates and rules inline
per-site directives (`# fail-closed-exempt:`, `# no-fixture:`) out on the
grounds that they are local and self-evident via their adjacent comment.
`# assertion-strength-exempt:` joins that inline class — as does the
`# hermetic-exempt:` valve it is modelled on, which no exemption-task gate
holds today either. Its discipline is the adjacent `<reason>`, sited on the
guard it excuses; the queue-linked discipline belongs to the array class and is
not inherited here.

**The fixed smoke guard.** Producer: `run-consumer-smoke.sh` driving
`delegation-kit/smoke/install.sh`. Consumer: the smoke runner's own pass/fail,
which now distinguishes a PAUSE regression from a STALE regression by the
status the message reports.

## Existing sections updated

- `gate-sdk/SPEC.md` — gains the `### check-assertion-strength` contract
  alongside `### check-test-hermetic`, whose smoke-scanning neighborhood it
  joins; the gate roster and the kit README roster gain the entry
  (`check-readme-roster` holds them in lockstep).
- `delegation-kit/SPEC.md` §usage-verdict — its exit-code statement notes that
  the bin's header declaration is machine-read by the gate, so the header stops
  being loose documentation and becomes a surface with a consumer.
- `delegation-kit/SPEC.md` §Testing — the smoke's assertion is now
  code-specific; the section describing the smoke's coverage updates with it.
- Generated projections, all regenerated as part of the unit: the pre-commit
  hook (`gen-pre-commit.sh --write`, from the new gate's `# graph:` manifest),
  the check-graph artifact (`check-graph.sh --emit`), the enforcement map
  (`enforcement-map.sh --emit`, since a new gate carries a tier), and the
  `docs/` kit-SPEC mirror.

## Landing checklist (kit conventions this unit must satisfy)

The gate copies `gate-sdk/templates/check-skeleton.sh` and inherits the four
contracts gate-sdk's meta-gates enforce — the single machine-keyable success
line, fail-closed on a harness error, a `good/`+`bad/` fixture pair, and a clean
self-lint. Tier `precommit`, matching its sibling. Its `# graph:` manifest
couples the surfaces it reads: each kit's smoke and gate-test scripts and the
declaring bins, `dir=one` — a one-way audit, so an edit to any of them re-fires
it. Configuration adds **no new knob**: the gate resolves its scan roots through
the existing kit-roots derivation, as `check-test-hermetic` does.

## Seam

Generic gate mechanism throughout. The one piece of content that could have been
a private literal — the verdict vocabulary the gate matches messages against —
is derived from the callee's own declared header instead, so no term list ships
in gate code and no consumer must configure one. Nothing here becomes consumer
config, and nothing private lands.

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
