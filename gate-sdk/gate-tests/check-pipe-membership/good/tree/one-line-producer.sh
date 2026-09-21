#!/usr/bin/env bash
set -euo pipefail
if find . -name '*.txt' -print -quit | grep -q .; then
    echo found
fi
