# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-duplicate-text` — new canon-kit gate, armed by the new `CANON_KIT_DUPLICATE_TEXT_GLOBS` (default empty, off): no run of `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` words or more (default `12`) is stated in two declared files, read on the prose word stream, so backtick-fenced code, link targets, path-shaped tokens, headings, table rows, generated regions and front matter never count. While the globs knob stays empty the gate finds nothing, though `check-gate-substrate-parity` still wants it registered or declared (below); to arm it, register it, set the globs to the files that must not restate each other, the exclude knob to the generated copies among them, and point each finding's later copy at its home. A deliberate keep takes `<!-- duplicate-text-exempt: <reason> -->`.
- `check-gate-substrate-parity` — assertion I reds on `check-duplicate-text`, a new canon-kit gate the binary carries, until your `gates.list` registers it or declares `# unregistered: check-duplicate-text — <reason>`. Register it, or declare why you do not.

## Knob changes

- `CANON_KIT_DUPLICATE_TEXT_GLOBS` — new: the files `check-duplicate-text` compares, an array of globs, default empty (the gate off).
- `CANON_KIT_DUPLICATE_TEXT_EXCLUDE` — new: the files subtracted from that set, such as a generated mirror, default empty.
- `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` — new: the shortest shared run, an integer of at least `3`, default `12`.
- `CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE` — new: `on` also pairs two units of one file, default `off`.

## Behavior changes

- **The marker writers and the `init` agent-file guard** — a target link resolving to a directory, or to anything else that is no regular file, now refuses as a link escaping the repository does, with nothing written; before, `init` replaced such a `CLAUDE.md` link with its seed. A target reached through a linked directory resolving outside the repository refuses too, where before only a linked leaf was contained. Nothing to reconcile unless a block or seed target of yours sits behind such a link: point the link at a regular file inside the repository, or unlink the directory.
- **`--rewrite`** — two operands whose names differ only in bytes that are not UTF-8 are now two files, where the arm used to refuse the second as the first named twice. Under a working directory whose name is not UTF-8, an operand resolving to a sibling directory alike in the lossy spelling now refuses as outside the repository, where it used to pass. Nothing to do.
