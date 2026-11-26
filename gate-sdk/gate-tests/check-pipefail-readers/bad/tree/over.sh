#!/usr/bin/env bash
# Two sites against a baseline row of one: both print, since which is new is unknowable.
set -o pipefail
ls | head -1
ls | grep -q y
