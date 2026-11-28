# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-duplicate-text` — new canon-kit gate, armed by the new `CANON_KIT_DUPLICATE_TEXT_GLOBS` (default empty, off): no run of `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` words or more (default `12`) is stated in two declared files, read on the prose word stream, so fenced code, link targets, path-shaped tokens, headings, table rows, generated regions and front matter never count. Nothing to do while the globs knob stays empty; to arm it, register it, set the globs to the files that must not restate each other, the exclude knob to the generated copies among them, and point each finding's later copy at its home. A deliberate keep takes `<!-- duplicate-text-exempt: <reason> -->`.
- `check-gate-substrate-parity` — assertion I reds on `check-duplicate-text`, a new canon-kit gate the binary carries, until your `gates.list` registers it or declares `# unregistered: check-duplicate-text — <reason>`. Register it, or declare why you do not.

## Knob changes

- `CANON_KIT_DUPLICATE_TEXT_GLOBS` — new: the files `check-duplicate-text` compares, an array of globs, default empty (the gate off).
- `CANON_KIT_DUPLICATE_TEXT_EXCLUDE` — new: the files subtracted from that set, such as a generated mirror, default empty.
- `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` — new: the shortest shared run, an integer of at least `3`, default `12`.
- `CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE` — new: `on` also pairs two units of one file, default `off`.
