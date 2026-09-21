# fixture — fence-command-head good

A tool reached through a sourced library's function, bound once and reused. The
sourced path's locator default is what the gate reads:

```bash
. "${WIDGET_ROOT:-lib}/tool.sh" && door="$(tool_door)"
"$door" --verbose
"$(tool_door)" --quiet
```

Builtins, a bundled program, a configured program, a tracked script, a machine
path whose basename is a bundled program, and a function the fence defines:

```sh
cd . && export LEVEL=1
git status --short | wc -l
widgetctl up --detach
bin/run.sh
./bin/run.sh --twice
/usr/bin/env true
greet() { printf 'hi\n'; }
greet
```

A heredoc body and a `case` pattern are data, never command heads:

```bash
cat > notes.txt <<'EOF'
--not-a-command
EOF
case "$1" in
  --help) printf 'usage\n' ;;
esac
```

A fence of another language is not read:

```text
--run-demo
```
