# Bundling several languages in a single installer is a notoriously difficult and
# clunky task to achieve, because Windows Installer is just too complex for everything
# for its own good. Luckily, there is a relatively simple, Tauri-friendly way of doing
# it without bootstrappers, involving embedded transforms and an undocumented Windows
# Installer feature. See:
# http://www.installsite.org/pages/en/msi/articles/embeddedlang/index.htm
# https://www.firegiant.com/wix/tutorial/transforms/morphing-installers/
#
# Windows Installer will take care of using the language transform that better matches
# the configured regional settings of the running system, falling back to English if
# no transform matches.
#
# This script depends on "cargo tauri build" to be run beforehand, as it generates the
# required transforms from the locale-specific MSIs that command generates.

$ErrorActionPreference = "Stop"

# The languages the installer will be in, barring en-US, followed by their LANGID. Source:
# https://learn.microsoft.com/en-us/windows/win32/msi/localizing-the-error-and-actiontext-tables
$AdditionalLanguages = ("es-ES",3082)#,("fr-FR",1036),("ja-JP",1041)
$EN_US_LOCID = 1033
# The Tauri WiX bundler puts the WiX executables at the following directory. Source:
# https://github.com/tauri-apps/tauri/blob/2901145c497299f033ba7120af5f2e7ead16c75a/tooling/bundler/src/bundle/windows/msi.rs#L31-L32
$WixToolsPath = "$env:LOCALAPPDATA\tauri\WixTools"

if ($env:CARGO_BUILD_TARGET -eq $null)
{
  Push-Location target\release\wix\x64
}
else
{
  Push-Location "target\$env:CARGO_BUILD_TARGET\release\wix\x64"
}

$SourceInstaller = (Get-Item ..\..\bundle\msi\*_en-US.msi).FullName
$InstallerPrefix = $SourceInstaller.TrimEnd("_en-US.msi")
$TargetInstaller = $InstallerPrefix + ".msi"
Copy-Item $SourceInstaller $TargetInstaller

# The GitHub CI runner Windows image probably has these Windows SDK scripts somewhere, but play it safe
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/microsoft/Windows-classic-samples/main/Samples/Win7Samples/sysmgmt/msi/scripts/WiSubStg.vbs" -OutFile "WiSubStg.vbs"
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/microsoft/Windows-classic-samples/main/Samples/Win7Samples/sysmgmt/msi/scripts/WiLangId.vbs" -OutFile "WiLangId.vbs"

$languageCodes = ,$EN_US_LOCID
foreach ($language in $AdditionalLanguages)
{
  $languageName, $languageCode = $language

  Write-Host "> Generating and embedding transform for $languageName ($languageCode)..."
  & "$WixToolsPath\torch.exe" -p -t language "$TargetInstaller" "${InstallerPrefix}_${languageName}.msi" -out "${languageCode}.mst"
  cscript WiSubStg.vbs "$TargetInstaller" "${languageCode}.mst" "${languageCode}"

  $languageCodes += $languageCode
}

Write-Host "> Finalizing multi-language $TargetInstaller..."
cscript WiLangId.vbs "$TargetInstaller" Package ($languageCodes -join ',')

# Luckily, Tauri's updater just installs update MSIs in passive mode due to our configuration,
# showing no GUI in most cases, so we can just use the en-US installer it generates and signs
# as-is. See:
# https://github.com/tauri-apps/tauri/blob/2901145c497299f033ba7120af5f2e7ead16c75a/core/tauri/src/updater/core.rs#L611-L623
Write-Host "- Done!"
Write-Host "  Distribution MSI: ${TargetInstaller}"
Write-Host "  Update MSI: ${SourceInstaller}, ${SourceInstaller}.zip and ${SourceInstaller}.zip.sig"

Pop-Location
