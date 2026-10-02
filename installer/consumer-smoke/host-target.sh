#!/bin/sh
# spec: installer/SPEC.md §The consumer smoke — prints the triple the bootstrap's own detector maps this host to, or with `--fallback <triple>` the triple the bootstrap's fallback map names for it, holding no mapping of its own
# no-port: installer/SPEC.md §The consumer smoke — the unix legs' and the --installer-smoke arm's extraction of the POSIX bootstrap's detector; it ships in no payload, so no adopter path runs it.
set -u

bootstrap="$(cd "$(dirname "$0")/../bin" 2>/dev/null && pwd)/checkwright.sh"

# spec: installer/SPEC.md §Platform resolution — the extraction shape check-install-platforms pins: the opening line to the first closing brace at column 0, and an unbounded body is no body
extract() {
    awk -v f="$1" '
        !inb && index($0, f "() {") == 1 { inb = 1 }
        inb { print }
        inb && /^}/ { done = 1; exit }
        END { if (!done) exit 1 }
    ' "$bootstrap"
}

for fn in host_arch host_shape target_of_host fallback_of_target; do
    body="$(extract "$fn")" || {
        printf 'host-target: %s carries no %s() bounded by a closing brace at column 0\n' "$bootstrap" "$fn" >&2
        exit 2
    }
    eval "$body"
done

if [ "${1:-}" = --fallback ]; then
    [ $# -eq 2 ] || { printf 'host-target: usage: host-target.sh [--fallback <triple>]\n' >&2; exit 2; }
    fallback_of_target "$2"
    printf '\n'
    exit 0
fi

answer="$(target_of_host)"
[ -n "$answer" ] || {
    printf 'host-target: the bootstrap maps this host, detected as %s, to no triple\n' "$(host_shape)" >&2
    exit 2
}
printf '%s\n' "$answer"
