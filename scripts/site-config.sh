# shellcheck shell=bash
# spec: site-kit/SPEC.md §Layout and configuration — this repo's site-kit consumer config

# comment-tier-exempt: these are this repo's own project host aliases — rule content the provenance seam bars a kit gate from carrying, so it lives here in consumer config; the docs/CNAME host equals the first entry and is skipped at compare, the rest are reachable hosts that must never be the cited docs URL
# shellcheck disable=SC2034  # consumed by site-kit/lib/site.sh after sourcing
SITE_KIT_ALIASES=(
    checkwright.dev
    www.checkwright.dev
    checkwright.com
    www.checkwright.com
    checkwright.github.io
)
