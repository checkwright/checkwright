# Install widget

The npm package is an installer rather than a dependency channel, and `npx
widget init` is what it runs. This preamble sits under the H1 alone, so no
section scope reaches it.

## Install

<!-- install-primary: tarball -->

Download the asset and its checksum, verify, extract, run:

```bash
curl -fsSL -o widget.tgz \
  https://example.invalid/widget/releases/download/vX.Y.Z/widget.tgz
```

If Node is already on your machine the same install is `npx widget init`. Same
payload, same result; only the fetch differs.

## Requirements

A section the configured regex does not select. `npx widget init` here is out of
scope entirely.
