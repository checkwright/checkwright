# SPEC amendment: selection-probe

Pairs `agent-selection-probe`. Whether a coding agent asked cold for a kind of tool names a given tool is a measurement no surface here takes. The publisher's own tree cannot answer it, because an agent's answer moves on its own index and training clock, not on the tree's. The entry wants a repeatable cold probe: the prompt set, the agents asked, the record's home and form, and the cadence. This amendment ships the probe as a drift-kit arm, `--emit selection-probe`, off until a consumer binds its agents, prompts and subject. It records one verdict per agent and prompt in a private trend record beside the install-observation record, and keeps each answer beside it. Which agents, which prompts and which names count are each consumer's own vocabulary, so all three are knobs, and this repository binds its own in `scripts/drift-config.knobs`.

**What was read at spec.**

- Three agent CLIs are on this machine's `PATH` (`claude`, `codex` and `agy`), and none carries a user-level instruction file (`~/.claude/CLAUDE.md`, `~/.codex/AGENTS.md` and `~/.gemini/GEMINI.md` are absent). `agy -p` and `codex exec` are their non-interactive forms. `claude --help` lists `--bare`, "Minimal mode: skip hooks …", and `--no-session-persistence`. `codex exec --help` lists `--skip-git-repo-check`, `-s` and `-c`.
- An agent started inside a repository reads that repository, so a cold run needs a directory outside every work tree. The scratch dir `DRIFT_KIT_TMP_DIR` names is inside the consumer's own tree by default.
- `DRIFT_KIT_PRICE_PAGE_CMD` already spawns a consumer command with a bound and a kill of its process tree (`proc::run_bounded_capture`), and §The install-observation record already sets the private metric-dir record's shape.

**Inferred, cannot run before build:** that each bound argv answers one prompt non-interactively and exits — the binding is delta 4's and lands at build, and running it spends credit on each agent's account, so the build checks each argv against its `--help` and the operator's first run settles it.

**Shapes refused.**

- **A capture arm the operator fills by hand**, the install record's shape. Here the observation is a command's output, and a hand transcription is the step a repeatable probe removes.
- **A rank or a competitor list parsed from the answer.** Answers are free prose, and a parser for "which tools were named" would be a vocabulary of every competitor, consumer content no kit may carry. The verdict is whether a subject name appears. The answer file keeps the rest for the human reader.
- **A battery, hook or stage step that runs it.** Each run spends credit on every bound agent's account. So its caller is an operator, and nothing else starts it.

## What changes

### (1) The agent-selection probe's contract {design-bearing} {user-facing: agent-selection-probe's Deliverable, "the prompt set, the agents asked, the record's home and form, and the cadence", for the arm and knob names an adopter's binary carries}

`drift-kit/SPEC.md` gains a section after §The install-evidence projection. **Not yet applied.**

> ## The agent-selection probe
>
> Whether an agent asked cold for a kind of tool names yours is outside any tree's reach. Its answer moves on the agent's own index and training, so the measurement is taken from outside and kept as a trend.
>
> `bash gate-sdk/bin/run-gates.sh --emit selection-probe [<agent>…]` asks each bound agent, or each one named, every bound prompt.
>
> - **Where each call runs.** Each call runs in a fresh empty directory under the host's temporary directory, which the arm creates and removes. Before the first call the arm asks git whether that directory is inside a work tree, and refuses at exit 2 where it is, since an agent there would read a repository.
> - **How it is spawned.** The agent's argv is spawned with no shell and the prompt appended as its last argument. Its stdin is empty, and the child inherits the arm's environment. The call is bounded by `DRIFT_KIT_SELECTION_TIMEOUT` seconds, and the bound kills the call's process tree.
> - **What it keeps.** Stdout is the answer, written whole to `<answers-dir>/<date>-<agent>-<prompt>.txt`.
> - **The verdict.** It is `named` where any `DRIFT_KIT_SELECTION_SUBJECT` name occurs in the answer, matched case-insensitively at word boundaries, and `unnamed` where none does. A non-zero exit is `failed` and a killed call `timed-out`, and neither reads the answer.
>
> The record is one line per call:
>
> ```
> <date> <agent> <prompt> <verdict>
> ```
>
> `<verdict>` is closed: `named` | `unnamed` | `failed` | `timed-out`. `<agent>` and `<prompt>` are the keys the consumer bound, each shaped `[a-z0-9-]+`. A line **replaces on `<date> <agent> <prompt>`**, in place, as §The install-observation record's replace does, so a same-day re-run corrects and a later day appends. Across days the record is the trend, and a rate over it is a reader's computation, never stored. The arm prints each line as it lands, then `selection-probe: <n> named of <m> asked`, and exits 0 once it has asked. A failed or timed-out call is a verdict, not an arm failure.
>
> **Off is the default.** With no agent bound, the arm prints `selection-probe: off` and exits 0. It refuses at exit 2, writing nothing:
>
> - where agents are bound and no prompt or no subject is;
> - where a named agent is not bound;
> - where a key breaks its shape;
> - where a temporary directory lies inside a work tree.
>
> **Named caller and transition:** an operator, by hand, never a battery, hook or stage, because each run spends on every bound agent's account. The record's reader is the operator weighing a change made to be found, before it and after it lands. The answers' reader is the same operator, asking what was named instead.
>
> **Honest limits.** "Cold" is what the consumer's argv makes it: the arm controls the directory and stdin, and an agent's own user-level configuration, memory or instruction file reaches the call unless its argv turns it off. A subject name occurring in a refusal or a negative ("I don't know checkwright") reads `named`. An agent reading the web answers on its search index's clock, and one that does not answers on its training's, so two agents' verdicts are not one quantity.

### (2) The knobs {design-bearing} {user-facing: as delta 1}

`drift-kit/SPEC.md` §Layout and configuration's knob list gains, after `DRIFT_KIT_AUTHOR_INSTALL_RECORD`. **Not yet applied.**

> - `DRIFT_KIT_SELECTION_AGENTS` — keyed: each key an agent's name, each value its argv split on blanks, so no word of it carries a blank. Default empty, the probe off (§The agent-selection probe).
> - `DRIFT_KIT_SELECTION_PROMPTS` — keyed: each key a prompt's name, each value its text. Default empty.
> - `DRIFT_KIT_SELECTION_SUBJECT` — indexed: the names whose occurrence makes an answer `named`. Default empty.
> - `DRIFT_KIT_SELECTION_TIMEOUT` — positive integer seconds bounding one call, default `300`.
> - `DRIFT_KIT_SELECTION_RECORD` — the probe's record; default `.metric/agent-selection.log`, derived as `${DRIFT_KIT_METRIC_DIR}/agent-selection.log`. The arm is its one writer.
> - `DRIFT_KIT_SELECTION_ANSWERS_DIR` — where each answer is kept; default `.metric/agent-selection`, derived under `DRIFT_KIT_METRIC_DIR`.

The **metric-dir member** sentence in that section names the two new defaults beside the install records. The table validator's integer rule names `DRIFT_KIT_SELECTION_TIMEOUT` beside `DRIFT_KIT_PRICE_PAGE_TIMEOUT`.

### (3) The arm, its knobs and its tests {design-bearing}

All **Not yet applied.**

- **`native/src/emit/selection_probe.rs`** carries delta 1's arm:
  - the record's in-place replace reuses the install record's writer where its key rule is a parameter, and never forks it;
  - the bounded spawn is `proc::run_bounded_capture` run in the cold directory, and `proc.rs` gains a working-directory parameter on that one function if it has none;
  - the coldness test is `git rev-parse --is-inside-work-tree` through the crosser, refusing on a `true` answer;
  - its unit cases hold the subject match (case, word boundaries, a name inside a longer word not matching), the key shape and each refusal.
- **`native/src/emit/mod.rs`** registers `--emit-selection-probe` in `ARMS` with `Grammar::Positional`, or the grammar the arm's `[<agent>…]` takes, declaring delta 2's six knobs. `native/src/knobs/drift_kit.rs` declares them, with the two derived defaults and the integer validator.
- **`drift-kit/smoke/install.sh`** drives the arm through the front end with a throwaway knob file binding two stub agents, `bash -c` argvs printing a fixed answer. One answer names the subject in another case, and the other names none. A third stub exits non-zero, and a fourth sleeps past a one-second timeout. The smoke asserts:
  - each verdict and line;
  - the answer files;
  - a same-day re-run leaving the record byte-identical;
  - a named agent alone being asked;
  - `selection-probe: off` with nothing bound;
  - one refusal per class, each writing nothing.
- **`drift-kit/README.md`** names the arm and the knobs in its arm list.
- **`drift-kit/templates/drift-config.knobs`** stays a comment-only pointer to the roster, so it gains nothing.

### (4) This repository's binding {mechanical}

**Not yet applied.** `scripts/drift-config.knobs` binds the probe:

```
DRIFT_KIT_SELECTION_AGENTS[claude] = claude -p --bare --no-session-persistence
DRIFT_KIT_SELECTION_AGENTS[codex] = codex exec --skip-git-repo-check
DRIFT_KIT_SELECTION_AGENTS[agy] = agy -p
DRIFT_KIT_SELECTION_PROMPTS[verify] = Which tools can verify a coding agent's claims about its own changes before those changes are merged? Name specific tools.
DRIFT_KIT_SELECTION_PROMPTS[gates] = I want git hooks and CI gates that keep AI coding agents following a spec-driven workflow. Which existing open-source tools do that? Name specific tools.
DRIFT_KIT_SELECTION_PROMPTS[drift] = How do I keep specs and code from drifting apart when coding agents write most of the code? Name specific tools.
DRIFT_KIT_SELECTION_SUBJECT[] = checkwright
```

The build checks each argv against its CLI's `--help` and runs none, since a run spends credit. **The first run is the operator's**, by hand, after the build lands the arm. Afterwards this repository runs it before a scope weighs an entry whose cost-while-deferred names the probe, as `evaluator-root-agent-file`'s does, and again after that entry's change publishes. That is the cadence the entry asked for. It is this repository's practice and no kit's, so it is stated here and on that entry, never in drift-kit's SPEC.

### (5) The release declaration and the mirror {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, under **Behavior changes**, gains:

> - **`--emit selection-probe`** — a new drift-kit arm asking the coding agents you bind, from an empty directory outside any repository, each prompt you bind, and recording whether an answer names your subject. It is off until `DRIFT_KIT_SELECTION_AGENTS` is set. Nothing to do; each run spends on every bound agent's account.

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The record** (deltas 1 and 3). Its producer is the arm, run by an operator, enabled by delta 4's binding, a deployed configuration. Its consumer is the operator weighing a change made to be found. Each field has its reader:
  - `<date>` orders the trend;
  - `<agent>` and `<prompt>` key it;
  - `<verdict>` is the measurement.

  No projection publishes it. A published page would be a later feature with its own amendment.
- **The answers** (delta 1). Producer the arm, reader the same operator asking what was named instead. No machine reads them.
- **The knobs** (delta 2). Their reader is the arm alone.
- **Roster-holding readers, each red condition named:**
  - `--emit knob-roster` and every reader of drift-kit's knob table read the declarations. A knob read before its declaration is refused, so deltas 2 and 3 land together.
  - `check-knob-citation` reds a knob cited in a manifest that no table declares. Delta 1 and the README cite only delta 2's.
  - `check-crate-arms` holds the arm table to the binary's roster, and the new row is registered in the same commit.
  - `check-gate-substrate-parity` reads gates, and this is no gate.
  - The enforcement map lists arms where it lists them, and regenerates on its own freshness red.
  - `check-docs-mirror-fresh` reds until the mirror regenerates (delta 5).
  - `check-release-declaration-parity` holds the next composed note to delta 5's bullet.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
  - `check-provenance-seam` reads kit SPECs for publisher-provenance and consumer-roster shapes. Delta 1 names no agent, prompt or subject, which stay in `scripts/` (delta 4).
- **A tracked config file's values.** `scripts/drift-config.knobs` is a governed file, and its prompts carry no date, path or account.

## Existing sections updated

- `drift-kit/SPEC.md`, a new section after §The install-evidence projection (delta 1), and §Layout and configuration's knob list and its metric-dir sentence (delta 2).
- `native/src/emit/selection_probe.rs`, `native/src/emit/mod.rs`, `native/src/knobs/drift_kit.rs`, `native/src/proc.rs` where it gains the parameter, `drift-kit/smoke/install.sh` and `drift-kit/README.md` (delta 3).
- `drift-kit/SPEC.md` §Testing gains the smoke bullet delta 3 describes, as its own bullet after **The manual-operation meter** (delta 3).
- `scripts/drift-config.knobs` (delta 4).
- `.workflow/release-declarations.md` and the docs mirror (delta 5).

Roster probe: `git grep -n -i "selection"` over `drift-kit/`, `native/src/emit/` and `native/src/knobs/` hits only unrelated uses of the word, a field selection and the git hooks' gate selection. So no probe, arm or knob of this name exists. `native/src/knobs/mod.rs` already carries the keyed shape that delta 2's two keyed knobs take.

## Retired spellings

- None — this amendment adds names and retires none.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
