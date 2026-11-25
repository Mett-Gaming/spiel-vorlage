# Macht aus der Vorlage ein eigenes Spiel: holt die Engine, setzt überall Name und Kurzname ein und
# räumt die Vorlagen-Dateien weg (auch sich selbst). Einmal ausführen, danach eintragen.
#
#   powershell -ExecutionPolicy Bypass -File einrichten.ps1 -Titel "MEIN NAME" -Id meinname
#
# -Titel  Name für Spieler, z. B. "KASSE 0"
# -Id     Kurzname: nur Kleinbuchstaben und Ziffern, beginnt mit einem Buchstaben. Daraus werden
#         Projektarchiv, Datenordner und (groß geschrieben) exe und Zip. Später nicht mehr ändern.
param(
    [Parameter(Mandatory = $true)][string]$Titel,
    [Parameter(Mandatory = $true)][string]$Id
)

$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot

function Stop-Setup([string]$text) {
    Write-Host "FEHLER: $text" -ForegroundColor Red
    exit 1
}

function Invoke-Git {
    & git @args
    if ($LASTEXITCODE -ne 0) { Stop-Setup "git $args ist fehlgeschlagen" }
}

if ($Id -cnotmatch '^[a-z][a-z0-9]{1,31}$') {
    Stop-Setup 'Die Id darf nur Kleinbuchstaben und Ziffern enthalten, beginnt mit einem Buchstaben und hat 2 bis 32 Zeichen.'
}
# Namen, die Rust oder die Engine selbst belegen
if (@('test', 'core', 'std', 'alloc', 'self', 'super', 'crate', 'mettgine', 'mpak', 'mettpack', 'meinspiel') -contains $Id) {
    Stop-Setup "Die Id '$Id' ist schon belegt. Bitte eine andere nehmen."
}
if ($Titel.Trim().Length -lt 2 -or $Titel.Length -gt 40 -or $Titel -match '["\\\r\n{}$`]') {
    Stop-Setup 'Der Titel braucht 2 bis 40 Zeichen, ohne Anführungszeichen, \, {, }, $ und `.'
}
$Titel = $Titel.Trim()
$Exe = $Id.ToUpperInvariant()

if (-not (Test-Path 'vorlage')) { Stop-Setup 'Dieses Projekt ist schon eingerichtet.' }

# Engine holen. Je nachdem, wie die Vorlage kopiert wurde, ist das Untermodul schon eingetragen.
$known = git ls-files --stage mettgine | Select-String '^160000'
if ($known) {
    Invoke-Git submodule update --init mettgine
} else {
    if (Test-Path 'mettgine') { Remove-Item 'mettgine' -Recurse -Force }
    Remove-Item '.gitmodules' -ErrorAction SilentlyContinue
    Invoke-Git submodule add -b main https://github.com/Mett-Gaming/mettgine.git mettgine
    Invoke-Git config -f .gitmodules submodule.mettgine.update merge
}
# auf dem Zweig arbeiten, nicht auf einem losen Eintrag; Stand bleibt der geprüfte der Vorlage
$pinned = git -C mettgine rev-parse HEAD
Invoke-Git -C mettgine checkout -q -B main $pinned
Invoke-Git -C mettgine branch -q --set-upstream-to=origin/main main

# Vorlagen-Dateien weg, README des Spiels an ihren Platz
Copy-Item 'vorlage/README-spiel.md' 'README.md' -Force
Remove-Item 'vorlage' -Recurse -Force
Remove-Item '.github/workflows/vorlage.yml' -ErrorAction SilentlyContinue

# Namen einsetzen (UTF-8 ohne BOM, Zeilenenden bleiben)
$utf8 = New-Object System.Text.UTF8Encoding($false)
$binary = '.ttf', '.png', '.k0m', '.k0s', '.k0a', '.glb', '.ogg', '.wav', '.ps1'
$files = Get-ChildItem -Recurse -File -Force | Where-Object {
    $rel = $_.FullName.Substring($PSScriptRoot.Length + 1).Replace('\', '/')
    $rel -notmatch '^(mettgine|target|dist|\.git)/' -and $binary -notcontains $_.Extension
}
foreach ($f in $files) {
    $old = [System.IO.File]::ReadAllText($f.FullName, $utf8)
    $new = $old.Replace('MEIN SPIEL', $Titel).Replace('MEINSPIEL', $Exe).Replace('meinspiel', $Id)
    if ($new -cne $old) { [System.IO.File]::WriteAllText($f.FullName, $new, $utf8) }
}

Remove-Item $PSCommandPath

Write-Host ''
Write-Host "$Titel ist eingerichtet." -ForegroundColor Green
Write-Host 'Weiter:'
Write-Host '  cargo run --release                          Spiel starten'
Write-Host '  cargo test --release                         Tests'
Write-Host '  cargo run --release -p mettpack -- pruefen   Beschreibung und Daten prüfen'
Write-Host '  git add -A'
Write-Host "  git commit -m `"✨ $Titel eingerichtet`""
Write-Host 'In den Launcher kommt es mit docs/launcher.md.'
