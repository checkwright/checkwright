# SPEC amendment: liquid-parse

GitHub Pages builds the docs site with Jekyll, and Jekyll runs Liquid over every page before kramdown sees it. A Liquid syntax error fails the whole build, and the host keeps serving the last good tree. Two pushes failed that way. The generated gate-sdk mirror carried an unbalanced `${{`. The site stayed frozen and the hosted install scripts were 404, and nothing in the iteration saw it. The emitter half is already fixed: the docs-mirror arm wraps each mirrored body in a raw block (canon-kit/SPEC.md §The reference-link grammar). This amendment adds the detection half, in two parts: a gate that parses every Liquid-processed docs file through the Liquid parser Pages runs, and a push watch that reads the push's deployment run.

**What was run at authoring** (2026-09-24, at `8488e867`):

- `curl -s https://pages.github.com/versions.json`: Pages pins `jekyll` 3.10.0, `liquid` 4.0.4, `kramdown` 2.4.0 and `jekyll-optional-front-matter` 0.3.2.
- The default parser below (liquid 4.0.4 under ruby 3.3, `error_mode: :warn`, `line_numbers: true`), fed every tracked `docs/**/*.md` with its front matter stripped: 76 documents, 0 failures.
- The same parser on the three tracked Liquid templates, `docs/_layouts/default.html`, `docs/_includes/nav.html` and `docs/search.json`: all three parse.
- The same parser on `git show 1fb12fa3^:docs/gate-sdk/SPEC.md`, the pre-hotfix mirror: `Liquid syntax error (line 3202): Variable '{{…' was not properly terminated`. So the oracle reds on the attested defect. The message's variable token runs for kilobytes, which is why delta 1 clips it.
- `git grep -o "{%-\? *[a-z_]*"` over the docs pages and templates: the tags in use are `raw`, `assign`, `for`, `if`, `unless`, `else` and `include`, all Liquid built-ins. No page uses a tag only Jekyll or a plugin registers.

## What changes

### (1) check-docs-liquid-parse and its parser knob {design-bearing}

**Not yet applied.** A new site-kit gate, `checks/check-docs-liquid-parse.gate`: `precommit`, binary-dispatched, `# install: on-surface`, its rule in `native/src/gates/docs_liquid_parse.rs`. It is born native.

**Invariant.** Every tracked file under `SITE_KIT_DOCS_DIR` that the Pages build runs Liquid over parses under the parser `SITE_KIT_LIQUID_PARSER` names.

**The corpus.** It is enumerated with `git ls-files` under the docs dir, with the gate-sdk prune set applied, and holds three classes:

- every `*.md` file outside an underscore-prefixed directory segment, which is §check-docs-render-fidelity's page set (Pages' optional-front-matter plugin renders a front-matter-less markdown page too);
- any other tracked file outside such a segment whose first line is the front-matter fence `---`, because Jekyll renders a file with front matter whatever its extension (`search.json` here);
- every tracked file under the docs dir's `_layouts/`, `_includes/` and `_posts/`, the three Jekyll directories whose files are Liquid templates or pages. Every other underscore directory stays excluded, as render-fidelity excludes it.

**Front matter** is not handed to the parser, since Jekyll reads it as YAML and never as Liquid. The gate replaces each line of the block, from the opening fence through the closing one, with an empty line. So every line number the parser reports is a line of the file. A `NUL` in the source is dropped before framing, on §check-docs-render-fidelity's rule.

**The parser contract.** `SITE_KIT_LIQUID_PARSER` is an array knob naming one command, which gets the whole corpus over one stream:

- It reads `NUL`-terminated documents from stdin.
- For each document, in order, it writes one `NUL`-terminated **verdict** to stdout: empty when the document parses, otherwise the parser's error message.

This is the renderer batch framing (§check-docs-render-fidelity, the batch stream), with a verdict in place of a rendered document. There is no per-document form. The gate reads only a verdict, so a second contract would buy nothing but a second program to probe.

**The default** is a `ruby -e` loop. It requires `liquid`, reads stdin whole in binary, and splits on the terminator. It calls `Liquid::Template.parse` on each document as UTF-8 with `error_mode: :warn` and `line_numbers: true`, which are Jekyll 3.10's own settings. On a `Liquid::Error` it writes the message with any `NUL` replaced by a space. Any other exception propagates, so the command dies and the count check below refuses.

**The probe.** Before enumerating, the gate runs the parser on two documents: `{{ probe }}` and the unterminated `{{ probe`. It must get back exactly two verdicts, the first empty and the second non-empty. Anything else exits 2 with a help line naming the knob and its contract. The probe exercises detection as well as framing, so a command that parses nothing, such as `cat` or a stub answering every document with an empty verdict, is refused rather than read as a clean corpus.

**Refusal order,** each refusal before any finding and winning over the ones after it:

1. not a git repository;
2. docs dir not found;
3. the probe;
4. the enumeration;
5. the count check. After the stream, verdicts read must equal documents written. Otherwise the gate exits 2, since a short count is a parser that died, truncated its output or framed it wrongly.

**Red** (exit 1) is one finding per file with a non-empty verdict: `<path>: <verdict>`. The verdict is cut to its first line, and that line to 200 characters with a trailing `…`. The help line names the remedies. Wrap Liquid-significant text a page means literally in a `{% raw %}` … `{% endraw %}` block, or a generated page's emitter does it. Or remove the token. **Clean** reports the files parsed, by class. An empty corpus is clean on a zero count.

**`--needs`** declares `git` and `?<TAB>SITE_KIT_LIQUID_PARSER`, so `--emit port-blockers` resolves the requirement from the knob. That is §check-docs-render-fidelity's knob-derived shape, with one knob.

**The `# graph:` couples** are render-fidelity's, plus the docs dir's `_layouts/`, `_includes/` and `_posts/` trees, `dir=one`, `valve=none`. A front-matter file in another place (`search.json`) is outside the hook's trigger, and the full battery and CI still read it.

**No valve.** A page whose Liquid does not parse fails the Pages build, so an exempted finding would still freeze the site.

**Honest limits,** stated in the section:

- **A parse is not a render.** A balanced `{{ … }}` or `{% … %}` parses and renders as something else. An unknown variable renders blank. So a page that means such a token literally loses it silently, and this gate does not see it. Delta 5 files this gap. (Grounds for the amendment only, not for the section: the gate-sdk mirror's balanced `${{ … }}` spellings rendered blank for weeks before the unbalanced one broke the build.)
- **Unknown tags.** The default parser is bare Liquid. A tag that only Jekyll or a plugin registers (`highlight`, `link`, `post_url`, `seo`) raises `Unknown tag` under it, so a site that uses one points the knob at a command that loads Jekyll and those plugins first.
- **Version fidelity.** The default parses with whichever `liquid` the host resolves. §check-docs-render-fidelity's exact-pin recipe applies unchanged: a `bundle exec` form whose lock pins `liquid` to the version the `github-pages` gem resolves. The kit does not fetch the pin at gate time, for the hermetic-oracle reason that section gives.
- **Over-reading.** A file that `_config.yml` excludes, or a post Jekyll would not render, is still read. A finding there is a false red, fixed by the same raw block.
- **Other markdown extensions.** A front-matter-less page with a markdown extension other than `.md` is not read.

In site-kit/SPEC.md: a new `## check-docs-liquid-parse` section after §check-docs-render-fidelity, stating the invariant, corpus, contract, probe, refusals and limits above undated, with no incident history (gate-sdk/SPEC.md §The provenance seam). §Knob defaults gains the row for `SITE_KIT_LIQUID_PARSER`, carrying the same port-blocker sentence the renderer rows carry. §Layout and configuration's registration sentence gains: `check-docs-liquid-parse` registers where a docs site is built by Jekyll, and a consumer whose host runs no Liquid omits it. The opening paragraph's tree-side truths gain "whether each page parses under the platform's template engine".

### (2) Its fixture pair, bespoke test and crate tests {mechanical}

**Not yet applied.**

- **`good/`** is a docs tree carrying:
  - a page with a raw-wrapped `${{ matrix.runner }}` inside inline code;
  - a page using `{% assign %}` and `{% for %}`;
  - a front-matter page with its fence;
  - `_layouts/default.html` with `{% if %}` … `{% endif %}`;
  - a `search.json` with front matter and `{%- for -%}`;
  - an `.html` file without front matter carrying a bare `{{`, which is outside the corpus.

  It reds nothing, and its clean line counts all three classes.
- **`bad/`** is a page with an unterminated `{{` in inline code, and a layout with an unclosed `{% if %}`. Each is one finding.
- **`site-kit/gate-tests/check-docs-liquid-parse-parser.test.sh`** holds the arms the pair cannot, each invoked through `gate_run` with its parser set through `SITE_KIT_KNOB_FILE`:
  - a parser that answers every document with an empty verdict fails the probe (exit 2);
  - a stub that passes the probe and then returns a short count exits 2;
  - an unresolvable command exits 2;
  - a front-matter page with an error on a line after its fence reports the file's own line number.
- **Crate unit tests** cover the three corpus classes, the front-matter blanking and the verdict clipping.

### (3) Registration and the dependency's surfaces {mechanical}

**Not yet applied.**

- **`scripts/gates.list`** registers the gate after `check-docs-render-fidelity`.
- **`native/src/knobs/site_kit.rs`** adds the knob with the delta 1 default. **`native/src/gates/mod.rs`** gains the `REGISTRY` row, with its knobs `SITE_KIT_DOCS_DIR`, `SITE_KIT_LIQUID_PARSER` and the two prune knobs, and its needs.
- **`site-kit/README.md`:** the gate roster, a prose line beside render-fidelity's, and the dependency sentence: the gate needs ruby with the `liquid` gem.
- **`site-kit/smoke/install.sh`** registers it beside `check-docs-render-fidelity`, which has the same disposition over the same surface: the smoke writes `docs/`, which is what the registration accounting's predicate asks (gate-sdk/SPEC.md §Consumer smoke).
- **`.github/workflows/gates.yml`:** the render-fidelity deps step also installs `liquid:4.0.4`, the Pages pin, and its comment names both gates.
- **installer/SPEC.md §The docs-site tier and `docs/install.md`'s Requirements prose:** the tier needs Ruby with the `kramdown-parser-gfm` gem, and with the `liquid` gem where the consumer registers the Liquid gate.
- **`docs/site-architecture.md` §Which parser serves which file:** Jekyll runs Liquid over a `docs/` page before kramdown renders it, and the Liquid half is site-kit/SPEC.md §check-docs-liquid-parse's. That page's roster pointer names both site-kit sections.

### (4) The push watch reads the deployment run {mechanical}

**Not yet applied.** Which runs a push is watched to is this consumer's content, carried by the close binding's `push-budget` slot (lifecycle-kit/SPEC.md §The state machine). Here a master push triggers two runs: the `gates` workflow and `pages-build-deployment`. So:

- **`.claude/commands/close.md` `push-budget`:** "Each push is watched to green on the `gates` workflow (`gh run watch`)" becomes "Each push is watched to green on every run it triggers: the `gates` workflow and `pages-build-deployment`. `gh run list --commit <sha>` names both, and `gh run watch <id>` waits on each. A red deployment is a red push."
- **CLAUDE.md's push sentence:** "watch the `gates` workflow to green (`gh run watch`) before calling the push done" becomes "watch every run it triggers to green before calling the push done", so the run set has one home, the binding.

No kit gains a knob or a template slot. The close template's `push-budget` slot already names "the remote run a push is watched to". The deployment run is a monitor-side object (site-kit/SPEC.md §The monitor boundary), watched by the pushing session and never gated.

### (5) The balanced-token gap is filed {mechanical}

**Applied at spec,** in the commit that added this file, through `--emit file-gap`. A balanced Liquid token that a docs page means literally renders as something else, usually blank, and neither delta 1 nor render-fidelity sees it.

## Producers and consumers

- **`check-docs-liquid-parse`.**
  - Producer: its descriptor and `REGISTRY` row. Enabling config: `scripts/gates.list` registers it, so the battery, the generated hook and CI's `gates` job run it.
  - Consumers:
    - the committing session, through the output contract;
    - the fixture runner, through the pair and the bespoke test;
    - `--emit port-blockers`, through `--needs`;
    - `--emit git-hooks` and `--emit graph`, through `# graph:`.
  - Roster-holding readers:
    - site-kit/README.md's gate roster (`check-readme-roster`);
    - `check-install-disposition` assertion A, satisfied by `# install: on-surface`. Assertion B binds `zero-config` members only, so the smoke slot is decided by the registration accounting (gate-sdk/SPEC.md §Consumer smoke) at validate;
    - `check-comment-tier`, where the descriptor's `# spec:` line binds to the new section;
    - the generated projections below.
- **`SITE_KIT_LIQUID_PARSER`.**
  - Producer: site-kit's static knob table. Its default is filled in process, and a consumer overrides it in `site-config.knobs`.
  - Consumers: the gate, for the probe and the stream, and the port arm, for the requirement.
  - Roster readers: `check-knob-citation`, satisfied by the §Knob defaults row; `--emit knob-roster`.
- **The verdict stream.** It has one field per document, the verdict. The gate reads it at the count check and at the finding.
- **The push-budget run set.** Producer: the binding. Consumer: the pushing session at close and any session that pushes mid-iteration. CLAUDE.md now points at the binding instead of spelling the set.
- **Point 5.** No corpus narrows.
- **Point 6.** The gate obliges every member of its corpus to parse. The members were enumerated by the runs above: 76 markdown pages and the three templates, each parsing at `8488e867` (empty verdict). No member is narrowed past.

## Existing sections updated

Roster from reading site-kit/SPEC.md whole; `git grep -l "check-docs-highlight-coverage"`, the latest site-kit gate's landing set; `git grep -n -i kramdown -- ':!docs/*-kit/*' ':!docs/gate-sdk/*'`; and `grep -n "push-budget\|gh run watch" .claude/commands/close.md CLAUDE.md`, run 2026-09-24.

- site-kit/SPEC.md: the opening paragraph, §Layout and configuration, §Knob defaults and the new §check-docs-liquid-parse (delta 1).
- `site-kit/checks/check-docs-liquid-parse.gate`, `native/src/gates/docs_liquid_parse.rs` (delta 1).
- `site-kit/gate-tests/check-docs-liquid-parse/`, `site-kit/gate-tests/check-docs-liquid-parse-parser.test.sh` (delta 2).
- `scripts/gates.list`, `native/src/knobs/site_kit.rs`, `native/src/gates/mod.rs`, `site-kit/README.md`, `site-kit/smoke/install.sh`, `.github/workflows/gates.yml`, installer/SPEC.md §The docs-site tier, `docs/install.md`, `docs/site-architecture.md` (delta 3).
- `.claude/commands/close.md`, `CLAUDE.md` (delta 4).
- `.workflow/gap-inbox.md` (delta 5), written at spec.
- `.workflow/release-declarations.md`, a Tightened gates bullet (delta 1): `check-docs-liquid-parse` is new in site-kit, `on-surface`, with the knob `SITE_KIT_LIQUID_PARSER` defaulting to a ruby Liquid parse. It needs ruby with the `liquid` gem where registered. Every Liquid-processed docs file must parse, so a page carrying an unbalanced `{{` or an unclosed tag reds.
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/site-kit/SPEC.md`, `docs/site-kit/README.md`, `docs/installer/SPEC.md`, `docs/enforcement.md`, `docs/check-graph.html`, `docs/value.md`, `scripts/git-hooks/pre-commit`, and `.workflow/surface-ceiling.txt` if its ceiling reds.

## Retired spellings

- None — no name is renamed or removed. The gate and the knob are new, and the push-budget and CLAUDE.md edits change prose, not governed names.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, the knob and the run set.
- [ ] **Instruction surfaces: instruction only.** The push-budget and CLAUDE.md sentences carry no grounds.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and the new section reads whole without this file.
- [ ] **The oracle reds the attested defect.** Build runs the landed gate against a scratch tree holding `1fb12fa3^`'s `docs/gate-sdk/SPEC.md` and sees the line-3202 finding, and against the live tree and sees it clean.
- [ ] **The deployment is watched.** The push that lands this unit is watched on both runs, per delta 4.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `pages-liquid-break-undetected` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above.
- [x] **Gaps filed at spec.** Delta 5.
- [ ] **Gaps filed at build.** Any cross-component gap the work finds is resolved that session.
