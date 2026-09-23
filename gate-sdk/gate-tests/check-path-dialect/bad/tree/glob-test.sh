#!/usr/bin/env bash
set -uo pipefail
[[ "$1" == /* ]] || set -- "$PWD/$1"
echo "$1"
