**Tooling-friction triage** — the recurring close-stage step that keeps the
permission-friction loop a habit, not a one-off cleanup. Splice this in place
of lifecycle-kit's `tooling-friction triage` placeholder (close skill, step 2).

1. **Rank what nothing granted.** Run `bash guard-kit/bin/scan-prompts.sh` — it
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
   **Diagnose before you reach for the allowlist.** A pattern can rank high while
   already being granted: an allowlist entry matches a *bare* command, so a call
   that chains (`&&`, `;`), redirects, or expands breaks the match and is decided
   out of band anyway. Check a ranked pattern against the committed allowlist first — if it is
   already there, the finding is (c) habit change, and adding coverage buys
   nothing while reading as a fix.
3. **Review the wakeup log** if the wakeup-guard is wired: read
   `.workflow/wakeup-attempts.log`, act on any surfaced intent, then delete it.
4. **Prune and narrow the local overlay.** Run
   `bash guard-kit/bin/compare-settings-allow.sh` — it reports two sets, and
   each has its own disposition.
   - **Redundant**: remove every listed `settings.local.json` entry (a committed
     glob already grants it).
   - **Too broad**: for every entry the breadth report names, either narrow the
     glob or record that its breadth is intended. The probe printed beside it is
     the witness — the command that glob auto-allows. An empty breadth report is
     not a proof of narrowness: probes are witnesses, and no completeness is
     claimed (the section is absent entirely when no probes are declared).

   - **Not content-pinned**: an entry naming a *script path* rather than a fixed
     command grants whatever that file says at run time, so its meaning changes
     whenever the file does — and a path under the gitignored scratch dir is
     rewritable by any session. Read the shape, not the literal: it reads as a
     specific command until one notices the target is writable. Such a run has a
     sanctioned form already — `bin/scratch-run.sh`, whose echo-at-execution is
     the compensating control (§scratch-run) — so route the run through the
     runner and remove the direct-path grant. Removing instances without applying
     this criterion re-arms for the next one.

   Then, by judgment, prune the remaining one-off exact-string local entries and
   promote recurring safe patterns to the committed `settings.json` as globs.
   Widening the committed set is the **consumer's** call, not the session's: an
   agent may propose a standing grant and may prune, but it does not widen its
   own auto-allow set on its own say-so.
5. **Clear the friction log** — its named reclaim path:
   `: > .workflow/prompt-friction.log`. Run each clear as its own bare command:
   the allowlist entry is an exact string, so compounding it (`&&`, `;`, a
   trailing `echo`/`wc`) breaks the match and buys the very out-of-band decision
   this step just triaged away.

Goal: the local set stays small, every durable pattern lives in the
committed, reviewable allowlist, and no local glob auto-allows a command you
declared bad.
