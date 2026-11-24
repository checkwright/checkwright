---
title: Writing gates
nav_parent: install
nav_child_order: 5
description: What a gate you write yourself needs, in shell, PowerShell or Rust, beyond what an install requires.
---

# Writing your own gates

This page holds what a gate you write needs beyond an install's own [requirements](requirements.md). [gate-sdk/SPEC.md](gate-sdk/SPEC.md#the-gate-authoring-surface) owns all three forms.

## Writing your own shell gates

A gate you write is a copy of gate-sdk's `templates/check-skeleton.sh`. It is a bash script that sources gate-sdk's library, so it needs the [`bash` floor](requirements.md#installing-and-running-the-shipped-gates) on every system, whatever your profile. `doctor` checks it once your `gates.list` registers the gate. On native Windows that bash is Git for Windows' bash, or write the gate in PowerShell instead. `check-shellcheck` lints your gate if you register it, and then the `shellcheck` row of that table binds you.

## Writing your own PowerShell gates

A PowerShell gate is two files in your gates directory: a `.gate` descriptor carrying the gate's `# graph:` line, its `# spec:` line and `# exec: <script>.ps1`, and the script it names. It needs a PowerShell host, which you name by setting `GATE_SDK_POWERSHELL` in `gate-sdk-config.knobs`: `powershell` for Windows PowerShell 5.1, which every Windows host carries, or `pwsh` for PowerShell 7 on any system. The gate runs as your shell gates do and prints the same lines; it reads a knob through `--emit knob-values <NAME>` on the binary its environment names. `check-psscriptanalyzer` lints it once you register it, and then the `PSScriptAnalyzer` module binds you. Running its fixture pair with `--run-gate-tests` also uses Git for Windows' bash.

## Writing your own Rust gates

A Rust gate is a program you build. Its `.gate` descriptor in your gates directory carries the gate's `# graph:` line, its `# spec:` line, and `# exec: <path>`, naming the built program relative to that directory with no extension. The battery adds `.exe` on Windows, so one descriptor serves every system. It needs a Rust toolchain on every machine that runs the gate. The battery runs the program at that path and never builds it, so build it before the battery runs and rebuild it after each edit: the battery does not compare it with its source. A gate not yet built fails as a gate that could not run. It prints the lines your shell gates print, and reads a knob through `--emit knob-values <NAME>` on the binary its environment names. The same descriptor starts a program built in any language.
