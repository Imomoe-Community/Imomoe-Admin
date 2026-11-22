param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [Parameter(Mandatory)][string]$BootstrapGoRoot,
    [Parameter(Mandatory)][string]$Gcc
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$work = [IO.Path]::GetFullPath($OutputDirectory)
$env:GOROOT_BOOTSTRAP = (Resolve-Path -LiteralPath $BootstrapGoRoot).Path
$env:CC = (Resolve-Path -LiteralPath $Gcc).Path
$env:PATH = "$(Split-Path -Parent $env:CC);$env:PATH"
$env:GOTOOLCHAIN = 'local'
$env:GOOS = 'windows'
$env:GOARCH = 'amd64'
$env:GOROOT = $null
New-Item -ItemType Directory -Path $work | Out-Null

function Get-Source($Owner, $Name, $Revision) {
    $archive = Join-Path $work "$Name.tar.gz"
    curl.exe -fsSL "https://codeload.github.com/$Owner/$Name/tar.gz/$Revision" -o $archive
    tar.exe -xf $archive -C $work
    Join-Path $work "$Name-$Revision"
}

$goRev = '35b807d2fca7b447ab54162e1f8c2172570d7d1b'
$goRoot = Get-Source golang go $goRev
Set-Content -LiteralPath "$goRoot/VERSION" -Value "go1.28-devel_$goRev" -Encoding ascii
$env:CGO_ENABLED = '0'
Push-Location "$goRoot/src"
try { cmd.exe /c make.bat } finally { Pop-Location }
$go = "$goRoot/bin/go.exe"

$components = [ordered]@{
    'permissions-connector-s2' = '6f19452665d0df055559e70b0797daa8c726bd1e'
    'permissions-storage' = '04ff9bac478ee61019a46f899f54225c78fa0371'
}
$patch = (Resolve-Path -LiteralPath "$PSScriptRoot/../integration/permissions-storage-offline-profile.patch").Path
$env:CGO_ENABLED = '1'
foreach ($name in $components.Keys) {
    $source = Get-Source fr0nch $name $components[$name]
    Push-Location $source
    try {
        if ($name -eq 'permissions-storage') { git apply $patch }
        $dll = Join-Path $work "lib$name.dll"
        & $go build -mod=readonly -buildmode=c-shared -trimpath '-ldflags=-s -w' -o $dll .
        & $go version -m $dll
    } finally {
        Pop-Location
    }
}
