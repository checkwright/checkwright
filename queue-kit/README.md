# queue-kit

A git-native, agent-readable task tracker: one Markdown file where `##` sections are queues, `### <slug>` headings are the tasks — each kebab-case slug the task's handle and link anchor — and square-bracket tags on the line under each heading are the state machine ([SPEC.md](SPEC.md) §The tag algebra owns the tag set). Gates hold the grammar a coding agent selects work by, and the arms under [Use](#use) read and project the file.

Why: an agent picks work by *parsing*, not reading — so everything selection trusts (section position, slugs, tags) must be grammar a gate can enforce, and everything a human writes freely (task prose) must stay off the parse path. Drift between what the prose says and what the parser sees is the failure mode; all but three of the gates each close one instance of it — a tag written off its tag line, a duplicate slug, a live reference left unlinked, a lost task, a forward precondition stated in prose but never tagged. The three exceptions hold a different axis: projection freshness, and the deferred pool's filing contract — its per-entry budget and its tag-line board tags. See [SPEC.md](SPEC.md) for the full contracts.

This file ships in the installer payload, which withholds each kit's `SPEC.md` and its `smoke/`. Every `SPEC.md` link on this page is repointed at the location `GATE_SDK_SPEC_BASE_URL` names when the payload is packed; with no base set the link stays relative (gate-sdk/SPEC.md §Consumer payload).

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-queue-hygiene
   check-queue-sections
   check-queue-wrap
   check-queue-entry-budget
   check-deferred-board-tags
   check-tag-lead-line
   check-task-names
   check-task-conservation
   check-queue-prose-precondition
   check-queue-slug-liveness
   check-roadmap-fresh
   ```
   <!-- gate-roster:end -->

   `check-queue-wrap` is for a queue you hard-wrap; if you keep it unwrapped, one line per paragraph as the template ships, leave it out and register canon-kit's `check-md-unwrapped` instead. They resolve through gate-sdk's registry path (your gates dir first, then each kit's `checks/`), and their `# graph:` manifests put them in the generated pre-commit hook, written by `--emit git-hooks --write` on the gate binary `GATE_SDK_NATIVE_BIN` names.

2. Give your queue file the section skeleton — copy `templates/TASK-QUEUE.md` and fill it in (it shows one example entry per grammar shape). The default sections are `New Features` / `Technical Debt` (active), `Deferred`, `Done` — plus an optional `Icebox` tier between the last two for backlogs whose carry weight has become the problem, off by default (`QUEUE_KIT_ICEBOX_SECTION`).

3. Optional — reshape the grammar: copy `templates/queue-config.knobs` into your gates dir and set section names, the wrap budget, prose-lead tokens, or the precondition trigger set, one `NAME = value` or `NAME[] = element` line each. Defaults are this repo's layout; `--emit knob-roster` on the gate binary `GATE_SDK_NATIVE_BIN` names prints them.

## Use

Run these arms with the gate binary `GATE_SDK_NATIVE_BIN` names, where `init` places it. In PowerShell, spell each line `./scripts/checkwright-gates <arm>` at the repository root, with `.exe` on native Windows:

```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --emit queue-index                       # header + active (• ready / ✗ blocked) + deferred + icebox tally
"$gates" --emit queue-index --collapse-deferred   # deferred as a one-line tally
"$gates" --emit queue-index --extent <slug>       # inclusive line range of one entry's subtree
"$gates" --emit queue-index --icebox-candidates   # the closing stage's eviction worklist
"$gates" --emit queue-counts                       # "<key><TAB><count>", one line per task section, in configured order
"$gates" --emit queue-counts --by <tag>           # the same grammar keyed "<section>/<value>", one line per partition
"$gates" --emit queue-edges                       # every live slug with inbound citations, then retired targets, each marked where its name is still a tracked file's stem
"$gates" --emit queue-edges --inbound <slug>      # one slug's inbound set, each edge with its citing line verbatim
"$gates" --emit entry-history <slug>              # commits where one entry's counted extent fell — live or departed slug; advisory, no verdict
"$gates" --lesson-sink <tag>                      # route a lesson body on stdin to its configured sink
"$gates" --emit roadmap          # the public roadmap block, to stdout
"$gates" --emit roadmap --write   # splice it into the configured projection page
"$gates" --emit queue-migrate --write TASK-QUEUE.md   # once, on upgrade: bullet entries become ### headings with a tag line
```

The roadmap projection is opt-in: it emits nothing until you set the horizon and track vocabularies and a projection page in your config (step 3 above), so an unconfigured consumer gets a clean skip rather than a kit-shaped roadmap.

## Test

Run this arm the same way:

<!-- fence-runnable -->
```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --run-gate-tests queue-kit/gate-tests queue-kit/checks
```
