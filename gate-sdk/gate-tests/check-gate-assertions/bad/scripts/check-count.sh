#!/usr/bin/env bash
# Synthetic gate for the check-gate-assertions bad-case fixture: its marker set
# matches the contract span exactly, so the only finding is the contract's own
# internal count-word-vs-span inconsistency.
set -uo pipefail

# assertion 1: the first count check
echo "checking count 1"

# assertion 2: the second count check
echo "checking count 2"
