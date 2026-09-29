# contract: gate-sdk/SPEC.md §upgrade-smoke — the accumulating release declaration surface; the note's three declaration-bearing sections in the note's grammar, appended by the session landing a kit-shipped change or the one discovering its omission, composed into the release note and drained to this header at the tag.

## Tightened gates

- `check-close-surfaces` — a new assertion D reds a gitignored roster row whose `reclaim=` does not rotate its log, since close reads every row before reclaiming it and a truncation erases the lines appended in between. Spell the reclaim `<gate binary> --emit capture-drain <path>`, and read the drain file it prints rather than the live log.

## Behavior changes

- **delegation-kit/SPEC.md §The turn-end liveness hook** and **§bin/wait-probe** — the turn-end hook's log and the probe's evidence file now reclaim by `--emit capture-drain` instead of truncation, so close reads the drain file it prints and removes it with `--done`. A consumer allowlisting the old `: > <log>` clear may drop that entry.
- **gate-sdk/SPEC.md §gen-pre-commit** — the generated hooks become a two-line handoff to the gate binary's `--git-hook` arm, which reads the `# graph:` manifests at commit time, so a manifest or registry edit no longer needs a regeneration, and `update` rewrites an adopter's hooks. The lines the hooks print are unchanged.
- **gate-sdk/SPEC.md §The `# graph:` manifest** — `gen=manual` is retired and a manifest carrying `gen=` reds `check-graph`; a hand-written hook region becomes a `trigger=*` shell gate that reads the staged set itself.
