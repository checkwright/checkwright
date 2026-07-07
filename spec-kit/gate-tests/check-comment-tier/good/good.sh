#!/usr/bin/env bash
# graph: couples=X dir=one valve=none tier=precommit
# spec: some/SPEC.md §thing — the header block leads with directives
#
# usage: this prose rides the run the spec: directive opened, so it is
#   blessed without carrying a directive of its own.
set -uo pipefail

# spec: some/SPEC.md §helper — a mid-file block that leads with a directive
# blesses its own continuation wording (the reason's run).
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
