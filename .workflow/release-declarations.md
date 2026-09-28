# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's three declaration-bearing sections in the note's grammar, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## Tightened gates

- `check-queue-hygiene` — a tag line's `[recurrence:]` array must now hold only calendar-valid `YYYY-MM-DD` dates, comma-separated; a space-separated array, an impossible date or an empty value reds, where the counting readers dropped it silently. Rewrite the array in that form; `--queue recur <slug>` stamps it so.

## Behavior changes

- **`--queue`** — new arm: one move or stamp on the queue file — `promote`, `done`, `clear-done`, `icebox`, `recur`, `demote`, `thaw` or `split` — written only when its postcondition on the live and done sets holds, then post-checked by every gate coupled to the queue file; exit 0 is written and green, 1 written with a red to fix before you commit, 2 nothing written. `--emit queue-history <slug>` prints one entry's section transitions, oldest first. Nothing to do; a narrow allowlist grants one verb as `Bash(<door> --queue <verb> *)`.
- **`lifecycle-kit/templates/stages/close.md`**, **`spec.md`**, **`scope.md`** and **`build.md`** — each queue move they instruct now names its verb in place of the hand edit: `--queue recur` for a judged recurrence, `--queue done` for a Done move, `--queue clear-done`, `--queue promote` and `--queue demote`. Act only if your stage bindings restate those moves rather than pointing at the templates: take the verbs.
