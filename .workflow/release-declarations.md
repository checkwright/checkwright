# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-exec-write` — reads a turbofish call with whitespace on either side of its `::`, where `std::fs::copy:: <&str, &str>(a, b)` and `std::fs::copy ::<&str, &str>(a, b)` read as no call and passed: such a site now reds like its unspaced spelling; move it into a helper, as for any other site.

## Behavior changes

- **`--emit pipefail-fix` and a redirected `grep`** — leaves a quiet or bounded `grep` stage carrying a redirection as a hand fix, where the rewrite reordered it: `grep -m1 x >out` became `grep x >out | sed -n '1,1p'`, writing every match to `out`, and `grep -q x 2>&1` became `grep x >/dev/null 2>&1`, discarding its errors. Such a hit still reds, now without `[auto-fix]`; fix it by hand.

- **`lib/test-hermetic.sh` sourced twice** — a second source in one shell under errexit reaches the private-dir checks, where the `mkdir` of the directory already present stopped it at exit 1 with no message. Nothing to do.
