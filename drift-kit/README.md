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
   by setting `CONTEXT_KIT_DRIFT_REPORT=drift-kit/bin/drift-report.sh` (or the
   `DRIFT_REPORT` default in your hook copy). The hook runs `--trend` and prints
   one line; absent the variable, the line is silently skipped.

Configuration follows the established kit pattern — override any knob in
`drift-config.sh` (registry path, extra KPI dirs, the queue/log/timings surfaces,
the Done/Deferred section headings); defaults are the extracted platform's.

## Use

```bash
bash drift-kit/bin/drift-report.sh            # full report: lead/lag rows under the honesty labels
bash drift-kit/bin/drift-report.sh --trend    # one compact line (fragments joined with ·)
```

A KPI plugin is `kpi-<name>.sh`, resolved through `kpis.list` against your KPI
dirs then each vendored kit's `kpis/`. Add your own by dropping a plugin in your
gates dir and naming it in the registry; shadow a bundled one with a same-named
file. The bundled set (drift-kit/SPEC.md §Bundled KPIs) covers the queue split,
the gate backlog, amendment/deferred age, prompt friction, the always-loaded
surface, the local permission overlay, and gate runtime.

## Test

```bash
bash gate-sdk/bin/run-consumer-smoke.sh drift-kit   # report contract: sections, per-KPI rows, degradation, one-line --trend
```
