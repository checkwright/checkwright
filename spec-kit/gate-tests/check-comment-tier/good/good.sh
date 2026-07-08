#!/usr/bin/env bash
# graph: couples=X dir=one valve=none tier=precommit
# spec: some/SPEC.md §thing — the header leads with directives
# usage: good.sh [--flag]   (usage: anchors its own window; this wrap is within cap)
set -uo pipefail

# spec: some/SPEC.md §helper — a mid-file directive blesses its own line plus
# its continuation wording up to the run cap, so this second line and the
# third line here both ride within the three-line window.
foo() { echo hi; }

x=1  # a trailing inline comment is out of scope by construction

# shellcheck disable=SC2034
unused=1

cat <<'EOF'
# this hash sits inside a heredoc body and is not a comment
# so the classifier must skip it entirely
EOF

# comment-tier-exempt: a genuinely unavoidable note with no owning section
bar() { echo bye; }
