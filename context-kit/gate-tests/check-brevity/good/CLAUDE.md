# Brevity fixture — good

## Shared conventions

- **Terse bullet:** one line, well within the budget.
- **Two-liner:** a second continuation line that keeps it at
  two, still comfortably within the four-line budget.
- **Over budget, no pointer:** this bullet runs long across
  several continuation lines but names no deeper doc, so it may
  legitimately own its content — the gate lets it pass even
  though it exceeds the four-line budget here.
- **Exempt and long:** <!-- brevity-exempt: every line load-bearing -->
  this bullet is over budget across five lines and it also cites
  a deeper doc HANDBOOK §Some section, which would normally flag
  it, but the exemption marker on the lead line blesses it, so
  the gate passes it anyway despite the pointer and the length.

## Next section

Body outside the governed section is never scanned.
