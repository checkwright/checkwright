# SPEC amendment: statusline-queue-section-counts

Rules the operator request `statusline-queue-section-counts`: surface
`TASK-QUEUE.md` section counts in the statusline as compact single-letter
counters — features, debt, deferred, icebox, with the deferred counter explicitly
wanted. The entry names one constraint that must not be assumed away (section
names are consumer config, so the counter must **resolve** them) and one it
believed about delivery (two surfaces, because a kit template is a consumer-copy
producer). The first holds. The second is false as filed, and the correction
makes the unit smaller.

**The ruling: the counter is a new one-job tool in queue-kit; the statusline
calls it and never learns a section name. One committed surface, not two.**

## The requested set is derivable, not a list

The four counters the operator asked for are exactly queue-kit's **task
sections** — `QUEUE_KIT_ACTIVE_SECTIONS` (New Features, Technical Debt) plus
`QUEUE_KIT_DEFERRED_SECTION` plus a configured `QUEUE_KIT_ICEBOX_SECTION`. That
set is already composed inside `lib/queue.sh`, where it backs `QUEUE_TASK_RE`.
`QUEUE_KIT_DONE_SECTION` is outside it, and is outside the request.

So the counter enumerates nothing. It counts the task sections, in their
configured order, and this repo's four counters fall out. A consumer with
different section names gets their names; a consumer with no icebox gets three.
That is the whole answer to the entry's provenance-seam constraint, and it is
stronger than "resolve the knobs" — there is no roster to resolve because the
derivation already exists and has a reader.

## Where the counter lives, and why not the two nearer homes

**Not a further mode on `bin/queue-index.sh`** — the obvious home, and the owning
spec already refuses it. `queue-kit/SPEC.md` §bin/queue-index.sh fixes the tool's
modes at `index`, `extent` and `icebox-candidates`, and §The tag algebra refuses a
further mode on the stated grounds that *folding jobs together gives one tool two
output grammars*. That refusal is on record for a different feature; it binds this
one identically. Reading it as precedent to be weighed would be exactly the
spec-over-precedent error — the owner doc settles it.

**Not by sourcing `lib/queue.sh` into the statusline** — measured, and refused on
the measurement. The lib **exits 2 at source time** on a missing
`QUEUE_KIT_CONFIG_FILE` and again on any malformed-config assertion. Sourcing runs
in the caller's shell, so a queue-config error would take down the *entire status
bar* — model, context gauge, both rate-limit gauges — for a fault in a component
contributing four characters. A subprocess isolates it: a non-zero exit yields
empty output and the counters simply do not render, which is the degradation shape
the template already uses twice (an absent `TASK-QUEUE.md` drops the iteration
name; an absent state file drops the `@stage` suffix).

The cross-kit dependency itself is not the objection and would have been fine:
`delegation-kit/templates/agent-budget-guard.sh` already sources
`guard-kit/lib/guard.sh` through a `GUARD_KIT_LIB` indirection with an
existence guard. The objection is specific to *this* lib's source-time exit.

**So: a new `queue-kit/bin/queue-counts.sh`.** One job, one output grammar, which
is precisely what the refused-further-mode clause asks for. It sources
`lib/queue.sh` in its own process, where the exit-2 contract is correct behavior
rather than collateral damage.

## The dependency is safe by the roster, not by assumption

`installer/profiles.list` is the oracle: every profile carrying delegation-kit —
`delegation`, and `full` by derivation — also carries queue-kit. `starter` carries
neither, so it ships no statusline template either. The `prose` profile landing
this same iteration carries neither. So a statusline that calls queue-kit's bin
cannot reach a tree without it through any profile that exists.

The existence guard is kept anyway, because profiles.list states that a further
profile is admitted whenever it fits the lattice, and a future
delegation-without-queue profile would otherwise turn a roster fact into a broken
status bar. Guard, not assumption — and it costs one `[ -x ]` test.

## What changes

- **(D1) `queue-kit/bin/queue-counts.sh`** — a new tool. Sources `lib/queue.sh`;
  emits one `<section-name><TAB><count>` line per **task section**, in configured
  order, counting top-level entry bullets by the same slug grammar the existing
  section scanners use. No flags, no modes, one grammar. **design-bearing** — the
  counted unit (top-level entries, not bullets and not lines) has to agree with
  what every other queue reader calls an entry, or two tools disagree about the
  size of the same queue.

- **(D2) `queue-kit/SPEC.md` §bin/queue-counts.sh** — a new per-component
  contract section beside §bin/queue-index.sh, stating the derivation (task
  sections, so Done is out and a configured icebox is in), the grammar, and the
  one-job boundary that keeps it separate from the index. It must also say why
  this is a second tool rather than a fourth mode, citing the refusal it is
  honoring — otherwise the next author reads two queue tools as an oversight and
  merges them. **design-bearing**.

- **(D3) `delegation-kit/templates/statusline-usage.sh`** — render the counters.
  Inside the existing `$ROOT` block, call the bin if it is executable, take the
  initial of each returned section name, and append a compact
  `·N12 T3 D48 I7`-shaped group to the status bar. The initial is derived from the
  returned name, so no section name appears in delegation-kit. Empty output, a
  non-zero exit, or an absent bin drops the group. **design-bearing** — the
  degradation paths are the contract, and the render must stay legible when a
  consumer's names collide on their first letter.

- **(D4) `delegation-kit/SPEC.md`** — the statusline template's contract gains the
  counters clause: what is rendered, that the section vocabulary is queue-kit's
  and never delegation-kit's, and that the group degrades silently. Its component
  listing line for `templates/statusline-usage.sh` is updated in the same edit.
  **mechanical** — a contract clause against a design D1–D3 fix.

## What this amendment deliberately does not do

**It does not update `~/.claude/statusline.sh`, and the entry's "two surfaces"
premise is why this needs saying.** The entry expected the operator's live
statusline to be a stale consumer copy needing a separate edit. Measured, the
picture is different in both directions:

- Inside this repo the operator's live statusline **is the template**:
  `.claude/settings.json` sets `statusLine` to
  `bash delegation-kit/templates/statusline-usage.sh`, and project settings
  outrank user settings. So D3 alone delivers the operator's ask, in the only
  repo where a `TASK-QUEUE.md` exists to count. One edit, not two.
- The user-level `~/.claude/statusline.sh` is real, and is a **drifted ancestor**
  — it still cites a `scripts/SPEC.md §statusline` path that no longer exists, and
  it reads the stage from the `TASK-QUEUE.md` `[stage:]` header, a cursor source
  CLAUDE.md retired in favour of `.workflow/WORKFLOW-STATE.txt`. It runs only
  *outside* this repo, where there is no queue to count.

It is out of tree, untracked, and ungoverned, so no gate can verify a change to it
and no commit can carry one. It is therefore out of scope by the request rather
than merely by cost: the counters it would render have nothing to count. Its
independent staleness — a status bar showing a stage from a retired source — is a
separate finding and is filed to the gap inbox at this amendment's authoring.

**It does not add a counter for the Done section**, which the request did not ask
for and the task-section derivation excludes. Adding it would replace the
derivation with a list.

## The seam

The seam holds by construction rather than by discipline, and that is the reason
for the tool split. delegation-kit's template never names a section, never reads a
`QUEUE_KIT_*` knob, and never learns how many sections exist — it renders whatever
lines it is handed, keyed by their own initials. Every section name stays inside
queue-kit, which already owns them. Had the counter lived in the statusline, the
seam would depend on a future editor remembering not to hardcode; here there is
nothing to hardcode.

Nothing new becomes consumer config: the section names are already
consumer-configured knobs, and this amendment adds a reader for them, not a
second place to set them.

## Producers and consumers

- **Producer** — `queue-kit/bin/queue-counts.sh`, invoked per statusline render.
  Its enabling config is the queue-kit config chain `lib/queue.sh` already loads
  (`QUEUE_KIT_CONFIG_FILE` or `<gates-dir>/queue-config.sh`, plus the gitignored
  local overlay); this repo sets `QUEUE_KIT_ICEBOX_SECTION=Icebox` there, which is
  what makes the fourth counter appear. Emitted everywhere it must be: the config
  chain is the same one every queue gate loads, so the tool cannot see a different
  section set from the gates.
- **Consumer** — `delegation-kit/templates/statusline-usage.sh`, by subprocess
  capture at render time. The mechanism is named because it is the contract: a
  subprocess, never a source, for the exit-2 reason above.
- **Named reader of each emitted field** — the section name is read for its
  initial (the counter's label); the count is read as the counter's value. Two
  fields, two readers, both in D3. No third field is emitted: an ordinal or a
  section kind was considered and is not added, because the render needs neither
  and a field with no reader is removed rather than shipped.
- **Frequency, stated because it is the one non-obvious load** — the statusline
  fires far more often than any per-session hook (delegation-kit/SPEC.md §The
  usage.txt contract records this for the usage snapshot). Measured at authoring:
  the equivalent full scan of this repo's ~3,600-line queue runs in 12ms, so the
  added cost is negligible and does not need a cache. Recording the number is what
  keeps a later session from adding one.

**Existing integration prose describing the prior flow**: delegation-kit/SPEC.md's
statusline component listing and its statusline contract clauses — D4 updates
both. queue-kit/SPEC.md §bin/queue-index.sh needs no edit: its mode roster is
unchanged by a sibling tool, and D2's new section carries the cross-reference.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **The acceptance oracle, named per delta.** D1: a `queue-kit/gate-tests/`
      behavioral test over a scratch queue, asserting (a) the emitted section set
      equals the configured task sections with `QUEUE_KIT_ICEBOX_SECTION` set,
      (b) it drops to three lines with the knob unset, and (c) the Done section
      never appears. Case (b) is the one that discriminates a resolved
      implementation from a hardcoded four-section one — a test run only against
      this repo's config passes either. D3: the statusline template is shell under
      `check-shellcheck`, and its degradation paths are asserted by running it
      with the bin absent and with the bin exiting 2, in both cases expecting an
      unchanged status bar rather than an empty one.
- [ ] **Seam check, run as a grep, not asserted by eye** — no `QUEUE_KIT_*`
      identifier and no section-name literal appears anywhere in delegation-kit
      after D3.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks. The drifted `~/.claude/statusline.sh` is already in the gap
      inbox, filed 2026-08-09 at this amendment's authoring.
