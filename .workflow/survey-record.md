# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-18 scope — Which programs does the native gate binary spawn through proc::run, and where does it still shell out to bash?
- corpus: native/src/
- oracle: grep -rno -E 'proc::run\("[a-z0-9_.-]+"' native/src/ | grep -v test
- rev: 747f4fb772aa2228754fa03c8b22858aef3593be
- finding: by program: git 107, bash 7, date 7, mktemp 5, cp 2, jq 2, ps 1, uname 1. The seven bash sites: emit/enter_stage.rs:1633 (capture_group_one), knobs/lifecycle_kit.rs:143 (ERE validation), evidence.rs:322 and emit/wait_probe.rs:500 (kill -0), emit/wait_probe.rs:202, emit/port_blockers.rs:291, emit/pub_index.rs:53. toolfloor.rs PROBE_SET publishes bash:4.3 git jq awk::GNU sort::coreutils shellcheck cargo; awk, sort, shellcheck, cargo have no proc::run site while date, mktemp, cp, ps, uname are spawned and unrostered.
- inferred: only proc::run call sites are counted; other proc.rs spawn helpers (spawn_target callers) and gate-dispatched scripts are not enumerated

## 2026-09-18 spec — Which programs does the native gate binary spawn in production, across every proc spawn face, and how does that set relate to PROBE_SET and GATE_SDK_PROGRAM_FLOOR?
- corpus: native/src/
- oracle: grep -rhoE 'proc::[a-z_]+\(\s*"[A-Za-z0-9_.-]+"' native/src | sort | uniq -c
- rev: 15273486c7ccae205aee5f04179123a3355a972c
- finding: Supersedes scope's same-day proc::run-only census, whose 'roster and spawn set disagree both ways' was an oracle artifact. Literal spawns by program: git, bash (about 20 sites across run/run_streamed/run_merged_in/run_to/run_with_env_in/run_stdout_in/run_bounded_capture, seven of them proc::run), date 7, mktemp 6, jq 2, cp 2, tar 1, npm 1, sh 1, uname 1, ps 1, plus sort through resolve_floor_tool. Every adopter PROBE_SET member is spawned or run by a shipped surface: sort (toolfloor floor_met), shellcheck (check-shellcheck), awk (installer/bin/checkwright.sh and templates, all POSIX awk), jq, git, bash. Spawned but unrostered: date, mktemp, cp (on GATE_SDK_PROGRAM_FLOOR, assumed) and uname, ps, tar (on neither set).
- inferred: programs passed through a variable (shellcheck via check-shellcheck, cargo/rustc via check-crate-arms, knob command words) are not reached by a literal grep and were read from the registry needs elements instead; test-only spawns (ere.rs awk oracle) are counted by the grep and excluded by reading
