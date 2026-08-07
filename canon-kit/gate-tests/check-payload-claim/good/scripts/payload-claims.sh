#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-payload-claim — fixture disclosure vocabulary: one pattern per class, each recognizing its own class's phrasings without reaching into a sentence about what the vendoring model copies
set -uo pipefail

printf '%s\t%s\n' \
    predicate-withheld 'implementation source does not ship|verified against a published digest' \
    all-source         'you read (before|all of it)|(source|copy) you can read'
