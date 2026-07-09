# Fixture registry — deliberately omits the gate-sdk row

A synthetic kit table that registers every kit root *except* `gate-sdk/` — the
landed-kit-fell-out-of-the-docs drift the gate catches. `check-kit-registration`
must REJECT, naming the unregistered root.

The surviving rows use the `](<kit>/index.md)` page form (the good case covers
the bare `](<kit>/)` form against the repo's README): a row linking to a page
under the kit root registers that root, so only `gate-sdk/` may be named.

| kit | status |
| --- | --- |
| [lifecycle-kit](lifecycle-kit/index.md) | landed |
| [queue-kit](queue-kit/index.md) | landed |
| [spec-kit](spec-kit/index.md) | landed |
| [guard-kit](guard-kit/index.md) | landed |
| [delegation-kit](delegation-kit/index.md) | landed |
| [context-kit](context-kit/index.md) | landed |
| [drift-kit](drift-kit/index.md) | landed |
| [evidence-kit](evidence-kit/index.md) | landed |
