# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's declaration-bearing sections in the note's grammar, Platforms excepted since composition derives it, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## New and tightened gates

- `check-exec-write` — reads a turbofish call whose type argument holds a fn-pointer arrow, where `std::fs::copy::<fn() -> ()>(a, b)` read as no call and passed: such a site now reds like its plain spelling; move it into a helper, as for any other site. A `fn(…)` pointer type is no longer read as a call to `fn`.

## Behavior changes

- **The platform archives' sidecars and attestation** — each `checkwright-gates-<version>-<target>.tar.gz` on a Release now carries its own `.sha256` beside it, and every Release asset other than a sidecar is the subject of a build attestation: the tarball's unchanged, the archives and the companion archive in a second. The `release` job verifies every asset's attestation before the Release exists. Nothing to do; a binary-only consumer can now check an archive before extracting it, with `sha256sum -c` against its sidecar and `gh attestation verify <archive> --repo checkwright/checkwright --source-ref refs/tags/v<version>`.
