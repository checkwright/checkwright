# Fixture: marked fences that run as declared

A fence writes a file, and the next fence in the same doc reads it.

<!-- fence-runnable -->
```bash
note="written by the first fence"
printf '%s\n' "$note" > note.txt
git init -q probe && git -C probe status --short
```

<!-- fence-runnable -->
```bash
test -f note.txt
"$GATE_SDK_NATIVE_BIN" --list
```

A fence that shows a failure declares the status it exits with.

<!-- fence-runnable: exit=3 -->
```sh
exit 3
```

An unmarked fence is never run, whatever it names.

```bash
curl -fsSL https://example.invalid/install.sh
```

Prose may name the marker, `<!-- fence-runnable -->`, without marking anything.
