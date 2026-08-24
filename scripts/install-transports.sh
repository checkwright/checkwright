#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-install-claim — this repo's transport vocabulary: one <id><TAB><ERE> line per install transport, the pattern that recognizes it in a governed install section
# no-port: CLAUDE.md §The provenance seam (never cross it) — operator-ruled 2026-08-24 for this repo's scripts/ config-and-vocabulary class, on the vocabulary half of scripts/measured-claims.sh's cause alone (scripts/ riding no installer payload is the half the ruling deliberately does NOT declare on). This file IS this project's transport vocabulary; its patterns spell the product's own distribution model, on the ground canon-config.sh states at CANON_KIT_INSTALL_TRANSPORTS_CMD.
set -uo pipefail

# comment-tier-exempt: the patterns are checkwright-specific by construction, and that is the false-positive contract — each matches its transport's recipe form and its prose form while staying out of the other's sentences, so a generic `npm install <package>` example elsewhere in the tree stays silent
printf '%s\t%s\n' \
    tarball 'releases/download/|[Rr]elease tarball|tarball is the primary' \
    npm     'npx( -y)? checkwright|npm (install|i)( -g)? checkwright'
