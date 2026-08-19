# widget — SPEC

Owns the widget semantics. The wire shape is illustrative (a json example, not
a copy of any tracked file) — the gate skips illustrative fences:

```json
{
  "id": "abc-123",
  "kind": "widget",
  "count": 3,
  "nested": { "left": 1, "right": 2 },
  "tags": ["x", "y", "z"],
  "enabled": true,
  "note": "shape only"
}
```

The implementation lives in `widget.sh`; this spec cites the path rather than
embedding the body.

The per-site valve, exercised here because nothing else reaches it: the block
below IS a verbatim copy of the tracked implementation and would fire, and the
marker directly above the opening fence is the only reason it does not.

<!-- spec-embedded-source-exempt: the copy is the subject of this paragraph, so citing the path instead would leave the paragraph about nothing -->
```bash
#!/usr/bin/env bash
set -euo pipefail
alpha_one="first line of widget logic"
beta_two="second line of widget logic"
gamma_three="third line of widget logic"
delta_four="fourth line of widget logic"
epsilon_five="fifth line of widget logic"
zeta_six="sixth line of widget logic"
echo "widget processing complete"
```
