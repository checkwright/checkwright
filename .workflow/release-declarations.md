# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-migration-guard` — reads a double-quoted SQL identifier as its name, a doubled quote inside it as one, where it read one as a string value and stripped it: `ALTER TABLE "users" DROP COLUMN "email"` now matches `ALTER TABLE .* DROP` and reds unless declared. It judges only an added or edited migration, so a clean upgrade cannot red it; a destructive migration you author after upgrading can.
- `check-binary-strings` — reads a string down to the length of the shortest deny term able to hit, where every run under 4 bytes went unread: a three-byte term alone in a three-byte run now reds. The floor never drops below the shortest term, so a list whose terms are all 4 bytes or longer reads exactly as before.

## Knob changes

- `GATE_SDK_PIPEFAIL_WAIVER_MIN` — new: the shortest trimmed reason a `pipefail-ok` waiver takes, read by `check-pipefail-readers` and `--emit pipefail-fix`, default `1`, the floor both read before. Any value but a positive integer refuses at exit 2. Nothing to do unless you hold waiver reasons to a length.
- `GATE_SDK_STALE_LITERAL_CHARS` — new: the fewest characters a literal's fragment takes to count for `--emit stale-literals`, default `8`, the floor it read before. Any value but a positive integer refuses at exit 2.
- `GATE_SDK_STALE_LITERAL_LETTERS` — new: the fewest ASCII letters such a fragment holds, default `4`, the floor it read before. Any value but a non-negative integer, or one above `GATE_SDK_STALE_LITERAL_CHARS`, refuses at exit 2 naming both; lower the two together.

## Behavior changes

- An optional configured list (the `GATE_SDK_DENY_TERMS` lists, the `GATE_SDK_SITE_RULES` file, the destructive-migration list) takes its absent default only when nothing is at its path. A lookup that fails otherwise, a permission error or a path through a non-directory, is now exit 3 where it read as no list and passed, across check-tree-terms, check-path-names, check-binary-strings, check-site-count and check-migration-guard.
- `check-migration-guard` takes a migration directory at any one width, `<digits>_<name>.sql`, where it took four digits alone: a five-digit history now reads clean. Every file at the merge base and in the working tree shares the width of the first by name, and a file off it, or a number too large for 64 bits, is exit 3, naming it.
