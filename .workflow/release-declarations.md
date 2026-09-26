# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's three declaration-bearing sections in the note's grammar, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## Tightened gates

- `check-survey-record` — a survey block's `corpus` must now be the literal `none` or space-separated git pathspecs, single-quoted where one carries a quote, a shell metacharacter, a glob character or pathspec magic, and in bare mode each pathspec must match a path at the block's `rev`. A block whose corpus is prose, carries an unquoted glob, or names nothing reds; rewrite its corpus to the pathspecs the survey read (scoping prose moves to `finding`), or `none` for a survey over no tree corpus.

## Behavior changes

- **`--emit file-survey`** — the arm now refuses at exit 2, filing nothing, a corpus outside the grammar above or naming a pathspec that matches no path tracked at HEAD, and names each unmatched pathspec. File the pathspecs the survey read, single-quoted where needed, or `none`; a `none` corpus's witness hint prints the oracle re-run alone.
- **`--emit file-gap`** and **`--emit file-survey`** — both now refuse at exit 2, writing nothing, when run in a linked worktree, where the tracked line would be lost with an uncommitted worktree. An isolated child hands the finding back to its dispatcher, who files it from the main checkout; a session that works and commits in a linked worktree appends the line by hand in the record's grammar.
- **`--hook workflow-state-guard`** — with `LIFECYCLE_KIT_STAGE_SESSION_TYPES` set, a dispatched stage session whose stage the cursor has left is now refused `Write` and `Edit` outside the scratch dir `GATE_SDK_TMP_DIR` names; its resume journal there stays writable. Such a session answers through its report and the live stage session lands the change. With the knob empty, nothing changes.
