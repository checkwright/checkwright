#!/usr/bin/env bash
#exception-list: tag spacing
TAG=(
    'axes/tag.rs' # permanent: fixture
)
# exception-list: next assignment
printf ignored
NEXT=(
    'axes/next.rs' # permanent: fixture
)
# exception-list: balance
BALANCE=( # )
    'axes/paren).rs' # permanent: fixture )
    'axes/after.rs' # permanent: fixture
)
# exception-list: first token
TOKEN=(
    'axes/first.rs' 'scripts/axes/ignored.rs' # permanent: fixture
)
# exception-list: opening element
OPEN=('axes/open.rs')
printf '%s\n' "${TAG[@]}" "${NEXT[@]}" "${BALANCE[@]}" "${TOKEN[@]}" "${OPEN[@]}"
