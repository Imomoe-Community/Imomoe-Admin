param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [Parameter(Mandatory)][string]$BootstrapGoRoot,
    [Parameter(Mandatory)][string]$Gcc
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$work = [IO.Path]::GetFullPath($OutputDirectory)
$env:GOROOT_BOOTSTRAP = (Resolve-Path -LiteralPath $BootstrapGoRoot).Path
$env:GOTOOLCHAIN = 'local'
$env:GOOS = 'windows'
$env:GOARCH = 'amd64'
$env:GOROOT = $null
New-Item -ItemType Directory -Path $work | Out-Null

$goRev = '35b807d2fca7b447ab54162e1f8c2172570d7d1b'
$goArchive = Join-Path $work 'go.tar.gz'
curl.exe -fsSL "https://codeload.github.com/golang/go/tar.gz/$goRev" -o $goArchive
tar.exe -xf $goArchive -C $work
$goRoot = Join-Path $work "go-$goRev"
Set-Content -LiteralPath "$goRoot/VERSION" -Value "go1.28-devel_$goRev" -Encoding ascii
$env:CGO_ENABLED = '0'
Push-Location "$goRoot/src"
try { cmd.exe /c make.bat } finally { Pop-Location }
& "$PSScriptRoot/build-permissions.ps1" -Go "$goRoot/bin/go.exe" -Gcc $Gcc -OutputDirectory $work
