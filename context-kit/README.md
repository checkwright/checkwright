# context-kit

Token-economics-aware context management for stateless agent sessions: an index-first reading toolset, a session-start hook that assembles a compact brief, a meter that tracks the always-loaded surface against a committed baseline, one gate over its governed always-loaded sections, a freshness-gated per-kit token-footprint projection, a close-stage brevity pass that reacts to the meter's delta, and a memory-off gate pair (settings pins plus a local memory-dir scan) that keeps the harness's ungoverned auto-memory surface disabled.

Why: a stateless session pays for context twice. The *on-demand* cost is opening a whole SPEC or source file when one section was needed — the index tools cut that ("index, then read the one you need"). The *standing* cost is the always-loaded surface (the instructions file, the session-start hook output) where every added line is a recurring per-session tax that grows silently, because no single session sees the trend — the meter, the gate, and the close-stage pass make that growth visible and actionable. See [SPEC.md](SPEC.md) for the full contracts.

This file ships in the installer payload, which withholds each kit's `SPEC.md` and its `smoke/`. Every `SPEC.md` link on this page is repointed at the location `GATE_SDK_SPEC_BASE_URL` names when the payload is packed; with no base set the link stays relative (gate-sdk/SPEC.md §Consumer payload).

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required); the meter's default hook approximation and the session-context template also expect [queue-kit](../queue-kit/). Then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-brevity
   check-surface-ratchet
   check-settings-pins
   check-settings-paths
   check-memory-off
   check-footprint-fresh
   ```
   <!-- gate-roster:end -->

   They resolve through gate-sdk's registry path (your gates dir first, then each kit's `checks/`), and their `# graph:` manifests put them in the generated pre-commit hook, which `--emit git-hooks --write` on the gate binary `GATE_SDK_NATIVE_BIN` names writes. Three of them wait on a step of yours: `check-settings-pins` skips clean until you create `settings-pins.conf` naming the settings keys to hold, such as the auto-memory-disabling ones (SPEC.md §check-settings-pins); `check-footprint-fresh` is for when you publish the footprint page; and `check-surface-ratchet` arms once you commit the ceilings `--emit always-loaded --ceiling` stamps (SPEC.md §The surface ratchet).

2. Wire the session-start hook — copy `templates/session-context.sh` into your gates dir, edit its `[EDIT ME]` sections (layout judgment, not mechanism), and merge `templates/settings-sessionstart.json` into `.claude/settings.json`.

3. Set the baseline — run `--emit always-loaded --update-baseline` on the gate binary `GATE_SDK_NATIVE_BIN` names and commit `.workflow/always-loaded-baseline.txt`.

4. Seed your env profile — the gate binary's `--emit env-probe` arm writes a marker-bounded machine profile (OS, package manager, toolchain versions, absent tools) into `ENV.local.md` and seeds a hand-authored gotchas scaffold above the markers. The file is local-only (gitignore it); re-run on demand when the box changes. The session-context hook emits it when present.

5. Optional — retune: copy `templates/context-config.knobs` into your gates dir and override any knob (surfaces, hook-body command, brevity file/section set/ budget/pointer pattern, env-profile file). Defaults are this repo's layout. Splice `templates/close-brevity.md` into your close skill.

## Use

Run these arms with the gate binary `GATE_SDK_NATIVE_BIN` names, where `init` places it. In PowerShell, spell each line `./scripts/checkwright-gates <arm>` at the repository root, with `.exe` on native Windows:

```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --emit md-index [paths…]            # markdown heading index + first sentences
"$gates" --emit md-section <file> <heading>  # print one section by heading
"$gates" --emit pub-index [paths…]           # public API surface (per-language extractors; ships rust, ts)
"$gates" --emit always-loaded                 # standing surface vs baseline (one line)
"$gates" --emit always-loaded --update-baseline   # a close-stage act
"$gates" --emit footprint     # per-kit token footprint (the committed page)
"$gates" --emit env-probe     # re-probe the local machine profile (ENV.local.md)
```

## Test

Run these arms the same way:

```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --run-gate-tests context-kit/gate-tests context-kit/checks  # the gate fixture pairs
"$gates" --run-index-tests                               # the advisory tools vs golden output
```
