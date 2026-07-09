# fixture — docs-cmd good

Run the suite (a command-position and a `bash`-prefixed invocation, both tracked):

```bash
bin/tool.sh
bash bin/tool.sh
```

Vendor an optional config. The destination is the consumer's — not tracked — and
rides in argument position, so neither `cp` operand is an invocation to resolve:

```bash
cp gate-sdk/templates/example-config.sh scripts/example-config.sh   # optional
```

Config knobs `GATE_SDK_GATES_DIR` and the cross-kit-read `GATE_SDK_LIB` resolve
against the kits' code, and the `SPEC_KIT_COMMENT_*` family stem resolves too.
