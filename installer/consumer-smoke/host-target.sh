#!/bin/sh
# spec: installer/SPEC.md §The consumer smoke — prints the triple the bootstrap's own detector maps this host to, holding no mapping of its own
# no-port: installer/SPEC.md §The consumer smoke, The port disposition — the smoke's own helper, on run-smoke.sh's ground: the payload carries nothing under installer/consumer-smoke/, so no adopter path runs it.
set -u

bootstrap="$(cd "$(dirname "$0")/../bin" 2>/dev/null && pwd)/checkwright.sh"

# spec: installer/SPEC.md §The gate binary — the extraction shape check-install-platforms pins: the opening line to the first closing brace at column 0, and an unbounded body is no body
extract() {
    awk -v f="$1" '
        !inb && index($0, f "() {") == 1 { inb = 1 }
        inb { print }
        inb && /^}/ { done = 1; exit }
        END { if (!done) exit 1 }
    ' "$bootstrap"
}

for fn in host_arch host_shape target_of_host; do
    body="$(extract "$fn")" || {
        printf 'host-target: %s carries no %s() bounded by a closing brace at column 0\n' "$bootstrap" "$fn" >&2
        exit 2
    }
    eval "$body"
done

answer="$(target_of_host)"
[ -n "$answer" ] || {
    printf 'host-target: the bootstrap maps this host, detected as %s, to no triple\n' "$(host_shape)" >&2
    exit 2
}
printf '%s\n' "$answer"
