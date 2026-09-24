# spec: installer/SPEC.md §The dependency boundary — the one-line install's PowerShell twin of docs/install.sh: docs/install.md's Windows recipe for one pinned release, and nothing else
# spec: installer/SPEC.md §The hosted install pin — `$pin` is the only version this script names, held to docs/install.sh and to the newest tag by check-install-pin
#
# usage: irm https://checkwright.dev/install.ps1 | iex
#        & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) [verb] [args...]
#   $env:CHECKWRIGHT_VERSION       the release to install (default: the pin below)
#   $env:CHECKWRIGHT_RELEASE_BASE  the download base (default: this project's GitHub Releases)

# spec: installer/SPEC.md §The dependency boundary — the whole body is one function called on the last line, so a truncated fetch defines nothing and runs nothing; every preference it sets is scoped to that function, so none leaks into the reader's session
# spec: installer/SPEC.md §The dependency boundary — it never calls `exit`, which under `iex` would close the reader's own session: the bootstrap's status is left in $LASTEXITCODE, and a failed step throws
function Install-Checkwright {
    $pin = '0.25.0'
    Set-StrictMode -Version Latest
    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'
    if (Test-Path Variable:PSNativeCommandArgumentPassing) { $PSNativeCommandArgumentPassing = 'Standard' }
    if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) { $PSNativeCommandUseErrorActionPreference = $false }

    $version = if ($env:CHECKWRIGHT_VERSION) { $env:CHECKWRIGHT_VERSION } else { $pin }
    $base = if ($env:CHECKWRIGHT_RELEASE_BASE) { $env:CHECKWRIGHT_RELEASE_BASE } else { 'https://github.com/checkwright/checkwright/releases/download' }
    $verbArgs = @($args)
    if ($verbArgs.Count -eq 0) { $verbArgs = @('init') }
    $tgz = "checkwright-$version.tgz"

    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

    $dir = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $dir | Out-Null
    try {
        [Console]::Error.WriteLine("checkwright install: fetching $tgz from $base")
        foreach ($file in @($tgz, "$tgz.sha256")) {
            try {
                Invoke-WebRequest "$base/v$version/$file" -OutFile (Join-Path $dir $file) -UseBasicParsing
            } catch {
                throw "checkwright install: download failed: $base/v$version/$file ($($_.Exception.Message))"
            }
        }

        $want = ((Get-Content -Raw -LiteralPath (Join-Path $dir "$tgz.sha256")).Trim() -split '\s+')[0]
        if ((Get-FileHash -LiteralPath (Join-Path $dir $tgz) -Algorithm SHA256).Hash -ne $want) {
            throw "checkwright install: verify failed: $tgz does not match its published digest"
        }

        # spec: installer/SPEC.md §The dependency boundary — the native steps' status is read from $LASTEXITCODE, so Windows PowerShell's wrapping of a native stderr line in a redirected host must not become a throw
        $ErrorActionPreference = 'Continue'
        & "$env:SystemRoot\System32\tar.exe" -xzf (Join-Path $dir $tgz) -C $dir
        if (-not $? -or $LASTEXITCODE -ne 0) { throw "checkwright install: extract failed: $tgz" }

        & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $dir 'package\bin\checkwright.ps1') @verbArgs
    } finally {
        Remove-Item -LiteralPath $dir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Install-Checkwright @args
