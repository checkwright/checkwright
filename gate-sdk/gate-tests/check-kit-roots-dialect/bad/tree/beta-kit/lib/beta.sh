# A top-level *.sh under lib/ is what `--emit enum-sets` derives an `<kit>-lib`
# set from, and it fail-closes on a lib/ that tracks none. One is enough.
beta_noop() { :; }
