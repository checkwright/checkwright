# shellcheck shell=bash
# spec: context-kit/SPEC.md §Index-first reading — the Rust public-item extractor (grep-grade; the shipped default). Defines PUB_LANG_GLOBS + pub_lang_extract; the dispatcher owns traversal, sort, and format.
# shellcheck disable=SC2034  # PUB_LANG_GLOBS is read by pub-index.sh after it sources this file
PUB_LANG_GLOBS=("*.rs")

pub_lang_extract() {   # $1 = file -> "kind name lineno" rows (unsorted)
    grep -n -E \
        '^\s*(pub|pub\([^)]*\))\s+(async\s+)?(fn|struct|enum|trait|type|const|static|mod)\s+[A-Za-z_]' \
        "$1" 2>/dev/null | awk '
    {
        line = $0
        if (! match(line, /^[0-9]+:/)) next
        lineno = substr(line, 1, RLENGTH - 1)
        rest = substr(line, RLENGTH + 1)
        sub(/^[ \t]*/, "", rest)
        if (rest !~ /^pub([ \t]|\()/) next
        sub(/^pub(\([^)]*\))?[ \t]+/, "", rest)
        sub(/^async[ \t]+/, "", rest)
        if (! match(rest, /^(fn|struct|enum|trait|type|const|static|mod)[ \t]+/)) next
        kw = substr(rest, 1, RLENGTH); sub(/[ \t]+$/, "", kw)
        rest = substr(rest, RLENGTH + 1)
        if (! match(rest, /^[A-Za-z_][A-Za-z0-9_]*/)) next
        name = substr(rest, 1, RLENGTH)
        print kw " " name " " lineno
    }' || true
}
