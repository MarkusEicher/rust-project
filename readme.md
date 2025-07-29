Lokal isolierte Rust Entwicklungsumgebung für Windows

Projekt klonen
git clone https://github.com/dein-benutzer/rust-projekt.git
cd rust-projekt

Rust-Toolchain installieren
Öffne ein Terminal in VS Code.
Starte das Setup-Skript:
.\setup.ps1


Automatische Terminal-Konfiguration
Die Datei .vscode/settings.json sorgt dafür, dass beim Öffnen des Terminals folgende Umgebungsvariablen gesetzt werden:
RUSTUP_HOME = <projektverzeichnis>\.rustup
CARGO_HOME = <projektverzeichnis>\.cargo_home
PATH wird automatisch um das lokale Cargo-Bin-Verzeichnis ergänzt
✅ Dadurch bleibt deine globale Rust-Installation unberührt – alles läuft isoliert im Projektordner!

📦 Projektstruktur
.
├── src/                  # Rust-Quellcode
├── .cargo_home/          # Lokale Cargo Umgebung (vom Git ignoriert)
├── .rustup/              # Lokaler Rustup Installer & Toolchain (vom Git ignoriert)
├── .vscode/              # Projekt-Editor-Einstellungen
│   └── settings.json     # Enthält Umgebungsvariablen fürs Terminal
├── setup.ps1             # Automatisches Rust-Setup
├── README.md
└── .gitignore
