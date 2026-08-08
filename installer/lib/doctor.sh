#!/usr/bin/env bash
# spec: installer/README.md §doctor — renders the toolchain floor as an exit status so init and a CI step can gate on the verdict without parsing a report, and reads the payload's own copy of the roster because at init time nothing is vendored in the consumer's tree yet
#
# usage: checkwright doctor
#   No --dry-run: doctor writes nothing, so it has no mutating form to guard.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FLOOR="$INSTALLER/payload/context-kit/lib/toolfloor.sh"

case "${1:-}" in
    '') ;;
    -h|--help)
        printf 'usage: checkwright doctor\n\n'
        printf 'Reports whether this machine meets the toolchain contract, and — when run\n'
        printf 'inside a repository that has been vendored into — what is installed there.\n'
        printf 'Exit status is the verdict: 0 meets the contract, 1 below it.\n'
        exit 0 ;;
    *) printf 'checkwright doctor: unknown argument: %s\n' "$1" >&2; exit 2 ;;
esac

[[ -f "$FLOOR" ]] || {
    printf 'checkwright doctor: no toolchain roster in this package (%s)\n' "${FLOOR#"$INSTALLER"/}" >&2
    printf '  help: doctor reads the roster from the package payload, which is assembled at pack time — run it from an installed package, not from a source checkout.\n' >&2
    exit 2
}
# shellcheck source=/dev/null  # payload path, assembled at pack time
source "$FLOOR"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"
# shellcheck source=./common/digest.sh
source "$INSTALLER/lib/common/digest.sh"

# spec: context-kit/SPEC.md §bin/env-probe — both version probes read from /dev/null, and `-V` is only the fallback: a tool rejecting `--version` would otherwise reach a `-V` that reads inherited stdin and hangs
probe_banner() {   # $1 = tool -> its raw version banner, empty when the tool is absent
    local raw
    command -v "$1" >/dev/null 2>&1 || return 0
    raw="$("$1" --version 2>/dev/null </dev/null)"
    [[ -n "$raw" ]] || raw="$("$1" -V 2>/dev/null </dev/null)"
    [[ -n "$raw" ]] || raw="present"
    printf '%s' "$raw"
}

# spec: installer/README.md §doctor — doctor defines no floor of its own: it renders whatever verdict the payload roster's own predicate returns, so the contract has one owner and this is a display of it
render() {   # $1 = tool, $2 = verdict words -> one report line; sets FAILED when the member is not clean
    local tool="$1" kind found floor
    read -r kind found floor <<<"$2"
    case "$kind" in
        ok)           printf '  %-12s %s\n' "$tool" "${FOUND_VERSION:-present}" ;;
        absent)       printf '  %-12s %s\n' "$tool" "NOT FOUND"; FAILED=1 ;;
        below)        printf '  %-12s %s (below the floor of %s)\n' "$tool" "$found" "$floor"; FAILED=1 ;;
        wrong-impl)   printf '  %-12s %s (not the %s implementation the contract requires)\n' "$tool" "${FOUND_VERSION:-$found}" "$TOOL_FLOOR_IMPL"; FAILED=1 ;;
        *)            printf '  %-12s could not be compared against the floor of %s\n' "$tool" "$TOOL_FLOOR_MIN"; FAILED=1 ;;
    esac
}

FAILED=0
ARTIFACT_FINDING=""
printf 'toolchain\n'
for elem in "${PROBE_SET[@]}"; do
    tool_floor_parse "$elem"
    banner="$(probe_banner "$TOOL_FLOOR_NAME")"
    FOUND_VERSION="$(tool_floor_version "$banner")"
    render "$TOOL_FLOOR_NAME" "$(tool_floor_check "$elem" "$banner")"
done

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || ROOT=""
[[ -n "$ROOT" ]] || ROOT="$PWD"
LOCK="$(lock_path "$ROOT")"

if [[ ! -f "$LOCK" ]]; then
    printf '\nNo %s here — nothing has been vendored into this directory.\n' "$CHECKWRIGHT_LOCK_FILE"
elif ! command -v jq >/dev/null 2>&1; then
    printf '\nFound %s, but jq is absent, so it cannot be read.\n' "$CHECKWRIGHT_LOCK_FILE"
elif ! lock_schema_ok "$LOCK"; then
    printf 'checkwright doctor: %s carries a schema this build does not know.\n' "$LOCK" >&2
    printf '  help: this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.\n' >&2
    exit 2
else
    version="$(lock_field "$LOCK" version)"
    # spec: installer/README.md §doctor — a manifest carrying `files` and no `version` is a residue rather than an install: `version` is the field an install always has and a residue never does, so its absence is the discriminator. This guards the whole installed block rather than sitting beside it — the identity lines, the artifact check and the omitted-gates block are per-install readings with nothing to compute once there is no install, and printing them beside the residue message would be the same mixed-verdict shape doctor's own exit-status carve-out already refuses
    if [[ -z "$version" ]]; then
        n="$(jq -r '.files | length' "$LOCK" 2>/dev/null)"; [[ -n "$n" ]] || n=0
        printf '\nno install here — %d file(s) remain that a previous install wrote and you have since edited. They are yours, and a future init will still protect them.\n' \
            "$n"
    else
        printf '\ninstalled\n'
        printf '  %-12s %s\n' version "$version"
        printf '  %-12s %s\n' commit "$(lock_field "$LOCK" commit)"
        printf '  %-12s %s\n' profile "$(lock_field "$LOCK" profile)"
        printf '  %-12s %s\n' kits "$(lock_field "$LOCK" kits)"

        # spec: installer/README.md §The gate binary — the recorded digest's second reader: re-verifying the binary in place is the only thing standing between a consumer and one swapped after install, and the path is resolved from the knob that owns it rather than from a copy the manifest would otherwise have to store
        artifact_target="$(jq -r '.artifact.target // ""' "$LOCK" 2>/dev/null)"
        artifact_digest="$(jq -r '.artifact.digest // ""' "$LOCK" 2>/dev/null)"
        if [[ -n "$artifact_target" ]]; then
            seam="$(lock_own_file "$LOCK" /gate-sdk-config.sh)"
            bin=""
            [[ -n "$seam" && -f "$ROOT/$seam" ]] \
                && bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$ROOT/$seam" | head -n1)"
            # spec: installer/README.md §doctor — an artifact finding reports without setting the verdict, and that is deliberate rather than lenient: the exit status is the toolchain contract init gates on, so failing it here would block the re-run that is this finding's own remedy
            if [[ -z "$bin" || ! -f "$ROOT/$bin" ]]; then
                printf '  %-12s %s — recorded, but nothing at the path GATE_SDK_NATIVE_BIN names; re-run init\n' \
                    artifact "$artifact_target"
                ARTIFACT_FINDING="the recorded gate binary is not on disk"
            elif [[ "$(digest_of "$ROOT/$bin")" == "$artifact_digest" ]]; then
                printf '  %-12s %s (verified in place)\n' artifact "$artifact_target"
            else
                printf '  %-12s %s — DIGEST MISMATCH, %s differs from what init wrote; re-run init\n' \
                    artifact "$artifact_target" "$bin"
                ARTIFACT_FINDING="the installed gate binary does not match its recorded digest"
            fi
        fi

        # spec: installer/README.md §The gate binary — the omitted-member record's second reader, reported against the reason that caused it because a remedy is what an adopter comes here for
        list="$(lock_own_file "$LOCK" /gates.list)"
        if [[ -n "$list" && -f "$ROOT/$list" ]]; then
            while read -r count reason; do
                [[ -n "$reason" ]] || continue
                case "$reason" in
                    substrate-unavailable)
                        printf '  %-12s %d gate(s), %s — no prebuilt binary is published for this platform\n' \
                            omitted "$count" "$reason" ;;
                    digest-unverifiable)
                        printf '  %-12s %d gate(s), %s — install sha256sum or shasum, then re-run init\n' \
                            omitted "$count" "$reason" ;;
                    *)
                        printf '  %-12s %d gate(s), %s\n' omitted "$count" "$reason" ;;
                esac
            done < <(awk '$1 == "#" && $2 == "omitted:" { print $4 }' "$ROOT/$list" | sort | uniq -c)
        fi
    fi

    # spec: installer/README.md §doctor — doctor's own report stops at the toolchain and identity fields; per-file divergence is a separate question with a separate verb, on both the install and the residue path, so DOCTOR: clean is never read as a claim about the tree's contents
    printf '\nrun checkwright diff to see which files, if any, have changed since.\n'
fi

if (( FAILED )); then
    printf '\nDOCTOR: below contract\n'
    printf '  help: install or upgrade each tool reported above; the floors are the ones the gate battery needs to run, not preferences.\n'
    exit 1
fi
# spec: installer/README.md §doctor — the verdict line names an artifact finding rather than swallowing it: the exit status stays the toolchain contract, but a run that reported a digest mismatch must not sign off as plainly clean
if [[ -n "$ARTIFACT_FINDING" ]]; then
    printf '\nDOCTOR: toolchain clean, 1 artifact finding — %s\n' "$ARTIFACT_FINDING"
    printf '  help: re-run init; it re-verifies the published digest and rewrites the binary.\n'
    exit 0
fi
printf '\nDOCTOR: clean\n'
exit 0
