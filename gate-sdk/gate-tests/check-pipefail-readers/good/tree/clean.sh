#!/usr/bin/env bash
# Early-exit readers before the set, a guarded producer, a waived site, full readers, and
# early-exit text inside quotes, a here-doc and a case pattern.
ls | head -1
set -euo pipefail
{ ls || true; } | head -1
ls | grep -q x # pipefail-ok: the listing is one directory, read whole before grep exits
ls | grep -c x
ls | sed -n '1,3p'
ls | awk '{ print $1 }'
echo "a | head -1"
cat <<'EOF'
ls | grep -q x
EOF
case "$1" in
    a|b) ls | sort ;;
esac
