CONSUMER COPY — paste this block into your CLAUDE.md (as `### Agent execution`)
so the load-bearing safety rules ride in the always-loaded file, never absent
from a session that could delegate. Fill the one shared-file example inline. The
full procedure stays in the skill.

### Agent execution (all stages)

Resident safety rules for every delegated `Agent` (an audit, a build sweep, a
one-off investigation). **Full protocol: `/agent-execution`** (resume-journal
mechanics, validate-after-commit command set, gate-driven worklist).

- **Supervisor owns SECURITY/design rulings; agents surface, never guess.**
- **Background + notification, never poll** (`run_in_background`; don't read the
  output file).
- **Serialize on shared files** (your generated-config script, a shared test
  fixture, an amendment under edit) — **and the git index/HEAD are shared for
  every committing agent** (serialize *or* `isolation: worktree`); **≤2-wide
  otherwise**, read-only fan-outs only.
- **One commit per unit; split** if >4 components, OR mixed
  mechanical+architectural, OR >300 tool calls.
- **Resume journal in the session dir** (agent writes findings inline + a `DONE`
  marker; supervisor deletes it post-commit); **validate after every agent
  commit** — a sub-agent's "passed" is not trustworthy, and an agent blocked by a
  gate weakens it rather than fix the code (diff every gate change;
  `check-gate-tamper` is the mechanical floor).
- **Budget-check before *each* dispatch** (`bash delegation-kit/bin/usage-gate.sh`
  — it computes reading-age + window-validity and prints one verdict, exit 0 OK /
  1 PAUSE / 2 STALE; never eyeball the raw pct, a dead-window value reads
  stale-high. Pause if PAUSE; the 5h window is the only pause axis).
- **Never revert substantial completed work on your own design judgment** —
  especially an expensive delegated sweep. Surface the tension and wait for
  explicit go-ahead before discarding it.
