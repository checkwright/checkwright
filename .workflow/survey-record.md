# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
















































## 2026-08-30 scope — Does any deferred entry reach the recurrence pre-emption threshold, and is the one-date-per-declaration state a defect?
- corpus: TASK-QUEUE.md
- oracle: grep -n 'recurrence:' over the ## Deferred section, unanchored
- rev: 74018ceb47845d391c785f3f0fdbe9f159d85096
- edges: none
- finding: MEASURED: 29 declarations across 283 deferred entries, every one carrying exactly ONE date, so ZERO entries reach the threshold of 2. Zero-at-threshold is the EXPECTED steady state, not a defect: git history holds 28 multi-date declarations (up to four dates) and appending demonstrably works (2a101d24, 8708890b append rather than rewrite); an entry reaching two dates is pre-empted into a unit set, built, and leaves Deferred, so the rule consumes its own input. Traced end to end on boundary-wipe-preserve-lifetime-scope (stamped d6dee4ac, second date 679f2e9c, gone at b17dd45b). No live declaration is malformed, so the anchored/unanchored grep delta at HEAD is zero. ONE REAL EXCEPTION: dated-measurement-restatement-class attests a 2026-08-25 second instance in its own body that was never stamped (the observing close 2c312f23 wrote the prose and no declaration; the declaration was first created at 8708890b carrying only 2026-08-29), so its true date count is 2 and it SHOULD pre-empt. Distinct from recurrence-threshold-counts-dates-not-incidences (different calendar days, not same-day collapse), from recurrence-declaration-grammar-ungated (well-formed and fully visible to the prescribed oracle), from recurrence-resolver-literal-match-only and from recurrence-obligation-residency. Two false claims also found in the live recurrence-declaration-grammar-ungated entry: it misattributes its witness to 8a29e8ec when 2c312f23 created the slugless line and 8a29e8ec only extended it, and its cost line claims a live unreadable declaration when none survives.
