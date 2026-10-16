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
# exception-list: tag inside an open array
OUTER=(
    'axes/outer.rs' # permanent: fixture
# exception-list: armed while OUTER is open
)
INNER=(
    'axes/inner.rs' # permanent: fixture
)
# exception-list: quote across lines
QUOTE=(
    'axes/multi.rs
)' # permanent: fixture
    'axes/after-quote.rs' # permanent: fixture
)
# exception-list: escapes
ESCAPE=(
    "axes/esc\").rs" # permanent: fixture
    'axes/single\' # permanent: fixture
    axes/bare\ name.rs # permanent: fixture
    axes/hash#mid.rs # permanent: fixture
    'axes/after-escape.rs' # permanent: fixture
)
printf '%s\n' "${TAG[@]}" "${NEXT[@]}" "${BALANCE[@]}" "${TOKEN[@]}" "${OPEN[@]}" "${OUTER[@]}" "${INNER[@]}" "${QUOTE[@]}" "${ESCAPE[@]}"
