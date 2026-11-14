---
title: Manual steps
nav_parent: install
nav_child_order: 2
---

# Installing step by step

These are the steps the one-line install runs, taken one at a time for a reader who wants to see each command before it runs. Run your system's first block on the [install page](install.md) before them, where that page gives one. Pick a version from the [releases](https://github.com/checkwright/checkwright/releases) page and put it in place of `X.Y.Z`. [Requirements](requirements.md) lists the tools.

## macOS and Linux

Download the release into a scratch directory, so it unpacks outside the repository:

```sh
v=X.Y.Z
cw="$(mktemp -d)" \
  && curl -fsSLo "$cw/checkwright-$v.tgz" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz" \
  && curl -fsSLo "$cw/checkwright-$v.tgz.sha256" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz.sha256"
```

Where `gh` and `jq` are installed and `gh` is signed in, check the build attestation too; a release older than attestation fails it:

```sh
gh attestation verify "$cw/checkwright-$v.tgz" --repo checkwright/checkwright --source-ref "refs/tags/v$v" --format json > "$cw/attestation.json" \
  && jq -r '.[] | .verificationResult | "verified \(.statement.subject[0].name) sha256:\(.statement.subject[0].digest.sha256) signed by \(.signature.certificate.buildSignerURI)"' "$cw/attestation.json"
```

A passing check prints one line, `verified`, then the tarball's name, its digest, and the release workflow that signed it, `publish.yml` at the version's tag. A failing one prints its error and stops with a non-zero exit. Asked for JSON, `gh` prints its result to a pipe too; without it, `gh` prints only to a terminal, so an agent sees nothing at all.

Then verify, extract and run `init` from your repository root:

<!-- unix-install:begin -->

```sh
( cd "$cw" \
  && { sha256sum -c "checkwright-$v.tgz.sha256" || shasum -a 256 -c "checkwright-$v.tgz.sha256"; } \
  && tar -xzf "checkwright-$v.tgz" ) \
  && sh "$cw/package/bin/checkwright.sh" init
```

<!-- unix-install:end -->

To verify, `git show --stat HEAD` lists everything the install brought in. Run the commands `init` prints to finish the setup.

To uninstall, `sh "$cw/package/bin/checkwright.sh" uninstall` reverses it in one commit. If `$cw` is gone, download any release, as [Managing](maintenance.md#managing) says.

## Windows

Download the release, in PowerShell. Each block here stops at its first error, then puts your session's preferences back:

<!-- windows-fetch:begin -->

```powershell
. {
  $keep = $ErrorActionPreference, $ProgressPreference
  try {
    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'
    $v = 'X.Y.Z'
    $cw = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory $cw | Out-Null
    $url = "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
    Invoke-WebRequest $url -OutFile "$cw\checkwright-$v.tgz" -UseBasicParsing
    Invoke-WebRequest "$url.sha256" -OutFile "$cw\checkwright-$v.tgz.sha256" -UseBasicParsing
  } finally { $ErrorActionPreference, $ProgressPreference = $keep }
}
```

<!-- windows-fetch:end -->

Check the attestation as on macOS and Linux, printing the same line without `jq`:

```powershell
. {
  $keep = $ErrorActionPreference
  try {
    $ErrorActionPreference = 'Stop'
    $json = gh attestation verify "$cw\checkwright-$v.tgz" --repo checkwright/checkwright --source-ref "refs/tags/v$v" --format json
    if ($LASTEXITCODE -ne 0) { throw 'the attestation check failed: read the error above' }
    foreach ($a in (($json -join "`n") | ConvertFrom-Json)) {
      $r = $a.verificationResult
      "verified $($r.statement.subject[0].name) sha256:$($r.statement.subject[0].digest.sha256) signed by $($r.signature.certificate.buildSignerURI)"
    }
  } finally { $ErrorActionPreference = $keep }
}
```

Then check the digest, unpack, and run `init`, still from your repository root:

<!-- windows-install:begin -->

```powershell
. {
  $keep = $ErrorActionPreference
  try {
    $ErrorActionPreference = 'Stop'
    $want = (Get-Content -Raw "$cw\checkwright-$v.tgz.sha256").Split(' ')[0]
    if ((Get-FileHash "$cw\checkwright-$v.tgz" -Algorithm SHA256).Hash -ne $want) { throw 'checksum mismatch: download both files again' }
    & "$env:SystemRoot\System32\tar.exe" -xzf "$cw\checkwright-$v.tgz" -C $cw
    if ($LASTEXITCODE -ne 0) { throw 'tar.exe could not unpack the tarball: download it again' }
    powershell -NoProfile -ExecutionPolicy Bypass -File "$cw\package\bin\checkwright.ps1" init
  } finally { $ErrorActionPreference = $keep }
}
```

<!-- windows-install:end -->

Verify as above. To uninstall, run the `init` line with `uninstall` in place of `init`.
