#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*.rs,*.toml,*.sql,*.rego,*.ts,*.tsx,*.yaml,*.yml,*.proto,*.sh,Dockerfile dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-spec-embedded-source — a canonical spec's fenced block must not verbatim-copy a tracked source file
#
# usage: check-spec-embedded-source.sh [scan-root]
#   Scans canonical specs + amendments under the root (default '.') against the
#   tracked source files whose kind matches each fence's language.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-spec-embedded-source: scan root not found: $ROOT" >&2; exit 2; }

# Build the language↔kind↔file maps from SPEC_KIT_EMBED_LANGS.
declare -A LANG2KIND EXT2KIND BASE2KIND
find_globs=()
for entry in "${SPEC_KIT_EMBED_LANGS[@]}"; do
    IFS='|' read -r kind aliases globs <<<"$entry"
    IFS=',' read -ra al <<<"$aliases"
    for a in "${al[@]}"; do [[ -n "$a" ]] && LANG2KIND["$a"]="$kind"; done
    IFS=',' read -ra gl <<<"$globs"
    for g in "${gl[@]}"; do
        [[ -n "$g" ]] || continue
        find_globs+=("$g")
        if [[ "$g" == \*.* ]]; then EXT2KIND["${g#*.}"]="$kind"; else BASE2KIND["$g"]="$kind"; fi
    done
done
for l in "${SPEC_KIT_EMBED_ILLUSTRATIVE[@]}"; do unset "LANG2KIND[$l]"; done

langmap=""; for k in "${!LANG2KIND[@]}"; do langmap+="$k=${LANG2KIND[$k]} "; done
extmap="";  for k in "${!EXT2KIND[@]}";  do extmap+="$k=${EXT2KIND[$k]} ";  done
basemap=""; for k in "${!BASE2KIND[@]}"; do basemap+="$k=${BASE2KIND[$k]} "; done

find_expr=(); first=1
for g in "${find_globs[@]}"; do
    if (( first )); then find_expr+=( -name "$g" ); first=0
    else find_expr+=( -o -name "$g" ); fi
done
mapfile -t candidates < <(gate_find "$ROOT" \( "${find_expr[@]}" \) -type f 2>/dev/null | sed 's#^\./##' | sort -u)
mapfile -t amendfiles < <(spec_amendments "$ROOT" | sed 's#^\./##' | sort -u)
mapfile -t specs < <( { spec_canonical_specs "$ROOT"; printf '%s\n' "${amendfiles[@]+"${amendfiles[@]}"}"; } | sed 's#^\./##' | grep -v '^$' | sort -u )

[[ ${#specs[@]} -eq 0 ]] && { echo "SPEC-EMBEDDED-SOURCE: clean (0 spec files found)"; exit 0; }

# The amendment set (newline-joined) marks the wire-delta exemption in awk.
amendset=""; for a in "${amendfiles[@]+"${amendfiles[@]}"}"; do amendset+="$a"$'\n'; done

flagged="$(awk -v threshold="$SPEC_KIT_EMBED_THRESHOLD" -v minlines="$SPEC_KIT_EMBED_MINLINES" \
               -v langmap="$langmap" -v extmap="$extmap" -v basemap="$basemap" \
               -v amendset="$amendset" -v wirekind="$SPEC_KIT_EMBED_WIRE_KIND" '
    function trim(s) { gsub(/^[[:space:]]+|[[:space:]]+$/, "", s); return s }
    function trivial(s) {
        return (s == "" || s == "{" || s == "}" || s == "(" || s == ")" || \
                s == "..." || s == "};" || s == "});" || s == "//" || s == "#")
    }
    function file_kind(fn,   b, e) {
        b = fn; sub(/.*\//, "", b)
        if (b in basekind) return basekind[b]
        e = b; sub(/.*\./, "", e)
        return (e in extkind) ? extkind[e] : ""
    }
    function lang_kind(l) { return (l in langkind) ? langkind[l] : "" }
    function emit_block(   l, parts, i, f, best, bestf, frac, nd) {
        if (blkkind == "" || nb < minlines) return
        # spec: spec-kit/SPEC.md §check-spec-embedded-source — amendment wire-delta exemption
        if (blkkind == wirekind && is_amendment) return
        nd = 0; for (l in bl) nd++
        if (nd < minlines) return
        delete hits
        for (l in bl) {
            if (!(l in idx)) continue
            split(idx[l], parts, "\x01")
            for (i in parts) if (parts[i] != "" && fkind[parts[i]] == blkkind) hits[parts[i]]++
        }
        best = 0; bestf = ""
        for (f in hits) { frac = hits[f] / nd; if (frac > best) { best = frac; bestf = f } }
        if (best + 0.0 >= threshold + 0.0)
            printf "  %s:%d  [%s] ~%d%% of %d lines copied from %s\n", \
                   FILENAME, blkstart, blklang, int(best * 100 + 0.5), nd, bestf
    }
    BEGIN {
        n = split(langmap, a, " "); for (i = 1; i <= n; i++) if (a[i] != "") { split(a[i], kv, "="); langkind[kv[1]] = kv[2] }
        n = split(extmap, a, " ");  for (i = 1; i <= n; i++) if (a[i] != "") { split(a[i], kv, "="); extkind[kv[1]]  = kv[2] }
        n = split(basemap, a, " "); for (i = 1; i <= n; i++) if (a[i] != "") { split(a[i], kv, "="); basekind[kv[1]] = kv[2] }
        n = split(amendset, a, "\n"); for (i = 1; i <= n; i++) if (a[i] != "") amend[a[i]] = 1
    }
    FNR == 1 {
        is_spec = (FILENAME ~ /SPEC[^/]*\.md$/)
        is_amendment = (FILENAME in amend)
        infence = 0; lastnb = ""
    }
    !is_spec {
        s = trim($0)
        if (trivial(s)) next
        if (!(s SUBSEP FILENAME in seen)) { seen[s SUBSEP FILENAME] = 1; idx[s] = idx[s] "\x01" FILENAME }
        if (!(FILENAME in fkind)) fkind[FILENAME] = file_kind(FILENAME)
        next
    }
    {
        if (!infence) {
            if ($0 ~ /^[[:space:]]*```[[:space:]]*[A-Za-z+]*[[:space:]]*$/) {
                lang = $0; sub(/^[[:space:]]*```[[:space:]]*/, "", lang); lang = tolower(trim(lang))
                skipblock = (lastnb ~ /spec-embedded-source-exempt:/) ? 1 : 0
                infence = 1; blklang = lang; blkkind = lang_kind(lang)
                blkstart = FNR; nb = 0; delete bl
                next
            }
            if (trim($0) != "") lastnb = $0
            next
        }
        if ($0 ~ /^[[:space:]]*```[[:space:]]*$/) {
            if (!skipblock) emit_block()
            infence = 0; lastnb = ""
            next
        }
        s = trim($0)
        if (trivial(s)) next
        nb++; bl[s] = 1
    }
' "${candidates[@]+"${candidates[@]}"}" "${specs[@]}")"; st=$?
fail_closed "$st" check-spec-embedded-source awk

if [[ -n "$flagged" ]]; then
    echo "check-spec-embedded-source: spec fenced block(s) verbatim-copy a tracked source"
    echo "file (the file is the home of the body); cite the path instead, or bless a"
    echo "genuinely-illustrative block:"
    echo ""
    echo "$flagged"
    echo "  help: replace the embedded body with a path reference to the cited source file, or, if the block is a genuine illustrative shape/schema example, add '<!-- spec-embedded-source-exempt: <reason> -->' on the line directly above the opening fence"
    exit 1
fi
echo "SPEC-EMBEDDED-SOURCE: clean (${#specs[@]} spec file(s) scanned; no fenced block copies a tracked source)"
exit 0
