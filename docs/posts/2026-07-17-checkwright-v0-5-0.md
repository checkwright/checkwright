---
release: v0.5.0
---

# Checkwright v0.5.0

*2026-07-17*

Checkwright is the verification layer under agent orchestration, and this
release hardens the lifecycle for concurrent and supervised work: a committed
mid-iteration gap channel that no boundary lets cross untriaged, and a
same-stage re-entry clause for lead-supervised batches. One vendored gate got
stricter to hold the new channel's merge safety, so the Tightened gates section
names it.

## Tightened gates

- **check-merge-attrs** — now also asserts forward-only union parity: the
  lifecycle gap inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`, default
  `.workflow/gap-inbox.md`) must carry a `merge=union` line in `.gitattributes`,
  so a gap filed on either side of a concurrent merge is never silently dropped.
  The reverse edge is deliberately absent — `merge=union` is git-native and a
  consumer's own append logs legitimately carry it, so an unexpected union
  attribute is not a finding. A clean upgrade that regenerates the lifecycle
  marker block (`bash lifecycle-kit/bin/install-lifecycle.sh`, which now writes
  the `merge=union` line) stays green; the gate reds only against a tree synced
  to the tightened gate without regenerating that block.

## Renamed knobs

None — nothing was renamed or removed.

## Behavior changes

Two lifecycle changes ship outside the battery — they land here, the fixed
section for what shifts that no gate scans.

- **lifecycle gap channel** — a new committed `.workflow/gap-inbox.md` and its
  `bin/file-gap.sh` filer (knob `LIFECYCLE_KIT_GAP_INBOX_FILE`) capture a
  mid-iteration gap append-only. The close stage now drains the inbox
  (disposition every bullet — promote, fix, or discard — then truncate to its
  header), and the next iteration's scope entry refuses to open while the inbox
  holds bullets, so no gap crosses the iteration boundary untriaged. A consumer
  running the lifecycle gains the drain step and the boundary refusal; nothing a
  clean tree reconciles beyond the marker-block regen the Tightened gates
  section already names.
- **same-stage re-entry** — the lifecycle SPEC and lead template now state that
  a stage may be re-entered under its own stage (a lead-supervised batch
  re-invokes the stage skill) and that batching is the lead's to own. A
  documentation-and-behavior clause for the supervised-batch posture; a
  lead-less consumer reconciles nothing.

## Upgrading

Sync the vendored kit directories wholesale at `v0.5.0` and regenerate the
generated artifacts — the pre-commit hook, the graph projection, and the
lifecycle marker block (`bash lifecycle-kit/bin/install-lifecycle.sh`, which
writes the gap inbox's `merge=union` `.gitattributes` line) — then run the full
battery.

**The one allowed red.** `check-merge-attrs` reds against a tree that synced the
tightened gate but did not regenerate the marker block, because the gap inbox
then lacks its `merge=union` line. The fix is the `install-lifecycle.sh` regen
this note names, the same step that keeps the pre-commit hook and graph
projection current.

The Tightened gates section is the mechanical allowed-red set: the gates a clean
upgrade may turn red. The behavior changes above are declared for reading, not a
mechanical scan. If a gate reds that this note does not name, the upgrade smoke
was supposed to catch it first —
[open an issue](https://github.com/checkwright/checkwright/issues), because that
is a defect in the release rather than work for you.
