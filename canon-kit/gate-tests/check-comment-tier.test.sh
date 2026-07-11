#!/usr/bin/env bash
# Behavioral cases the one-pair good/bad harness cannot hold: the slash-comment
# surface, positional-construct rescue (with a consumer-supplied language
# roster), the .txt state-file restricted roster, and the templates/ surface
# (this gate governs template stubs via _with_templates where check-spec-pointer
# exempts them — a difference only the derived surface shows, since an explicit
# CANON_KIT_COMMENT_SURFACE bypasses the split), and the count-shape override's
# two edges: the comment-tier-exempt valve suppresses it, positional rescue does
# not. Each needs a per-case CANON_KIT_CONFIG_FILE or scan root, which
# run-gate-tests passes to neither. The good/bad pair covers the shell classifier
# (directive run, exempt, heredoc skip) and the count override's mainline; these
# cover the mechanism a consumer activates by widening the surface, plus the
# paragraph-join wrap the pair's single-substring expect.txt cannot pin —
# which line a wrapped total reports at, and that the exempt valve reaches it.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
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
mkcfg "$SANDBOX/rs.sh" 'CANON_KIT_COMMENT_SURFACE=(ok.rs)'   'CANON_KIT_COMMENT_POSITIONAL=(unsafe)'
mkcfg "$SANDBOX/bad.sh" 'CANON_KIT_COMMENT_SURFACE=(bad.rs)' 'CANON_KIT_COMMENT_POSITIONAL=(unsafe)'
mkcfg "$SANDBOX/txt.sh" 'CANON_KIT_COMMENT_SURFACE=(state.txt)'

# The count-shape override's two edges. The exempt valve suppresses a count in
# its window; positional rescue does not — it clears the tier flag a plain block
# would raise, leaving the count to report on its own.
cat >"$SANDBOX/exempt.sh" <<'EOF'
# comment-tier-exempt: this note pins six gates on purpose, the valve engaged
noop() { echo x; }
EOF
cat >"$SANDBOX/pos.sh" <<'EOF'
# spec: some/SPEC.md §p — the header leads with a directive
warm() { echo x; }

# six gates guard the construct below
run() { unsafe; }
EOF
mkcfg "$SANDBOX/exempt-cfg.sh" 'CANON_KIT_COMMENT_SURFACE=(exempt.sh)'
mkcfg "$SANDBOX/pos-cfg.sh" 'CANON_KIT_COMMENT_SURFACE=(pos.sh)' 'CANON_KIT_COMMENT_POSITIONAL=(unsafe)'

# Derived-surface trees (no CANON_KIT_COMMENT_SURFACE) so the templates/ prune
# axis is live. A thinned template stub passes; a narrating one is red.
: >"$SANDBOX/derived.sh"
mkdir -p "$SANDBOX/tmpl-good/templates" "$SANDBOX/tmpl-bad/templates"
cat >"$SANDBOX/tmpl-good/templates/thin.sh" <<'EOF'
# shellcheck shell=bash
# spec: your-kit/SPEC.md §Layout and configuration — knob table + install; set only what you override
EOF
cat >"$SANDBOX/tmpl-bad/templates/narration.sh" <<'EOF'
# shellcheck shell=bash
# This standalone header narrates the config and restates the design rationale
# that belongs in the owning kit SPEC — exactly the prose a governed template
# must not carry once the tier gate scans templates/.
#YOUR_KIT_KNOB=default
EOF

check_case() {  # $1=label $2=cfg $3=want-rc $4=want-substring $5=root(default SANDBOX)
    local label="$1" cfg="$2" want="$3" sub="$4" root="${5:-$SANDBOX}" out rc
    out="$(env CANON_KIT_CONFIG_FILE="$SANDBOX/$cfg" "$GATE" "$root" 2>&1)"; rc=$?
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
check_case "templates-thinned-ok"  derived.sh 0 "COMMENT-TIER: clean"          "$SANDBOX/tmpl-good"
check_case "templates-narration-red" derived.sh 1 "the tier gate scans templates" "$SANDBOX/tmpl-bad"
check_case "count-exempt-valve" exempt-cfg.sh 0 "COMMENT-TIER: clean"
check_case "count-survives-positional-rescue" pos-cfg.sh 1 "restated collection total: six gates"

# The bad fixture's expect.txt asserts the count override, so the window-spill
# half of the pair keeps its assertion here.
check_case "window-spill-flagged" derived.sh 1 "spills outside the window" \
    "$DIR/gate-tests/check-comment-tier/bad"

# The paragraph-join window over a comment block: a total wrapped across two
# comment lines is caught (the per-line scan cannot see it) and reported at the
# cardinal's line; the exempt valve suppresses it across the wrap as on one line.
cat >"$SANDBOX/wrapped.sh" <<'EOF'
# spec: some/SPEC.md §w — this window pins two comment
#   gates across the wrap.
foo() { echo x; }
EOF
cat >"$SANDBOX/wrapx.sh" <<'EOF'
# comment-tier-exempt: this note pins two comment
#   gates on purpose, the valve engaged across the wrap.
noop() { echo x; }
EOF
mkcfg "$SANDBOX/wrapped-cfg.sh" 'CANON_KIT_COMMENT_SURFACE=(wrapped.sh)'
mkcfg "$SANDBOX/wrapx-cfg.sh"   'CANON_KIT_COMMENT_SURFACE=(wrapx.sh)'

check_case "count-wrap-flagged" wrapped-cfg.sh 1 "wrapped.sh:1: restated collection total: two comment gates"
check_case "count-wrap-exempt-valve" wrapx-cfg.sh 0 "COMMENT-TIER: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-comment-tier.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-comment-tier.test.sh: clean (slash surface + positional rescue + txt restricted roster + templates/ governance + count override edges + paragraph-join wrap)"
exit 0
