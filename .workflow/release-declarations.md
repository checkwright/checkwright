# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-amendment-queue` — a queue path that is a dangling symlink now refuses at exit 2 (`not a file`), where it read as an absent queue and passed as not applicable. Nothing to do unless a queue link in your tree dangles: repoint it, or remove it to read the queue as absent.

## Behavior changes

- **`escalation-guard`, `wakeup-guard`, `agent-budget-guard`, `agent-dispatch-guard`** — each now runs outside any repository: `--knobs <file>` and `init --user <member>` accept them, where `--knobs` refused at exit 2 and a registration firing with no repository printed the no-tree notice and allowed the call. There every path knob must be absolute, and a relative one takes the member's own route: `wakeup-guard` still denies and names the fault in its denial, the budget guard reads `STALE`, the dispatch guard leaves D5 unenforced with its note, and `escalation-guard` declines. Nothing to do in a repository; to run one for your user, `init --user <member>`.
- **`init --user`** — the printed registration now carries the member's `matcher` for a before-tool-call member, and the knob file carries a log knob only for a member keeping a log (`DELEGATION_KIT_STOP_LOG` for `subagent-stop-liveness`, `GUARD_KIT_WAKEUP_LOG` for `wakeup-guard`). Nothing to do for an existing `subagent-stop-liveness` install.
- **`--hook <member> --observe`** — an observed `session-context` now logs `would=advise reason=withheld` for its plain-text brief, where it logged `would=error`. Nothing to do.
- **`check-todo-task-liveness`, `check-deprecation-task`** — an absent queue file (`CANON_KIT_QUEUE_FILE`) now reads not applicable and exits 0 with a clean line naming the path, where each refused at exit 2 (`queue file not found`) before scanning, so a `TODO(task:)` or deprecation marker on the surface goes unresolved there; a queue path naming anything but a file still refuses (`not a file`). Nothing to do; a tree tracking work outside the queue grammar can now register both gates.
