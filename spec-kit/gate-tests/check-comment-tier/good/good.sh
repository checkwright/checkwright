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

# spec: some/SPEC.md §bounds — a directive blesses at most three checks per run,
#   a bound rather than a total; three of the twelve gates below stay partitive,
#   and the inline-code `six gates` reads as a meta-reference.
tally() { echo n; }

# spec: some/SPEC.md §wrapbound — a bound survives the join: at most three
#   gates run per suite, the comparator carrying across the wrap.
gauge() { echo m; }
