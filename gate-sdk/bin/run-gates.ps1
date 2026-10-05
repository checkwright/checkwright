#!/usr/bin/env pwsh
# spec: gate-sdk/SPEC.md §run-gates — the PowerShell twin of bin/run-gates.sh, for a native-Windows host with no bash on PATH: the same residue in the same order, held to the stub by the executed comparison --run-front-end-parity rather than by reading
# no-port: gate-sdk/SPEC.md §run-gates, The front-end's port disposition — the stub's own per-file bootstrap cause: the front-end locates the binary it runs, which the binary cannot do for itself. It serves a harness shim and a pre-build clone rather than an adopter door now, and neither audience leaves the binary able to answer for itself. Structural, not a sizing judgment.
#
# usage: run-gates.ps1 [gates-dir] | --only <name>... [-- <arg>...] | --for <path>... | --emit <arm> [args...] | -h | --help
#        every arm, its refusals and the knobs print from the tool itself: run-gates.ps1 --help

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# spec: gate-sdk/SPEC.md §run-gates — the installer bootstrap's two pinned settings, on its ground: tokens reach the binary unrewritten, and the binary's non-zero status stays a status rather than a thrown exception
if (Test-Path Variable:PSNativeCommandArgumentPassing) { $PSNativeCommandArgumentPassing = 'Standard' }
if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) { $PSNativeCommandUseErrorActionPreference = $false }

# spec: gate-sdk/SPEC.md §run-gates — the stub's lines are UTF-8 with a bare LF, so they are written as those bytes rather than through a console encoding that differs per host
function Write-StubError {
    param([string] $Text)
    $bytes = (New-Object System.Text.UTF8Encoding($false)).GetBytes($Text + "`n")
    $err = [Console]::OpenStandardError()
    $err.Write($bytes, 0, $bytes.Length)
    $err.Flush()
}

function Write-StubLine {
    param([string] $Text)
    $bytes = (New-Object System.Text.UTF8Encoding($false)).GetBytes($Text + "`n")
    $out = [Console]::OpenStandardOutput()
    $out.Write($bytes, 0, $bytes.Length)
    $out.Flush()
}

$onWindows = [System.IO.Path]::DirectorySeparatorChar -eq '\'
$cmp = if ($onWindows) { [StringComparison]::OrdinalIgnoreCase } else { [StringComparison]::Ordinal }
$SDK =(Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).ProviderPath

# spec: gate-sdk/SPEC.md §run-gates — the stub's repository mark, held here as there: a non-empty GIT_DIR, or a .git entry at the working directory or above, the ascent stopping where GIT_CEILING_DIRECTORIES stops git's own, which ignores a relative entry
function Test-RepositoryMark {
    if ($env:GIT_DIR) { return $true }
    $ceilings = @()
    if ($env:GIT_CEILING_DIRECTORIES) {
        foreach ($c in $env:GIT_CEILING_DIRECTORIES.Split([System.IO.Path]::PathSeparator)) {
            if ($c -cnotmatch '^([/\\]|[A-Za-z]:[/\\])') { continue }
            $r = Resolve-Path -LiteralPath $c -ErrorAction SilentlyContinue
            if ($r) { $ceilings += $r.ProviderPath.TrimEnd('\', '/') }
        }
    }
    $d = (Get-Location).ProviderPath
    $n = 0
    while ($d) {
        if ($n -gt 0 -and ($ceilings -contains $d.TrimEnd('\', '/'))) { return $false }
        if (Get-Item -LiteralPath (Join-Path $d '.git') -Force -ErrorAction SilentlyContinue) { return $true }
        $d = Split-Path -Parent $d
        $n++
    }
    return $false
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — an entry of any type, a dangling link included, which Get-Item does not report on every host
function Test-Entry {
    param([string] $Path)
    if (Get-Item -LiteralPath $Path -Force -ErrorAction SilentlyContinue) { return $true }
    try {
        $fi = New-Object System.IO.FileInfo($Path)
        if ($fi.Exists -or [System.IO.Directory]::Exists($Path)) { return $true }
        $lt = $fi.PSObject.Properties['LinkTarget']
        if ($lt -and $lt.Value) { return $true }
    } catch { }
    return $false
}

# spec: gate-sdk/SPEC.md §run-gates — a git directory's physical path, every symbolic link and junction on it resolved one component at a time before a `..` past it is taken, which the stub's `cd … && pwd -P` and the crate's canonicalize compare and Resolve-Path, folding a `..` lexically and following no link, does not; $null where it does not resolve
function Resolve-GitDir {
    param([string] $Path)
    if (-not $Path) { return $null }
    $seps = [char[]]@([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
    $base = (Get-Location).ProviderPath
    try {
        $full = if ([System.IO.Path]::IsPathRooted($Path)) { $Path } else { $base + [System.IO.Path]::DirectorySeparatorChar + $Path }
        $root = [System.IO.Path]::GetPathRoot($full)
    } catch { return $null }
    $cur = if ($root.TrimEnd($seps)) { $root } else { [System.IO.Path]::GetPathRoot($base) }
    $todo = New-Object 'System.Collections.Generic.List[string]'
    $todo.AddRange([string[]] $full.Substring($root.Length).Split($seps, [StringSplitOptions]::RemoveEmptyEntries))
    $hops = 0
    while ($todo.Count -gt 0) {
        $name = $todo[0]
        $todo.RemoveAt(0)
        if ($name -ceq '.') { continue }
        if ($name -ceq '..') {
            $up = Split-Path -Parent $cur
            if ($up) { $cur = $up }
            continue
        }
        $next = Join-Path $cur $name
        $item = Get-Item -LiteralPath $next -Force -ErrorAction SilentlyContinue
        if (-not $item) { return $null }
        $kind = $item.PSObject.Properties['LinkType']
        if ($kind -and (@('SymbolicLink', 'Junction') -ccontains [string] $kind.Value)) {
            $hops++
            if ($hops -gt 40) { return $null }
            $target = [string] (@($item.Target) | Select-Object -First 1)
            if (-not $target) { return $null }
            $root = ''
            if ([System.IO.Path]::IsPathRooted($target)) {
                $root = [System.IO.Path]::GetPathRoot($target)
                $cur = $root
            }
            $todo.InsertRange(0, [string[]] $target.Substring($root.Length).Split($seps, [StringSplitOptions]::RemoveEmptyEntries))
            continue
        }
        $cur = $next
    }
    return $cur.TrimEnd($seps)
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_skipped_mark, re-held here because the library is bash
function Get-SkippedMark {
    param([string] $Top, [string] $Prefix)
    if ($env:GIT_DIR) { return $null }
    $runs = @($Prefix.Split('/') | Where-Object { $_ })
    for ($n = $runs.Count; $n -ge 1; $n--) {
        $run = ($runs[0..($n - 1)]) -join '/'
        $mark = "$Top/$run/.git"
        if (-not (Test-Entry $mark)) { continue }
        $target = $mark
        if (Test-Path -LiteralPath $mark -PathType Leaf) {
            $target = $null
            $line = @(Get-Content -LiteralPath $mark -TotalCount 1 -ErrorAction SilentlyContinue)
            if ($line.Count -gt 0 -and ([string] $line[0]).StartsWith('gitdir: ')) {
                $target = ([string] $line[0]).Substring(8)
                if ($target -cnotmatch '^([/\\]|[A-Za-z]:[/\\])') { $target = "$Top/$run/$target" }
            }
        }
        $selected = $null
        try { $selected = Resolve-GitDir ([string] (& git rev-parse --absolute-git-dir 2>$null)) } catch { $selected = $null }
        $resolved = if ($target) { Resolve-GitDir $target } else { $null }
        if ($selected -and $resolved -and [string]::Equals($selected, $resolved, $cmp)) { return $null }
        return $mark
    }
    return $null
}

# spec: gate-sdk/SPEC.md §run-gates — the stub's fail-open set on its one declaration line, held to the crate by check-front-end-fail-open and read off the leading token before the lookup and the grammar below
$argv = @($args)
$lead = if ($argv.Count -gt 0) { [string] $argv[0] } else { '' }
$unavailable = 2
$FailOpenArms = @('--hook', '--statusline')
if ($FailOpenArms -ccontains $lead) { $unavailable = 0 }

# spec: gate-sdk/SPEC.md §The harness-integration arm — a tree whose repository git refuses or skips runs no binary, so a fail-open arm declines there as on an absent binary
function Exit-Refused {
    param([string] $Line)
    Write-StubError $Line
    if ($lead -ceq '--hook') {
        Write-StubLine -Text '{"systemMessage":"run-gates: git refuses or skips the repository this session stands in, so every hook guard in this tree is off and each guarded call is allowed. git status prints its reason, or git --git-dir=<that .git> status for a skipped one"}'
    }
    exit $unavailable
}

# spec: gate-sdk/SPEC.md §run-gates — the one lookup, split as the crate splits it (§The crate's crosser); the mark test runs before the directory changes
$answer = $null
try { $answer = @(& git rev-parse --show-toplevel --show-prefix 2>$null) } catch { $answer = $null }
if ($answer -and $LASTEXITCODE -ne 0) { $answer = $null }
$top = $null
if ($answer) {
    $text = $answer -join "`n"
    $at = $text.IndexOf("`n")
    $top = if ($at -ge 0) { $text.Substring(0, $at) } else { $text }
    $prefix = if ($at -ge 0) { $text.Substring($at + 1).TrimEnd("`n") } else { '' }
    if ($top -and (Get-SkippedMark -Top $top -Prefix $prefix)) {
        Exit-Refused 'run-gates: git skipped the repository a .git entry marks between here and the toplevel it answered, and answered the enclosing one; git --git-dir=<that .git> status prints its reason'
    }
}
if (-not $top -or -not (Test-Path -LiteralPath $top -PathType Container)) {
    if (Test-RepositoryMark) {
        Exit-Refused 'run-gates: git refuses the repository marked here (GIT_DIR, or a .git entry here or above); git status prints its reason'
    }
    Exit-Refused 'run-gates: not inside a git repository'
}
Set-Location -LiteralPath $top
$here = (Get-Location).ProviderPath
[Environment]::CurrentDirectory = $here

# spec: gate-sdk/SPEC.md §Layout and configuration — the gate-sdk root locator, relative when the root lies under the repository root, as the stub exports it
$sep = [System.IO.Path]::DirectorySeparatorChar
$prefix = $here.TrimEnd($sep) + $sep
if ($SDK.StartsWith($prefix, $cmp)) {
    $env:GATE_SDK_ROOT = $SDK.Substring($prefix.Length).Replace('\', '/')
} else {
    $env:GATE_SDK_ROOT = $SDK
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_sdk_gates_dir, re-held here because the library is bash
function Get-GatesDir {
    if ($env:GATE_SDK_GATES_DIR) { return $env:GATE_SDK_GATES_DIR }
    return 'scripts'
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — _gate_prebinary_file_value: the first line naming the knob, blanks trimmed round its name and value, comment and '='-less lines skipped; split on LF alone, as bash's read splits
function Get-KnobFileValue {
    param([string] $File, [string] $Name)
    if (-not (Test-Path -LiteralPath $File -PathType Leaf)) { return $null }
    $blank = [char[]]@(' ', "`t")
    foreach ($line in ([System.IO.File]::ReadAllText($File) -split "`n")) {
        $line = $line.TrimStart($blank)
        if (-not $line -or $line.StartsWith('#') -or -not $line.Contains('=')) { continue }
        $at = $line.IndexOf('=')
        if ($line.Substring(0, $at).TrimEnd($blank) -cne $Name) { continue }
        return $line.Substring($at + 1).Trim($blank)
    }
    return $null
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — _gate_prebinary_knob: the environment, the local overlay, the tracked file or GATE_SDK_KNOB_FILE, then the default; an empty value at any tier falls through
function Get-PrebinaryKnob {
    param([string] $Name, [string] $Default)
    $v = [Environment]::GetEnvironmentVariable($Name)
    if ($v) { return $v }
    $dir = Get-GatesDir
    $v = Get-KnobFileValue -File "$dir/gate-sdk-config.local.knobs" -Name $Name
    if ($v) { return $v }
    $tracked = if ($env:GATE_SDK_KNOB_FILE) { $env:GATE_SDK_KNOB_FILE } else { "$dir/gate-sdk-config.knobs" }
    $v = Get-KnobFileValue -File $tracked -Name $Name
    if ($v) { return $v }
    return $Default
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_native_bin_spelled: the door as a command token, `./`-prefixed unless the value is already anchored
function Get-NativeBinSpelled {
    param([string] $Default)
    $b = Get-PrebinaryKnob -Name 'GATE_SDK_NATIVE_BIN' -Default $Default
    if ($b -cmatch '^(/|\\|\./|\.\./|[A-Za-z]:[/\\])') { return $b }
    return "./$b"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_exe_suffix's host half
$exeSuffix = if ($onWindows) { '.exe' } else { '' }

# spec: gate-sdk/SPEC.md §run-gates — the stub's residual argv grammar, case for case
switch -CaseSensitive -Regex ($lead) {
    '^(-h|--help)$' { $argv = @('--run') + $argv; break }
    '^(--only|--for)$' { $argv = @('--run', '--gates-dir', (Get-GatesDir)) + $argv; break }
    '^--$' {
        $dir = if ($argv.Count -gt 1 -and [string] $argv[1]) { [string] $argv[1] } else { Get-GatesDir }
        $argv = @('--run', '--gates-dir', $dir)
        break
    }
    '^-' { break }
    '^$' { $argv = @('--run', '--gates-dir', (Get-GatesDir)); break }
    default { $argv = @('--run', '--gates-dir', $lead) }
}

function Test-Runnable {
    param([string] $Path)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { return $false }
    if (-not $onWindows) {
        $mode = (Get-Item -LiteralPath $Path).PSObject.Properties['UnixMode']
        if ($mode -and $mode.Value -notmatch 'x') { return $false }
    }
    return $true
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — _gate_main_checkout_bin: inside a linked worktree whose common dir is <main>/.git, the main checkout's own resolution of the knob, rooted; $null otherwise
function Get-MainCheckoutBin {
    param([string] $Default)
    $gd = $null
    $cd = $null
    try {
        $gd = Resolve-GitDir ([string] (& git rev-parse --git-dir 2>$null))
        $cd = Resolve-GitDir ([string] (& git rev-parse --git-common-dir 2>$null))
    } catch { return $null }
    if (-not $cd -or -not $gd -or [string]::Equals($gd, $cd, $cmp)) { return $null }
    if ((Split-Path -Leaf $cd) -cne '.git') { return $null }
    $main = Split-Path -Parent $cd
    Set-Location -LiteralPath $main
    [Environment]::CurrentDirectory = $main
    try {
        $b = Get-NativeBinSpelled -Default $Default
    } finally {
        Set-Location -LiteralPath $here
        [Environment]::CurrentDirectory = $here
    }
    if ([System.IO.Path]::IsPathRooted($b)) { return $b }
    return (Join-Path $main $b)
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_harness_bin's linked-worktree half: the main checkout's binary when runnable; $null otherwise
function Get-MainCheckoutExe {
    param([string] $Default)
    $e = Get-MainCheckoutBin -Default $Default
    if ($e -and (Test-Runnable $e)) { return $e }
    return $null
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_native_source_stamp, re-held here because the library is bash: the same three git invocations, the manifest hashed from a file because a pipe into a native command appends a line ending
function Get-SourceStamp {
    param([string] $Crate)
    try {
        $paths = @(& git -C $Crate ls-files 2>$null)
        if ($LASTEXITCODE -ne 0 -or $paths.Count -eq 0) { return $null }
        $hashes = @(& git -C $Crate hash-object -- @paths 2>$null)
        if ($LASTEXITCODE -ne 0 -or $hashes.Count -ne $paths.Count) { return $null }
        $sb = New-Object System.Text.StringBuilder
        for ($i = 0; $i -lt $paths.Count; $i++) { [void] $sb.Append("$($hashes[$i]) $($paths[$i])`n") }
        $tmp = [System.IO.Path]::GetTempFileName()
        try {
            [System.IO.File]::WriteAllBytes($tmp, (New-Object System.Text.UTF8Encoding($false)).GetBytes($sb.ToString()))
            $stamp = [string] (& git hash-object --no-filters -- $tmp 2>$null)
            if ($LASTEXITCODE -ne 0) { return $null }
        } finally {
            Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue
        }
    } catch { return $null }
    if (-not $stamp) { return $null }
    return $stamp.Trim()
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_verdict_bin's linked-worktree half: the main checkout's binary linked into the door path when the door is gitignored and, in a tree carrying crate source, its source stamp is the tree's; never a build. Returns the refused condition, or $null once the door runs
function Resolve-VerdictLink {
    param([string] $Default, [string] $Door, [string] $DoorExe)
    $mainBin = Get-MainCheckoutBin -Default $Default
    if (-not $mainBin) { return '' }
    if (-not (Test-Runnable $mainBin)) { return 'the main checkout has no binary' }
    $ignored = $false
    try { & git check-ignore -q -- $Door 2>$null; $ignored = ($LASTEXITCODE -eq 0) } catch { $ignored = $false }
    if (-not $ignored) { return "$Door is not gitignored here, so a linked binary would enter the tree" }
    $crate = Get-PrebinaryKnob -Name 'GATE_SDK_NATIVE_CRATE' -Default 'native'
    if ($crate.EndsWith('/')) { $crate = $crate.Substring(0, $crate.Length - 1) }
    $tracked = $false
    if (Test-Path -LiteralPath $crate -PathType Container) {
        try { $tracked = (@(& git -C $crate ls-files 2>$null).Count -gt 0) } catch { $tracked = $false }
    }
    if ($tracked) {
        $stamp = Get-SourceStamp -Crate $crate
        $baked = $null
        try { $baked = [string] (& $mainBin --source-stamp 2>$null | Select-Object -First 1) } catch { $baked = $null }
        if (-not $stamp -or -not $baked -or $baked.Trim() -cne $stamp) {
            return "the main checkout's binary was not built from this worktree's crate source"
        }
    }
    $parent = Split-Path -Parent $DoorExe
    try { New-Item -ItemType Directory -Force -Path $parent | Out-Null } catch { }
    try {
        New-Item -ItemType SymbolicLink -Path $DoorExe -Target $mainBin -ErrorAction Stop | Out-Null
    } catch {
        try { Copy-Item -LiteralPath $mainBin -Destination $DoorExe -ErrorAction Stop } catch { }
    }
    if (-not (Test-Runnable $DoorExe)) { return "the main checkout's binary could not be linked to $Door" }
    return $null
}

$defaultBin = "native/target/release/checkwright-gates$exeSuffix"
$bin = Get-NativeBinSpelled -Default $defaultBin
$exe = if ([System.IO.Path]::IsPathRooted($bin)) { $bin } else { Join-Path $here $bin }
$runnable = Test-Runnable $exe
# spec: gate-sdk/SPEC.md §The harness-integration arm — a fail-open arm, and only one, asks the main checkout before it declines
$why = ''
if (-not $runnable -and $unavailable -eq 0) {
    $mainExe = Get-MainCheckoutExe -Default $defaultBin
    if ($mainExe) {
        $exe = $mainExe
        $runnable = $true
    }
} elseif (-not $runnable) {
    $why = Resolve-VerdictLink -Default $defaultBin -Door $bin -DoorExe $exe
    if ($null -eq $why) {
        $why = ''
        $runnable = $true
    }
}
if (-not $runnable) {
    # comment-tier-exempt: the dash is spelled by code point because Windows PowerShell 5.1 reads a BOM-less script in the ANSI code page, which would turn a literal one into three characters and the message into different bytes from the stub's
    $dash = [string][char]0x2014
    $remedy = if ($why) {
        "In this linked worktree $why; run it in the main checkout rather than building here"
    } else {
        'Build it: bash gate-sdk/bin/build-native.sh'
    }
    Write-StubError ("run-gates: $($argv[0]) dispatches to the native binary, but $bin is absent or not " +
        "executable $dash it could not run. $remedy")
    # spec: gate-sdk/SPEC.md §The harness-integration arm — exit-0 stderr reaches the harness's debug log alone, so the fail-open --hook decline speaks through the one envelope every hook event accepts
    if ($argv[0] -ceq '--hook') {
        Write-StubLine -Text '{"systemMessage":"run-gates: the gate binary is absent or not executable, so every hook guard in this tree is off and each guarded call is allowed. Build it: bash gate-sdk/bin/build-native.sh"}'
    }
    exit $unavailable
}

& $exe @argv
exit $LASTEXITCODE
