---
title: Fixture
---

# A page whose every block is one paste

```sh
v=X.Y.Z
cw="$(mktemp -d)" \
  && curl -fsSLo "$cw/a.tgz" "https://example.invalid/v$v/a.tgz" \
  && tar -xzf "$cw/a.tgz" -C "$cw"
```

```bash
( cd "$cw" && tar -xzf a.tgz ) \
  && sh "$cw/bin/run.sh" init   # one list
```

```sh
if [ -d "$cw" ]; then
  rm -rf "$cw"
fi
```

```text
first
second
```

```powershell
irm https://example.invalid/install.ps1 | iex
```

```powershell
. {
  $cw = Join-Path ([IO.Path]::GetTempPath()) 'a'
  New-Item -ItemType Directory $cw | Out-Null
}
```
