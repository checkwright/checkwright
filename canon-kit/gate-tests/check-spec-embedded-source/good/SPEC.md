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
