#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-install-claim — this repo's transport vocabulary: one <id><TAB><ERE> line per install transport, the pattern that recognizes it in a governed install section
set -uo pipefail

# comment-tier-exempt: the patterns are checkwright-specific by construction, and that is the false-positive contract — each matches its transport's recipe form and its prose form while staying out of the other's sentences, so a generic `npm install <package>` example elsewhere in the tree stays silent
printf '%s\t%s\n' \
    tarball 'releases/download/|[Rr]elease tarball|tarball is the primary' \
    npm     'npx( -y)? checkwright|npm (install|i)( -g)? checkwright'
