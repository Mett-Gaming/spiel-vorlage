# MEIN SPIEL

Ein Satz, worum es geht.

Läuft auf der eigenen Rust/Vulkan-Engine [Mettgine](https://github.com/Mett-Gaming/mettgine) (Git-Untermodul in `mettgine/`) und wird über den [Mett Launcher](https://github.com/Mett-Gaming/mett-launcher) installiert. Ein Vulkan SDK ist zum Bauen nicht nötig.

- Credits und Lizenzen: [CREDITS.md](CREDITS.md)
- Anleitung für Spieler im Paket: [packaging/LIESMICH.txt](packaging/LIESMICH.txt)
- In den Launcher aufnehmen: [docs/launcher.md](docs/launcher.md)
- Arbeitsregeln für KI-Agenten: [AGENTS.md](AGENTS.md)

## Bauen und starten
```powershell
git clone --recurse-submodules https://github.com/Mett-Gaming/meinspiel
cd meinspiel
git -C mettgine checkout main
cargo run --release
cargo test --release
```

## Steuerung
| Taste | Aktion |
|---|---|
| WASD, Pfeile, linker Stick, Kreuz | bewegen |
| Esc, Start | Pause |
| F11 | Vollbild |

## Aufbau
| Datei | Inhalt |
|---|---|
| `src/game.rs` | Spielregeln ohne Grafik, mit Tests |
| `src/app.rs` | Fenster, Menüs, Eingabe, Bild |
| `src/hud.rs`, `src/sounds.rs` | Texte, Klänge |
| `src/settings.rs`, `src/achievements.rs` | Einstellungen, Erfolge |
| `assets/`, `spiel.toml` | Spieldaten und was davon ausgeliefert wird |
| `erfolge/erfolge.json` | Erfolge für den Launcher |

Einstellungen, Protokoll und Absturzberichte liegen in `%APPDATA%\.mettlauncher\meinspiel`.
