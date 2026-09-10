#!/usr/bin/env pwsh
# Hermetic fixture stand-in for the PowerShell bootstrap's host detector, the twin of the file
# beside it. Inside `Get-HostTarget`'s body every mapped triple is the sole single-quoted operand of
# a `return`, and `return ''` is the no-mapping arm rather than a triple.
function Get-HostTarget {
    switch -Regex (Get-HostShape) {
        '^linux/x64$'    { return 'x86_64-unknown-linux-gnu' }
        '^darwin/arm64$' { return 'aarch64-apple-darwin' }
    }
    return ''
}
