#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — scratch-consumer install+violation harness (a bin/ tool, never a registered gate)
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/consumer-smoke.sh
source "$SDK/lib/consumer-smoke.sh"

KEEP=0
kit_args=()
for a in "$@"; do
    case "$a" in
        --keep) KEEP=1 ;;
        -*) echo "run-consumer-smoke: unknown option: $a" >&2; exit 2 ;;
        *) kit_args+=("$a") ;;
    esac
done

roots=()
if [[ ${#kit_args[@]} -gt 0 ]]; then
    for r in "${kit_args[@]}"; do
        [[ -d "$r" ]] || { echo "run-consumer-smoke: not a directory: $r" >&2; exit 2; }
        roots+=("$(cd "$r" && pwd)")
    done
else
    while IFS= read -r r; do roots+=("$r"); done < <(gate_kit_roots)
fi

ordered=("$SDK")
for r in "${roots[@]}"; do
    [[ "$r" == "$SDK" ]] && continue
    ordered+=("$r")
done
roots=("${ordered[@]}")

for r in "${roots[@]}"; do
    [[ -f "$r/smoke/install.sh" ]] || {
        echo "run-consumer-smoke: $r has no smoke/install.sh — a vendored kit must ship one" >&2
        echo "  help: add smoke/install.sh (+ optional smoke/violation.sh); see gate-sdk/SPEC.md §Consumer smoke." >&2
        exit 2
    }
done

SCRATCH=""
cleanup() {
    [[ -n "$SCRATCH" ]] || return 0
    if [[ "$KEEP" == "1" ]]; then
        echo "CONSUMER-SMOKE: --keep, scratch retained at $SCRATCH"
    else
        rm -rf "$SCRATCH"
    fi
}
trap cleanup EXIT

csmoke_vendor_and_install "${roots[@]}" || exit 2
installed="$CSMOKE_INSTALLED"

run_battery() { ( cd "$SCRATCH" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery is not green on the freshly installed consumer (zero config)"
    printf '%s\n' "$out"
    echo "  help: an install step left a gate red; reproduce with --keep and run gate-sdk/bin/run-gates.sh in the scratch dir."
    exit 1
fi

# spec: gate-sdk/SPEC.md §Consumer smoke — hard reset (not checkout) so a violation that staged its shape is unstaged too
restore() { ( cd "$SCRATCH" && git reset -q --hard && git clean -qfd ); }

# spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: one pass over the union of the vendored kits' checks/ against the scratch registry, between the green-battery assertion and the violation phase
# spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: thecorroborating probe's tree: the invoking repo that ships the kits, where a kit's own surfaces exist
HOST_TREE="${SDK%/*}"
# spec: gate-sdk/SPEC.md §Consumer smoke — the scratch consumer is zero-config, so its registry sits at gate-sdk's default gates dir, where that kit's own smoke/install.sh writes it
scratch_list="$SCRATCH/scripts/gates.list"
[[ -f "$scratch_list" ]] || {
    echo "run-consumer-smoke: no gate registry at $scratch_list after install" >&2
    exit 2
}

declare -A acct_registered=() acct_kit=() acct_root=()
declare -A acct_reason=() acct_declkit=() acct_selfset=()

while IFS= read -r g; do
    [[ -n "$g" ]] && acct_registered["$g"]=1
done < <(gates_list_members "$scratch_list")

# spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting reads *declaration paths*, both spellings: a gate whose implementation ported to a compiled subcommand still ships, still earns or forfeits a scratch-battery slot, and must not leave this pass's universe by changing substrate
shopt -s nullglob
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    acct_root["$kit"]="$r"
    for f in "$SCRATCH/$kit"/checks/check-*.sh "$SCRATCH/$kit"/checks/check-*.gate; do
        g="$(basename "$f")"; g="${g%.*}"
        acct_kit["$g"]="$kit"
    done
done
shopt -u nullglob

for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    while IFS= read -r line; do
        line="${line#*smoke-unregistered:}"
        read -r dg reason <<<"$line"
        reason="${reason#—}"; reason="${reason#--}"; reason="${reason#-}"; reason="${reason# }"
        if [[ -z "$dg" || -z "$reason" ]]; then
            echo "CONSUMER-SMOKE: FAIL — $kit/smoke/install.sh has a '# smoke-unregistered:' line missing its gate name or its reason"
            echo "  help: the shape is '# smoke-unregistered: <gate-name> — <reason>'; both fields are read (gate-sdk/SPEC.md §Consumer smoke)."
            exit 1
        fi
        acct_reason["$dg"]="$reason"
        acct_declkit["$dg"]="$kit"
    done < <(grep -E '^[[:space:]]*#[[:space:]]*smoke-unregistered:' "$SCRATCH/$kit/smoke/install.sh" || true)
done

mapfile -t acct_unreg < <(
    for g in "${!acct_kit[@]}"; do
        [[ -n "${acct_registered[$g]:-}" ]] || printf '%s\n' "$g"
    done | sort
)

acct_self=0
acct_hand=0
acct_bad=()
acct_stale=()
acct_contra=()

# spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting probes through gate_command's substrate-blind dispatch, so a ported gate is probed as the gate it is rather than skipped for want of a .sh; an unresolvable dispatch is the exit-2 the contract already means by "could not run"
acct_probe() {  # acct_probe <tree> <checks-dir> <gate>
    local argv=()
    mapfile -t argv < <( cd "$1" && gate_command "$3" "$2" 2>/dev/null )
    [[ ${#argv[@]} -gt 0 ]] || return 2
    ( cd "$1" && "${argv[@]}" ) >/dev/null 2>&1
}

acct_start_ns=$(date +%s%N)
for g in "${acct_unreg[@]}"; do
    [[ -n "$g" ]] || continue
    kit="${acct_kit[$g]}"
    acct_probe "$SCRATCH" "$SCRATCH/$kit/checks" "$g"
    rc_s=$?
    rc_h=""
    if [[ "$rc_s" -eq 2 ]]; then
        # spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: exit 2 is usage/environment failure generally, so the permanent exemption is granted only when the same gate does NOT exit 2 where its surface exists
        acct_probe "$HOST_TREE" "${acct_root[$kit]}/checks" "$g"
        rc_h=$?
        if [[ "$rc_h" -ne 2 ]]; then
            # spec: gate-sdk/SPEC.md §Consumer smoke — a declaration the probe contradicts is reported rather than silently exempted: the gate declares it reads a surface a fresh install writes and the probe derived that the surface is absent, so one of the two is wrong and the arm exists so a wrong declaration is not as invisible as a missing one
            if [[ "$(awk 'sub(/^# install:[[:space:]]+/, "") { print $1; exit }' \
                    "$SCRATCH/$kit/checks/$g".{sh,gate} 2>/dev/null | head -n1)" == zero-config ]]; then
                acct_contra+=("$kit ships $g declaring zero-config, yet the probe finds its surface absent in the scratch consumer")
            fi
            acct_selfset["$g"]=1
            acct_self=$((acct_self + 1))
            continue
        fi
    fi
    # spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: a declaration is honoured only from the kit that ships the gate, so a foreign-kit line leaves the omission unaccounted as well as stale
    if [[ -n "${acct_reason[$g]:-}" && "${acct_declkit[$g]}" == "$kit" ]]; then
        acct_hand=$((acct_hand + 1))
        continue
    fi
    acct_bad+=("$kit ships $g — scratch exit $rc_s${rc_h:+, invoking-repo exit $rc_h}")
done
acct_ms=$(( ($(date +%s%N) - acct_start_ns) / 1000000 ))

if [[ ${#acct_reason[@]} -gt 0 ]]; then
    for g in "${!acct_reason[@]}"; do
        dk="${acct_declkit[$g]}"
        if [[ -n "${acct_registered[$g]:-}" ]]; then
            acct_stale+=("$dk declares $g unregistered, but it is registered")
        elif [[ "${acct_kit[$g]:-}" != "$dk" ]]; then
            acct_stale+=("$dk declares $g, which that kit does not ship")
        elif [[ -n "${acct_selfset[$g]:-}" ]]; then
            acct_stale+=("$dk declares $g, which the probe already exempts — probe first, reasons second")
        fi
    done
fi

# spec: gate-sdk/SPEC.md §Consumer smoke — a probe may leave scratch artifacts; the violation phase starts from the committed baseline
restore

# spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: the measured cost is reported on every run, red or green, and never cached across runs
echo "CONSUMER-SMOKE: accounting — ${#acct_unreg[@]} unregistered gate(s) probed in ${acct_ms}ms ($acct_self self-declared, $acct_hand hand-declared, ${#acct_bad[@]} unaccounted)"
for l in "${acct_contra[@]:-}"; do [[ -n "$l" ]] && echo "  contradicted declaration: $l"; done

if [[ ${#acct_bad[@]} -gt 0 || ${#acct_stale[@]} -gt 0 ]]; then
    echo "CONSUMER-SMOKE: FAIL — the registration accounting is not satisfied"
    for l in "${acct_bad[@]:-}"; do [[ -n "$l" ]] && echo "  unaccounted: $l"; done
    for l in "${acct_stale[@]:-}"; do [[ -n "$l" ]] && echo "  stale declaration: $l"; done
    echo ""
    echo "  help: register the gate in that kit's smoke/install.sh, or declare the omission"
    echo "        beside its registration block with '# smoke-unregistered: <gate-name> — <reason>'"
    echo "        (gate-sdk/SPEC.md §Consumer smoke)."
    exit 1
fi

fired=0
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    vio="$SCRATCH/$kit/smoke/violation.sh"
    if [[ ! -f "$vio" ]]; then
        echo "CONSUMER-SMOKE: $kit has no violation script — install coverage only"
        continue
    fi
    expected="$( ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$vio" ) | head -n1 )"
    if [[ -z "$expected" ]]; then
        echo "CONSUMER-SMOKE: FAIL — $kit/smoke/violation.sh printed no expected-gate name on line 1"
        restore
        exit 1
    fi
    out="$(run_battery)"; rc=$?
    if [[ "$rc" -eq 0 ]]; then
        echo "CONSUMER-SMOKE: FAIL — $kit violation did not turn the battery red (expected gate $expected)"
        printf '%s\n' "$out"
        restore
        exit 1
    fi
    if ! grep -qF "FAIL: $expected" <<<"$out"; then
        echo "CONSUMER-SMOKE: FAIL — $kit violation fired, but no 'FAIL: $expected' line (wrong gate caught it)"
        printf '%s\n' "$out"
        restore
        exit 1
    fi
    restore
    fired=$((fired + 1))
done

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "CONSUMER-SMOKE: FAIL — the battery did not return to green after the final restore"
    printf '%s\n' "$out"
    exit 1
fi

echo "CONSUMER-SMOKE: clean ($installed kits installed, $fired violations fired, ${#acct_registered[@]} gates registered, $acct_self self-declared, $acct_hand hand-declared)"
exit 0
