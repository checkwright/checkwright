# context-kit

Token-economics-aware context management. Everything a stateless session reads
costs tokens, and a session that reads whole files to answer a narrow question
burns its budget before it starts work. context-kit makes reading cheap and
deliberate.

It ships index-first reading tools that surface a file's shape before its body,
a session-start hook that assembles a compact orienting brief, an always-loaded
meter with a committed baseline, and a gate over the density of the densest
always-loaded section.

## Install

Vendor the `context-kit/` directory into your repo, wire its session-start
hook, and register its gate in `gates.list`.

## Quick start

```bash
bash context-kit/bin/md-index.sh <file.md>           # outline before body
bash context-kit/bin/pub-index.sh <component>/src/   # public API surface
```

## Contracts

The reading-tool contracts and the always-loaded budget are defined in the
kit's `SPEC.md`; its `README.md` lists the mechanism. Back to the
[kit map](../index.md#the-kits).
