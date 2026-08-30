#!/usr/bin/env bash
# spec: CLAUDE.md §Housekeeping — assemble the installer package out of tree and npm-pack it there; the payload is derived from the repo's own kit roots at pack time, so no second copy of any kit is ever checked in or written inside the worktree
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

usage() {
    printf 'usage: %s [--version <semver>] [--out <dir>] [--artifacts <dir>] [--root <dir>] [-h|--help]\n' \
        "${0##*/}"
    printf '  --version    default: the newest reachable git tag\n'
    printf '  --out        default: INSTALLER_PACK_TMP_DIR\n'
    printf '  --artifacts  <dir>/<target>/ holds a roster target'\''s binary + sidecar; omitted, none pack\n'
    printf '  --root       the work-tree top level packed and stamped; default: the git toplevel of the current directory\n'
}

VERSION=""
OUT=""
ARTIFACTS=""
ROOT=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --version) VERSION="${2:-}"; shift 2 ;;
        --out) OUT="${2:-}"; shift 2 ;;
        --artifacts) ARTIFACTS="${2:-}"; shift 2 ;;
        --root) ROOT="${2:-}"; shift 2 ;;
        # spec: installer/README.md §The consumer smoke — help is adopted on its own merits and does not extend gate-sdk/SPEC.md §The bin/-tool contract to a consumer's scripts/: with four flags the packer is past the point where an unknown-argument refusal is the only discovery route
        -h|--help) usage; exit 0 ;;
        *) echo "pack-installer: unknown argument: $1" >&2; exit 2 ;;
    esac
done

# spec: installer/README.md §The consumer smoke — a caller that already holds the tree it means says so, rather than letting the current directory select one it never named; the value is validated to a work-tree top level because silently promoting a subdirectory to its toplevel is the same silent correction this flag exists to remove
if [[ -n "$ROOT" ]]; then
    [[ -d "$ROOT" ]] || {
        echo "pack-installer: --root is not a directory: $ROOT" >&2
        exit 2
    }
    ROOT_TOP="$( { cd "$(git -C "$ROOT" rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )" || {
        echo "pack-installer: --root is not inside a git work tree: $ROOT" >&2
        exit 2
    }
    [[ "$(cd "$ROOT" && pwd -P)" == "$(cd "$ROOT_TOP" && pwd -P)" ]] || {
        echo "pack-installer: --root names a subdirectory of the work tree at $ROOT_TOP, not its top level: $ROOT" >&2
        echo "  help: pass the top level; promoting a subdirectory to it would pack a tree you did not name." >&2
        exit 2
    }
else
    ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )" || {
        echo "pack-installer: not inside a git work tree — the payload's commit stamp has no source." >&2
        exit 2
    }
fi
cd "$ROOT" || exit 2
ROOT="$(pwd -P)"

for tool in npm jq git tar; do
    command -v "$tool" >/dev/null 2>&1 || {
        echo "pack-installer: $tool not found on PATH — the pack step cannot run." >&2
        exit 2
    }
done

[[ -f installer/package.json ]] || {
    echo "pack-installer: installer/package.json not found — there is no package to pack." >&2
    exit 2
}

# spec: CLAUDE.md §Housekeeping — a dirty tree would stamp a commit that does not describe the payload, and that stamp is the whole of what makes a vendored tree resolvable to an upstream state
if [[ -n "$(git status --porcelain)" ]]; then
    echo "pack-installer: the worktree at $ROOT is dirty — refusing to stamp a commit the payload does not match." >&2
    echo "  help: this is checked once per invocation, against the tree as it is now — not as it was when your run started, so a concurrent edit during a long run trips it here rather than at the point you invoked the run." >&2
    echo "  help: commit or stash first; the stamp is what makes the vendoring auditable." >&2
    exit 2
fi

COMMIT="$(git rev-parse HEAD)"
[[ "$COMMIT" =~ ^[0-9a-f]{40}$ ]] || {
    echo "pack-installer: could not resolve HEAD to a 40-hex commit." >&2
    exit 2
}

if [[ -z "$VERSION" ]]; then
    VERSION="$(git describe --tags --abbrev=0 2>/dev/null)" || VERSION=""
    VERSION="${VERSION#v}"
fi
[[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$ ]] || {
    echo "pack-installer: no usable version — pass --version, or tag the commit being packed." >&2
    echo "  help: the version comes from the tag, never from an edit to installer/package.json." >&2
    exit 2
}

BASE="${INSTALLER_PACK_TMP_DIR:-${TMPDIR:-/tmp}}"
[[ -d "$BASE" ]] || { echo "pack-installer: scratch base not a directory: $BASE" >&2; exit 2; }
ASM="$(mktemp -d "$BASE/checkwright-pack.XXXXXX")" || exit 2
cleanup() { rm -rf "$ASM"; }
trap cleanup EXIT
OUT="${OUT:-$BASE}"
[[ -d "$OUT" ]] || { echo "pack-installer: output directory not found: $OUT" >&2; exit 2; }

# comment-tier-exempt: vendor git-tracked paths only, unconditionally — :71's clean-tree check does not see ignored paths (git status --porcelain omits them), so a gitignored artifact under a packed root (a fixture's own scratch dir, e.g.) rode a verbatim `cp -R` straight into the payload and broke a consumer's `init`. `git archive` at the stamped commit is the tracked-set boundary; --strip-components peels exactly SRC's own path depth so the archived prefix does not nest one level too deep in DST.
pack_tracked() {
    local src="${1%/}" dst="$2" depth=1 rest="${1%/}" links pl
    # spec: gate-sdk/SPEC.md §Consumer payload — refuse an unvendorable tracked symlink BEFORE the pipeline, never after it: the pipeline's status is tar's, so today's failure lands mid-kit having already written a partial vendor, where a pre-flight writes nothing and names the cause. On a native Windows host tar cannot create a dangling link — Windows picks the file-versus-directory kind from the target, and a target that does not exist has no kind to pick — so the payload must carry no symlink at all.
    links="$(git ls-files -s -- "$src" | grep '^120000 ' | cut -f2-)"
    if [[ -n "$links" ]]; then
        echo "pack-installer: $src carries tracked symlink(s), which the payload may not:" >&2
        while IFS= read -r pl; do printf '  %s\n' "$pl" >&2; done <<<"$links"
        echo "  help: the payload reproduces the tracked set with tar, and a host that cannot create a symlink aborts the extraction part-way through the kit." >&2
        echo "  help: remove it from the packed set; an assertion that needs one constructs it at run time in its own sandbox instead." >&2
        return 2
    fi
    while [[ "$rest" == */* ]]; do
        depth=$((depth + 1))
        rest="${rest%/*}"
    done
    mkdir -p "$dst" || return 1
    git archive "$COMMIT" -- "$src" | tar -x --strip-components="$depth" -C "$dst"
}

pack_tracked installer "$ASM" || exit 2

# spec: CLAUDE.md §Housekeeping — the payload's kit set is gate_kit_roots_rel, the same derivation the battery runs on, so the shipped set cannot drift from the governed one
mkdir -p "$ASM/payload" || exit 2
packed=0
while IFS= read -r kit; do
    kit="${kit%/}"
    [[ -n "$kit" && -d "$kit" ]] || continue
    pack_tracked "$kit" "$ASM/payload/${kit##*/}" || exit 2
    packed=$((packed + 1))
done < <(gate_kit_roots_rel)
[[ "$packed" -gt 0 ]] || { echo "pack-installer: no kit roots enumerated — the payload would be empty." >&2; exit 2; }

# spec: gate-sdk/SPEC.md §Consumer payload — the prebuilt gate binaries ride beside the kit roots, one directory per roster target, each with the digest sidecar its build leg emitted; the script never builds one, so a locally-built binary can never substitute for a released one
artifacts=0
if [[ -n "$ARTIFACTS" ]]; then
    [[ -d "$ARTIFACTS" ]] || { echo "pack-installer: artifact directory not found: $ARTIFACTS" >&2; exit 2; }
    ROSTER="$(gate_native_targets_file)"
    mapfile -t targets < <(gate_native_targets) || {
        echo "pack-installer: no target roster at $ROSTER — there is no declared platform set to pack artifacts for." >&2
        exit 2
    }
    [[ ${#targets[@]} -gt 0 ]] || { echo "pack-installer: the target roster at $ROSTER declares no targets." >&2; exit 2; }
    mkdir -p "$ASM/payload/artifact" || exit 2
    for target in "${targets[@]}"; do
        # spec: gate-sdk/SPEC.md §Consumer payload — the artifact name is derived per roster line from
        # that *target*'s executable suffix, never once from the host's: one payload carries every
        # target, so a host-derived name is correct only while every line is the host's platform class
        binary="$(gate_native_bin)"; binary="${binary##*/}"; binary="${binary%.exe}$(gate_exe_suffix "$target")"
        src="$ARTIFACTS/$target"
        [[ -d "$src" ]] || {
            echo "pack-installer: roster target '$target' has no artifact directory at $src." >&2
            echo "  help: every declared target's build leg must have run; a roster target no leg built is a broken payload, not a narrower one." >&2
            exit 2
        }
        [[ -f "$src/$binary" && -f "$src/$binary.sha256" ]] || {
            echo "pack-installer: $target is missing $binary or its .sha256 sidecar in $src." >&2
            exit 2
        }
        ( cd "$src" && sha256sum -c --status "$binary.sha256" ) || {
            echo "pack-installer: $target's sidecar does not match the binary beside it in $src." >&2
            echo "  help: the digest is emitted once by the build leg and only ever moved; a mismatch here means the bytes changed after it was written." >&2
            exit 2
        }
        mkdir -p "$ASM/payload/artifact/$target" || exit 2
        cp "$src/$binary" "$src/$binary.sha256" "$ASM/payload/artifact/$target/" || exit 2
        artifacts=$((artifacts + 1))
    done
    # spec: gate-sdk/SPEC.md §Consumer payload — the roster's one publication, copied verbatim, never regenerated or filtered
    cp "$ROSTER" "$ASM/payload/artifact/targets.list" || exit 2
fi

stamped="$(jq --arg v "$VERSION" --arg c "$COMMIT" \
    '.version = $v | .checkwright.commit = $c' "$ASM/package.json")" || {
    echo "pack-installer: could not stamp installer/package.json." >&2
    exit 2
}
printf '%s\n' "$stamped" > "$ASM/package.json" || exit 2

( cd "$ASM" && npm pack ) >/dev/null || {
    echo "pack-installer: npm pack failed in $ASM." >&2
    exit 2
}

shopt -s nullglob
tarballs=("$ASM"/*.tgz)
shopt -u nullglob
[[ ${#tarballs[@]} -eq 1 ]] || {
    echo "pack-installer: expected exactly one tarball in $ASM, found ${#tarballs[@]}." >&2
    exit 2
}
mv "${tarballs[0]}" "$OUT/" || exit 2

echo "PACK: ${OUT%/}/${tarballs[0]##*/} (version $VERSION, commit ${COMMIT:0:12}, root $ROOT, $packed kit(s) in payload, $artifacts prebuilt gate binary/binaries)"
exit 0
