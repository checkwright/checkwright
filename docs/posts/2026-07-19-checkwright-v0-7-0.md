---
release: v0.7.0
---

# Checkwright v0.7.0

*2026-07-19*

Checkwright is the verification layer under agent orchestration, and this
release closes an ambient-credential leak in the consumer-smoke harness and
tightens the gate that catches the class. A smoke script drives its kit's bins
under the real `$HOME`, so a bin that resolves its credential file from the
ambient `~/.claude` reads live login state and its verdict turns on the
wall-clock age of the operator's credential — a test that passes or fails by
what time it is run. The fix pins the credential at an absent path; the gate
now enforces the pin.

## Tightened gates

- **check-test-hermetic** — gains assertion B: a credential-managing smoke
  script (one that assigns a `*_CRED_FILE`) must pin *every* own-kit bin call
  to a `*_CRED_FILE` path on the invocation line, or carry the existing
  `# hermetic-exempt: <reason>` valve. The trigger is the script's own
  `*_CRED_FILE` assignment, so a smoke script that manages no credentials is
  never held to the rule, and the scan keys on the `$SMOKE_KIT_ROOT/bin/`
  own-kit-bin convention, so no kit's credential-consuming bin roster is
  spelled out in gate code. Honest limit, stated because it bounds what the
  assertion buys you: the trigger catches *partial* pinning, not *absent*
  pinning — a smoke script that consumes a credential and pins nothing at all
  is not flagged.

## Renamed knobs

None.

## Behavior changes

- **delegation-kit/smoke/install.sh** — both `usage-verdict.sh` calls are
  pinned to an absent credential path, so the smoke no longer reroutes to STALE
  inside the 600-second login window of the ambient credential's mtime. If you
  have copied this smoke out, apply the same pin: the failure it removes is
  intermittent and wall-clock-dependent, which makes it expensive to diagnose
  from a red battery alone.
- **drift-kit knowledge-friction cue** — the capture roster now names deriving a
  new deliverable by consulting a prior or sibling one as a stampable
  non-owning surface, and states the fact-vs-work-shaped channel seam inline: a
  "this chrome should be owned or generated" conclusion is work-shaped and
  routes to the gap inbox, while the knowledge-friction log captures the
  narrower re-derived fact. Sessions will stamp more often; the grammar,
  helper, triage, and KPI are unchanged.
- **delegation-kit/templates/agent-execution.md** — the budget-check bullet now
  states that the harness statusline does not refresh while the main session is
  parked on a background dispatch, so its displayed budget freezes for the
  length of the dispatch, and names the sanctioned external poll for a live
  reading. Enforcement is unaffected: the per-dispatch budget check reads a
  fresh verdict at dispatch time, so only passive display goes stale.
- **docs/install.md upgrade contract** — a knob *removal* is now expressed
  `old → ∅` under Renamed knobs rather than earning a section of its own, and
  the mapping from the four consumer-owned residue classes onto the three note
  sections is stated explicitly. No section was added and `check-release-bump`
  is unchanged; if you author notes against this grammar, express removals in
  the new form.

## Upgrading

Sync the vendored kit directories wholesale at `v0.7.0`, then regenerate the
generated artifacts — the pre-commit hook
(`bash gate-sdk/bin/gen-pre-commit.sh --write`) and the graph projection
(`bash gate-sdk/checks/check-graph.sh --emit > docs/check-graph.html`) — and
run the full battery.

**The allowed red.** `check-test-hermetic` may red on a credential-managing
smoke script of yours that calls an own-kit bin without a `*_CRED_FILE` pin on
the invocation line. Clear it by pinning the call at an absent path, which is
what makes the call deterministic, or by adding a `# hermetic-exempt: <reason>`
marker where the script establishes hermeticity some other way.

The behavior changes above are declared for reading, not a mechanical scan. If
a gate reds that this note does not name, the upgrade smoke was supposed to
catch it first —
[open an issue](https://github.com/checkwright/checkwright/issues), because
that is a defect in the release rather than work for you.
