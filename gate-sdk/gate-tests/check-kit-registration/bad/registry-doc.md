# Fixture registry — deliberately omits the gate-sdk row

A synthetic kit table that registers every kit root *except* `gate-sdk/` — the
landed-kit-fell-out-of-the-docs drift the gate catches. `check-kit-registration`
must REJECT, naming the unregistered root. A kit landing without a row here is
named beside it, which the expectation tolerates.

The surviving rows use the bare `](<kit>/)` form (the good case covers the
`](<kit>/index.md)` page form against the repo's registry): a row linking the
kit directory itself registers that root.

| kit | status |
| --- | --- |
| [canon-kit](canon-kit/) | landed |
| [context-kit](context-kit/) | landed |
| [delegation-kit](delegation-kit/) | landed |
| [doctrine-kit](doctrine-kit/) | landed |
| [drift-kit](drift-kit/) | landed |
| [evidence-kit](evidence-kit/) | landed |
| [guard-kit](guard-kit/) | landed |
| [lifecycle-kit](lifecycle-kit/) | landed |
| [queue-kit](queue-kit/) | landed |
| [site-kit](site-kit/) | landed |
