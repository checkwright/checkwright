#!/usr/bin/env bash
# Behavioral cases the one-pair good/bad harness cannot hold: the slash-comment
# surface, positional-construct rescue (with a consumer-supplied language
# roster), and the .txt state-file restricted roster. Each needs a per-case
# SPEC_KIT_CONFIG_FILE carrying array knobs, which run-gate-tests passes to
# neither. The good/bad pair covers the shell classifier (directive run, exempt,
# heredoc skip); these cover the mechanism a consumer activates by widening the
# surface.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # spec-kit/
GATE="$DIR/checks/check-comment-tier.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

# A Rust source whose only non-directive comment sits immediately above an
# `unsafe` construct — the positional roster rescues it.
cat >"$SANDBOX/ok.rs" <<'EOF'
// spec: some/SPEC.md §a — the header leads with a directive
fn a() {}

// SAFETY: the index is bounds-checked on the line above
let v = unsafe { get() };
EOF

# A Rust source with a standalone // block that cites nothing and sits above an
# ordinary function — no rescue, flagged.
cat >"$SANDBOX/bad.rs" <<'EOF'
// spec: some/SPEC.md §b — header ok
fn a() {}

// this standalone slash block restates the code and cites no section
fn b() {}
EOF

# A .workflow state file: only contract:/see headers bless; a spec: header that
# is fine on a shell source is flagged here.
cat >"$SANDBOX/state.txt" <<'EOF'
# contract: some/SPEC.md §state — the txt roster blesses this
# and this continuation line rides the contract run
2026-01-01 data row not a comment
# spec: on the txt surface this directive is not in the roster
EOF

mkcfg() {  # $1=file  $2..=lines
    local f="$1"; shift
    printf '%s\n' "$@" >"$f"
}
mkcfg "$SANDBOX/rs.sh" 'SPEC_KIT_COMMENT_SURFACE=(ok.rs)'   'SPEC_KIT_COMMENT_POSITIONAL=(unsafe)'
mkcfg "$SANDBOX/bad.sh" 'SPEC_KIT_COMMENT_SURFACE=(bad.rs)' 'SPEC_KIT_COMMENT_POSITIONAL=(unsafe)'
mkcfg "$SANDBOX/txt.sh" 'SPEC_KIT_COMMENT_SURFACE=(state.txt)'

check_case() {  # $1=label $2=cfg $3=want-rc $4=want-substring
    local label="$1" cfg="$2" want="$3" sub="$4" out rc
    out="$(env SPEC_KIT_CONFIG_FILE="$SANDBOX/$cfg" "$GATE" "$SANDBOX" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

check_case "slash-positional-rescue" rs.sh  0 "COMMENT-TIER: clean"
check_case "slash-standalone-flag"    bad.sh 1 "standalone slash block restates"
check_case "txt-restricted-roster"    txt.sh 1 "not in the roster"

if [[ "$fails" -gt 0 ]]; then
    echo "check-comment-tier.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-comment-tier.test.sh: clean (slash surface + positional rescue + txt restricted roster, 3 cases)"
exit 0
