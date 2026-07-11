# lifecycle-kit

The iteration stage state machine for coding-agent-assisted delivery: a
`## Iteration: <name>  [stage: <stage>]` header line in the task queue, an
evidence file of `<iteration> <stage> <session-id> <date>` stamps, stage
skills (scope/align/build/validate/close by default — stages are config), and
gates that make skipping a stage, or flipping a header without running
its skill, fail the commit.

Why: a stateless agent session doesn't reliably re-read process prose. So the
process state lives in two files a gate can read, every stage skill stamps its
invocation as its first step (the flip+stamp mechanized by
`bin/enter-stage.sh <stage>`, so the misformat-prone hand ritual is one
command), and the *arriving* stage flips the header
atomically with its stamp — `check-stage-evidence` verifies the current
stage's stamp and the stamp file's grammar, `check-stage-entry` verifies the
predecessor stamp, the drained queue at validate entry, and the
cross-component audit trigger at build entry. See [SPEC.md](SPEC.md) for the
full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   ```
   check-stage-evidence
   check-stage-entry
   check-lesson-disposition
   ```

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Give the queue file its header and each evidence file its skeleton — the
   stage-stamp file and the lesson-disposition file
   (`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, both boundary-reset to their header):

   ```
   ## Iteration: —  [stage: scope]
   ```

   ```
   # contract: lifecycle-kit/SPEC.md §check-stage-evidence

   ---

   ```

   ```
   # contract: lifecycle-kit/SPEC.md §check-lesson-disposition
   ```

3. Adopt `templates/skills/*.md` in your agent-skill directory (e.g.
   `.claude/commands/`): either copy each in and fill its named slots
   (`*<slot-name: …>*`) with your project's ritual, or — to track the kit —
   make each skill a binding shim that references the template (SPEC.md
   §templates/skills/).

4. Optional — reshape the machine: copy `templates/lifecycle-config.sh` into
   your gates dir and override stages, predecessors, drain/audit stages,
   section names, or file paths. Defaults are this repo's own lifecycle.

After install the battery is red at `check-stage-evidence` until your first
`/scope` session runs (it names the iteration and stamps the evidence file as
its first step) — the bootstrap header is a stage like any other, fail-closed
by design.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```
