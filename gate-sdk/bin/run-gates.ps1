#!/usr/bin/env pwsh
# spec: gate-sdk/SPEC.md §run-gates — the PowerShell twin of bin/run-gates.sh, for a native-Windows host with no bash on PATH: the same residue in the same order, held to the stub by the executed comparison --run-front-end-parity rather than by reading
# no-port: gate-sdk/SPEC.md §run-gates, The front-end's port disposition — the stub's own per-file bootstrap cause: the front-end locates the binary it runs, which the binary cannot do for itself. Structural, not a sizing judgment.
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

$onWindows = [System.IO.Path]::DirectorySeparatorChar -eq '\'
$SDK = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).ProviderPath

$top = $null
try { $top = & git rev-parse --show-toplevel 2>$null } catch { $top = $null }
if (-not $top -or -not (Test-Path -LiteralPath $top -PathType Container)) {
    Write-StubError 'run-gates: not inside a git repository'
    exit 2
}
Set-Location -LiteralPath $top
$here = (Get-Location).ProviderPath
[Environment]::CurrentDirectory = $here

# spec: gate-sdk/SPEC.md §Layout and configuration — the gate-sdk root locator, relative when the root lies under the repository root, as the stub exports it
$sep = [System.IO.Path]::DirectorySeparatorChar
$cmp = if ($onWindows) { [StringComparison]::OrdinalIgnoreCase } else { [StringComparison]::Ordinal }
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

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_exe_suffix's host half
$exeSuffix = if ($onWindows) { '.exe' } else { '' }

# spec: gate-sdk/SPEC.md §run-gates — the stub's two-name test, read off the leading token before the grammar below rewrites it
$argv = @($args)
$lead = if ($argv.Count -gt 0) { [string] $argv[0] } else { '' }
$unavailable = 2
if ($lead -ceq '--hook' -or $lead -ceq '--statusline') { $unavailable = 0 }

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

$bin = Get-PrebinaryKnob -Name 'GATE_SDK_NATIVE_BIN' -Default "native/target/release/checkwright-gates$exeSuffix"
$exe = if ([System.IO.Path]::IsPathRooted($bin)) { $bin } else { Join-Path $here $bin }
$runnable = Test-Path -LiteralPath $exe -PathType Leaf
if ($runnable -and -not $onWindows) {
    $mode = (Get-Item -LiteralPath $exe).PSObject.Properties['UnixMode']
    if ($mode -and $mode.Value -notmatch 'x') { $runnable = $false }
}
if (-not $runnable) {
    # comment-tier-exempt: the dash is spelled by code point because Windows PowerShell 5.1 reads a BOM-less script in the ANSI code page, which would turn a literal one into three characters and the message into different bytes from the stub's
    $dash = [string][char]0x2014
    Write-StubError ("run-gates: $($argv[0]) dispatches to the native binary, but $bin is absent or not " +
        "executable $dash it could not run. Build it: bash gate-sdk/bin/build-native.sh")
    exit $unavailable
}

& $exe @argv
exit $LASTEXITCODE
