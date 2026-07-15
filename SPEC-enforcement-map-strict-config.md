# SPEC amendment: enforcement-map-strict-config

gate-sdk ruling: converge `bin/enforcement-map.sh`'s registry-knob reads on
the strict set-but-missing distinction the kit config loaders already share
(gate-sdk/SPEC.md §lib/test-hermetic.sh: fail closed, exit 2), keeping the
unset-with-default-absent degrade that serves the zero-config consumer. Merge
target: gate-sdk/SPEC.md §enforcement-map (plus the emitter, its test, and the
generated docs mirror).

## What changes

- **Envelope: all four consumer-relocatable registry knobs, not one.** The
  filed entry named `EVIDENCE_KIT_CONFIG_FILE`; the silent-wrong-config class
  lives in each knob *read*, and the emitter has four with the same lenient
  shape: `DRIFT_KIT_KPIS_FILE`, `CONTEXT_KIT_SETTINGS_FILE`,
  `EVIDENCE_KIT_CONFIG_FILE` (files), `GATE_SDK_ENFORCE_SCAN_DIR` (a
  directory). Converging one read and leaving three lenient in the same file
  reintroduces the inconsistency the unit exists to remove. The rule,
  uniform: **explicitly set and the path missing → stderr line naming the
  knob, exit 2; unset with the default absent → the section degrades as
  today.** The current `: "${VAR:=default}"` expansion erases set-ness, so
  the emitter captures set-ness before defaulting (the
  `evidence-kit/lib/evidence.sh` shape); any helper is file-local, no new
  governed name.
- **Settings knob additionally checks parseability when set.** A typo
  *inside* an explicitly adopted settings file is the same laundering vector
  as a typo in its path, and the validity predicate (`jq -e`) already sits in
  the code path: set + unparseable → exit 2. The default-path file
  unparseable keeps degrading (a stray `.claude/` must not break a
  zero-config consumer; in this repo `check-settings-pins` owns the tracked
  file's health). `jq` *absence* keeps degrading in both modes: a toolchain
  gap, owned by the install requirements / env-probe parity, not a config
  typo.
- **Check-before-emit ordering.** All knob checks run before the first stdout
  byte (both bare and `--emit` modes), so a misconfigured
  `--emit > docs/enforcement.md` regen leaves an empty projection and a
  nonzero exit — never a plausible partial page that byte-matches itself on
  the next freshness check (the laundering the filed entry names).
- **Exit-contract prose amended.** §enforcement-map's "advisory by
  construction: exit is always 0" becomes: advisory means it never joins
  `gates.list` and a *healthy* run exits 0 whatever registries are absent; a
  misconfigured run (a set-but-missing registry knob) exits 2, fail-closed.
  The same section's "degrades per class" paragraph gains the distinction:
  *not adopted* (unset, default absent) degrades; *adopted but broken*
  (set-but-missing) refuses. The emitter's header comment moves in lockstep.
- **Test flips with the contract.** The three `/nonexistent` cases in
  `gate-sdk/gate-tests/enforcement-map.test.sh` currently assert the lenient
  skip; they become strict cases (exit 2, empty stdout, stderr names the
  knob), a fourth covers the scan-dir knob, and degrade coverage is
  re-established by construction — unset knob with an absent default (e.g. a
  scratch `GATE_SDK_GATES_DIR` holding a `gates.list` copy but no
  `kpis.list`/`evidence-config.sh`), keeping the section-drops-independently
  assertions the test's header promises.

## Ruled out

- **Sourcing `evidence-kit/lib/evidence.sh` for the strict check** — the
  degrade-per-class contract exists precisely because evidence-kit may be
  unvendored, and a gate-sdk → evidence-kit source dependency inverts the kit
  layering. Only the existence distinction is converged, not the loader.
- **Extending strictness to `GATE_SDK_GATES_DIR`** — the layout root every
  gate-sdk entry point shares, owned by §Layout and configuration (whose
  config seam is already strict on `GATE_SDK_CONFIG_FILE`); ruling its
  missing-directory behavior is a gate-sdk-core unit touching every tool, and
  its wrongness already surfaces through the battery's roster-collapse
  tripwire (§run-gates) rather than one silently thinner page. No filing: an
  owned mechanism covers the failure mode.

## Producers and consumers

- **The exit-2 misconfiguration event** — producer: the knob checks at
  emitter entry, before any output, in both modes. Consumers, all
  pre-existing: `check-enforcement-fresh` routes the emitter's status through
  `fail_closed` (a bad knob reds pre-commit instead of laundering);
  `scripts/gen-value-rollup.sh` already exits 2 on a nonzero emitter; a
  manual regen surfaces the stderr line naming the knob. No new knob, file,
  field, or tag — every name this amendment touches already exists.

## Existing sections updated

- gate-sdk/SPEC.md §enforcement-map: the exit-contract sentence and the
  degrades-per-class paragraph (above).
- `gate-sdk/bin/enforcement-map.sh` header comment (the "exit is always 0"
  line).
- The test's header spec line (degraded-registry coverage description now
  also names the strict refusal).
- `docs/gate-sdk/SPEC.md` via the docs-mirror regen; `docs/enforcement.md` is
  byte-identical under correct config — no projection change.

## Definition of Done

- [ ] **Causal completeness** — the exit-2 event has named, reachable
      producers and named consumers; no new field exists.
- [ ] **Merged with no information lost** — the rulings land in
      §enforcement-map's prose; the section reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — no spec still claims "exit is always 0" or
      an unconditional per-class degrade (mirror included).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
