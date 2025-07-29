# setup.ps1
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$env:RUSTUP_HOME = "$root\.rustup"
$env:CARGO_HOME = "$root\.cargo_home"
$env:PATH = "$env:CARGO_HOME\bin;$env:PATH"

# Installiere rustup lokal
if (!(Test-Path "$env:RUSTUP_HOME\rustup-init.exe")) {
    Invoke-WebRequest https://win.rustup.rs -OutFile "$env:RUSTUP_HOME\rustup-init.exe"
    & "$env:RUSTUP_HOME\rustup-init.exe" --no-modify-path -y
}

Write-Host "Lokales Rustup Setup abgeschlossen. Du kannst jetzt mit 'cargo build' loslegen."
