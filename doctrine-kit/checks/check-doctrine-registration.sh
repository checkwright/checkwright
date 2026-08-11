#!/usr/bin/env bash
# graph: couples=CLAUDE.md,doctrine-kit/DOCTRINE.md dir=one valve=none tier=precommit
# install: zero-config
# spec: doctrine-kit/SPEC.md §check-doctrine-registration — the always-loaded agent file links the doctrine file and holds its methodology-rule digest in per-rule lockstep with DOCTRINE.md, fail-closed when a scanned file or section is missing
#
# usage: check-doctrine-registration.sh [agent-file [doctrine-file]]
#   paths resolve relative to cwd (= repo root in a battery run); defaults come
#   from DOCTRINE_KIT_AGENT_FILE / DOCTRINE_KIT_DOCTRINE_FILE (doctrine-kit/lib/doctrine.sh).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"

AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "check-doctrine-registration: agent file not found: $AGENT_FILE" >&2; exit 2; }  # exit 2: fail-closed

# spec: doctrine-kit/SPEC.md §check-doctrine-registration — the doctrine-side section headings are kit mechanism (the kit ships DOCTRINE.md), never config
METH_SECTION="## Methodology-maintenance rules"
CRAFT_SECTION="## Engineering-craft rules"

grep -qF -- "]($DOCTRINE_FILE" "$AGENT_FILE"; st=$?
if [[ "$st" -ne 0 ]]; then
    [[ "$st" -eq 1 ]] || fail_closed "$st" check-doctrine-registration grep
    echo "check-doctrine-registration: $AGENT_FILE carries no markdown link to the doctrine file:"
    echo "  $DOCTRINE_FILE"
    echo "  help: install the doctrine reference block into the always-loaded agent file —"
    echo "        bash doctrine-kit/bin/install-doctrine.sh — so a session that loads it"
    echo "        follows the link to the delivery doctrine. Override the paths with"
    echo "        DOCTRINE_KIT_AGENT_FILE / DOCTRINE_KIT_DOCTRINE_FILE."
    exit 1
fi

[[ -f "$DOCTRINE_FILE" ]] \
    || { echo "check-doctrine-registration: doctrine file not found: $DOCTRINE_FILE" >&2; exit 2; }  # exit 2: fail-closed

read -r -d '' SECTION_WALK <<'AWK' || true
function hlevel(line,   n) {
    if (line !~ /^#+[[:space:]]/) return 0
    n = 0
    while (substr(line, n + 1, 1) == "#") n++
    return n
}
# Enter the governed section: heading line whose text starts with the knob.
!insec {
    if (hlevel($0) > 0 && substr($0, 1, length(section)) == section) {
        insec = 1; seen = 1; start_lvl = hlevel($0)
    }
    next
}
# A heading at the same level or higher closes the section.
insec && hlevel($0) > 0 && hlevel($0) <= start_lvl { insec = 0; next }
insec {
    if (mode == "doctrine") {
        # Numbered rule lead-in: bold name, trailing in-bold period dropped.
        if ($0 ~ /^[0-9]+\.[[:space:]]+\*\*/) {
            name = $0
            sub(/^[0-9]+\.[[:space:]]+\*\*/, "", name)
            sub(/\*\*.*/, "", name)
            sub(/\.$/, "", name)
            print "R\t" name
        }
    } else {
        # Digest bullet lead-in.
        if ($0 ~ /^- \*\*/) {
            name = $0
            sub(/^- \*\*/, "", name)
            sub(/\*\*.*/, "", name)
            print "D\t" name
        # Declared trim: <!-- doctrine-digest-trim: <rule name> — <reason> -->
        } else if ($0 ~ /doctrine-digest-trim:/) {
            name = $0
            sub(/^.*doctrine-digest-trim:[[:space:]]*/, "", name)
            sub(/[[:space:]]*—.*$/, "", name)
            print "T\t" name
        }
    }
    next
}
END { if (!seen) print "@@NOSECTION" }
AWK

# spec: doctrine-kit/SPEC.md §check-doctrine-registration — assertions D and E share one walk over a register's numbered rules, counting a named per-rule trailer and grading its value: D is `Stages` over the craft register, E is `Digest` over the methodology register
read -r -d '' TRAILER_WALK <<'AWK' || true
function hlevel(line,   n) {
    if (line !~ /^#+[[:space:]]/) return 0
    n = 0
    while (substr(line, n + 1, 1) == "#") n++
    return n
}
function flush() {
    if (have) print "C\t" cur_name "\t" tcount "\t" bad
}
BEGIN {
    mark = "^[[:space:]]*\\*" trailer ":\\*"
    strip = mark "[[:space:]]*"
}
!insec {
    if (hlevel($0) > 0 && substr($0, 1, length(section)) == section) {
        insec = 1; seen = 1; start_lvl = hlevel($0)
    }
    next
}
insec && hlevel($0) > 0 && hlevel($0) <= start_lvl { flush(); insec = 0; have = 0; next }
insec {
    if ($0 ~ /^[0-9]+\.[[:space:]]+\*\*/) {
        flush()
        name = $0
        sub(/^[0-9]+\.[[:space:]]+\*\*/, "", name)
        sub(/\*\*.*/, "", name)
        sub(/\.$/, "", name)
        cur_name = name; have = 1; tcount = 0; bad = 0
    } else if ($0 ~ mark) {
        tcount++
        val = $0
        sub(strip, "", val)
        sub(/[[:space:]]+$/, "", val)
        if (grammar == "stages") {
            if (val != "—" && val !~ /^[a-z]+(,[[:space:]]+[a-z]+)*$/) bad = 1
        } else if (val == "") bad = 1
    }
    next
}
END { flush(); if (!seen) print "@@NOSECTION" }
AWK

doctrine_out="$(awk -v section="$METH_SECTION" -v mode=doctrine "$SECTION_WALK" "$DOCTRINE_FILE")"; st=$?
fail_closed "$st" check-doctrine-registration awk
if [[ "$doctrine_out" == "@@NOSECTION" ]]; then
    echo "check-doctrine-registration: no '$METH_SECTION' section in $DOCTRINE_FILE — cannot certify the digest against an unreadable rule set" >&2
    echo "  help: the methodology-maintenance section heading is kit mechanism; restore it in the doctrine file" >&2
    exit 2  # exit 2: fail-closed
fi

digest_out="$(awk -v section="$DOCTRINE_KIT_DIGEST_SECTION" -v mode=digest "$SECTION_WALK" "$AGENT_FILE")"; st=$?
fail_closed "$st" check-doctrine-registration awk
if [[ "$digest_out" == "@@NOSECTION" ]]; then
    printf 'check-doctrine-registration: no heading matches %s in %s: %s\n' \
        DOCTRINE_KIT_DIGEST_SECTION "$AGENT_FILE" "'$DOCTRINE_KIT_DIGEST_SECTION'" >&2
    echo "  help: a renamed or deleted digest section silently disarms this gate — repoint DOCTRINE_KIT_DIGEST_SECTION at the live heading, or restore the heading it names" >&2
    exit 2  # exit 2: fail-closed
fi

craft_out="$(awk -v section="$CRAFT_SECTION" -v trailer=Stages -v grammar=stages "$TRAILER_WALK" "$DOCTRINE_FILE")"; st=$?
fail_closed "$st" check-doctrine-registration awk
if [[ "$craft_out" == "@@NOSECTION" ]]; then
    echo "check-doctrine-registration: no '$CRAFT_SECTION' section in $DOCTRINE_FILE — cannot certify the stage-routing trailers against an unreadable rule set" >&2
    echo "  help: the engineering-craft section heading is kit mechanism; restore it in the doctrine file" >&2
    exit 2  # exit 2: fail-closed
fi

craft_count=0
craft_findings=()
while IFS=$'\t' read -r kind name scount bad; do
    [[ "$kind" == C ]] || continue
    craft_count=$((craft_count + 1))
    if [[ "$scount" -ne 1 ]]; then
        craft_findings+=("craft rule carries $scount *Stages:* trailer(s), want exactly one: $name")
    elif [[ "$bad" == 1 ]]; then
        craft_findings+=("craft rule's *Stages:* value is malformed (want a comma list of lowercase stages, or —): $name")
    fi
done <<< "$craft_out"

# spec: doctrine-kit/SPEC.md §check-doctrine-registration — assertion E: every methodology rule carries exactly one non-empty *Digest:* trailer, the surface install-doctrine derives its bullet from
meth_out="$(awk -v section="$METH_SECTION" -v trailer=Digest -v grammar=digest "$TRAILER_WALK" "$DOCTRINE_FILE")"; st=$?
fail_closed "$st" check-doctrine-registration awk

digest_findings=()
while IFS=$'\t' read -r kind name dcount bad; do
    [[ "$kind" == C ]] || continue
    if [[ "$dcount" -ne 1 ]]; then
        digest_findings+=("methodology rule carries $dcount *Digest:* trailer(s), want exactly one: $name")
    elif [[ "$bad" == 1 ]]; then
        digest_findings+=("methodology rule's *Digest:* value is empty: $name")
    fi
done <<< "$meth_out"

declare -A in_digest in_trim in_doctrine
digest_names=()
trim_count=0
while IFS=$'\t' read -r kind name; do
    [[ -n "$name" ]] || continue
    case "$kind" in
        D) digest_names+=("$name"); in_digest["$name"]=1 ;;
        T) in_trim["$name"]=1; trim_count=$((trim_count + 1)) ;;
    esac
done <<< "$digest_out"

rule_count=0
missing=()
while IFS=$'\t' read -r _kind name; do
    [[ -n "$name" ]] || continue
    rule_count=$((rule_count + 1))
    in_doctrine["$name"]=1
    if [[ -z "${in_digest[$name]:-}" && -z "${in_trim[$name]:-}" ]]; then
        missing+=("$name")
    fi
done <<< "$doctrine_out"

orphans=()
for name in "${digest_names[@]+"${digest_names[@]}"}"; do
    [[ -z "${in_doctrine[$name]:-}" ]] && orphans+=("$name")
done

if [[ ${#missing[@]} -gt 0 || ${#orphans[@]} -gt 0 || ${#craft_findings[@]} -gt 0 || ${#digest_findings[@]} -gt 0 ]]; then
    if [[ ${#missing[@]} -gt 0 || ${#orphans[@]} -gt 0 ]]; then
        echo "check-doctrine-registration: the digest and the doctrine are out of lockstep in $AGENT_FILE:"
        for name in "${missing[@]+"${missing[@]}"}"; do
            echo "  doctrine rule absent from the digest: $name"
        done
        for name in "${orphans[@]+"${orphans[@]}"}"; do
            echo "  digest bullet owns no doctrine rule: $name"
        done
        echo "  help: a re-vendored DOCTRINE.md changed the methodology-rule set — reconcile the"
        echo "        '$DOCTRINE_KIT_DIGEST_SECTION' digest in $AGENT_FILE: add '- **<name>** — …' for"
        echo "        each absent rule (or declare a trim '<!-- doctrine-digest-trim: <name> — <reason> -->'),"
        echo "        and rename or remove any bullet that owns no rule."
    fi
    if [[ ${#craft_findings[@]} -gt 0 ]]; then
        echo "check-doctrine-registration: the stage-routing trailers are out of grammar in $DOCTRINE_FILE:"
        for f in "${craft_findings[@]}"; do
            echo "  $f"
        done
        echo "  help: every engineering-craft rule owns exactly one '*Stages:* <stage>[, <stage>…]'"
        echo "        trailer ('*Stages:* —' for none) so a re-vendored DOCTRINE.md that adds an"
        echo "        untagged craft rule reddens instead of silently dropping out of stage routing."
    fi
    if [[ ${#digest_findings[@]} -gt 0 ]]; then
        echo "check-doctrine-registration: the digest trailers are out of grammar in $DOCTRINE_FILE:"
        for f in "${digest_findings[@]}"; do
            echo "  $f"
        done
        echo "  help: every methodology rule owns exactly one non-empty '*Digest:* <one-line summary>'"
        echo "        trailer — install-doctrine.sh derives that rule's digest bullet from it, so an"
        echo "        untrailered rule would ship every consumer a digest one rule short."
    fi
    exit 1
fi

echo "DOCTRINE-REGISTRATION: clean ($AGENT_FILE links $DOCTRINE_FILE; $rule_count methodology rule(s) in per-rule digest lockstep, $trim_count declared trim(s), each carrying one *Digest:* trailer; $craft_count craft rule(s) each carry one *Stages:* trailer)"
exit 0
