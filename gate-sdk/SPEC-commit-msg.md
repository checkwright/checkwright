# SPEC amendment: commit-msg

CLAUDE.md bans local paths, private repo/project names, accounts, and
internal session/commit references "in tracked files or commit messages" —
and only the tracked-files half is even reachable by the current
framework: the generator emits a `pre-commit` hook, which never sees the
message. Enforcement-surface ruling: a **generated `commit-msg` hook**,
not a history scan — the hook rejects the message before the commit
exists, while a scan finds leaked history after push, when the only remedy
is a destructive rewrite (a CI/history backstop stays with the deferred
hosted-attestation rung). Seam ruling: pattern *mechanism* and the
generic patterns ship in the kit; private term lists are rule content and
live in local-only consumer config that is never tracked — tracking the
banned terms would itself be the leak.

## What changes

- **`tier=commit-msg`** — the `# graph:` manifest tier set gains one
  value. `gen-pre-commit.sh` emits a second hook file, `commit-msg`,
  whenever a registered gate declares it, passing the hook's `$1`
  (message file path) through to each such gate; `check-graph` freshness
  covers both emitted hooks, and `install-hooks.sh` already wires the
  whole hooks dir. Couples on a commit-msg gate describe its config
  files (regeneration triggers), not tree paths — the message is not a
  tracked surface.
- **`check-commit-msg`** (gate-sdk/checks, tier=commit-msg) — the message
  file matches no banned pattern. Patterns come from two knobs:
  `GATE_SDK_MSG_PATTERN_FILES` (tracked, must exist — fail-closed) and
  `GATE_SDK_MSG_PATTERN_FILES_LOCAL` (gitignored, skipped when absent —
  a fresh clone without the operator's private list still commits). Each
  file is `grep -E` pattern-per-line, `#` comments allowed. Kit defaults
  (shipped as the tracked template): the `claude.com/claude-code` promo
  URL (the `Co-Authored-By` trailer stays — footer convention, not a
  leak) and absolute home paths (`/home/…`, `/Users/…`). This repo adds
  a gitignored local file carrying the private repo/project/account
  terms.
- **`check-tree-terms`** (gate-sdk/checks, tier=precommit) — the
  tracked-files half: no tracked file matches the same pattern set. The
  pattern files themselves and their templates are self-exempt (they
  contain what they ban). Runs over `git ls-files` output, so untracked
  local config (`BRIEF.local.md`, the local pattern file) is out of scope
  by construction.

## Producers and consumers

- `commit-msg` hook: produced by `gen-pre-commit.sh` from the registry;
  consumed by git at commit time, which feeds the message path to
  `check-commit-msg`. Regeneration is forced the same way as the
  pre-commit hook — `check-graph` reddens on a stale emit.
- Pattern files: produced by the kit template (generic defaults) and the
  consumer (tracked repo-specific + local-only private lists); consumed by
  both gates — one pattern set, two surfaces (message, tree), so the two
  halves of the CLAUDE.md rule cannot drift apart.
- Both gates register in this repo's `scripts/gates.list`; fixture pairs
  drive them with explicit file arguments (a fixture message file / case
  tree), per the fixture-pair contract.

## Existing sections updated

- gate-sdk/SPEC.md §gen-pre-commit — emits both hooks; tier grammar.
- gate-sdk/SPEC.md §check-graph — freshness assertion spans both emitted
  hooks.
- gate-sdk/SPEC.md §install-hooks — no change to wiring (hooks-dir
  granularity), noted as covering the new hook.
- gate-sdk/SPEC.md §Layout and configuration — the two pattern-file knobs.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
