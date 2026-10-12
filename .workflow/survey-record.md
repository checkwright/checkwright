# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-12 scope — Which open guard finding moved its verdict since the published v0.31.0, the release bar's question?
- corpus: native/src/guard guard-kit/guard-tests
- oracle: ./native/target/release/checkwright-gates --scratch-run .tmp/scope-compare.sh .tmp/scope-probe-cases.tsv
- rev: e4c1714fed3b488e741ec33457ade3bd33b5e388
- finding: Two rows moved. 'until grep -q DONE log; do sleep 1; done 2>/dev/null &' foreground: fall-through then, block now. 'until [ -f marker ]; do sleep 5; done|cat' foreground: fall-through then, allow now; its spaced spelling is allow at both. Every git stash spelling, the for-variable and '>& done' rows, 'sleep 5 >& done' and the led wait fell through at v0.31.0 and fall through or block now, none granting.
- inferred: that this repository's knob file would not change a row: the v0.31.0 binary refuses it on a knob it does not know, so both binaries ran from one zero-config scratch git repository; the live-producer rows of git-rules-word-read-siblings were not run. The oracle needs the v0.31.0 x86_64-unknown-linux-musl Release binary unpacked under .tmp/v031 and made executable.

## 2026-10-12 scope — Which deferred entries lead this iteration's unit set?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: e4c1714fed3b488e741ec33457ade3bd33b5e388
- finding: One non-event row: heterogeneous-agent-delegation, iteration/low, held at scope 2026-10-09 by operator direction. No deferred entry is at the recurrence threshold. The guard-kit surface carries nine rows, the most of any; none has a live inbound edge, and their retired inbound targets (git-stash-quoted-verb, loop-keyword-as-argument, wait-loop-lead-and-condition) are landed fixes the new entries report defects in. The release that tarball-attestation-observed, front-door-job-observed and companion-toolkit-profile wait on is held by the guard findings.
- inferred: that a re-spelling oracle would have caught each of the two closes' review findings: read off their recorded spellings, no such lane exists to run
