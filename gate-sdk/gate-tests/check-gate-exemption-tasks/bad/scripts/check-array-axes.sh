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
# exception-list: tag inside an open array
OUTER=(
    'axes/outer.rs' # permanent: fixture
# exception-list: armed while OUTER is open
)
INNER=(
    'axes/inner.rs' # until: absent-axis-task
)
# exception-list: quote across lines
QUOTE=(
    'axes/multi.rs
)' # permanent: fixture
    'axes/after-quote.rs' # until: absent-axis-task
)
# exception-list: escapes
ESCAPE=(
    "axes/esc\").rs" # permanent: fixture
    'axes/single\' # permanent: fixture
    axes/bare\ name.rs # permanent: fixture
    axes/hash#mid.rs # permanent: fixture
    'axes/after-escape.rs' # until: absent-axis-task
)
printf '%s\n' "${TAG[@]}" "${NEXT[@]}" "${BALANCE[@]}" "${TOKEN[@]}" "${OPEN[@]}" "${OUTER[@]}" "${INNER[@]}" "${QUOTE[@]}" "${ESCAPE[@]}"
