# queue-kit

A git-native, agent-readable task tracker: one Markdown file where `##`
sections are queues, bold kebab-case slugs are the task handles, and
square-bracket tags (`[blocked-by:]`, `[needs-spec]`, `[spec:]`) are the state
machine. Six gates hold the grammar a coding agent selects work by, and a
`queue-index.sh` tool renders the compact selection surface.

Why: an agent picks work by *parsing*, not reading — so everything selection
trusts (section position, slugs, tags) must be grammar a gate can enforce, and
everything a human writes freely (task prose) must stay off the parse path.
Drift between what the prose says and what the parser sees is the failure mode;
five of the six gates each close one instance of it — a tag reflowed off its
lead line, a duplicate slug, a lost task, a forward precondition stated in
prose but never tagged. See [SPEC.md](SPEC.md) for the full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   ```
   check-queue-hygiene
   check-queue-wrap
   check-tag-lead-line
   check-task-names
   check-task-conservation
   check-queue-prose-precondition
   ```

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Give your queue file the section skeleton — copy
   `templates/TASK-QUEUE.md` and fill it in (it shows one example entry per
   grammar shape). The default sections are `New Features` / `Technical Debt`
   (active), `Deferred`, `Done`.

3. Optional — reshape the grammar: copy `templates/queue-config.sh` into your
   gates dir and override section names, the wrap budget, prose-lead tokens, or
   the precondition trigger set. Defaults are the extracted platform's.

## Use

```bash
bash queue-kit/bin/queue-index.sh                     # header + active (• ready / ✗ blocked) + deferred
bash queue-kit/bin/queue-index.sh --collapse-deferred # deferred as a per-### tally
bash queue-kit/bin/queue-index.sh --extent <slug>     # inclusive line range of one entry's subtree
```

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks
```
