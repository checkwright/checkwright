#!/usr/bin/env bash
# no-fixture: a fixture-less member, so the static source-grep is its only output-contract oracle
# Fixture gate (scanned as text): emits a conforming machine-keyable success line
# and a help: remedy on its failure path.
if false; then
    echo "  help: nothing to fix in the fixture"
    exit 1
fi
echo "SAMPLE: clean (nothing to check in the fixture)"
exit 0
