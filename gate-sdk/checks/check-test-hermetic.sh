#!/usr/bin/env bash
# graph: couples=kit:gate-tests/*.test.sh,kit:smoke/install.sh,kit:smoke/violation.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-test-hermetic — two assertions (A) every bespoke gate-tests/*.test.sh sources lib/test-hermetic.sh or carries a `# hermetic-exempt:` marker; (B) a credential-managing smoke script pins every own-kit bin call ("$SMOKE_KIT_ROOT/bin/*") to a *_CRED_FILE path so it cannot resolve the ambient ~/.claude credential
#
# usage: check-test-hermetic.sh [dir...]         assertion A over gate-tests dir(s)
#        check-test-hermetic.sh --smoke [dir...]  assertion B over smoke dir(s)
#        (no args) both, over each kit's gate-tests/ and smoke/
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

MODE=all
if [[ "${1:-}" == "--smoke" ]]; then
    MODE=smoke; shift
elif [[ $# -gt 0 ]]; then
    MODE=tests
fi

fail=0
tests_total=0
smoke_total=0

# assertion A: every bespoke gate-tests/*.test.sh sources the hermetic bootstrap or is exempt
scan_tests() {
    local -a scan_dirs=()
    if [[ $# -gt 0 ]]; then
        scan_dirs=("$@")
    else
        local k
        while IFS= read -r k; do scan_dirs+=("$k/gate-tests"); done < <(gate_kit_roots)
    fi

    local -a files=()
    local d
    shopt -s nullglob
    for d in "${scan_dirs[@]}"; do
        files+=("$d"/*.test.sh)
    done
    shopt -u nullglob
    [[ ${#files[@]} -gt 0 ]] || { echo "check-test-hermetic: no *.test.sh under: ${scan_dirs[*]}" >&2; exit 2; }

    local -a leaky=()
    local f
    for f in "${files[@]}"; do
        tests_total=$((tests_total + 1))
        grep -q 'lib/test-hermetic\.sh' "$f" && continue
        grep -qE '^#[[:space:]]*hermetic-exempt:' "$f" && continue
        leaky+=("$f")
    done

    if [[ ${#leaky[@]} -gt 0 ]]; then
        echo "check-test-hermetic: bespoke test(s) neither source lib/test-hermetic.sh nor"
        echo "carry a '# hermetic-exempt:' marker (gate-sdk/SPEC.md §check-test-hermetic — a"
        echo "test on the invoker's cwd config can green wrongly on the consumer's posture):"
        for f in "${leaky[@]}"; do echo "  $f"; done
        echo "  help: source the bootstrap as the test's first act —"
        echo "        source \"\$(dirname \"\${BASH_SOURCE[0]}\")/../../gate-sdk/lib/test-hermetic.sh\""
        echo "  (per-case config overrides after the source still win by ordering), OR add a"
        echo "  '# hermetic-exempt: <reason>' line for a test that proves hermeticity otherwise."
        fail=1
    fi
}

# assertion B: a credential-managing smoke script (one that assigns *_CRED_FILE) must pin
# every own-kit bin call, else the bin resolves its cred file from the ambient ~/.claude
scan_smoke() {
    local -a smoke_dirs=()
    if [[ $# -gt 0 ]]; then
        smoke_dirs=("$@")
    else
        local k
        while IFS= read -r k; do [[ -d "$k/smoke" ]] && smoke_dirs+=("$k/smoke"); done < <(gate_kit_roots)
    fi

    local -a leaky=()
    local d name f line lineno content
    for d in "${smoke_dirs[@]}"; do
        for name in install.sh violation.sh; do
            f="$d/$name"
            [[ -e "$f" ]] || continue
            [[ -r "$f" ]] || { echo "check-test-hermetic: unreadable smoke script: $f" >&2; exit 2; }
            smoke_total=$((smoke_total + 1))
            grep -qE '^#[[:space:]]*hermetic-exempt:' "$f" && continue
            grep -q '_CRED_FILE=' "$f" || continue
            while IFS= read -r line; do
                lineno="${line%%:*}"
                content="${line#*:}"
                [[ "$content" == *_CRED_FILE=* ]] && continue
                leaky+=("$f:$lineno")
            done < <(grep -nF 'SMOKE_KIT_ROOT/bin/' "$f")
        done
    done

    if [[ ${#leaky[@]} -gt 0 ]]; then
        echo "check-test-hermetic: credential-managing smoke script(s) invoke an own-kit bin"
        echo "(\$SMOKE_KIT_ROOT/bin/…) with no *_CRED_FILE= pin on the line — the bin resolves"
        echo "its credential file from the ambient \$HOME/.claude, so the smoke reads live"
        echo "credential state (gate-sdk/SPEC.md §check-test-hermetic):"
        for f in "${leaky[@]}"; do echo "  $f"; done
        echo "  help: prefix the invocation with a hermetic pin to an absent path, e.g."
        echo "        <KIT>_CRED_FILE=\"\$pp/absent.json\" bash \"\$SMOKE_KIT_ROOT/bin/…\" …"
        echo "  (an absent path zeroes the login timestamp so no ambient auth event leaks in),"
        echo "  OR add a '# hermetic-exempt: <reason>' line for a script hermetic otherwise."
        fail=1
    fi
}

case "$MODE" in
    tests) scan_tests "$@" ;;
    smoke) scan_smoke "$@" ;;
    all)   scan_tests; scan_smoke ;;
esac

[[ "$fail" -eq 0 ]] || exit 1

echo "TEST-HERMETIC: clean ($tests_total bespoke test(s) pinned to kit defaults; $smoke_total smoke script(s) checked for ambient-credential leaks)"
exit 0
