if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host " [!] ERROR: Please run PowerShell as ADMINISTRATOR." -ForegroundColor Red
    Write-Host "Press any key to continue..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
    exit
}

$steamRegistry = Get-ItemProperty -Path "HKCU:\Software\Valve\Steam" -ErrorAction SilentlyContinue
if ($null -ne $steamRegistry -and $null -ne $steamRegistry.SteamPath) {
    $steamPath = $steamRegistry.SteamPath.Replace("/", "\")
}
else {
    $steamPath = "C:\Program Files (x86)\Steam"
}

$asciiArt = @'
  ____                      ____  _                         ____ _     ___ 
 / __ \                    / ___|| |_ ___  __ _ _ __ ___   / ___| |   |_ _|
| |  | |_ __   ___ _ __   \___ \| __/ _ \/ _` | '_ ` _ \  | |   | |    | | 
| |__| | '_ \ / _ \ '_ \   ___) | ||  __/ (_| | | | | | | | |___| |___ | | 
 \____/| .__/ \___|_| |_| |____/ \__\___|\__,_|_| |_| |_|  \____|_____|___|
       |_|
    Github: Abrahamqb
    Twitter: TheJbrequi                                                                  
'@

Clear-Host
Write-Host $asciiArt -ForegroundColor Cyan
Write-Host " =====================================================================" -ForegroundColor Gray

$tempFolder = $env:TEMP
$desktopPath = [System.Environment]::GetFolderPath("Desktop")

if (Test-Path $steamPath) {
    Write-Host " Steam is installed in the default location: $steamPath." -ForegroundColor Blue
}
else {
    Write-Host " Steam is not installed in the default location: $steamPath" -ForegroundColor Red
}
Write-Host " =====================================================================" -ForegroundColor Gray

Start-Sleep -Seconds 2

#1
function PatchSteam {
    Clear-Host
    Write-Host " --- Patch Steam --- " -ForegroundColor Cyan
    if (-not (Test-Path $steamPath)) {
        Write-Host " Steam path not found: $steamPath" -ForegroundColor Red
        Write-Host " Press any key to continue..." -ForegroundColor Gray
        $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
        return
    }

    Write-Host " Steam found at: $steamPath" -ForegroundColor Blue
    Write-Host " Downloading inject.zip, please wait..." -ForegroundColor Gray

    $tempPath = Join-Path $steamPath "temp"
    if (-not (Test-Path $tempPath)) { New-Item -ItemType Directory -Path $tempPath | Out-Null }
    $zipPath = Join-Path $tempPath "inject.zip"

    try {
        $headers = @{ "User-Agent" = "OpenSteamManager" }
        Invoke-WebRequest -Uri "https://github.com/Abrahamqb/OpenSteamMore-Dev/releases/latest/download/inject.zip" `
            -OutFile $zipPath -Headers $headers -ErrorAction Stop

        Write-Host " Extracting files to Steam folder..." -ForegroundColor Gray
        Expand-Archive -Path $zipPath -DestinationPath $steamPath -Force

        Remove-Item $zipPath -Force -ErrorAction SilentlyContinue

        Write-Host " Steam Patched!" -ForegroundColor Green
    }
    catch {
        Write-Host " Error: $($_.Exception.Message)" -ForegroundColor Red
    }
    Write-Host " Press any key to continue..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
}

#2
function DeletePatchSteam {
    Clear-Host
    Write-Host " --- Unpatch Steam --- " -ForegroundColor Cyan

    $jsonPath = Join-Path $steamPath "OpenSteamDel.json"
    if (-not (Test-Path $jsonPath)) {
        Write-Host " OpenSteamDel.json not found. Nothing to remove." -ForegroundColor Yellow
        Write-Host " Press any key to continue..." -ForegroundColor Gray
        $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
        return
    }

    try {
        $jsonContent = Get-Content -Path $jsonPath -Raw
        $filesToDelete = $jsonContent | ConvertFrom-Json

        if ($null -ne $filesToDelete) {
            foreach ($file in $filesToDelete) {
                $filePath = Join-Path $steamPath $file
                if (Test-Path $filePath) {
                    Remove-Item $filePath -Force
                    Write-Host " Deleted: $file" -ForegroundColor Gray
                }
            }
        }

        Write-Host " Unpatched Steam!" -ForegroundColor Green
    }
    catch {
        Write-Host " Error: $($_.Exception.Message)" -ForegroundColor Red
    }
    Write-Host " Press any key to continue..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
}

#3
function SearchGame {
    Clear-Host
    Write-Host " --- Search & Load Game Lua and Manifest --- " -ForegroundColor Cyan

    $InternalAPI = "https://api.steamproof.net"
    $DepotKeysUrl = "https://gitlab.com/steamautocracks/manifesthub/-/raw/main/depotkeys.json"
    $CacheDays = 7

    $ID = Read-Host " Enter the Game ID (e.g., 12345)"
    if ([string]::IsNullOrWhiteSpace($ID)) { return }
    $ID = $ID.Trim()

    $luaPathSteam = Join-Path $steamPath "config\stplug-in"
    $manifestPathSteam = Join-Path $steamPath "depotcache"
    $cachePath = Join-Path $steamPath "cache"
    $depotKeysCacheFile = Join-Path $cachePath "depotkeys.json"

    try {
        if (-not (Test-Path $luaPathSteam)) { New-Item -ItemType Directory -Path $luaPathSteam -Force | Out-Null }
        if (-not (Test-Path $manifestPathSteam)) { New-Item -ItemType Directory -Path $manifestPathSteam -Force | Out-Null }
        if (-not (Test-Path $cachePath)) { New-Item -ItemType Directory -Path $cachePath -Force | Out-Null }

        Write-Host " Syncing with SteamProof API..." -ForegroundColor Blue

        $wc = New-Object System.Net.WebClient
        $wc.Headers.Add("User-Agent", "OpenSteam-Manager/1.0")
        $wc.Encoding = [System.Text.Encoding]::UTF8

        $depotKeys = @{}

        $shouldRefreshCache = $true
        if (Test-Path $depotKeysCacheFile) {
            $lastWriteUtc = (Get-Item $depotKeysCacheFile).LastWriteTimeUtc
            $isExpired = $lastWriteUtc -lt (Get-Date).ToUniversalTime().AddDays(-$CacheDays)

            if (-not $isExpired) {
                try {
                    $cachedKeysRaw = Get-Content -Path $depotKeysCacheFile -Raw -Encoding UTF8
                    $cachedKeysObj = $cachedKeysRaw | ConvertFrom-Json -AsHashtable
                    if ($cachedKeysObj) {
                        $depotKeys = $cachedKeysObj
                        $shouldRefreshCache = $false
                        Write-Host " Using cached depotkeys.json" -ForegroundColor Gray
                    }
                }
                catch {
                    $shouldRefreshCache = $true
                }
            }
        }

        if ($shouldRefreshCache) {
            try {
                Write-Host " Downloading fresh depotkeys.json..." -ForegroundColor Gray
                $keysRaw = $wc.DownloadString($DepotKeysUrl)
                [System.IO.File]::WriteAllText($depotKeysCacheFile, $keysRaw, (New-Object System.Text.UTF8Encoding $false))
                $depotKeys = $keysRaw | ConvertFrom-Json -AsHashtable
            }
            catch {
                if (Test-Path $depotKeysCacheFile) {
                    Write-Host " Warning: failed to refresh depotkeys.json, using old cache." -ForegroundColor Yellow
                    $cachedKeysRaw = Get-Content -Path $depotKeysCacheFile -Raw -Encoding UTF8
                    $depotKeys = $cachedKeysRaw | ConvertFrom-Json -AsHashtable
                }
                else {
                    throw "No se pudo descargar depotkeys.json y no existe caché local."
                }
            }
        }

        $urlInfo = "$InternalAPI/apps/depots?ids=$ID"
        $urlDownload = "$InternalAPI/app/$ID/manifests/download"

        $infoRaw = $wc.DownloadString($urlInfo)
        $dlRaw = $wc.DownloadString($urlDownload)

        $infoResp = $infoRaw | ConvertFrom-Json
        $dlResp = $dlRaw | ConvertFrom-Json

        if ($infoResp.apps.Count -eq 0) {
            throw "La API no devolvió apps."
        }

        $appData = $infoResp.apps[0]
        $finalLuaFile = Join-Path $luaPathSteam "$ID.lua"

        $sb = New-Object System.Text.StringBuilder
        [void]$sb.AppendLine("-- Open Steam Lua Generator")
        [void]$sb.AppendLine("addappid($ID)")

        foreach ($depot in $appData.depots) {
            $dId = [string]$depot.depotId

            if ($depotKeys.ContainsKey($dId) -and -not [string]::IsNullOrWhiteSpace($depotKeys[$dId])) {
                [void]$sb.AppendLine("addappid($dId,1,`"$($depotKeys[$dId])`")")
            }
            else {
                [void]$sb.AppendLine("addappid($dId,0,`"`")")
            }
        }

        [System.IO.File]::WriteAllText($finalLuaFile, $sb.ToString(), (New-Object System.Text.UTF8Encoding $false))
        Write-Host " Base Lua written: $finalLuaFile" -ForegroundColor Green

        Write-Host " Downloading physical manifests..." -ForegroundColor Gray
        foreach ($m in $dlResp.manifests) {
            $destManifest = Join-Path $manifestPathSteam "$($m.depotId)_$($m.manifestId).manifest"
            if (-not (Test-Path $destManifest)) {
                $wc.DownloadFile($m.url, $destManifest)
            }
        }

        $ts = (Get-Date).ToUniversalTime().ToString("yyyy-MM-dd HH:mm UTC")
        $section = "`r`n`r`n-- SteamProof Manifests (updated $ts)"

        foreach ($m in $dlResp.manifests) {
            $dId = $m.depotId
            $dInfo = $appData.depots | Where-Object { $_.depotId -eq $dId } | Select-Object -First 1
            $mSize = $null

            if ($dInfo -and $dInfo.PSObject.Properties.Name -contains "maxSize") {
                $mSize = $dInfo.maxSize
            }

            if ($mSize) {
                $section += "`r`nsetManifestid($dId, `"$($m.manifestId)`", $mSize)"
            }
            else {
                $section += "`r`nsetManifestid($dId, `"$($m.manifestId)`")"
            }
        }

        [System.IO.File]::AppendAllText($finalLuaFile, $section, (New-Object System.Text.UTF8Encoding $false))
        Write-Host " All done! API manifests injected." -ForegroundColor Green

        if (Get-Command RestartSteam -ErrorAction SilentlyContinue) { RestartSteam }
    }
    catch {
        Write-Host " Error: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host " Debug: Intento conectar a $InternalAPI" -ForegroundColor DarkGray
    }
    finally {
        if ($wc) { $wc.Dispose() }
        Write-Host "`n Press any key to return..." -ForegroundColor Gray
        $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
    }
}

#4
function InstallMillennium {
    Clear-Host
    write-Host " --- Install Millennium (Plugin Loader) --- " -ForegroundColor Cyan
    Write-Host "Installing Millennium..." -ForegroundColor Gray
    Invoke-WebRequest -Uri "https://github.com/SteamClientHomebrew/Installer/releases/latest/download/MillenniumInstaller-Windows.exe" -OutFile "$tempFolder\Millennium.exe"
    Start-Process "$tempFolder\Millennium.exe" -Wait
    if (Test-Path "$steamPath\millennium.dll") {
        Write-Host "Millennium installed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Installation finished. Please verify millennium.dll in Steam folder." -ForegroundColor Yellow
    }
    Remove-Item "$tempFolder\Millennium.exe" -ErrorAction SilentlyContinue
    pause
}

#5
function DownloadLuaManager {
    Clear-Host
    write-Host " --- Download LuaManager (Plugin) --- " -ForegroundColor Cyan
    if (-not (Test-Path "$steamPath\plugins")) { New-Item -ItemType Directory -Path "$steamPath\plugins" | Out-Null }
    Write-Host "Downloading LuaManager..." -ForegroundColor Gray
    Invoke-WebRequest -Uri "https://github.com/Abrahamqb/OpenSteam/raw/refs/heads/master/Plugins/LuaManager.zip" -OutFile "$tempFolder\LuaManager.zip"
    Write-Host "Extracting to plugins folder..." -ForegroundColor Green
    Expand-Archive -Path "$tempFolder\LuaManager.zip" -DestinationPath "$steamPath\plugins\" -Force
    Write-Host "Done! Remember to activate it in Millennium settings." -ForegroundColor Yellow
    pause
}

#6
function InstallOpenSteamDesktop {
    Clear-Host
    Write-Host " --- Install OpenSteam.exe to Desktop --- " -ForegroundColor Cyan
    Write-Host "Downloading on OpenSteam to desktop..." -ForegroundColor Blue
    Invoke-WebRequest -Uri "https://github.com/Abrahamqb/OpenSteam/releases/latest/download/OpenSteam.exe" -OutFile "$desktopPath\OpenSteam.exe"
    Write-Host "Executable created successfully!" -ForegroundColor Green
    Write-Host "Press any key to continue..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
    Start-Sleep -Seconds 2
}

#7
function RestartSteam {
    Clear-Host
    Write-Host " --- Restart Steam --- " -ForegroundColor Cyan
    Write-Host "Restarting Steam..." -ForegroundColor Gray
    Get-Process -Name "Steam" -ErrorAction SilentlyContinue | Stop-Process -Force
    Get-Process -Name "steamwebhelper" -ErrorAction SilentlyContinue | Stop-Process -Force
    Start-Sleep -Seconds 2
    Start-Process "$steamPath\Steam.exe"
    Write-Host "Steam restarted successfully!" -ForegroundColor Green
    Start-Sleep -Seconds 2
}

while ($true) {
    Clear-Host
    Write-Host $asciiArt -ForegroundColor Cyan

    Write-Host " =====================================================================" -ForegroundColor Gray
    $patchStatus = if (Test-Path "$steamPath\xinput1_4.dll") { "[PATCHED]" } else { "[NOT PATCHED]" }
    $statusColor = if ($patchStatus -eq "[PATCHED]") { "Green" } else { "Red" }
    Write-Host " 1. Patch Steam $patchStatus" -ForegroundColor $statusColor
    Write-Host " 2. Remove Patch" -ForegroundColor Red
    Write-Host " 3. Search Game" -ForegroundColor Blue
    Write-Host " 4. Install Millennium (Plugin Loader)" -ForegroundColor Yellow
    Write-Host " 5. Download LuaManager (Plugin)" -ForegroundColor Yellow
    Write-Host " 6. Download OpenSteam.exe to Desktop" -ForegroundColor Blue
    Write-Host " 7. Restart Steam" -ForegroundColor Magenta
    Write-Host " 8. Exit" -ForegroundColor Red
    Write-Host " =====================================================================" -ForegroundColor Gray
    
    $choice = Read-Host " Enter your choice"
    switch ($choice) {
        "1" { PatchSteam }
        "2" { DeletePatchSteam }
        "3" { SearchGame }
        "4" { InstallMillennium }
        "5" { DownloadKernelLua }
        "6" { InstallOpenSteamDesktop }
        "7" { RestartSteam }
        "8" { exit }
        default { 
            Write-Host " Invalid choice!" -ForegroundColor Red
            Start-Sleep -Seconds 1 
        }
    }
}

