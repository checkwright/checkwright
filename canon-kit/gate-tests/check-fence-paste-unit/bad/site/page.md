# A page whose blocks run half

```sh
brew install bash
export PATH="$(brew --prefix)/bin:$PATH"
```

```sh
v=X.Y.Z
cw="$(mktemp -d)"
curl -fsSLo "$cw/a.tgz" "https://example.invalid/v$v/a.tgz"
```

```powershell
$cw = Join-Path ([IO.Path]::GetTempPath()) 'a'
New-Item -ItemType Directory $cw | Out-Null
```
