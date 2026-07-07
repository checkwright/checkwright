**Tooling-friction triage** — the recurring close-stage step that keeps the
permission-friction loop a habit, not a one-off cleanup. Splice this in place
of lifecycle-kit's `tooling-friction triage` placeholder (close skill, step 2).

1. **Rank what prompted.** Run `bash guard-kit/bin/scan-prompts.sh` — it
   filters the friction log against the committed allowlist and the harness's
   built-in read-only auto-allows, then ranks the survivors by command pattern.
2. **Resolve each recurring pattern by the triage criterion** — never default
   to the allowlist:
   - **Allowlist** (`Bash(...)` in the committed settings) when the command is
     safe *and already in the form to reinforce* — static, glob-matched.
   - **Guard rule** when a *better form exists* (steer to it) or the decision
     needs logic a static glob cannot express.
   - **Habit change** (a noted convention) for a true one-off.
   Caution: an allowlist entry can *mask* a steering opportunity — before
   blessing a form, confirm it is the one to reinforce.
3. **Review the wakeup log** if the wakeup-guard is wired: read
   `.workflow/wakeup-attempts.log`, act on any surfaced intent, then delete it.
4. **Prune the local overlay.** Run
   `bash guard-kit/bin/compare-settings-allow.sh` and remove every listed
   `settings.local.json` entry (a committed glob already grants it). Then, by
   judgment, prune the remaining one-off exact-string local entries and promote
   recurring safe patterns to the committed `settings.json` as globs.
5. **Clear the friction log** — its named reclaim path:
   `: > .workflow/prompt-friction.log`.

Goal: the local set stays small, and every durable pattern lives in the
committed, reviewable allowlist.
