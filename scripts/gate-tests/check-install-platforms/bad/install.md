# Install fixture

## Requirements

<!-- platforms:begin -->

- `x86_64-unknown-linux-gnu` (joined) — declared joined while the roster carries
  no such live line: the declaration flipped ahead of the roster write, arm A.
- `aarch64-apple-darwin` (held:) — held with the precondition left empty, which
  is the silently granted hold arm C's first half refuses.
- `x86_64-apple-darwin` (held: a green producer leg consumed by a platform smoke leg) — held
  and yet a live roster line: the roster write whose declaration flip was
  forgotten, arm C's second half.

<!-- platforms:end -->
