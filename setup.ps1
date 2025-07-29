# ================================
# setup.ps1 – Rust Umgebung Setup
# ================================

# Setze Umgebungsvariablen
$env:RUSTUP_HOME = "$PSScriptRoot\.rustup"
$env:CARGO_HOME = "$PSScriptRoot\.cargo_home"

# Stelle sicher, dass Verzeichnisse existieren
foreach ($dir in @($env:RUSTUP_HOME, $env:CARGO_HOME)) {
    if (!(Test-Path $dir)) {
        try {
            New-Item -ItemType Directory -Path $dir | Out-Null
            Write-Host "Verzeichnis erstellt: $dir"
        } catch {
            Write-Error "Konnte Verzeichnis nicht erstellen: $dir"
            exit 1
        }
    }
}

# Download rustup-init.exe
$rustupExe = "$env:RUSTUP_HOME\rustup-init.exe"
if (!(Test-Path $rustupExe)) {
    try {
        Write-Host "Lade rustup-init.exe herunter..."
        Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile $rustupExe
        Write-Host "Download erfolgreich: $rustupExe"
    } catch {
        Write-Error "Download fehlgeschlagen: $_"
        exit 1
    }
} else {
    Write-Host "rustup-init.exe bereits vorhanden, überspringe Download."
}

# Führe Installer aus
try {
    Write-Host "Starte Installation..."
    & $rustupExe --no-modify-path -y
    Write-Host "Rust erfolgreich installiert!"
} catch {
    Write-Error "Installation fehlgeschlagen: $_"
    exit 1
}
