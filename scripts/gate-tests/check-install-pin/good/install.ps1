function Install-Checkwright {
    $pin = '0.21.0'
    $tgz = "checkwright-${pin}.tgz"
    foreach ($file in @($tgz, "$tgz.sha256")) { Write-Output $file }
}

Install-Checkwright @args
