# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-13 scope — Which open guard finding moved its verdict since the published v0.31.0, the release bar's question?
- corpus: native/src/guard guard-kit/guard-tests
- oracle: ./native/target/release/checkwright-gates --scratch-run .tmp/scope-compare.sh .tmp/v031/checkwright-gates-0.31.0-x86_64-unknown-linux-musl/checkwright-gates native/target/release/checkwright-gates < .tmp/scope-probe-cases.txt
- rev: 4ca2b05f92329890e14a60d62c64a54436e9b5be
- finding: Four rows moved and two are wrong grants at both. ': > .tmp/a>README.md' and ': >.tmp/a>README.md': allow then and now. ': >& .tmp/a>README.md': fall-through then, allow now. 'echo x >> .tmp/a>README.md' and 'echo x >> .tmp/a>>README.md': block then, allow now. 'cat -- '-n' README.md': allow then, block now. 'grep x README.md | sed -n '\-x-p'': fall-through at both. The spaced ': > .tmp/a >README.md' falls through at both.
- inferred: that this repository's knob file would not change a row: both binaries ran from one zero-config scratch git repository whose .tmp/ is ignored. The oracle needs the v0.31.0 x86_64-unknown-linux-musl Release binary unpacked under .tmp/v031 and made executable, which a boundary wipe or the session sweep removes.

## 2026-10-13 scope — Which deferred entries lead this iteration's unit set?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 4ca2b05f92329890e14a60d62c64a54436e9b5be
- finding: One non-event row: heterogeneous-agent-delegation, iteration/low, held at scope 2026-10-09 by operator direction. No deferred entry is at the recurrence threshold; guard-drain-fix-unreviewed carries one date and its lead's judgment of a third incidence. The guard-kit surface carries eight rows, the most of any, none with a live inbound edge. The release that tarball-attestation-observed, front-door-job-observed and companion-toolkit-profile wait on is held by scratch-grant-glued-redirect, a wrong grant, and cat-operand-after-double-dash, a regression from v0.31.0. Supersession: sed-program-escaped-dash and cat-operand-after-double-dash are reshaped by guard-rules-skeleton-word-read, whose lift of the same two rules rewrites the reads they fix; session-sweep-live-scratch is reshaped by session-sweep-horizon-baked, one function.
- inferred: that the word-read lift of rules cat_file and ro_pipeline would rewrite the reads the two small guard entries fix: read off guard-tests/variant-limits.tsv's rows for those rules and the entries' deliverables, no lift exists to run

## 2026-10-13 build — Which bash decision rows keep a grant when a write onto the sandbox's tracked file is appended, after scratch-grant-glued-redirect's fix?
- corpus: guard-kit/guard-tests/cases.tsv
- oracle: an out-of-crate deriver appending ' >tracked.md', '>tracked.md', '>>tracked.md', '>&tracked.md', '2>tracked.md', '&>tracked.md', '>|tracked.md', ' <>tracked.md' and '<>tracked.md' to each eligible cell, fed to --run-guard-tests as its first positional; grep the non-variant FAIL lines for got 'allow' or 'rewrite'
- rev: 29be9b66add9b8dac1fae2bf952023bce4be5a7a
- finding: no row is granted 'allow'; rule abs_script's one absolute-script row is 'rewrite' under every suffix but '>|', the finding already in the gap inbox
- inferred: background-cases.tsv was not derived
