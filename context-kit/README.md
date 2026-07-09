# context-kit

Token-economics-aware context management for stateless agent sessions: an
index-first reading toolset, a session-start hook that assembles a compact
brief, a meter that tracks the always-loaded surface against a committed
baseline, one gate over the densest always-loaded section, and a close-stage
brevity pass that reacts to the meter's delta.

Why: a stateless session pays for context twice. The *on-demand* cost is
opening a whole SPEC or source file when one section was needed — the index
tools cut that ("index, then read the one you need"). The *standing* cost is the
always-loaded surface (the instructions file, the session-start hook output)
where every added line is a recurring per-session tax that grows silently,
because no single session sees the trend — the meter, the gate, and the
close-stage pass make that growth visible and actionable. See
[SPEC.md](SPEC.md) for the full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required); the meter's default
hook approximation and the session-context template also expect
[queue-kit](../queue-kit/). Then:

1. Register the gate — add `check-brevity` to your `gates.list`. It resolves
   through gate-sdk's registry path (your gates dir first, then each kit's
   `checks/`), and its `# graph:` manifest puts it in the generated pre-commit
   hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

2. Wire the session-start hook — copy `templates/session-context.sh` into your
   gates dir, edit its `[EDIT ME]` sections (layout judgment, not mechanism),
   and merge `templates/settings-sessionstart.json` into `.claude/settings.json`.

3. Set the baseline — `bash context-kit/bin/always-loaded.sh --update-baseline`
   and commit `.workflow/always-loaded-baseline.txt`.

4. Optional — retune: copy `templates/context-config.sh` into your gates dir and
   override any knob (surfaces, hook-body command, brevity file/section/budget/
   pointer pattern). Defaults are this repo's layout. Splice
   `templates/close-brevity.md` into your close skill.

## Use

```bash
bash context-kit/bin/md-index.sh [paths…]           # markdown heading index + first sentences
bash context-kit/bin/md-section.sh <file> <heading> # print one section by heading
bash context-kit/bin/pub-index.sh [paths…]          # Rust public API surface
bash context-kit/bin/always-loaded.sh               # standing surface vs baseline (one line)
bash context-kit/bin/always-loaded.sh --update-baseline   # a close-stage act
```

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh context-kit/gate-tests context-kit/checks  # the check-brevity pair
bash context-kit/bin/run-index-tests.sh                                        # the advisory tools vs golden output
```
