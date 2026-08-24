#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-payload-claim — this repo's disclosure vocabulary: one <claim-id><TAB><ERE> line per disclosure class, the pattern that recognizes that class asserted in a governed doc
# no-port: CLAUDE.md §The provenance seam (never cross it) — operator-ruled 2026-08-24 for this repo's scripts/ config-and-vocabulary class, on the vocabulary half of scripts/measured-claims.sh's cause alone (scripts/ riding no installer payload is the half the ruling deliberately does NOT declare on). This file IS this project's payload-disclosure vocabulary; a spelling of what a payload discloses is one project's distribution model, on the ground canon-config.sh states at CANON_KIT_PAYLOAD_CLAIMS_CMD.
set -uo pipefail

# comment-tier-exempt: the patterns are checkwright-specific by construction, and their boundary is the gate's false-positive contract — all-source recognizes the readable-everything phrasings this tree actually used, and stays silent on a sentence about the kit directories vendoring as committed source, which stays true; predicate-withheld recognizes the ruled phrasing (a gate's implementation source not shipping, the predicate withheld, the binary arriving digest-verified) and is matched only while some other class is the declared one
printf '%s\t%s\n' \
    predicate-withheld 'implementation source does not ship|withholds the predicate|predicate withheld|verified against a published digest|digest-verified binary' \
    all-source         'you read (before|all of it)|(source|copy) you can read'
