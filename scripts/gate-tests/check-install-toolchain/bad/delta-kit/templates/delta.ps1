#!/usr/bin/env pwsh
# delta-kit stays OUT of the audience, so the green line is not a vacuous one: this
# interpreter carries `sh` as a substring and is matched as a path component, and the
# hook template beside it spawns the binary rather than a shell.
Write-Output "delta"
