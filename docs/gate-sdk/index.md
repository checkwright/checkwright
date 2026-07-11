# gate-sdk

A self-testing lint framework for the prose, spec, and config surfaces
conventional linters ignore. gate-sdk is the foundation the other kits build
on: they ship their checks as gates that register into its runner, and its
meta-gates hold those gates to a fixed shape.

A gate is a small script that scans a surface, emits one machine-keyable
success line or names each finding with a remedy, and fails the commit when it
finds a violation. gate-sdk supplies the runner, the golden-fixture test
harness, the `# graph:` coupling manifests, and a generated pre-commit hook.

## Install

gate-sdk is vendored whole: copy the `gate-sdk/` directory into your repo,
keep a `gates.list` registry naming the gates you run, and opt each clone into
the generated hook. Consumers never edit vendored kit files — configuration is
external, so an upgrade replaces the directory losslessly.

## Quick start

```bash
bash gate-sdk/bin/run-gates.sh                       # run the full battery
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/install-hooks.sh                   # opt this clone into the hook
```

## Contracts

The gate contracts — the output, fail-closed, fixture-pair, and self-lint
disciplines every gate keeps — are defined in the kit's
[`SPEC.md`](https://github.com/checkwright/checkwright/blob/master/gate-sdk/SPEC.md#the-gate-model);
its [`README.md`](https://github.com/checkwright/checkwright/blob/master/gate-sdk/README.md)
lists the mechanism. Both ship inside the vendored `gate-sdk/`
directory. Back to the [kit map](../index.md#the-kits) or the
[install guide](../install.md).
