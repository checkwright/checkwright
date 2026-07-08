# Fixture registry — deliberately omits the gate-sdk row

A synthetic kit table that registers every kit root *except* `gate-sdk/` — the
landed-kit-fell-out-of-the-docs drift the gate catches. `check-kit-registration`
must REJECT, naming the unregistered root.

| kit | status |
| --- | --- |
| [lifecycle-kit/](lifecycle-kit/) | landed |
| [queue-kit/](queue-kit/) | landed |
| [spec-kit/](spec-kit/) | landed |
| [guard-kit/](guard-kit/) | landed |
| [delegation-kit/](delegation-kit/) | landed |
| [context-kit/](context-kit/) | landed |
| [drift-kit/](drift-kit/) | landed |
| [evidence-kit/](evidence-kit/) | landed |
