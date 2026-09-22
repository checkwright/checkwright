# Brevity fixture — good

## Shared conventions

- **Terse bullet:** one line, well within the budget.
- A plain bullet with no bold name, within budget, citing the
  HANDBOOK §Some section: measured, and clean.
- **Two-liner:** a second continuation line that keeps it at
  two, still comfortably within the code-point budget.
- **Over budget, no pointer:** this bullet runs long across
  several continuation lines but names no deeper doc, so it may
  legitimately own its content — the gate lets it pass even
  though it exceeds the code-point budget here, because length
  alone is not the finding: a long bullet that cites nothing may
  be the only home its content has, and trimming it would lose it.
- **Exempt and long:** <!-- brevity-exempt: every line load-bearing -->
  this bullet is over budget across six lines and it also cites
  a deeper doc HANDBOOK §Some section, which would normally flag
  it, but the exemption marker on the lead line blesses it, so
  the gate passes it anyway despite the pointer and the length,
  since every one of its lines was judged load-bearing by hand.

## Next section

Body outside the governed section is never scanned.
