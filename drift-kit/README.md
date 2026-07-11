# drift-kit

Drift reporting for coding-agent sessions. Gates block what a single diff makes
mechanically decidable; **drift is what accumulates between commits** — a
backlog ages, an always-loaded surface swells, friction recurs, and every
stateless session looks fine because none of them ever sees the slope. drift-kit
is that trend surface: an advisory `drift-report.sh` that collates pluggable
KPIs from the other kits' governed surfaces, groups them under honest weight
labels (**lead** — act before drift compounds; **lag** — undercounts by
construction), and emits a one-line trend summary the session-start hook injects,
so every session opens seeing the slope it cannot otherwise see.

Advisory by construction: the report exits 0, never fails a session, and reads
**trend, not level** — a KPI's absolute value is noise; its direction across
sessions is the signal. See [SPEC.md](SPEC.md) for the report frame, the plugin
contract, the bundled KPI set, and the knowledge-friction loop.

Like [guard-kit](../guard-kit/), drift-kit registers **no gates**: its surface
is an advisory `bin/` tool and a KPI registry, so nothing joins `gates.list`. It
follows gate-sdk's resolution and smoke conventions without depending on its
registry.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/), then:

1. Copy the registry and config into your gates dir (default `scripts/`):

   ```bash
   cp drift-kit/templates/kpis.list       scripts/kpis.list
   cp drift-kit/templates/drift-config.sh scripts/drift-config.sh   # optional
   ```

   Prune `scripts/kpis.list` to the KPIs whose surfaces your repo has; each
   bundled KPI degrades to a visible `n/a` row when its surface is absent, so an
   over-broad registry is safe but noisy.

2. Wire the trend line — point context-kit's session-context hook at the report
   by pointing `CONTEXT_KIT_DRIFT_REPORT` at `drift-kit/bin/drift-report.sh` (or
   the `DRIFT_REPORT` default in your hook copy). The hook runs `--trend` and prints
   one line; absent the variable, the line is silently skipped.

Configuration follows the established kit pattern — override any knob in
`drift-config.sh` (registry path, extra KPI dirs, the queue/log/timings surfaces,
the Done/Deferred section headings); defaults are this repo's layout.

### The knowledge-friction loop (optional)

`kpi-knowledge-friction` measures re-derivations a session had to make because no
doc owned the fact (drift-kit/SPEC.md §The knowledge-friction loop). It shows
`n/a` until you install the capture half:

1. Add one bullet to your always-loaded instructions file: *the moment you catch
   yourself re-deriving a fact off a non-owning surface, append*
   `<date> <fact> ← <surface>` *to `.workflow/knowledge-friction.log`.* That
   bullet is the loop's only hook — earn its cost by the log actually filling.
   `bin/kfric.sh "<fact>" "<surface>"` is the shipped affordance that stamps
   that grammar prompt-free — raw append stays legal
   (drift-kit/SPEC.md §The knowledge-friction loop).
2. Gitignore the log (per-iteration scratch), and splice
   `templates/close-knowledge.md` into your close skill so each entry becomes a
   doc-owner tiering edit and the log is cleared — its reclaim path.

## Use

```bash
bash drift-kit/bin/drift-report.sh            # full report: lead/lag rows under the honesty labels
bash drift-kit/bin/drift-report.sh --trend    # one compact line (fragments joined with ·)
bash drift-kit/bin/trajectory.sh --emit       # governed-trajectory table (one row per closed iteration)
```

`bin/trajectory.sh` is the published-evidence extractor (drift-kit/SPEC.md §The
published-evidence extractor): a pure function of committed git history that
emits one row per closed iteration — stages run, commit shape, amendment
latency, validate attestations, gate-roster growth — for a consumer to pin
behind a freshness gate.

A KPI plugin is `kpi-<name>.sh`, resolved through `kpis.list` against your KPI
dirs then each vendored kit's `kpis/`. Add your own by dropping a plugin in your
gates dir and naming it in the registry; shadow a bundled one with a same-named
file. The bundled set (drift-kit/SPEC.md §Bundled KPIs) covers, as lead KPIs, the
queue split, the gate backlog, amendment/deferred age, prompt friction, the
always-loaded surface, the local permission overlay, and gate runtime; and one
lag KPI, `kpi-knowledge-friction`, fed by the loop below.

## Test

```bash
bash gate-sdk/bin/run-consumer-smoke.sh drift-kit   # report contract: sections, per-KPI rows, degradation, one-line --trend
```
