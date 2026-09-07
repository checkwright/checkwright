#!/usr/bin/env pwsh
# spec: installer/README.md §The install boundary — the second hand-kept bootstrap, authored against that section's five steps in the one language a native Windows host runs without a POSIX shell; the bash half is installer/lib/init.sh's opening and this is its twin, not a transliteration of it
#
# usage: checkwright.ps1 [argv...]
#   Resolves, verifies and executes this package's gate binary, forwarding argv verbatim.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# spec: installer/README.md §The install boundary — step 5 forwards argv verbatim, so the two settings that would rewrite it are pinned rather than inherited: 'Standard' stops the binder re-quoting the caller's tokens, and the native-error preference would otherwise turn the artifact's own non-zero status into a thrown exception and lose it
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

# spec: installer/README.md §The gate binary — step 2: the twin of installer/lib/init.sh's target_of_host(). It reads the platform and architecture off the runtime rather than shelling out to uname, because the host this half exists for need carry no POSIX shell at all
function Get-HostTarget {
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
    switch -Regex ("$os/$arch") {
        '^windows/(x64|amd64)$' { return 'x86_64-pc-windows-msvc' }
        '^linux/x64$'           { return 'x86_64-unknown-linux-gnu' }
        '^linux/arm64$'         { return 'aarch64-unknown-linux-gnu' }
        '^darwin/x64$'          { return 'x86_64-apple-darwin' }
        '^darwin/arm64$'        { return 'aarch64-apple-darwin' }
    }
    return ''
}

# spec: installer/README.md §The gate binary — step 3: selection has three outcomes and collapsing any two is the defect, so the payload's own roster is read rather than a directory's presence inferred from — a platform never committed to and one whose artifact went missing are different answers
function Select-Artifact {
    param([string] $Payload, [string] $Target)
    $dir = Join-Path $Payload 'artifact'
    if (-not (Test-Path -LiteralPath $dir -PathType Container)) {
        return [pscustomobject]@{ Omit = 'substrate-unavailable' }
    }
    $roster = Join-Path $dir 'targets.list'
    if (-not (Test-Path -LiteralPath $roster -PathType Leaf)) {
        Die 'this payload carries prebuilt gate binaries but no target roster' `
            'the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower.'
    }
    $declared = @(Get-Content -LiteralPath $roster |
        Where-Object { $_ -notmatch '^\s*(#|$)' } |
        ForEach-Object { $_.Trim() })
    if (-not $Target -or $declared -notcontains $Target) {
        return [pscustomobject]@{ Omit = 'substrate-unavailable' }
    }
    $src = Join-Path $dir $Target
    $names = @(Get-ChildItem -LiteralPath $src -File -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -notlike '*.sha256' } | Sort-Object Name)
    if ($names.Count -ne 1 -or -not (Test-Path -LiteralPath ($names[0].FullName + '.sha256') -PathType Leaf)) {
        Die "the payload declares $Target but carries no complete artifact for it" `
            'a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on — refusing rather than installing a battery that silently shrank.' 1
    }
    return [pscustomobject]@{ Path = $names[0].FullName; Sidecar = $names[0].FullName + '.sha256' }
}

# spec: installer/README.md §The install boundary — step 4, and the one step where this half is simpler than the bash one rather than parallel to it: PowerShell carries Get-FileHash, so there is no hasher to resolve between and no digest-unverifiable outcome to reach
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

# spec: installer/README.md §The install boundary — the omit-and-declare outcome, which on this half declares and stops rather than declaring and proceeding: what proceeds on the bash half is conditional install logic, which sits behind the invoke and does not exist yet. The exit status is the bash half's for the same outcome, so the two agree on everything an adopter can observe here
if ($selected.PSObject.Properties.Name -contains 'Omit') {
    [Console]::Error.WriteLine("checkwright: omitting the prebuilt gate binary ($($selected.Omit))")
    [Console]::Error.WriteLine('  help: this host maps to no target this payload declares, so there is nothing to verify or run here.')
    exit 0
}

Test-ArtifactDigest -Artifact $selected.Path -Sidecar $selected.Sidecar

# spec: installer/README.md §The install boundary — step 5 is execute and not install: the artifact runs in place out of the payload, where the step above just verified it, and argv is forwarded verbatim
& $selected.Path @args
exit $LASTEXITCODE
