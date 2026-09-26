function Install-Checkwright {
    $pin = '0.21.0'
    $tgz = "checkwright-$pin.tgz"
    Write-Output $tgz
}

Install-Checkwright @args
