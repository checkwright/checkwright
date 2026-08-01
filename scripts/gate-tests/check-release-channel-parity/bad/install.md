## Versioning

### The release channel

Release channel: **stable**

Both invariants are violated here, deliberately. Invariant A: `stable` demands
the absence of `--prerelease` and the workflow carries it. Invariant B: the
version line passed in `args` is 0.x, which demands `preview`. The B arm is the
one a later simplification pass would delete, so `expect.txt` pins B's message —
dropping B leaves A still reddening while the expected substring vanishes, and
the fixture fails.
