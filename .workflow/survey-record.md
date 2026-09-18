# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-18 scope — Which programs does the native gate binary spawn through proc::run, and where does it still shell out to bash?
- corpus: native/src/
- oracle: grep -rno -E 'proc::run\("[a-z0-9_.-]+"' native/src/ | grep -v test
- rev: 747f4fb772aa2228754fa03c8b22858aef3593be
- finding: by program: git 107, bash 7, date 7, mktemp 5, cp 2, jq 2, ps 1, uname 1. The seven bash sites: emit/enter_stage.rs:1633 (capture_group_one), knobs/lifecycle_kit.rs:143 (ERE validation), evidence.rs:322 and emit/wait_probe.rs:500 (kill -0), emit/wait_probe.rs:202, emit/port_blockers.rs:291, emit/pub_index.rs:53. toolfloor.rs PROBE_SET publishes bash:4.3 git jq awk::GNU sort::coreutils shellcheck cargo; awk, sort, shellcheck, cargo have no proc::run site while date, mktemp, cp, ps, uname are spawned and unrostered.
- inferred: only proc::run call sites are counted; other proc.rs spawn helpers (spawn_target callers) and gate-dispatched scripts are not enumerated
