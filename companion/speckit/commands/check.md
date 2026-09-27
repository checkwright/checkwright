---
description: "Run Checkwright's gate battery and report its verdict"
---

## User Input

$ARGUMENTS

## Steps

Run the battery from the repository root, in `sh`, which on Windows is Git for Windows' `sh`:

```sh
gates=./scripts/checkwright-gates
"$gates" --run
```

Report the verdict. On green, report the summary line. On red, list each failing gate with its finding and its `help:` line.

Fix nothing unless the user asks. Never edit a file under a vendored kit directory, drop a gate, or commit with `--no-verify`.
