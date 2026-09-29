#!/usr/bin/env pwsh
# spec: installer/SPEC.md §The consumer smoke — the Windows driver: each profile installed from the packed tarball through the PowerShell bootstrap, its battery run through the binary, its hooks committing clean and refused, its uninstall back to the pre-init tree, and one upgrade at the lattice minimum
# no-port: installer/SPEC.md §The consumer smoke, The port disposition — the disposition run-smoke.sh declares, on the same ground: the repo's own acceptance harness, carried by no payload, driving the bootstrap, the binary and git as black boxes, so a crate-side form would test the binary from inside the binary. Structural, not a sizing judgment.
param(
    [Parameter(Mandatory)] [string] $Package,
    [Parameter(Mandatory)] [string] $Scratch,
    [Parameter(Mandatory)] [string] $Version,
    [Parameter(Mandatory)] [string] $Target,
    [Parameter(Mandatory)] [string] $UpgradePackage,
    [Parameter(Mandatory)] [string] $UpgradeVersion
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
# spec: installer/SPEC.md §The install boundary — the bootstrap's two pinned settings, taken by its caller too
if (Test-Path Variable:PSNativeCommandArgumentPassing) { $PSNativeCommandArgumentPassing = 'Standard' }
if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) { $PSNativeCommandUseErrorActionPreference = $false }

# spec: installer/SPEC.md §The consumer smoke — the consumer layout's two constants, each the operand of an assertion that reds on a wrong value
$GatesDir = 'scripts'
$ProfileDerived = 'full'

$Base = Join-Path $Scratch 'ps1-smoke'
if (Test-Path -LiteralPath $Base) { Remove-Item -Recurse -Force -LiteralPath $Base }
New-Item -ItemType Directory -Force -Path $Base | Out-Null

function Fail([string] $message) {
    Write-Host "RUN-SMOKE-PS1: FAIL - $message"
    exit 1
}

function Say([string] $message) { Write-Host "  $message" }

function Invoke-Captured([string] $exe, [string[]] $argv, [string] $cwd) {
    Push-Location -LiteralPath $cwd
    try {
        $out = (& $exe @argv 2>&1 | Out-String)
        $code = $LASTEXITCODE
    } finally { Pop-Location }
    return [pscustomobject]@{ Code = $code; Out = $out }
}

function Invoke-Bootstrap([string] $package, [string] $cwd, [string[]] $argv) {
    $entry = @('-NoProfile', '-File', (Join-Path $package 'bin/checkwright.ps1')) + $argv
    return Invoke-Captured 'pwsh' $entry $cwd
}

function New-Consumer([string] $name) {
    $c = Join-Path $Base $name
    New-Item -ItemType Directory -Force -Path $c | Out-Null
    git -C $c init -q
    git -C $c config user.email 'smoke@example.invalid'
    git -C $c config user.name 'ps1 smoke'
    git -C $c config maintenance.auto false
    git -C $c config gc.auto 0
    git -C $c commit -q --allow-empty -m 'seed'
    return $c
}

function Read-Lock([string] $consumer, [string] $field) {
    $path = Join-Path $consumer 'checkwright.lock'
    if (-not (Test-Path -LiteralPath $path)) { return $null }
    $v = Get-Content -Raw -LiteralPath $path | ConvertFrom-Json
    foreach ($part in ($field -split '\.')) {
        if ($null -eq $v -or -not ($v.PSObject.Properties.Name -contains $part)) { return $null }
        $v = $v.$part
    }
    return $v
}

# spec: installer/SPEC.md §init — the follow-up block: the indented lines after `next:`, a trailing comment dropped
function Get-Followups([string] $out) {
    $cmds = New-Object System.Collections.Generic.List[string]
    $inBlock = $false
    foreach ($l in ($out -split "`r?`n")) {
        if (-not $inBlock) { if ($l -ceq 'next:') { $inBlock = $true }; continue }
        if ($l -notmatch '^\s+\S') { break }
        $cmds.Add(($l -replace '#.*$', '').Trim())
    }
    return , $cmds
}

# spec: installer/SPEC.md §init — the binary init placed, read off the follow-up line that runs its --install-hooks
function Get-HooksLine([string] $label, [string] $consumer, [string] $out) {
    $hooksLine = $null
    foreach ($cmd in (Get-Followups $out)) {
        $toks = @($cmd -split '\s+' | Where-Object { $_ })
        if ($toks.Count -gt 1 -and $toks[1] -ceq '--install-hooks') { $hooksLine = $toks }
    }
    if ($null -eq $hooksLine) { Write-Host $out; Fail "${label}: init printed no --install-hooks follow-up line" }
    if (-not (Test-Path -LiteralPath (Join-Path $consumer $hooksLine[0]) -PathType Leaf)) {
        Fail "${label}: the follow-up line names $($hooksLine[0]), which init did not place"
    }
    return , $hooksLine
}

# spec: installer/SPEC.md §The consumer smoke — every directory holding a bash.exe leaves PATH, the system directory's WSL launcher excepted, and git stays reachable through its cmd directory
function Get-BashlessPath {
    $sysDir = [Environment]::SystemDirectory.TrimEnd('\')
    $gitRoot = Split-Path (Split-Path (Split-Path (& git --exec-path)))
    $kept = New-Object System.Collections.Generic.List[string]
    foreach ($d in ($env:PATH -split ';')) {
        if (-not $d) { continue }
        if ((Test-Path -LiteralPath (Join-Path $d 'bash.exe')) -and $d.TrimEnd('\') -ne $sysDir) { continue }
        $kept.Add($d)
    }
    $gitCmd = Join-Path $gitRoot 'cmd'
    if ((Test-Path -LiteralPath (Join-Path $gitCmd 'git.exe')) -and -not $kept.Contains($gitCmd)) { $kept.Add($gitCmd) }
    return ($kept -join ';')
}

function Assert-BatteryGreen([string] $label, [string] $consumer, [string] $bin) {
    $b = Invoke-Captured $bin @('--run') $consumer
    if ($b.Code -ne 0 -or $b.Out -notmatch 'All \d+ gates passed') {
        Write-Host $b.Out
        Fail "${label}: the battery is not green on the consumer init just made (exit $($b.Code))"
    }
    if (git -C $consumer status --porcelain) {
        git -C $consumer status --porcelain | Out-Host
        Fail "${label}: the battery left the worktree dirty"
    }
    Say "battery: $([regex]::Match($b.Out, 'All \d+ gates passed\.').Value)"
}

# spec: installer/SPEC.md §Profiles — the rows of profiles.list, a '#' anywhere ending a line; the payload-derived profile is never a row
$rows = New-Object System.Collections.Generic.List[object]
foreach ($line in (Get-Content -LiteralPath (Join-Path $Package 'profiles.list'))) {
    $l = ($line -replace '#.*$', '').Trim()
    if (-not $l) { continue }
    $parts = $l -split "`t"
    if ($parts.Count -ne 2) { Fail "profiles.list carries a row that is not '<profile><TAB><kit>': $line" }
    $rows.Add([pscustomobject]@{ Profile = $parts[0]; Kit = $parts[1] })
}
$profiles = @($rows | ForEach-Object { $_.Profile } | Select-Object -Unique)
if ($profiles.Count -eq 0) { Fail "the package's profiles.list names no profile" }
if ($profiles -contains $ProfileDerived) { Fail "profiles.list carries a row for the payload-derived $ProfileDerived" }
# spec: installer/SPEC.md §Profiles — the bounded lattice's minimum is the one profile every other contains, so it has the fewest kits
$minimum = $null
foreach ($p in $profiles) {
    $n = @($rows | Where-Object { $_.Profile -eq $p }).Count
    if ($null -eq $minimum -or $n -lt $minimumKits) { $minimum = $p; $minimumKits = $n }
}
$profiles += $ProfileDerived
Write-Host "profiles: $($profiles -join ' '); lattice minimum $minimum"

$sessionPath = $env:PATH
foreach ($p in $profiles) {
    Write-Host "profile arm ($p): init, battery, hooks clean and refused, uninstall"
    $c = New-Consumer "profile-$p"
    $seed = git -C $c rev-parse 'HEAD^{tree}'
    $r = Invoke-Bootstrap $Package $c @('init', '--profile', $p)
    if ($r.Code -ne 0 -or $r.Out -notmatch 'INIT: vendored \d+ kit') { Write-Host $r.Out; Fail "${p}: init exited $($r.Code)" }
    if ((Read-Lock $c 'profile') -ne $p -or (Read-Lock $c 'artifact.target') -ne $Target) {
        Fail "${p}: the manifest does not record profile $p with the $Target artifact"
    }
    if (-not (Test-Path -LiteralPath (Join-Path $c "$GatesDir/gates.list"))) { Fail "${p}: init wrote no $GatesDir/gates.list" }
    $hooksLine = Get-HooksLine $p $c $r.Out
    $bin = Join-Path $c $hooksLine[0]
    Say "init: $([regex]::Match($r.Out, 'INIT: [^\r\n]*').Value)"

    # spec: installer/SPEC.md §doctor — a probed bash row is an owed bash; an unprobed or absent one is not
    $d = Invoke-Bootstrap $Package $c @('doctor')
    $owesBash = ($d.Out -match '(?m)^  bash( |$)') -and -not ($d.Out -match '(?m)^  bash +not probed')
    try {
        if ($owesBash) {
            Say "bash: owed by this profile's doctor report, so PATH keeps it"
        } else {
            $env:PATH = Get-BashlessPath
            $left = @(Get-Command bash -CommandType Application -All -ErrorAction SilentlyContinue |
                Where-Object { (Split-Path $_.Source).TrimEnd('\') -ne [Environment]::SystemDirectory.TrimEnd('\') })
            if ($left.Count -gt 0) { Fail "${p}: bash still resolves after the strip: $($left.Source -join ', ')" }
            Say "bash: owed by nothing this profile's doctor report names, so it is stripped from PATH"
        }

        Assert-BatteryGreen $p $c $bin

        $h = Invoke-Captured $bin @($hooksLine | Select-Object -Skip 1) $c
        if ($h.Code -ne 0 -or -not (git -C $c config --get core.hooksPath)) { Write-Host $h.Out; Fail "${p}: the printed --install-hooks line exited $($h.Code)" }
        $initHead = git -C $c rev-parse HEAD

        [IO.File]::WriteAllText((Join-Path $c 'ps1-note.txt'), "a clean note`n")
        git -C $c add ps1-note.txt
        $clean = Invoke-Captured 'git' @('commit', '-m', 'docs: add a note') $c
        if ($clean.Code -ne 0 -or (git -C $c rev-parse HEAD) -eq $initHead) { Write-Host $clean.Out; Fail "${p}: a clean commit through the hooks failed" }
        if ($clean.Out -notmatch '(?m)^pre-commit: \d+ gate\(s\) passed\.') { Write-Host $clean.Out; Fail "${p}: the clean commit landed with no pre-commit summary line" }
        Say "hooks: a clean commit passed through the handoff"

        # spec: installer/SPEC.md §The consumer smoke — a planted home-directory path, which gate-sdk's zero-config pattern seed names, so every profile's hook refuses it; the refusing gate is read off the hook's own line and held to the consumer's registry
        $head = git -C $c rev-parse HEAD
        [IO.File]::WriteAllText((Join-Path $c 'ps1-leak.txt'), ('see /' + 'home' + "/ps1-smoke/notes`n"))
        git -C $c add ps1-leak.txt
        $refused = Invoke-Captured 'git' @('commit', '-m', 'docs: add a leak') $c
        if ($refused.Code -eq 0 -or (git -C $c rev-parse HEAD) -ne $head) { Write-Host $refused.Out; Fail "${p}: a commit planting a home-directory path landed" }
        $m = [regex]::Match($refused.Out, '(?m)^pre-commit: ([a-z0-9-]+) failed \(see above\)\.\r?$')
        $registry = @(Get-Content -LiteralPath (Join-Path $c "$GatesDir/gates.list"))
        if (-not $m.Success -or $registry -notcontains $m.Groups[1].Value) {
            Write-Host $refused.Out
            Fail "${p}: the refused commit was not refused by a registered gate the pre-commit hook named"
        }
        Say "hooks: a planted home-directory path was refused by $($m.Groups[1].Value)"

        git -C $c reset -q --hard $initHead
        $u = Invoke-Bootstrap $Package $c @('uninstall')
        if ($u.Code -ne 0) { Write-Host $u.Out; Fail "${p}: uninstall exited $($u.Code)" }
        if ((git -C $c rev-parse 'HEAD^{tree}') -ne $seed) { Write-Host $u.Out; Fail "${p}: the tree after uninstall is not the tree from before init" }
        if (git -C $c status --porcelain) { Fail "${p}: uninstall left the worktree dirty" }
        Say "uninstall: the tree object is back to its pre-init state"
    } finally {
        $env:PATH = $sessionPath
    }
}

# spec: installer/SPEC.md §The consumer smoke — the upgrade arm: update run from the tree packed at the next patch version, which the leg's pack step extracts
$next = $UpgradeVersion
$upPackage = $UpgradePackage
Write-Host "upgrade arm ($minimum): $Version -> $next through update"
if (-not (Test-Path -LiteralPath (Join-Path $upPackage 'bin/checkwright.ps1'))) { Fail "the upgrade package at $upPackage carries no bin/checkwright.ps1" }

$c = New-Consumer 'upgrade'
$r = Invoke-Bootstrap $Package $c @('init', '--profile', $minimum)
if ($r.Code -ne 0) { Write-Host $r.Out; Fail "the upgrade arm's starting install exited $($r.Code)" }
if ((Read-Lock $c 'version') -ne $Version) { Fail "the upgrade arm did not start at $Version" }
$hooksLine = Get-HooksLine 'upgrade' $c $r.Out
$r = Invoke-Bootstrap $upPackage $c @('update')
if ($r.Code -ne 0) { Write-Host $r.Out; Fail "update from the $next tarball exited $($r.Code)" }
if ((Read-Lock $c 'version') -ne $next) { Fail "the manifest records $(Read-Lock $c 'version') after updating to $next" }
if ((Read-Lock $c 'profile') -ne $minimum) { Fail "update re-read no $minimum profile from the manifest" }
Assert-BatteryGreen 'upgrade' $c (Join-Path $c $hooksLine[0])
Say "update: the manifest moved to $next at the $minimum profile and the battery is green"

Write-Host "RUN-SMOKE-PS1: clean ($($profiles.Count) profile(s) installed, committed through and reversed; $Version -> $next through update)"
exit 0
