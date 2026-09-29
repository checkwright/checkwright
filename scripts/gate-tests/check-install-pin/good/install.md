# Install

```sh
curl -fsSLo "$cw/checkwright-$v.tgz" "https://example.invalid/v$v/checkwright-$v.tgz"
curl -fsSLo "$cw/checkwright-$v.tgz.sha256" "https://example.invalid/v$v/checkwright-$v.tgz.sha256"
```

```powershell
Invoke-WebRequest $url -OutFile "$cw\checkwright-$v.tgz"
```

<!-- commit-cost:begin -->

On `linux x86_64` with `4` logical CPUs, the hook ran `12` pre-commit gates over every tracked file in **900 ms**, the median of three, measured at v0.21.0.

<!-- commit-cost:end -->
