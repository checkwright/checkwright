#!/usr/bin/env pwsh
# Hermetic fixture stand-in for the PowerShell bootstrap's host detector. It carries the same
# forward defect as the file beside it, because both defects exist in both bootstraps: a
# three-way binding that left the twin out would reproduce the defect one surface over.
function Get-HostTarget {
    switch -Regex (Get-HostShape) {
        '^linux/x64$'    { return 'x86_64-unknown-linux-gnu' }
        '^linux/arm64$'  { return 'aarch64-unknown-linux-gnu' }
        '^darwin/arm64$' { return 'aarch64-apple-darwin' }
        '^darwin/x64$'   { return 'x86_64-apple-darwin' }
    }
    return ''
}
