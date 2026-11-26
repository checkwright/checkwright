# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-migration-guard` — reads a double-quoted SQL identifier as its name, a doubled quote inside it as one, where it read one as a string value and stripped it: `ALTER TABLE "users" DROP COLUMN "email"` now matches `ALTER TABLE .* DROP` and reds unless declared. It judges only an added or edited migration, so a clean upgrade cannot red it; a destructive migration you author after upgrading can.

## Behavior changes

- An optional configured list (the `GATE_SDK_DENY_TERMS` lists, the `GATE_SDK_SITE_RULES` file, the destructive-migration list) takes its absent default only when nothing is at its path. A lookup that fails otherwise, a permission error or a path through a non-directory, is now exit 3 where it read as no list and passed, across check-tree-terms, check-path-names, check-binary-strings, check-site-count and check-migration-guard.
