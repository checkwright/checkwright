# lifecycle-kit

The iteration stage state machine for coding-agent-assisted delivery: a
`## Iteration: <name>` header line in the task queue naming the iteration, an
evidence file of `<iteration> <stage> <session-id> <date> <head>` stamps whose
**last stamp is the current stage** and whose `<head>` binds each stamp to the
commit it was taken at, stage
skills (scope/align/build/validate/close by default — stages are config), and
gates that make skipping a stage, or claiming one without running
its skill, fail the commit.

Why: a stateless agent session doesn't reliably re-read process prose. So the
process state lives in two files a gate can read, and every stage skill stamps
its invocation as its first step (mechanized by
`bash gate-sdk/bin/run-gates.sh --enter-stage <stage>`, so the misformat-prone hand ritual is one
command). That stamp *is* the stage transition — there is no second copy of
the cursor to keep in sync, and stage motion writes no queue at all.
`check-stage-evidence` verifies the stamp file's grammar and that every stamp
belongs to the header's iteration; `check-stage-entry` verifies the
predecessor stamp, the drained queue at validate entry, and the
cross-component audit trigger at build entry. See [SPEC.md](SPEC.md) for the
full contracts.

The linear stage walk is the default; the gate-legal ways to leave it —
abandon, split, reopen — compose existing mechanism with no new tooling
([SPEC.md](SPEC.md) §Deviation transitions).

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-stage-evidence
   check-stage-entry
   check-lesson-disposition
   check-lifecycle-registration
   check-stage-skill-coverage   # skill-shim trio: needs the skills dir (step 3)
   check-skill-binding
   check-shim-restatement
   check-merge-attrs            # multi-operator: needs the .gitattributes step (step 4)
   check-close-surfaces
   check-survey-record          # inert until a survey is filed (--emit file-survey)
   check-scratch-citation       # no permanent surface points a reader into per-iteration scratch
   check-gap-inbox-neutrality   # inert until a gap is filed (--emit file-gap)
   ```
   <!-- gate-roster:end -->

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Give the queue file its header and each evidence file its skeleton — the
   stage-stamp file and the lesson-disposition file
   (`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, both boundary-reset to their header).
   The queue header line:

       ## Iteration: —

   The stage-stamp skeleton:

       # contract: lifecycle-kit/SPEC.md §check-stage-evidence

       ---

   The lesson-disposition skeleton:

       # contract: lifecycle-kit/SPEC.md §check-lesson-disposition

3. Adopt `templates/stages/*.md` in your agent-skill directory (e.g.
   `.claude/commands/`): by default make each skill a binding shim that
   references the template (SPEC.md §templates/stages/) — it tracks the kit, so
   a re-vendor reaches it and the shim gates hold it thin. Or, as the sanctioned
   fork, copy each in and fill its named slots (`*<slot-name: …>*`) with your
   project's ritual — you then own the ritual prose, upgrades don't reach it,
   and the shim gates don't cover it (kept for legitimate structural
   divergence).

4. Point your always-loaded agent file at the machine — run
   `bash gate-sdk/bin/run-gates.sh --install-lifecycle`. It writes a marker-bounded
   registration block (the state machine, the stage roster as skill
   invocations, the SPEC link) into `LIFECYCLE_KIT_AGENT_FILE` (default
   `CLAUDE.md`), the roster derived from your config so a reshape (step 5)
   flows in on a re-run. `check-lifecycle-registration` (step 1) holds the
   block in lockstep. The same run also writes the merge-attribute block into
   `.gitattributes` — a `merge=iteration-scoped` line per per-iteration state
   surface (these resolve to the arriving branch at a merge — SPEC.md
   §Multi-operator semantics) and a `merge=union` line for the committed gap
   inbox (its append-only bullets must survive a concurrent merge — SPEC.md §The
   committed gap inbox) — and registers the keep-ours driver in your clone's git
   config (per-clone, the `install-hooks.sh` opt-in class; the git-native union
   driver needs no such step). `check-merge-attrs` (step 1) holds the block in
   parity with the derived supersede and union sets.

5. Optional — narrow the hand-edit window: register
   `bash gate-sdk/bin/run-gates.sh --hook workflow-state-guard` as a
   `PreToolUse(Write|Edit)` hook (guard-kit's `templates/settings-hooks.json`
   carries the block). It refuses an agent write to the stage-stamp file, whose
   only sanctioned writer is the `--enter-stage` arm — the gates that would catch a
   hand-stamp all fire at commit, and an uncommitted one moves the cursor for a
   whole session (SPEC.md §check-stage-evidence). Requires guard-kit vendored.

6. Optional — reshape the machine: copy `templates/lifecycle-config.sh` into
   your gates dir and override stages, predecessors, drain/audit stages,
   section names, or file paths. Defaults are this repo's own lifecycle.

After install the battery is red at `check-stage-evidence` until your first
`/scope` session runs (it names the iteration and stamps the evidence file as
its first step) — the bootstrap header is a stage like any other, fail-closed
by design.

## Use

```bash
bash gate-sdk/bin/run-gates.sh --enter-stage <stage>          # stamp a stage entry (the transition itself)
bash gate-sdk/bin/run-gates.sh --install-lifecycle    # (re)write the registration and merge-attribute blocks
bash gate-sdk/bin/run-gates.sh --emit file-gap "<gap>"   # route a work-shaped finding to the gap inbox
bash gate-sdk/bin/run-gates.sh --emit file-survey "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"
bash gate-sdk/bin/run-gates.sh --emit cite-survey "<heading-substring>"   # one carried survey, inline-ready
bash gate-sdk/bin/run-gates.sh --emit session-id                       # the canonical stamp id, by the derivation order
```

`--emit session-id` is [SPEC.md](SPEC.md) §bin/session-id.sh's derivation order,
which `--enter-stage` reads for you: reach for it directly only where a session
writes an id itself, as `templates/lead.md`'s session-role marker step does. It
takes no argument and resolves no knob. **The front-end route reads the cwd
`bin/run-gates.sh` cds to** — the git toplevel — so a caller standing elsewhere
whose sessions dir is the cwd-slugged default invokes the binary's
`--emit-session-id` arm directly instead, which is what `--enter-stage` does.

The two survey arms are the capture and citation affordances of
[SPEC.md](SPEC.md) §The survey record, reached through gate-sdk's battery
front-end because a bridged arm's knobs are resolved by the caller that already
sources the kit libraries. `--` ends option processing for either, and a
positional beginning with `-` without it is a refusal — the shape half of
gate-sdk/SPEC.md §The bin/-tool contract, which outlives the port.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```
