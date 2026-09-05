**Always-loaded brevity pass** — the recurring close-stage step that reacts to
the standing per-session context cost. Splice this into your close skill (the
guard-kit `close-triage.md` pattern). It reacts to the meter's *delta*, not
its level: close is net-additive by design, so only growth since the iteration
baseline is actionable.

1. **Measure the delta, then the growth.** Run `bash context-kit/bin/always-loaded.sh`
   — it prints the total, the per-part split, and the delta against the committed
   baseline. Then run it with `--growth`: every governed prose file that grew net
   since the baseline commit, largest first. Both lists are the worklist.
2. **Walk the growth since baseline, asking two distinct questions per block:**
   - **Staleness** — *is it still true?* Outdated context is a standing tax on
     every session that reads it.
   - **Brevity** — *is each block worth its recurring per-session token cost?*
     Dense is fine; redundant or over-explained is not.
3. **Resolve by rewording or deleting, never by annotating.** Outdated context
   goes to git history, not to a `formerly…` note — a narration line is new
   standing cost that documents the old cost. Two mechanical floors sit under
   this step: `check-brevity` (over-budget bullets that already point to a
   deeper doc) and `check-manifest-temporal` (the lexical share — a fixed set
   of `formerly…`-class markers in the manifest set); this pass is the semantic
   residue neither gate can decide.
4. **No file is exempt.** An on-demand file pays its cost at every open, and a
   SPEC a stage opens each iteration is always-loaded in effect; the always-loaded
   surface is only the tier that pays most often. Walk every file the growth
   list names with the same two questions, and state the growth figure in the
   close commit beside the delta.
5. **Re-baseline and commit.** Finish with
   `bash context-kit/bin/always-loaded.sh --update-baseline` and commit the
   baseline file, so next iteration's delta and growth measure from this close.

Goal: every governed prose file grows only where the growth earns its cost, and
every session pays for context that is still true and still terse.
