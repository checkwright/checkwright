#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-install-claim — fixture transport vocabulary: recipe form and prose form per transport, neither pattern reaching into the other's sentences
set -uo pipefail

printf '%s\t%s\n' \
    tarball 'releases/download/|[Rr]elease tarball' \
    npm     'npx( -y)? widget|npm (install|i)( -g)? widget'
