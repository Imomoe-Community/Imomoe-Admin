[CmdletBinding(DefaultParameterSetName = 'Build')]
param(
    [Parameter(Mandatory, ParameterSetName = 'Build')][string]$Go,
    [Parameter(Mandatory, ParameterSetName = 'Build')][string]$Gcc,
    [Parameter(Mandatory)][string]$OutputDirectory,
    [Parameter(ParameterSetName = 'Build')][string]$SourceDirectory,
    [Parameter(Mandatory, ParameterSetName = 'Download')][switch]$DownloadOnly
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
if (-not $DownloadOnly) {
    $goExe = (Get-Command -Name $Go -CommandType Application -ErrorAction Stop).Source
    $env:CC = (Resolve-Path -LiteralPath $Gcc).Path
    $env:PATH = "$(Split-Path -Parent $env:CC)$([IO.Path]::PathSeparator)$env:PATH"
    $env:CGO_ENABLED = '1'
    $env:GOTOOLCHAIN = 'local'
    $env:GOROOT = $null
    $env:GOARCH = 'amd64'
    $env:GOOS, $ext = if ($IsWindows) { ('windows', 'dll') } else { ('linux', 'so') }
}
$work = [IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $work -Force | Out-Null

$components = [ordered]@{
    'permissions-connector-s2' = @{
        Revision = '6715a6dcae42617922f9817ca774487c1b9f972a'
    }
    'permissions-storage' = @{
        Revision = '04ff9bac478ee61019a46f899f54225c78fa0371'
        Patch = 'permissions-storage-offline-profile.patch'
    }
}
foreach ($name in $components.Keys) {
    $component = $components[$name]
    $rev = $component.Revision
    $file = "$name-$rev.tar.gz"
    if ($SourceDirectory) {
        $archive = (Resolve-Path -LiteralPath (Join-Path $SourceDirectory $file)).Path
    } else {
        $archive = Join-Path $work $file
        curl -fsSL "https://codeload.github.com/fr0nch/$name/tar.gz/$rev" -o $archive
    }
    if ($DownloadOnly) { continue }
    tar -xf $archive -C $work
    Push-Location (Join-Path $work "$name-$rev")
    try {
        if ($component.ContainsKey('Patch')) {
            git apply --no-index "$PSScriptRoot/../integration/$($component.Patch)"
        }
        $binary = Join-Path $work "lib$name.$ext"
        & $goExe build -mod=readonly -buildmode=c-shared -trimpath '-ldflags=-s -w' -o $binary .
        & $goExe version -m $binary
    } finally {
        Pop-Location
    }
}
