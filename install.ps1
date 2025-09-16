# ./install --dev --status
function Get-TimeStamp {
    return "[{0:MM/dd/yy} {0:HH:mm:ss}]" -f (Get-Date)
}

$modPath = Get-Item -Path .\modPath.txt | Get-Content -Tail 1
$features = ""

If ($args[0] -like "*devhook*") {
    $features = $features+"devhook"
    cargo skyline install --install-path $modPath --features=$features
    Write-Output "$(Get-TimeStamp) Installed devhook plugin"
}
elseif ($args[0] -like "*dev*") {
    $features = $features+"dev"
    cargo skyline install --install-path rom:/smashline/development.nro --features=$features
    Write-Output "$(Get-TimeStamp) Installed dev plugin"
}
else{
    cargo skyline install --install-path $modPath
    Write-Output "$(Get-TimeStamp) Installed plugin"
}