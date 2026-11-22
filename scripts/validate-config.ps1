#Requires -Version 7.5
[CmdletBinding()]
param([string]$ConfigRoot = (Join-Path $PSScriptRoot '../configs/imomoe_admin'))

$ErrorActionPreference = 'Stop'
$configs = [ordered]@{
    'admin.jsonc' = 'admin'
    'announcements.jsonc' = 'announcements'
    'translations/en.jsonc' = 'translations'
    'translations/zh-CN.jsonc' = 'translations'
}
foreach ($file in $configs.Keys) {
    $path = Join-Path $ConfigRoot $file
    $schema = Join-Path $ConfigRoot "schemas/$($configs[$file]).schema.json"
    try {
        # Test-Json does not fetch external references unless explicitly requested.
        if (-not (Test-Json -LiteralPath $path -Options IgnoreComments,AllowTrailingCommas -SchemaFile $schema)) {
            throw 'Schema validation returned false.'
        }
    } catch {
        throw "${path}: $($_.Exception.Message)"
    }
    Write-Output "OK $file"
}
