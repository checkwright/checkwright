# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-26 scope — Does checkwright write a lock, holder or record file at a predictable name in a shared directory such as the system temp dir?
- corpus: native/src gate-sdk/lib gate-sdk/bin installer scripts docs/install.sh
- oracle: git grep -nE 'temp_dir\(\)|TMPDIR:-/tmp|\.lock"|\.run"' -- native/src gate-sdk/lib gate-sdk/bin installer scripts docs/install.sh
- rev: fb70e903c02eea24a548d348f3a0d79a7f2f2235
- finding: Read outside cfg(test) modules and outside fixtures, smoke and test suites. No lock, holder or record file is written to a shared dir by default: run-validate.lock, the .run liveness records and the run scratch live under the repo-local tmp dir, checkwright.lock in the consumer tree, user.lock in the per-user data dir. Not a lock, filed as shared-temp-fixed-names: gate-sdk/lib/test-hermetic.sh names three fixed files under TMPDIR or /tmp in a standalone suite run (the test-build gitconfig pin in proc.rs likewise); and proc.rs spawn captures checkwright-out.<pid>.<seq> fall back to the system temp dir, File::create, when no run scratch is held.
- inferred: whether a consumer knob pointing GATE_SDK_TMP_DIR or EVIDENCE_KIT_LOCK_FILE into a shared dir is a case worth guarding

## 2026-11-26 scope — Which deferred entries rank into this set, by cost class, roadmap and surface, and what cites or supersedes each candidate?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: fb70e903c02eea24a548d348f3a0d79a7f2f2235
- finding: No deferred row is session- or iteration-class and no recurrence reaches the threshold (record-stamp-encoding-compression holds one date against 2). No roadmap row bears on the adopter's asks. The gate-sdk surface carries the most rows: the five entries drained from the inbox this scope plus exit-status-funnel-typing, manifest-name-deny-terms and evaluator-root-agent-file. No live entry cites a candidate, and a grep of the deferred pool and icebox for the candidates' names (migration, proc spawn, LC_ALL, locale, release-bump, release-disposition, test-hermetic, pipefail, binary-strings, stale-literals, script-preflight, exec-write, TMPDIR) finds no replacing or re-conventioning entry: supersession none.
- inferred: nothing
