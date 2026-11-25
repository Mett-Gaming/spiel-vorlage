# Vorlage für neue Spiele

Hiermit fängt ein neues Spiel für den [Mett Launcher](https://github.com/Mett-Gaming/mett-launcher) mit der Engine [Mettgine](https://github.com/Mett-Gaming/mettgine) an. Die Vorlage ist selbst ein kleines, spielbares Spiel (eine Kugel sammelt Ringe) und damit von der ersten Minute an lauffähig.

## Schon eingebaut
- Fenster, randloses Vollbild, Menüs, Optionen, Pause, Tastatur, Maus und Controller
- Spielregeln getrennt von der Grafik, mit Tests
- Einstellungen, Protokoll und Absturzberichte am richtigen Ort
- Erfolge, Overlay des Launchers und Discord-Anzeige
- Spieldaten als Pakete neben der exe, so lädt der Launcher nur Geändertes
- Bau, Tests und Veröffentlichung auf GitHub, ausgelöst durch eine Versionsmarke
- `AGENTS.md`: die Regeln, nach denen KI-Agenten hier arbeiten

## Voraussetzungen
Windows 10/11, [Git](https://git-scm.com), [Rust](https://rustup.rs) (mit den C++-Buildtools, die der Installer anbietet), eine Grafikkarte mit Vulkan 1.3. Zum Anlegen des Projektarchivs musst du Mitglied von `Mett-Gaming` sein.

## Neues Spiel anlegen
1. Oben auf dieser Seite **Use this template → Create a new repository**: Besitzer `Mett-Gaming`, Name = Kurzname des Spiels (nur Kleinbuchstaben und Ziffern), öffentlich.
2. Holen und einrichten, den Kurznamen überall gleich schreiben:
   ```powershell
   git clone https://github.com/Mett-Gaming/<kurzname>
   ```
   ```powershell
   cd <kurzname>
   ```
   ```powershell
   powershell -ExecutionPolicy Bypass -File einrichten.ps1 -Titel "Dein Titel" -Id <kurzname>
   ```
   Das Skript holt die Engine, setzt die Namen ein und löscht, was nur zur Vorlage gehört (auch sich selbst).
3. Ausprobieren:
   ```powershell
   cargo run --release
   ```
4. Eintragen und hochladen:
   ```powershell
   git add -A
   ```
   ```powershell
   git commit -m "✨ Spiel eingerichtet"
   ```
   ```powershell
   git push
   ```
5. Alles Weitere sagst du deinem Agenten in normaler Sprache. Er liest `AGENTS.md`.
6. Soll es in den Launcher, führt [docs/launcher.md](docs/launcher.md) durch die letzten Schritte.

Mit einem Agenten geht auch Schritt 2 bis 4 in einem Satz: „Richte dieses Projekt als *Titel* mit der Id *kurzname* ein.“

## Was du dir vorher überlegst
- **Kurzname und exe ändern sich nie wieder.** Der Launcher und die Spielstände hängen daran.
- **Kein Nachbau** eines bestehenden Spiels: eigener Name, eigene Figuren, eigener Look.
- **Nur freie Assets** (CC0 bevorzugt, CC-BY mit Namensnennung), jedes mit Eintrag in `CREDITS.md`.

## Die Vorlage selbst pflegen
Platzhalter sind `MEIN SPIEL` (Titel), `MEINSPIEL` (exe, Zip) und `meinspiel` (Kurzname). Der Ablauf „Vorlage prüfen“ richtet bei jedem Hochladen ein Probespiel ein und baut es. Neuer Engine-Stand: `git submodule update --remote --merge`, testen, `git add mettgine`, eintragen. Die Marke in `.github/workflows/build.yml` nur anheben, wenn sich der gemeinsame Ablauf geändert hat.
