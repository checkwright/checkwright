# spec: installer/SPEC.md §The dependency boundary — the one-line install's PowerShell twin of docs/install.sh: docs/manual-install.md's Windows recipe for one pinned release, and nothing else
# spec: installer/SPEC.md §The hosted install pin — `$pin` is the only version this script installs and `$attestFrom` its attestation floor, both held to docs/install.sh by check-install-pin, the pin to the newest tag too
#
# usage: irm https://checkwright.dev/install.ps1 | iex
#        & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) [verb] [args...]
#   $env:CHECKWRIGHT_VERSION       the release to install (default: the pin below)
#   $env:CHECKWRIGHT_RELEASE_BASE  the download base (default: this project's GitHub Releases)

# spec: installer/SPEC.md §The dependency boundary — the whole body is one function called on the last line, so a truncated fetch defines nothing and runs nothing; every preference it sets is scoped to that function, so none leaks into the reader's session
# spec: installer/SPEC.md §The dependency boundary — it never calls `exit`, which under `iex` would close the reader's own session: the bootstrap's status is left in $LASTEXITCODE, and a failed step throws
function Install-Checkwright {
    $pin = '0.31.0'
    $attestFrom = '0.31.1'
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

        # spec: installer/SPEC.md §The dependency boundary — the attestation check: the floor over dotted digit runs, a version that does not parse compared as attested, then the three-part presence test and the four outcomes
        $attested = $true
        if ($version -match '^[0-9]+(\.[0-9]+)*$') {
            $have = $version.Split('.')
            $floor = $attestFrom.Split('.')
            for ($i = 0; $i -lt [Math]::Max($have.Count, $floor.Count); $i++) {
                $a = if ($i -lt $have.Count) { $have[$i].TrimStart('0') } else { '' }
                $b = if ($i -lt $floor.Count) { $floor[$i].TrimStart('0') } else { '' }
                $c = if ($a.Length -ne $b.Length) { $a.Length - $b.Length } else { [string]::CompareOrdinal($a, $b) }
                if ($c -lt 0) { $attested = $false; break }
                if ($c -gt 0) { break }
            }
        }
        if (-not $attested) {
            [Console]::Error.WriteLine("checkwright install: v$version predates build attestation; checked against its digest only")
        } else {
            $gh = Get-Command gh -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1
            $ghReady = $false
            if ($gh) {
                & $gh.Source attestation verify --help *> $null
                if ($LASTEXITCODE -eq 0) {
                    & $gh.Source auth token --hostname github.com *> $null
                    $ghReady = ($LASTEXITCODE -eq 0)
                }
            }
            if (-not $ghReady) {
                [Console]::Error.WriteLine("checkwright install: build attestation not checked (gh is not installed or not signed in); to check it: gh attestation verify $tgz --repo checkwright/checkwright --source-ref refs/tags/v$version")
            } else {
                $ghOut = & $gh.Source attestation verify (Join-Path $dir $tgz) --repo checkwright/checkwright --signer-workflow checkwright/checkwright/.github/workflows/publish.yml --source-ref "refs/tags/v$version" 2>&1
                if ($LASTEXITCODE -ne 0) {
                    $ghOut | ForEach-Object { [Console]::Error.WriteLine("$_") }
                    throw "checkwright install: verify failed: $tgz carries no build attestation from checkwright/checkwright's publish workflow"
                }
                [Console]::Error.WriteLine('checkwright install: build attestation verified')
            }
        }

        & "$env:SystemRoot\System32\tar.exe" -xzf (Join-Path $dir $tgz) -C $dir
        if (-not $? -or $LASTEXITCODE -ne 0) { throw "checkwright install: extract failed: $tgz" }

        & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $dir 'package\bin\checkwright.ps1') @verbArgs
    } finally {
        Remove-Item -LiteralPath $dir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Install-Checkwright @args
