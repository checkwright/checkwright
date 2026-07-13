# context-kit

Token-economics-aware context management for stateless agent sessions: an
index-first reading toolset, a session-start hook that assembles a compact
brief, a meter that tracks the always-loaded surface against a committed
baseline, one gate over the densest always-loaded section, a freshness-gated
per-kit token-footprint projection, a close-stage
brevity pass that reacts to the meter's delta, and a memory-off gate pair
(settings pins plus a local memory-dir scan) that keeps the harness's
ungoverned auto-memory surface disabled.

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

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-brevity
   check-settings-pins
   check-memory-off
   check-footprint-fresh
   ```
   <!-- gate-roster:end -->

   They resolve through gate-sdk's registry path (your gates dir first, then
   each kit's `checks/`), and their `# graph:` manifests put them in the
   generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh --write`.
   The memory-off gates are inert until you opt in — `check-settings-pins`
   skips clean with no pins file, so create `settings-pins.conf` (one
   `<jq path> = <expected JSON>` per line) naming the keys to hold, e.g. the
   auto-memory-disabling ones. `check-footprint-fresh` byte-gates a committed
   `docs/footprint.md` against `bin/footprint.sh --emit`; register it when you
   publish that projection.

2. Wire the session-start hook — copy `templates/session-context.sh` into your
   gates dir, edit its `[EDIT ME]` sections (layout judgment, not mechanism),
   and merge `templates/settings-sessionstart.json` into `.claude/settings.json`.

3. Set the baseline — `bash context-kit/bin/always-loaded.sh --update-baseline`
   and commit `.workflow/always-loaded-baseline.txt`.

4. Seed your env profile — `bash context-kit/bin/env-probe.sh` writes a
   marker-bounded machine profile (OS, package manager, toolchain versions,
   absent tools) into `ENV.local.md` and seeds a hand-authored gotchas scaffold
   above the markers. The file is local-only (gitignore it); re-run on demand
   when the box changes. The session-context hook emits it when present.

5. Optional — retune: copy `templates/context-config.sh` into your gates dir and
   override any knob (surfaces, hook-body command, brevity file/section/budget/
   pointer pattern, env-profile file). Defaults are this repo's layout. Splice
   `templates/close-brevity.md` into your close skill.

## Use

```bash
bash context-kit/bin/md-index.sh [paths…]           # markdown heading index + first sentences
bash context-kit/bin/md-section.sh <file> <heading> # print one section by heading
bash context-kit/bin/pub-index.sh [paths…]          # public API surface (per-language extractors; ships rust, ts)
bash context-kit/bin/always-loaded.sh               # standing surface vs baseline (one line)
bash context-kit/bin/always-loaded.sh --update-baseline   # a close-stage act
bash context-kit/bin/footprint.sh                   # per-kit token footprint (--emit: the committed page)
```

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh context-kit/gate-tests context-kit/checks  # the gate fixture pairs
bash context-kit/bin/run-index-tests.sh                                        # the advisory tools vs golden output
```
