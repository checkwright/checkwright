#!/usr/bin/env pwsh
# The front end — its code stays ASCII, so only this comment carries a dash.
$dash = [char]0x2014
if ($dash) {
    # indented — still a full-line comment
    Write-Output "done$dash"
}
