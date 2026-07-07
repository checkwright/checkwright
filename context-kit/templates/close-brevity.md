**Always-loaded brevity pass** — the recurring close-stage step that reacts to
the standing per-session context cost. Splice this into your close skill (the
guard-kit `close-triage.md` pattern). It reacts to the meter's *delta*, not
its level: close is net-additive by design, so only growth since the iteration
baseline is actionable.

1. **Measure the delta.** Run `bash context-kit/bin/always-loaded.sh` — it
   prints the total, the per-part split, and the delta against the committed
   baseline. A near-zero delta means little grew; a large one is the worklist.
2. **Walk the growth since baseline, asking two distinct questions per block:**
   - **Staleness** — *is it still true?* Outdated context is a standing tax on
     every session that reads it.
   - **Brevity** — *is each block worth its recurring per-session token cost?*
     Dense is fine; redundant or over-explained is not.
3. **Resolve by rewording or deleting, never by annotating.** Outdated context
   goes to git history, not to a "formerly…" note — a narration line is new
   standing cost that documents the old cost. Two mechanical floors sit under
   this step: `check-brevity` (over-budget bullets that already point to a
   deeper doc) and `check-manifest-temporal` (the lexical share — a fixed set
   of `formerly…`-class markers in the manifest set); this pass is the semantic
   residue neither gate can decide.
4. **On-demand files are exempt.** SPECs, handbooks, and other read-when-needed
   docs pay their cost only when opened — do not brevity-trim them here; this
   pass governs the *always-loaded* surface alone.
5. **Re-baseline and commit.** Finish with
   `bash context-kit/bin/always-loaded.sh --update-baseline` and commit the
   baseline file, so next iteration's delta measures from this close.

Goal: the always-loaded surface grows only where the growth earns its recurring
cost, and every session pays for context that is still true and still terse.
