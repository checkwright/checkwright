#!/usr/bin/env pwsh
# spec: installer/README.md §The install boundary — the second hand-kept bootstrap, authored against that section's five steps in the one language a native Windows host runs without a POSIX shell; installer/bin/checkwright.sh is its twin, not its original and not a transliteration of it
#
# usage: checkwright.ps1 [argv...]
#   Resolves, verifies and executes this package's gate binary. A dashless leading token is prefixed
#   with `--`; everything after it is forwarded verbatim.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# spec: installer/README.md §The install boundary — step 5 forwards the caller's tokens through unrewritten, so the two settings that would rewrite them are pinned rather than inherited: 'Standard' stops the binder re-quoting the caller's tokens, and the native-error preference would otherwise turn the artifact's own non-zero status into a thrown exception and lose it
if (Test-Path Variable:PSNativeCommandArgumentPassing) { $PSNativeCommandArgumentPassing = 'Standard' }
if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) { $PSNativeCommandUseErrorActionPreference = $false }

function Die {
    param([string] $Message, [string] $Help, [int] $Code = 2)
    [Console]::Error.WriteLine("checkwright: $Message")
    if ($Help) { [Console]::Error.WriteLine("  help: $Help") }
    exit $Code
}

# spec: installer/README.md §The install boundary — step 1: the package's own payload directory, resolved from the script's own location through the symlink chain, because npm installs a bin entry as a link and the unresolved path's parent is node_modules
function Resolve-InstallerRoot {
    $self = $PSCommandPath
    while ($true) {
        $item = Get-Item -LiteralPath $self -Force
        if (-not $item.LinkType) { break }
        $target = @($item.Target)[0]
        if (-not $target) { break }
        if (-not [System.IO.Path]::IsPathRooted($target)) {
            $target = Join-Path (Split-Path -Parent $self) $target
        }
        $self = $target
    }
    return (Split-Path -Parent (Split-Path -Parent $self))
}

# spec: installer/README.md §The gate binary — the detector's whole input, factored out so a refusal can name what the host was detected AS rather than only that it mapped to nothing; the twin of installer/bin/checkwright.sh's host_shape()
function Get-HostShape {
    $rt = 'System.Runtime.InteropServices.RuntimeInformation' -as [type]
    if ($rt) {
        $p = [System.Runtime.InteropServices.OSPlatform]
        $os = ''
        if ($rt::IsOSPlatform($p::Windows)) { $os = 'windows' }
        elseif ($rt::IsOSPlatform($p::Linux)) { $os = 'linux' }
        elseif ($rt::IsOSPlatform($p::OSX)) { $os = 'darwin' }
        $arch = $rt::OSArchitecture.ToString()
    } else {
        # comment-tier-exempt: the runtime type is absent only on a Windows PowerShell below .NET 4.7.1, which is a Windows-only host by construction, so the environment's own architecture is the whole of what is left to read
        $os = 'windows'
        $arch = $env:PROCESSOR_ARCHITECTURE
    }
    return "$os/$arch"
}

# spec: installer/README.md §The gate binary — step 2: the twin of installer/bin/checkwright.sh's target_of_host(). It reads the platform and architecture off the runtime rather than shelling out to uname, because the host this half exists for need carry no POSIX shell at all
# spec: installer/README.md §The gate binary — every mapped triple here is the sole single-quoted operand of a `return` and the empty `return ''` is the no-mapping arm, which is the shape check-install-platforms extracts this detector's triple set from
function Get-HostTarget {
    switch -Regex (Get-HostShape) {
        '^windows/(x64|amd64)$' { return 'x86_64-pc-windows-msvc' }
        '^linux/x64$'           { return 'x86_64-unknown-linux-gnu' }
        '^linux/arm64$'         { return 'aarch64-unknown-linux-gnu' }
        '^darwin/x64$'          { return 'x86_64-apple-darwin' }
        '^darwin/arm64$'        { return 'aarch64-apple-darwin' }
    }
    return ''
}

# spec: installer/README.md §The gate binary — the libc question, answered beside the other refusals rather than inside the detector: the detector answers which published artifact fits this host's OS and architecture, this answers whether the host's C library is the one that artifact was linked against. pwsh runs on Linux, so this half's Linux arms are reachable and owe the same discriminator against the same signals. Each verdict rests on a POSITIVE signal, so an unidentifiable libc is 'unknown' and refuses rather than being read as glibc
function Get-LibcFlavour {
    if (@(Get-ChildItem -Path '/lib' -Filter 'ld-musl-*' -ErrorAction SilentlyContinue).Count -gt 0) {
        return 'musl'
    }
    foreach ($probe in @(@('getconf', 'GNU_LIBC_VERSION'), @('ldd', '--version'))) {
        # comment-tier-exempt: the probe is absent on exactly the hosts this discriminator exists to refuse, and under $ErrorActionPreference='Stop' an absent one is a terminating CommandNotFoundException rather than a verdict — so presence is tested and the invocation is caught, leaving an unanswerable probe as no signal instead of a crash
        if (-not (Get-Command $probe[0] -CommandType Application -ErrorAction SilentlyContinue)) { continue }
        try { $out = (& $probe[0] $probe[1] 2>&1 | Out-String) } catch { continue }
        if ($out -match '(?i)gnu libc|glibc') { return 'gnu' }
    }
    return 'unknown'
}

# spec: installer/README.md §The gate binary — step 3: selection keeps three outcomes and only one of them proceeds, so the payload's own roster is read rather than a directory's presence inferred from — a platform never committed to and one whose artifact went missing are different answers, told apart by message and remedy
function Select-Artifact {
    param([string] $Payload, [string] $Target)
    $shape = Get-HostShape
    $unrostered = 'the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take.'
    $dir = Join-Path $Payload 'artifact'
    if (-not (Test-Path -LiteralPath $dir -PathType Container)) {
        Die "this host, detected as $shape, maps to no target this payload declares" $unrostered
    }
    $roster = Join-Path $dir 'targets.list'
    if (-not (Test-Path -LiteralPath $roster -PathType Leaf)) {
        Die 'this payload carries prebuilt gate binaries but no target roster' `
            'the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower.'
    }
    # spec: installer/README.md §The gate binary — the one case where the roster comparison cannot refuse for us: .NET's OSPlatform and OSArchitecture cannot tell glibc from musl any more than uname can, so a musl host resolves to a triple that IS on the roster and would be handed a binary that dies in the dynamic loader. This is the second way to reach the unsupported-host refusal, and it fires before the roster comparison because the reason is the same one — no published artifact fits this host
    if ($Target -like '*-linux-gnu') {
        switch (Get-LibcFlavour) {
            'gnu' { }
            'musl' {
                Die "this host, detected as $shape, runs a musl C library, and every Linux artifact this payload carries is linked against glibc" `
                    'musl and glibc are not interchangeable at the dynamic loader, so a glibc build would die there rather than run. This payload carries no musl artifact, so there is no adopter action to take.'
            }
            default {
                Die "this host, detected as $shape, did not identify its C library, and every Linux artifact this payload carries is linked against glibc" `
                    "neither 'getconf GNU_LIBC_VERSION' nor 'ldd --version' identified a GNU libc here, and no musl loader was found under /lib — so nothing establishes that a glibc build would run, and this refuses rather than handing you one that may die in the loader. Install GNU libc's getconf or ldd so the probe can answer."
            }
        }
    }
    $declared = @(Get-Content -LiteralPath $roster |
        Where-Object { $_ -notmatch '^\s*(#|$)' } |
        ForEach-Object { $_.Trim() })
    if (-not $Target -or $declared -notcontains $Target) {
        Die "this host, detected as $shape, maps to no target this payload declares" $unrostered
    }
    $src = Join-Path $dir $Target
    $names = @(Get-ChildItem -LiteralPath $src -File -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -notlike '*.sha256' } | Sort-Object Name)
    if ($names.Count -ne 1 -or -not (Test-Path -LiteralPath ($names[0].FullName + '.sha256') -PathType Leaf)) {
        Die "the payload declares $Target but carries no complete artifact for it" `
            'a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on — refusing rather than running a battery that silently shrank.' 1
    }
    return [pscustomobject]@{ Path = $names[0].FullName; Sidecar = $names[0].FullName + '.sha256' }
}

# spec: installer/README.md §The install boundary — step 4, and the one step where this half is simpler than the bash one rather than parallel to it: PowerShell carries Get-FileHash, so there is no hasher to resolve between and this half reaches no refusal the bash one owes a resolution first
function Test-ArtifactDigest {
    param([string] $Artifact, [string] $Sidecar)
    $want = (@(Get-Content -LiteralPath $Sidecar -TotalCount 1) -split '\s+')[0]
    $got = (Get-FileHash -LiteralPath $Artifact -Algorithm SHA256).Hash
    # comment-tier-exempt: Get-FileHash returns upper-case hex where the sidecar was written by sha256sum in lower case, so the comparison folds case rather than the two halves disagreeing on a digest they both computed correctly
    if (-not $want -or $want.ToLowerInvariant() -ne $got.ToLowerInvariant()) {
        Die "the prebuilt gate binary does not match its published digest" `
            'nothing unverified is ever executed, so this refuses rather than warning. Re-download the package; a persistent mismatch means the artifact was altered after it was built.' 1
    }
}

$INSTALLER = Resolve-InstallerRoot
$PAYLOAD = Join-Path $INSTALLER 'payload'
if (-not (Test-Path -LiteralPath $PAYLOAD -PathType Container)) {
    Die 'this package carries no payload' `
        "the bootstrap runs the gate binary out of the package's own payload/, assembled at pack time — run it from an installed package, not from a source checkout."
}

$selected = Select-Artifact -Payload $PAYLOAD -Target (Get-HostTarget)

Test-ArtifactDigest -Artifact $selected.Path -Sidecar $selected.Sidecar

# spec: installer/README.md §The install boundary — step 5 is execute and not install: the artifact runs in place out of the payload, where the step above just verified it, under one unconditional argv rule — a dashless leading token is prefixed with `--` and everything after it is forwarded verbatim. The rule introduces no verb table into either bootstrap, so the two halves agree on the verb word instead of one forwarding it and the other consuming it
$forward = @($args)
if ($forward.Count -gt 0 -and $forward[0] -notlike '-*') {
    $forward[0] = '--' + $forward[0]
}
& $selected.Path @forward
exit $LASTEXITCODE
