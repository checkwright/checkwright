## What this changes

<!-- One paragraph: what the tree does after this PR that it did not before, and
why. For a gate change, state what it now catches or stops false-flagging. -->

<!-- Keep this PR — description and fixtures alike — to generic mechanism.
Never include your own private rule content (term lists, coupling
vocabularies, product constants); a fixture reproduces on structure, not on
your vocabulary. -->

## Checklist

- [ ] The gate battery passes locally (`bash gate-sdk/bin/run-gates.sh`) and CI is green.
- [ ] Gate changes ship the `good/`/`bad/` fixture pair that proves them.
- [ ] Every commit is DCO-signed (`git commit -s`).
