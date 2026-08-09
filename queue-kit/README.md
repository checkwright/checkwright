# queue-kit

A git-native, agent-readable task tracker: one Markdown file where `##`
sections are queues, bold kebab-case slugs are the task handles, and
square-bracket tags (`[blocked-by:]`, `[design-pending]`, `[spec:]`,
`[drain-exempt:]`, `[roadmap:]`, `[precondition-ok:]`, plus the Lessons Learned
channel's `[attend]` and the consumer-named harvest tags) are the state machine.
Gates hold the grammar a coding agent selects work by, a
`queue-index.sh` tool renders the compact selection surface, a `queue-counts.sh`
tool tallies each task section for a status readout, a `queue-edges.sh`
tool sums the citations pointing *at* an entry, and a `roadmap.sh`
tool projects the entries curated with `[roadmap:]` onto a generated public page.

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
bash queue-kit/bin/queue-index.sh                       # header + active (• ready / ✗ blocked) + deferred + icebox tally
bash queue-kit/bin/queue-index.sh --collapse-deferred   # deferred as a per-### tally
bash queue-kit/bin/queue-index.sh --extent <slug>       # inclusive line range of one entry's subtree
bash queue-kit/bin/queue-index.sh --icebox-candidates   # the closing stage's eviction worklist
bash queue-kit/bin/queue-counts.sh                      # "<section><TAB><count>" per task section, in configured order
bash queue-kit/bin/queue-edges.sh                       # every live slug with inbound citations, and the entries citing it
bash queue-kit/bin/queue-edges.sh --inbound <slug>      # one slug's inbound set, each edge with its citing line verbatim
bash queue-kit/bin/lesson-sink.sh <tag>                 # route a harvested lesson body to its configured sink
bash queue-kit/bin/roadmap.sh --emit                    # the public roadmap block, to stdout
bash queue-kit/bin/roadmap.sh --write                   # splice it into the configured projection page
```

The roadmap projection is opt-in: it emits nothing until you set the horizon and
track vocabularies and a projection page in your config (step 3 above), so an
unconfigured consumer gets a clean skip rather than a kit-shaped roadmap.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks
```
