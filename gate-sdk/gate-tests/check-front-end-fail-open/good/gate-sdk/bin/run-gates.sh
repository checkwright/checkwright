#!/usr/bin/env bash
ARM_UNAVAILABLE_STATUS=2
FAIL_OPEN_ARMS='--hook --statusline'
for arm in $FAIL_OPEN_ARMS; do
    if [[ "${1-}" == "$arm" ]]; then
        ARM_UNAVAILABLE_STATUS=0
    fi
done
exit "$ARM_UNAVAILABLE_STATUS"
