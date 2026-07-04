# lifecycle-kit

The iteration stage state machine for coding-agent-assisted delivery: a
`## Iteration: <name>  [stage: <stage>]` header line in the task queue, an
evidence file of `<iteration> <stage> <session-id> <date>` stamps, five stage
skills (scope/align/build/validate/close by default — stages are config), and
two gates that make skipping a stage, or flipping a header without running
its skill, fail the commit.

Why: a stateless agent session doesn't reliably re-read process prose. So the
process state lives in two files a gate can read, every stage skill stamps its
invocation as its first step, and the *arriving* stage flips the header
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
   ```

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Give the queue file its header and the evidence file its skeleton:

   ```
   ## Iteration: —  [stage: scope]
   ```

   ```
   # contract: lifecycle-kit/SPEC.md §check-stage-evidence

   ---

   ```

3. Copy `templates/skills/*.md` into your agent-skill directory (e.g.
   `.claude/commands/`) and fill the `<…>` placeholders with your project's
   ritual.

4. Optional — reshape the machine: copy `templates/lifecycle-stages.sh` into
   your gates dir and override stages, predecessors, drain/audit stages,
   section names, or file paths. Defaults are the extracted platform's
   five-stage lifecycle.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```
