# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — this repo's delegation-kit consumer config
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this is the seeded-copy side of delegation-kit's config seam, the file this repo actually edits and the more edited of the two, so porting it deletes the seam outright. Its D2 roster, history path, gate-file globs and meta-path prefixes are all this repo's own wiring, set against kit defaults that ship empty or generic. The 2026-08-24 vocabulary ruling left it owed deliberately — a one-member harness-generic type roster plus path globs — which answers "does this hold private vocabulary?" and never "is this an edit seam?"; that verdict is untouched here and the two grounds are cumulative. Structural, not a sizing judgment.

# spec: delegation-kit/SPEC.md §The delegation model — this repo's D2 roster: audit-sweep is dispatched read-only by its own agent-definition description ("it mutates nothing"); stage-session is not read-only and stays off, and a type with no Edit/Write tools is still not exempt from declaring isolation once dispatched read-only (it still reaches git through its shell) — there is none such in this roster today
# shellcheck disable=SC2034  # consumed by the --hook agent-dispatch-guard arm, which the config bridge resolves this roster for
DELEGATION_KIT_READONLY_TYPES=(audit-sweep)

# spec: delegation-kit/SPEC.md §usage-verdict — sample the footprint per verdict into the gitignored measurement dir; --emit-usage-trend reports the evolution
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_USAGE_HISTORY=".metric/usage-history.log"

# spec: gate-sdk/SPEC.md §The non-gate arm — the poller is an arm now, so the refresh command dispatches it through the front-end rather than naming a template path the port deleted
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_REFRESH_CMD="bash gate-sdk/bin/run-gates.sh --usage-poll"

# comment-tier-exempt: `*` spans '/' in a bash [[ == ]] glob, so */checks/*.sh reaches every kit's gates; this array REPLACES the kit default (delegation.sh guards it with `declare -p ... ||`), so the default's scripts/check-*.sh glob is restated here rather than inherited — the repo's consumer-resident gates would otherwise fall outside tamper coverage
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_GATE_FILES=(
    "*/checks/*.sh"
    "*/checks/*.gate"
    "scripts/check-*.sh"
    "scripts/check-*.gate"
    "gate-sdk/lib/gate.sh"
    "gate-sdk/bin/run-gate-tests.sh"
)

# comment-tier-exempt: the kit roots are auto-unioned by delegation.sh (a vendored kit's edits are meta-layer by definition), so only the non-kit prefixes are declared here; native/ is declared explicitly because it ships no checks/ or smoke/ and so is never a kit root by gate_kit_roots's predicate, despite carrying ported gates' Rust implementations (gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate, check-gate-tamper row)
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_META_PATHS=(
    scripts/ .workflow/ .claude/ docs/ .github/ native/
)
