#!/usr/bin/env bash
# Never sets pipefail and matches no library glob: not linted.
ls | head -1
ls | grep -q x
