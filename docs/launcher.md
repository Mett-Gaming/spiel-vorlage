# In den Mett Launcher aufnehmen

Der Launcher zeigt ein Spiel, sobald es im Katalog [launcher-catalog](https://github.com/Mett-Gaming/launcher-catalog) steht und mindestens eine Veröffentlichung hat.

## 1. Erste Veröffentlichung
1. `cargo run --release -p mettpack -- pruefen` und `cargo test --release` sind grün, alles ist eingetragen und auf `main` hochgeladen.
2. Versionsmarke setzen und hochladen:
   ```powershell
   git tag -a v0.1.0 -m "MEIN SPIEL v0.1.0"
   ```
   ```powershell
   git push origin v0.1.0
   ```
3. GitHub baut und veröffentlicht `MEINSPIEL-v0.1.0.zip` und `MEINSPIEL-v0.1.0.dateien.json`. Danach nichts mehr daran austauschen.
4. Versionshinweise durch eine kurze Liste für Spieler ersetzen. Der Launcher zeigt sie an.

## 2. Bilder
Im Katalog unter `covers/`, keine fremden Marken, nur Eigenes oder Freies:

| Datei | Größe | Inhalt |
|---|---|---|
| `covers/meinspiel.png` | 600 × 800 | Titelbild mit Namen |
| `covers/meinspiel-hero.png` | 1920 × 620 | breites Bild **ohne Text**, der Launcher schreibt den Namen darüber |

## 3. Eintrag im Katalog
Der fertige Eintrag steht in [katalog-eintrag.json](katalog-eintrag.json): `tagline` ausfüllen und in `games.json` des Katalogs in die Liste `games` einfügen. Alle Felder erklärt die README des Katalogs. Nach dem Hochladen dort prüft ein Ablauf die Datei; beim nächsten Start zeigen alle Launcher das Spiel.

In andere Projektarchive lädt ein Agent nur auf ausdrückliche Bitte hoch.

## 4. Optional
- **Neuigkeit:** Eintrag in [news](https://github.com/Mett-Gaming/news).
- **Mehrspieler:** `"multiplayer": true` im Katalog, im Spiel `mettgine::net` (Vorbild: HACKEPETER).
- **Engine-Liste:** In `mettgine/AGENTS.md` stehen die Spiele, die bei Engine-Änderungen mitgeprüft werden. Aufnehmen nur auf Wunsch.

## Prüfliste
- [ ] Kurzname im Projektarchiv, in `spiel.toml`, in `src/settings.rs` und im Katalog gleich
- [ ] `assetPattern` passt auf den Zip-Namen, `exe` stimmt
- [ ] Spiel startet aus dem entpackten Zip ohne Launcher
- [ ] Nichts wird neben die exe geschrieben
- [ ] `CREDITS.md` und `packaging/LIESMICH.txt` sind vollständig
- [ ] Beispiel-Erfolge durch eigene ersetzt (Ids danach nie mehr ändern)
