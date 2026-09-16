#!/usr/bin/env bash
# Behavioral test of the config-driven paths the one-pair good/bad harness cannot
# hold: the kit-roots-off default, an absent queue file, the _EXTRA marker union,
# the fence skip as an absence (expect.txt asserts presence only), and the
# fail-closed config arms.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

mkdir -p "$SANDBOX/widget-toolkit"
cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md — sandbox queue

## New Features

- **sprocket-rework-unit** — rework the sprocket.
EOF
cat >"$SANDBOX/widget-toolkit/SPEC.md" <<'EOF'
# widget-toolkit — SPEC

The scan was signed off by the maintainer on 2026-08-12 and stays.

The retry cap is the `sprocket-rework-unit` entry's to settle.

```text
The retry cap is the sprocket-rework-unit entry's, signed off 2026-08-12.
```
EOF
: >"$SANDBOX/off.knobs"
cat >"$SANDBOX/on.knobs" <<'EOF'
CANON_KIT_SCAN_KIT_ROOTS = 1
EOF
cat >"$SANDBOX/extra.knobs" <<'EOF'
CANON_KIT_SCAN_KIT_ROOTS = 1
CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA[] = signed off
EOF
cat >"$SANDBOX/badere.knobs" <<'EOF'
CANON_KIT_SCAN_KIT_ROOTS = 1
CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA[] = (unclosed
EOF

run() {  # $1=knob file  $2..=extra env assignments; prints output, returns rc
    local kf="$1"; shift
    (cd "$SANDBOX" && gate_env GATE_SDK_KIT_DIRS=widget-toolkit CANON_KIT_KNOB_FILE="$kf" "$@" \
        && gate_run check-provenance-seam "$DIR/checks" 2>&1)
}

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=output  $5=rc
    if [[ "$5" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $5 -- $4"; fails=$((fails + 1)); return
    fi
    if [[ -n "$3" ]] && ! grep -qF -- "$3" <<<"$4"; then
        echo "  FAIL [$1]: exit $5 OK but output lacks '$3':"; printf '    %s\n' "$4"
        fails=$((fails + 1))
    fi
}

# The default leaves kit roots a dependency's: nothing is scanned, and the clean
# line says why rather than claiming a clean corpus.
out="$(run "$SANDBOX/off.knobs")"; rc=$?
check_case "kit-roots-off" 0 "CANON_KIT_SCAN_KIT_ROOTS is 0" "$out" "$rc"

# The bundled markers do not know "signed off", so only the slug trips — once, the
# fenced copy adding nothing.
out="$(run "$SANDBOX/on.knobs")"; rc=$?
check_case "slug-trips" 1 "widget-toolkit/SPEC.md:5  queue-slug: sprocket-rework-unit" "$out" "$rc"
if [[ "$(grep -c '  widget-toolkit/SPEC.md:' <<<"$out")" -ne 1 ]]; then
    echo "  FAIL [fence-skipped]: want exactly one finding, the fenced copies adding none:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

# The extra unions onto the bundled base: the consumer's marker now dates an
# attribution, and the base's slug finding still stands beside it.
out="$(run "$SANDBOX/extra.knobs")"; rc=$?
check_case "extra-union" 1 "widget-toolkit/SPEC.md:3  dated-attribution: 2026-08-12 with marker 'signed off'" "$out" "$rc"
check_case "extra-keeps-base" 1 "queue-slug: sprocket-rework-unit" "$out" "$rc"

# An absent queue file switches the slug arm off and says so; the other arms judge.
mv "$SANDBOX/TASK-QUEUE.md" "$SANDBOX/queue.off"
out="$(run "$SANDBOX/on.knobs")"; rc=$?
check_case "absent-queue" 0 "queue-slug arm off: no queue file" "$out" "$rc"
mv "$SANDBOX/queue.off" "$SANDBOX/TASK-QUEUE.md"

# Fail-closed: a marker that does not compile, and a slug floor that is not positive.
out="$(run "$SANDBOX/badere.knobs")"; rc=$?
check_case "malformed-marker" 2 "does not compile" "$out" "$rc"
out="$(run "$SANDBOX/on.knobs" CANON_KIT_SEAM_SLUG_MIN_LEN=0)"; rc=$?
check_case "zero-floor" 2 "CANON_KIT_SEAM_SLUG_MIN_LEN" "$out" "$rc"

if [[ "$fails" -gt 0 ]]; then
    echo "check-provenance-seam.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-provenance-seam.test.sh: clean (kit-roots-off default + fence skip + _EXTRA union + absent queue file + malformed marker + non-positive slug floor)"
exit 0
