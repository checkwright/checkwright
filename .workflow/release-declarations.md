# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-exec-write` — reads a turbofish call with whitespace on either side of its `::`, where `std::fs::copy:: <&str, &str>(a, b)` and `std::fs::copy ::<&str, &str>(a, b)` read as no call and passed: such a site now reds like its unspaced spelling; move it into a helper, as for any other site.

## Knob changes

- `GATE_SDK_MIGRATION_UNRELATED_REFS` — new: what `check-migration-guard` does with a ref sharing no merge base with the base ref, an orphan or a shallow cut, default `off`, which names it on the verdict line, or `error`, which refuses at exit 3 naming each such ref. Any other value refuses at exit 2. Select `error` where your CI must judge every sibling ref, and fetch the history its merge base needs.

## Behavior changes

- **`--verdict-key [--gates-dir <dir>]`** — new arm: prints the key a bare registry run would store this tree under, `{"contract":"1.0.0","gateSet":…,"tool":{"version":…},"treeId":…}`, and runs no member, so a caller hands the three fields to `--verdict-lookup` without re-deriving them. Exit 0 the key; 2 the `usage` error document for any other operand, and the `refused` one for no repository, an unborn `HEAD`, a modified worktree, an unstamped binary, an unreadable member file or a refused registry; 3 the `input` document for an unreadable registry, the `tool` one for a refused repository. The goldens are published at `/contracts/verdict-key/v1/`. Nothing to do unless you derive the key yourself: read it here instead.

- **`check-migration-guard` names an unrelated ref** — a ref the clash check skips for sharing no merge base with the base ref is now named on the clean line, `, <j> unrelated ref(s) skipped: <ref> …` after the count of refs read, and on a red run's header before its closing colon, where it was skipped silently. A shallow CI clone now shows the siblings its clash check never read. Nothing to do; a line parser keyed on the clean line's end reads one more clause.

- **`--emit pipefail-fix` and a redirected `grep`** — leaves a quiet or bounded `grep` stage carrying a redirection as a hand fix, where the rewrite reordered it: `grep -m1 x >out` became `grep x >out | sed -n '1,1p'`, writing every match to `out`, and `grep -q x 2>&1` became `grep x >/dev/null 2>&1`, discarding its errors. Such a hit still reds, now without `[auto-fix]`; fix it by hand.

- **`--run` outside a git repository** — the binary's `--run` arm now refuses at exit 2 with `run-gates: not inside a git repository`, and under `--json` prints the `refused` error document, where it ran the registry and exited 0 with a pass document carrying no `treeId` and no `worktree`. Run it from inside the checkout it judges; the front end already refused there. A `--queue` verb's post-check is that run, so a move written outside a repository now exits 1.

- **`--run`'s usage spelling** — the usage text and `--run --help` now both spell `--run [--json] [--gates-dir <dir>]`; the bare `<dir>` positional stays accepted as an alias. Nothing to do.

- **The platform archives' binary mode** — each `checkwright-gates-<version>-<target>.tar.gz` now carries its binary at `0755`, where it shipped at `0644` and had to be made executable before it ran. Nothing to do; a `chmod +x` after extraction is no longer needed.

- **`lib/test-hermetic.sh` sourced twice** — a second source in one shell under errexit reaches the private-dir checks, where the `mkdir` of the directory already present stopped it at exit 1 with no message. Nothing to do.
