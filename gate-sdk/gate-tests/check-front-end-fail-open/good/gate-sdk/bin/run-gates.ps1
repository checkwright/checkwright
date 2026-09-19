$lead = if ($args.Count -gt 0) { [string] $args[0] } else { '' }
$unavailable = 2
$FailOpenArms = @('--hook', '--statusline')
if ($FailOpenArms -ccontains $lead) { $unavailable = 0 }
exit $unavailable
