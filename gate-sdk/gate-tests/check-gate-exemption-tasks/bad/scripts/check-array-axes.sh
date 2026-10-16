#!/usr/bin/env bash
#exception-list: tag spacing
TAG=(
    'axes/tag.rs' # until: absent-axis-task
)
# exception-list: next assignment
printf ignored
NEXT=(
    'axes/next.rs' # until: absent-axis-task
)
# exception-list: balance
BALANCE=( # )
    'axes/paren).rs' # until: absent-axis-task )
    'axes/after.rs' # until: absent-axis-task
)
# exception-list: first token
TOKEN=(
    'axes/first.rs' 'scripts/axes/ignored.rs' # until: absent-axis-task
)
# exception-list: opening element
OPEN=('axes/open.rs')
printf '%s\n' "${TAG[@]}" "${NEXT[@]}" "${BALANCE[@]}" "${TOKEN[@]}" "${OPEN[@]}"
