# queue-kit

A git-native, agent-readable task tracker: one Markdown file where `##`
sections are queues, bold kebab-case slugs are the task handles, and
square-bracket tags (`[blocked-by:]`, `[design-pending]`, `[spec:]`,
`[drain-exempt:]`, `[roadmap:]`, `[precondition-ok:]`, plus the Lessons Learned
channel's `[attend]` and the consumer-named harvest tags) are the state machine.
Gates hold the grammar a coding agent selects work by, and four more arms of the
same binary read the file: `queue-index` renders the compact selection surface,
`queue-counts` tallies each task section for a status readout, `queue-edges`
sums the citations pointing *at* an entry, and `roadmap` projects the entries
curated with `[roadmap:]` onto a generated public page. A fifth, `--lesson-sink`,
routes a harvested lesson body to its configured sink.

Why: an agent picks work by *parsing*, not reading — so everything selection
trusts (section position, slugs, tags) must be grammar a gate can enforce, and
everything a human writes freely (task prose) must stay off the parse path.
Drift between what the prose says and what the parser sees is the failure mode;
all but two of the gates each close one instance of it — a tag reflowed off its
lead line, a duplicate slug, a lost task, a forward precondition stated in
prose but never tagged. The two exceptions hold a different axis: projection
freshness, and the deferred pool's per-entry budget. See [SPEC.md](SPEC.md) for
the full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-queue-hygiene
   check-queue-sections
   check-queue-wrap
   check-queue-entry-budget
   check-tag-lead-line
   check-task-names
   check-task-conservation
   check-queue-prose-precondition
   check-queue-slug-liveness
   check-roadmap-fresh
   ```
   <!-- gate-roster:end -->

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Give your queue file the section skeleton — copy
   `templates/TASK-QUEUE.md` and fill it in (it shows one example entry per
   grammar shape). The default sections are `New Features` / `Technical Debt`
   (active), `Deferred`, `Done` — plus an optional `Icebox` tier between the
   last two for backlogs whose carry weight has become the problem, off by
   default (`QUEUE_KIT_ICEBOX_SECTION`).

3. Optional — reshape the grammar: copy `templates/queue-config.sh` into your
   gates dir and override section names, the wrap budget, prose-lead tokens, or
   the precondition trigger set. Defaults are this repo's layout.

## Use

```bash
bash gate-sdk/bin/run-gates.sh --emit queue-index                       # header + active (• ready / ✗ blocked) + deferred + icebox tally
bash gate-sdk/bin/run-gates.sh --emit queue-index --collapse-deferred   # deferred as a per-### tally
bash gate-sdk/bin/run-gates.sh --emit queue-index --extent <slug>       # inclusive line range of one entry's subtree
bash gate-sdk/bin/run-gates.sh --emit queue-index --icebox-candidates   # the closing stage's eviction worklist
bash gate-sdk/bin/run-gates.sh --emit queue-counts                       # "<section><TAB><count>" per task section, in configured order
bash gate-sdk/bin/run-gates.sh --emit queue-edges                       # every live slug with inbound citations, and the entries citing it
bash gate-sdk/bin/run-gates.sh --emit queue-edges --inbound <slug>      # one slug's inbound set, each edge with its citing line verbatim
bash gate-sdk/bin/run-gates.sh --lesson-sink <tag>                      # route a lesson body on stdin to its configured sink
bash gate-sdk/bin/run-gates.sh --emit roadmap          # the public roadmap block, to stdout
bash gate-sdk/bin/run-gates.sh --emit roadmap --write   # splice it into the configured projection page
```

The roadmap projection is opt-in: it emits nothing until you set the horizon and
track vocabularies and a projection page in your config (step 3 above), so an
unconfigured consumer gets a clean skip rather than a kit-shaped roadmap.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks
```
