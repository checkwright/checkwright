# shellcheck shell=bash
# spec: context-kit/SPEC.md §Index-first reading — the TypeScript public-surface extractor (grep-grade; export-declared items, .d.ts included as public by construction). Re-exports and multi-line declarations are stated honest limits, not parsed.
# shellcheck disable=SC2034  # PUB_LANG_GLOBS is read by pub-index.sh after it sources this file
PUB_LANG_GLOBS=("*.ts" "*.tsx")

pub_lang_extract() {   # $1 = file -> "kind name lineno" rows (unsorted)
    grep -n -E '^[[:space:]]*export[[:space:]]+' "$1" 2>/dev/null | awk '
    {
        if (! match($0, /^[0-9]+:/)) next
        lineno = substr($0, 1, RLENGTH - 1)
        rest = substr($0, RLENGTH + 1)
        sub(/^[ \t]*export[ \t]+/, "", rest)
        if (match(rest, /^default([ \t]|$)/)) {
            sub(/^default[ \t]*/, "", rest)
            sub(/^async[ \t]+/, "", rest)
            sub(/^(function|class)[ \t]+/, "", rest)
            if (match(rest, /^[A-Za-z_$][A-Za-z0-9_$]*/)) name = substr(rest, 1, RLENGTH)
            else name = "default"
            print "default " name " " lineno
            next
        }
        sub(/^async[ \t]+/, "", rest)
        if (! match(rest, /^(function|class|interface|type|enum|const|let|var)[ \t]/)) next
        kw = substr(rest, 1, RLENGTH - 1)
        rest = substr(rest, RLENGTH)
        sub(/^[ \t]+/, "", rest)
        if (kw == "const" && match(rest, /^enum[ \t]+/)) { kw = "enum"; sub(/^enum[ \t]+/, "", rest) }
        if (! match(rest, /^[A-Za-z_$][A-Za-z0-9_$]*/)) next
        name = substr(rest, 1, RLENGTH)
        print kw " " name " " lineno
    }' || true
}
