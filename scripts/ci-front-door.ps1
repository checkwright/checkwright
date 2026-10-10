# spec: RELEASING.md §The front-door rehearsal — the Windows legs' driver, the PowerShell twin of scripts/ci-front-door.sh: a stranger's first session, first upgrade and package install of one published release, from a seat holding no checkout
# no-port: directed 2026-10-10 by the operator, asked and answered in a `/lead` session and lead-relayed: a direction and no ruling. The subject is the line a stranger types into a shell and the bootstrap that places the compiled binary, so a compiled driver would test the artifact with itself. The cause is this file's and never a class: nothing else may cite it, and a second driver is a new file with its own disposition.
# usage: ci-front-door.ps1 <version> [<previous-version>]
#   Run outside every git work tree, on a seat with no `checkwright` on PATH.
#   Reads nothing from a checkout; each install runs in a child of this host.
param(
    [Parameter(Position = 0)][string]$Version = '',
    [Parameter(Position = 1)][string]$Previous = ''
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Continue'
if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) { $PSNativeCommandUseErrorActionPreference = $false }
$global:LASTEXITCODE = 0

if (-not $Version -or $Version.StartsWith('-')) {
    [Console]::Error.WriteLine('usage: ci-front-door.ps1 <version> [<previous-version>]')
    [Console]::Error.WriteLine('  <version>           the published release to rehearse, as X.Y.Z')
    [Console]::Error.WriteLine('  <previous-version>  the release before it; empty or absent skips the upgrade session')
    exit 2
}

$found = Get-Command checkwright -ErrorAction SilentlyContinue | Select-Object -First 1
if ($found) {
    [Console]::Error.WriteLine("front-door: seat: 'checkwright' already resolves on PATH ($($found.Source)), so this seat is not clean")
    exit 2
}
$inside = & git rev-parse --is-inside-work-tree 2>$null
if ($LASTEXITCODE -eq 0 -and "$inside".Trim() -eq 'true') {
    [Console]::Error.WriteLine("front-door: seat: the working directory $PWD is inside a git work tree, so this seat is not clean")
    exit 2
}

$InstallUrl = 'https://checkwright.dev/install.ps1'
$Package = 'checkwright'
$Lock = 'checkwright.lock'
$PsHost = (Get-Process -Id $PID).Path

$tempRoot = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [IO.Path]::GetTempPath() }
$Base = Join-Path $tempRoot ('front-door.' + [IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $Base | Out-Null

$script:Log = New-Object 'System.Collections.Generic.List[string]'
$script:Session = ''
$script:Repo = ''
$script:GatesDir = ''
$script:Notes = 0

function Invoke-InRepo {
    param([string]$File, [string[]]$Arguments = @())
    $code = 127
    Push-Location -LiteralPath $script:Repo
    try {
        $out = & $File @Arguments 2>&1
        $code = $LASTEXITCODE
        foreach ($line in @($out)) { $script:Log.Add("$line") }
    } catch {
        $script:Log.Add("$_")
    } finally {
        Pop-Location
    }
    return $code
}

function Write-Ok {
    param([string]$Name)
    Write-Host "front-door: $($script:Session): ${Name}: ok"
    $script:Log.Clear()
}

function Write-Bad {
    param([string]$Name, [string]$Reason)
    Write-Host "front-door: $($script:Session): ${Name}: FAILED: $Reason"
    foreach ($line in $script:Log) { Write-Host $line }
    $script:Log.Clear()
}

# spec: installer/SPEC.md §The dependency boundary — the one-line install as the Windows install page prints it: piped to iex bare, the scriptblock form where it carries arguments
function Invoke-OneLine {
    param([string]$At, [string[]]$VerbArgs = @())
    if ($VerbArgs.Count -eq 0) {
        $line = "irm $InstallUrl | iex"
    } else {
        $tail = ($VerbArgs | ForEach-Object { "'" + $_ + "'" }) -join ' '
        $line = "& ([scriptblock]::Create((irm $InstallUrl))) $tail"
    }
    $text = "`$env:CHECKWRIGHT_VERSION = '$At'; $line; exit `$LASTEXITCODE"
    return Invoke-InRepo $PsHost @('-NoProfile', '-Command', $text)
}

function New-Repo {
    param([string]$Session, [string]$Name)
    $script:Session = $Session
    $script:Repo = $Base
    $path = Join-Path $Base $Name
    if ((Invoke-InRepo 'git' @('init', '-q', $path)) -ne 0) { return $false }
    $script:Repo = $path
    if ((Invoke-InRepo 'git' @('config', 'user.name', 'front door')) -ne 0) { return $false }
    if ((Invoke-InRepo 'git' @('config', 'user.email', 'front-door@example.invalid')) -ne 0) { return $false }
    return $true
}

function Get-LockField {
    param([string]$Name)
    $path = Join-Path $script:Repo $Lock
    if (-not (Test-Path -LiteralPath $path)) { return '' }
    foreach ($line in Get-Content -LiteralPath $path) {
        if ($line -match ('^\s*"' + [regex]::Escape($Name) + '":\s*"([^"]*)"')) { return $Matches[1] }
    }
    return ''
}

function Test-InstallAt {
    param([string]$Name, [string]$At, [string[]]$VerbArgs = @())
    if ((Invoke-OneLine $At $VerbArgs) -ne 0) { Write-Bad $Name "the one-line install at $At exited non-zero"; return $false }
    if (-not (Test-Path -LiteralPath (Join-Path $script:Repo $Lock))) { Write-Bad $Name "the install left no $Lock"; return $false }
    $got = Get-LockField 'version'
    if ($got -ne $At) { Write-Bad $Name "$Lock names version '$got', not $At"; return $false }
    Write-Host "front-door: $($script:Session): ${Name}: ok"
    return $true
}

# spec: installer/SPEC.md §init — the follow-up block's stated grammar: the `next:` banner, one command per indented line, commentary from the first `#`
function Test-FollowUp {
    param([string]$Name)
    $commands = New-Object 'System.Collections.Generic.List[string]'
    $on = $false
    foreach ($line in $script:Log) {
        if ($on) {
            if ($line -notmatch '^\s+\S') { break }
            $command = ($line -replace '#.*$', '').Trim()
            if ($command) { $commands.Add($command) }
        } elseif ($line -eq 'next:') {
            $on = $true
        }
    }
    $script:Log.Clear()
    $hooks = $false
    foreach ($command in $commands) {
        if ($command -like '*--install-hooks*') { $hooks = $true }
        if (-not $script:GatesDir) {
            foreach ($token in ($command -split '\s+')) {
                if ($token -match '[/\\]') { $script:GatesDir = Split-Path -Parent $token; break }
            }
        }
        if ((Invoke-InRepo $PsHost @('-NoProfile', '-Command', "$command; exit `$LASTEXITCODE")) -ne 0) {
            Write-Bad $Name "the printed command '$command' exited non-zero"
            return $false
        }
    }
    if ($commands.Count -eq 0) { Write-Bad $Name 'init printed no follow-up block'; return $false }
    if (-not $hooks) { Write-Bad $Name 'no printed command places the hooks'; return $false }
    Write-Ok "$Name ($($commands.Count) printed command(s))"
    return $true
}

function Test-CommitLands {
    param([string]$Name)
    $script:Log.Clear()
    $script:Notes = $script:Notes + 1
    $note = "front-door-note-$($script:Notes).txt"
    [IO.File]::WriteAllText((Join-Path $script:Repo $note), "front-door rehearsal note $($script:Notes)`n")
    if ((Invoke-InRepo 'git' @('add', '--', $note)) -ne 0) { Write-Bad $Name 'git add failed'; return $false }
    if ((Invoke-InRepo 'git' @('commit', '-m', "docs: add front-door rehearsal note $($script:Notes)")) -ne 0) {
        Write-Bad $Name 'a clean commit was refused'
        return $false
    }
    if (-not ($script:Log | Where-Object { $_ -match '^pre-commit: ' })) {
        Write-Bad $Name 'the commit landed with no pre-commit hook line, so no placed hook ran'
        return $false
    }
    Write-Ok $Name
    return $true
}

# spec: installer/SPEC.md §The consumer smoke — the planted defect is that arm's: a pipeline that can lose its match, the refusing gate read off the hook's own line and held to the installed registry
function Test-CommitRefused {
    param([string]$Name)
    $script:Log.Clear()
    $plant = 'front-door-plant.sh'
    $body = @(
        '#!/usr/bin/env bash',
        'set -o pipefail',
        'names=(a b c)',
        'if printf "%s\n" "${names[@]}" | grep -q b; then echo found; fi'
    ) -join "`n"
    [IO.File]::WriteAllText((Join-Path $script:Repo $plant), "$body`n")
    if ((Invoke-InRepo 'git' @('add', '--', $plant)) -ne 0) { Write-Bad $Name 'git add failed'; return $false }
    if ((Invoke-InRepo 'git' @('commit', '-m', 'chore: add a pipeline that can lose its match')) -eq 0) {
        Write-Bad $Name 'a commit carrying the planted defect landed'
        return $false
    }
    $gate = ''
    foreach ($line in $script:Log) {
        if ($line -match '^pre-commit: (\S+) failed') { $gate = $Matches[1]; break }
    }
    if (-not $gate) { Write-Bad $Name 'the refusal names no gate'; return $false }
    $registry = Join-Path (Join-Path $script:Repo $script:GatesDir) 'gates.list'
    if (-not (Test-Path -LiteralPath $registry) -or -not (@(Get-Content -LiteralPath $registry) -contains $gate)) {
        Write-Bad $Name "the refusal names $gate, which $($script:GatesDir)/gates.list does not register"
        return $false
    }
    if ((Invoke-InRepo 'git' @('rm', '-q', '-f', '--cached', '--', $plant)) -ne 0) { Write-Bad $Name 'could not unstage the plant'; return $false }
    Remove-Item -LiteralPath (Join-Path $script:Repo $plant) -Force
    Write-Ok "$Name (refused by $gate)"
    return $true
}

function Test-ProfileMove {
    param([string]$Name)
    $script:Log.Clear()
    $installed = Get-LockField 'profile'
    if ((Invoke-OneLine $Version @('init', '--help')) -ne 0) { Write-Bad $Name 'init --help exited non-zero'; return $false }
    $roster = ''
    foreach ($line in $script:Log) {
        if ($line -match '^profiles:\s*(.*)$') { $roster = $Matches[1].Trim(); break }
    }
    $other = ''
    foreach ($candidate in ($roster -split '\s+')) {
        if ($candidate -and $candidate -ne $installed) { $other = $candidate }
    }
    if (-not $other) {
        Write-Bad $Name "the artifact's roster '$roster' names no profile other than the installed '$installed'"
        return $false
    }
    $script:Log.Clear()
    if ((Invoke-OneLine $Version @('init', '--profile', $other)) -ne 0) {
        Write-Bad $Name "init --profile $other over the hooked $installed install exited non-zero"
        return $false
    }
    $got = Get-LockField 'profile'
    if ($got -ne $other) { Write-Bad $Name "$Lock names profile '$got', not $other"; return $false }
    Write-Ok "$Name ($installed -> $other)"
    return $true
}

function Test-FirstSession {
    if (-not (New-Repo 'one-line first session' 'first')) { Write-Bad 'repository' 'git init failed'; return $false }
    $script:GatesDir = ''
    if (-not (Test-InstallAt 'install' $Version)) { return $false }
    if (-not (Test-FollowUp 'follow-up block')) { return $false }
    if (-not (Test-CommitLands 'clean commit')) { return $false }
    if (-not (Test-CommitRefused 'first refusal')) { return $false }
    if (-not (Test-ProfileMove 'profile move')) { return $false }
    if (-not (Test-CommitLands 'commit after the profile move')) { return $false }
    return $true
}

function Test-FirstUpgrade {
    if (-not (New-Repo 'one-line first upgrade' 'upgrade')) { Write-Bad 'repository' 'git init failed'; return $false }
    $script:GatesDir = ''
    if (-not (Test-InstallAt "install at $Previous" $Previous)) { return $false }
    if (-not (Test-FollowUp "follow-up block at $Previous")) { return $false }
    if (-not (Test-CommitLands "commit at $Previous")) { return $false }
    if (-not (Test-InstallAt "update to $Version" $Version @('update'))) { return $false }
    if (-not (Test-CommitLands 'commit after the update')) { return $false }
    return $true
}

function Test-PackageRoute {
    if (-not (New-Repo 'package' 'package')) { Write-Bad 'repository' 'git init failed'; return $false }
    $npx = Get-Command npx -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $npx) { Write-Bad 'init' 'npx does not resolve on PATH'; return $false }
    if ((Invoke-InRepo $npx.Source @('--yes', "$Package@$Version", 'init')) -ne 0) {
        Write-Bad 'init' "npx $Package@$Version init exited non-zero"
        return $false
    }
    if (-not (Test-Path -LiteralPath (Join-Path $script:Repo $Lock))) { Write-Bad 'init' "the install left no $Lock"; return $false }
    $got = Get-LockField 'version'
    if ($got -ne $Version) { Write-Bad 'init' "$Lock names version '$got', not $Version"; return $false }
    Write-Ok 'init'
    return $true
}

$findings = $false
try {
    if (-not (Test-FirstSession)) { $findings = $true }
    if ($Previous) {
        if (-not (Test-FirstUpgrade)) { $findings = $true }
    } else {
        Write-Host 'front-door: one-line first upgrade: skipped, no earlier release was given'
    }
    if (-not (Test-PackageRoute)) { $findings = $true }
} finally {
    Remove-Item -LiteralPath $Base -Recurse -Force -ErrorAction SilentlyContinue
}

if ($findings) {
    Write-Host "front-door: v${Version}: a step above failed"
    exit 1
}
Write-Host "front-door: v${Version}: every session clean"
exit 0
