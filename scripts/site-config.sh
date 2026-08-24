# shellcheck shell=bash
# spec: site-kit/SPEC.md §Layout and configuration — this repo's site-kit consumer config
# no-port: CLAUDE.md §The provenance seam (never cross it) — operator-ruled 2026-08-24 for this repo's scripts/ config-and-vocabulary class, on the vocabulary half of scripts/measured-claims.sh's cause alone (scripts/ riding no installer payload is the half the ruling deliberately does NOT declare on). SITE_KIT_ALIASES is this project's own host set, ruled consumer-side in the comment below, so porting it would put one project's hosts in every adopter's binary.

# comment-tier-exempt: these are this repo's own project host aliases — rule content the provenance seam bars a kit gate from carrying, so it lives here in consumer config; the docs/CNAME host equals the first entry and is skipped at compare, the rest are reachable hosts that must never be the cited docs URL
# shellcheck disable=SC2034  # consumed by site-kit/lib/site.sh after sourcing
SITE_KIT_ALIASES=(
    checkwright.dev
    www.checkwright.dev
    checkwright.com
    www.checkwright.com
    checkwright.github.io
)
