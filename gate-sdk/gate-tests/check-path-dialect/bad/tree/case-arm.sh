#!/usr/bin/env bash
set -uo pipefail
case "$1" in
    /*) echo "$1" ;;
    *) echo "$PWD/$1" ;;
esac
