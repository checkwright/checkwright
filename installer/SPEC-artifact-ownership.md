# SPEC amendment: artifact-ownership

installer/SPEC.md gives the installed gate binary two owners. §The gate binary says a compiled artifact has no version the adopter authored, so a binary failing its recorded digest is corrupt or substituted, never edited, and `init` rewrites it rather than reporting it as changed. §uninstall says the binary "needs no special case", so the roster walk treats it like any row and keeps and reports it when its hash differs, and §The manifest says §init alone reads the row as ownership "for every path but this one". `native/src/installer/uninstall.rs` references no artifact, so it implements §uninstall's reading.

**The ruling: the binary is never the adopter's, in every verb.** `uninstall` removes it whatever its hash. Keeping it protects nothing: a kept path goes into the residual manifest so that "a future init still protects" it (§The manifest), yet the next `init` rewrites a mismatched binary by §The gate binary's exemption. So the keep branch records the binary as the adopter's and then the next install overwrites it. The other answer, extending the keep rule to `init`, is refused. It would reverse the exemption that lets `doctor`'s remedy for a digest mismatch, a bare re-run, work.

**Measured at authoring (2026-09-27):**

- `grep -n "artifact" native/src/installer/uninstall.rs` returns nothing. The partition loop (`remove`, from the `entries` binding to the end of the `for` over them) sends every present row whose `git hash-object` hash differs from the recorded one to `keep`, unless `--force`.
- `native/src/install.rs` `place` returns `own\t<dest>` for the artifact on every run, and `init.rs` records each `own` path in `files`. So the binary is a `files` row at a `git hash-object` hash (installer/SPEC.md §The manifest, "The artifact is on both maps").
- `native/src/installer/doctor.rs` `seam_binary` resolves the installed binary's path from `GATE_SDK_NATIVE_BIN` in the seam file the manifest records. That is the path owner §The manifest names ("The `artifact` key carries no path"). It is private to `doctor.rs` and has one caller.
- The consumer smoke's seam arm (`installer/consumer-smoke/run-smoke.sh`, the block after the comment naming "the protection branch") runs `uninstall` on a consumer carrying two committed adopter edits. It asserts that the two edits survive, that every other roster path is gone and that the residual roster is exactly those two. No arm tampers with the binary before an `uninstall`.

## What changes

### (1) uninstall removes the artifact row whatever its hash {design-bearing}

**Not yet applied.** In `native/src/installer/uninstall.rs` `remove`, the artifact row joins `remove_set` whenever it is present, under the same test as `--force`. The artifact row is the `files` entry whose path equals the path `GATE_SDK_NATIVE_BIN` names in the recorded seam file, and it is resolved only when the manifest carries an `artifact` key. The resolution is `seam_binary`, which moves from `doctor.rs` to `native/src/installer/mod.rs` beside `files_under`. `doctor` and `uninstall` then call one function, so there is one answer to where the binary is. Other rows keep the hash rule unchanged. Two cases resolve no artifact row and so exempt nothing: a manifest with no `artifact` key (a residual manifest, or an install that placed no binary), and a seam naming a path off the roster. Every row then meets the hash rule, which is today's behaviour.

The partition moves out of `remove` into a pure function over the entries, their hashes, the artifact path and `--force`. It returns the remove, keep and gone sets, so a unit test can drive it without a repository.

The `USAGE` text's last line becomes:

> init wrote it is kept and reported, except the gate binary, which is never yours; --force removes it anyway.

`--dry-run` lists the binary under what it would remove, with no separate line. The binary is on the remove side of the plan, and the plan already names every path it removes.

### (2) The SPEC states one ownership rule {mechanical}

**Not yet applied.** In installer/SPEC.md §uninstall, the paragraph "The gate binary needs no special case. …" becomes:

> **The gate binary is removed whatever its hash.** It is never yours (§The gate binary): a binary that fails its recorded hash is corrupt or substituted, not edited, so keeping it would record it as yours in the residual manifest and the next `init` would rewrite it anyway. `uninstall` finds its row by the path `GATE_SDK_NATIVE_BIN` names in the recorded seam file, as §doctor does. A manifest with no `artifact` key has no such row, and every row meets the hash rule.

In installer/SPEC.md §The manifest, "and §doctor, §uninstall and §init each read the one they mean, §init alone reading the row as ownership for every path but this one." becomes:

> and §doctor, §uninstall and §init each read the one they mean. §init and §uninstall read the row as ownership for every path but this one.

In installer/SPEC.md §The gate binary, "so `init` rewrites it from the payload copy it just verified rather than adding it to the changed-file report." becomes:

> so `init` rewrites it from the payload copy it just verified rather than adding it to the changed-file report, and `uninstall` removes it rather than keeping it (§uninstall).

### (3) The smoke tampers with the binary before the protection branch {mechanical}

**Not yet applied.** In `installer/consumer-smoke/run-smoke.sh`'s seam arm, after the same-version re-run and before `diff`, the arm appends a byte to the installed binary (`scripts/checkwright-gates`, suffixed where the host's artifact carries one, the path read from the consumer's seam as `doctor` reads it) and commits the change. After `uninstall`, it asserts three things. The binary is off the tree. The residual roster is still exactly the two seam surfaces, which the existing assertion already checks and which now also proves the binary is not in it. `uninstall`'s output does not name the binary among the kept files. `diff`'s assertion needs no change: it checks that the two edited surfaces are named, and a third named path does not break that.

A unit test in `uninstall.rs` drives the partition from delta 1 through four cases: the artifact row at a mismatched hash is removed, an ordinary mismatched row is kept, a row off the tree is gone, and with no artifact path a mismatched artifact-path row is kept.

## Producers and consumers

- **The artifact-row path.** Producer: `seam_binary` over the recorded seam file, called by `uninstall` only when the manifest's `artifact` key is present. Consumers: the partition, which removes that row whatever its hash, and `doctor`'s existing re-verification, which is unchanged.
- **No new state, event, field or knob.** The manifest schema is unchanged. The residual manifest now never records the binary. Its readers (`doctor`'s residue reading and `init`'s next run) read `files` as before.
- **Point 5.** The keep set narrows by the artifact row. Its readers are the residual-manifest writer, `uninstall`'s kept-files report and the "nothing to remove" branch. None reds on a smaller set, and the "nothing to remove" branch is now reached less often, because a tampered binary alone counts as something to remove. The smoke's residual-roster assertion is an exact-set equality, and the set it expects is unchanged.
- **Oracle.** The unit test (delta 3) at commit, through `check-crate-arms`. The consumer smoke at validate, through the `installer_smoke` suite.

## Existing sections updated

Roster from `grep -n "needs no special case\|alone reading the row\|changed-file report" installer/SPEC.md`, `grep -n "seam_binary" -r native/src` and `grep -n "protection branch" installer/consumer-smoke/run-smoke.sh`, run 2026-09-27.

- `native/src/installer/uninstall.rs` (deltas 1 and 3).
- `native/src/installer/doctor.rs` and `native/src/installer/mod.rs`, for the moved `seam_binary` (delta 1).
- installer/SPEC.md §uninstall, §The manifest and §The gate binary (delta 2).
- `installer/consumer-smoke/run-smoke.sh` seam arm (delta 3).
- The on-site mirror of `installer/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (delta 2).
- `.workflow/release-declarations.md` gets one Behavior changes bullet: `uninstall` now removes the installed gate binary even when it no longer matches the hash `init` recorded, where it kept it and reported it as yours. Nothing to do (delta 1).

## Retired spellings

- None — no delta renames or deletes a spelling. The retired passage is prose, not a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the artifact-row resolution and the narrowed keep set.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit rewrites the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls installer/SPEC-*.md`).
- [ ] **Entry moved.** `uninstall-artifact-ownership-asymmetry` moves to Done in the landing commit, at a stage before the drain stage. Its oracles are local (the crate tests and the consumer smoke).
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
