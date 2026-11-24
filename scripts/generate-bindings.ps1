param(
    [Parameter(Mandatory)] [string] $Generator,
    [Parameter(Mandatory)] [string] $S2SdkManifest,
    [Parameter(Mandatory)] [string] $NativeBaseManifest,
    [Parameter(Mandatory)] [string] $PermissionsManifest
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$out = Join-Path $root 'crates/admin-plugin/src/generated'
$bindingRoot = Join-Path $root 'bindings'
$names = @{
    s2sdk = @(
        'AddConsoleCommand','RemoveCommand','AddCommandListener','RemoveCommandListener',
        'GetMaxClients','IsClientInGame','IsClientConnected','IsClientAuthorized','IsClientAlive','IsFakeClient','IsClientSourceTV',
        'GetClientName','GetClientTeam','GetClientSteamID64','GetClientLanguageCode','GetClientPawn','EntPointerToEntHandle',
        'PrintToChat','PrintToChatColored','PrintToServer','PrintCentreHtml','KickClient','GetClientListening','SetClientListening',
        'OnClientAuthenticated_Register','OnClientAuthenticated_Unregister','OnClientDisconnect_Register',
        'OnClientDisconnect_Unregister','OnClientPutInServer_Register','OnClientPutInServer_Unregister','OnMapEnd_Register','OnMapEnd_Unregister','OnGameFrame_Register',
        'OnGameFrame_Unregister','OnEntityDeleted_Register','OnEntityDeleted_Unregister',
        'ForcePlayerSuicide','ForceRespawnClient','ChangeClientTeam','SwitchClientTeam','GetClientHealth',
        'SetClientHealth','ApplyAbsVelocityImpulseToClient','GetClientAbsOrigin','SetClientAbsOrigin',
        'GetEntityMoveType','SetEntityMoveType','IsValidEntHandle','FindConVar','IsConVarFlagSet',
        'GetConVarValue','SetConVarValue','ServerCommand','HookEvent','UnhookEvent',
        'GetEventPlayerSlot'
    )
    imomoe_native_base = @('GetClientUserID')
    permissions = @('HasPermission','CanAffectUser','LoadUser')
}
[IO.Directory]::CreateDirectory($bindingRoot) | Out-Null
$inputs = @{s2sdk=$S2SdkManifest; permissions=$PermissionsManifest; imomoe_native_base=$NativeBaseManifest}
foreach ($name in 's2sdk','permissions','imomoe_native_base') {
    $m = Get-Content -LiteralPath $inputs[$name] -Raw | ConvertFrom-Json -AsHashtable -Depth 100
    if ($m.name -ne $name) { throw "Expected plugin '$name', got '$($m.name)'." }
    $m.methods = @($m.methods | Where-Object name -In $names[$name])
    $missing = @($names[$name] | Where-Object { $_ -notin $m.methods.name })
    if ($missing.Count) { throw "Missing $name exports: $($missing -join ', ')" }
    $neededPrototypes = @($m.methods.paramTypes | Where-Object prototype | ForEach-Object prototype)
    if ($m.Contains('prototypes')) {
        $m.prototypes = @($m.prototypes | Where-Object name -In $neededPrototypes)
    }
    $neededEnums = @((@($m.methods.paramTypes) + @($m.methods.retType) + @($m.prototypes.paramTypes) + @($m.prototypes.retType)) |
        Where-Object enum | ForEach-Object enum)
    $m.enums = @($m.enums | Where-Object name -In $neededEnums)
    # Rust enums cannot declare multiple names for the same ABI discriminant.
    foreach ($enum in $m.enums) {
        $seen = [System.Collections.Generic.HashSet[long]]::new()
        $enum.values = @($enum.values | Where-Object { $seen.Add([long]$_.value) })
    }
    $m.Remove('classes')
    $manifest = Join-Path $bindingRoot "$name.pplugin"
    $m | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $manifest -Encoding utf8
    & $Generator -lang rust -manifest $manifest -output $out -overwrite
    if ($LASTEXITCODE -ne 0) { throw "plugify-gen failed for $name" }
}

