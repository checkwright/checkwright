# SPEC amendment: session-id child-flag verification

## What changes

`bin/session-id.sh`'s flagged branch **verifies** `CLAUDE_CODE_CHILD_SESSION`
instead of trusting it. Today (SPEC §bin/session-id.sh, source 3) the flag is
"trusted, never verified": a top-level session carrying it is sent down the
narrowed `<dir>/$CLAUDE_CODE_SESSION_ID/subagents/*.jsonl` scan, finds
nothing, and exits 2 — and the harness now sets the flag in top-level sessions
too (observed across four sessions 2026-07-14, including this scope's own
stage entry), so every stage entry dead-ends and needs the
`LIFECYCLE_KIT_SESSION_ID` escape by hand, whose env-prefix form additionally
breaks the Bash allowlist match and re-prompts each time.

New behavior of the flagged branch (`CLAUDE_CODE_CHILD_SESSION` and
`CLAUDE_CODE_SESSION_ID` both set):

1. Run the narrowed subagents scan as today. **Non-empty scan → unchanged**:
   newest subagent transcript wins (the genuine-child path).
2. **Empty scan with `<dir>/$CLAUDE_CODE_SESSION_ID.jsonl` present** → the
   flag is spurious: a genuine child's transcript lives under `subagents/`
   while it runs, so an empty scan plus a top-level transcript for the env
   uuid means the uuid names a live top-level session — fall back to
   `CLAUDE_CODE_SESSION_ID` (source 2's answer), exit 0.
3. **Empty scan, no such top-level transcript** → exit 2 as today; the help
   text drops the standing spurious-flag escape instruction (that case now
   self-heals via 2) and points at `LIFECYCLE_KIT_SESSIONS_DIR`, since only a
   wrong sessions dir or a genuinely broken layout still reaches it.

Ruled out — mtime-based discrimination (prefer the top-level transcript when
it out-mtimes the subagent hits): the canonical spec already records that a
lead's top-level transcript is concurrently written and can out-mtime the
dispatched child's, so an mtime rule would misidentify genuine children. The
empty-scan test is the only safe discriminator.

Accepted residual races, stated in the merged spec sentence:

- A genuine child stamping before its transcript's first write would fall
  back to the *lead's* uuid. Accepted: by the time a child can execute a
  tool call its transcript has its first writes, so the window is
  theoretical.
- A spurious-flagged session that dispatched subagents earlier in the same
  session has a non-empty narrowed scan and stamps the newest subagent's id.
  Unchanged from today's trusting behavior (no regression, a provenance
  smudge not a correctness break); out of scope.

No new names: no knob, no file, no tag. The change flips a canonical-spec
contract sentence, which is why it takes an amendment rather than landing as
debt.

## Producers and consumers

- **Producer** of the fallback id: the harness, which exports
  `CLAUDE_CODE_SESSION_ID` into every Bash environment (already deployed —
  no enabling config to emit). The spurious flag's producer is the harness
  regression itself; the verification consumes disk state
  (`<dir>/$CLAUDE_CODE_SESSION_ID.jsonl`, the `subagents/` glob) that the
  harness's transcript writer already produces.
- **Consumer**: `bin/enter-stage.sh`, via `session-id.sh` stdout, stamps the
  id into `.workflow/WORKFLOW-STATE.txt`; `check-stage-evidence` reads the
  stamp. Interface unchanged: 8-char normalized id on stdout, exit 0/2 —
  no new fields, so no new readers.

## Existing sections updated

- lifecycle-kit/SPEC.md §bin/session-id.sh: source 2's "Skipped when
  `CLAUDE_CODE_CHILD_SESSION` is set" sentence and source 3's "The flag is
  trusted, never verified: … source 1 is the designed escape" sentence are
  replaced by the verified contract above (narrowed scan → empty-scan
  fallback → exit 2), including the accepted-race sentence.
- `bin/session-id.sh` exit-2 help text for the flagged branch: rewritten per
  point 3 (no standing escape instruction).
- lifecycle-kit/smoke/install.sh session-id block: one new case — flag set,
  empty `subagents/`, top-level `<uuid>.jsonl` present → emits the env
  uuid's 8 chars (the spurious-flag path); the existing narrowed-scan case
  already covers the genuine-child path.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles (the exit-2 help sentence quoted in docs or
      queue prose included).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
