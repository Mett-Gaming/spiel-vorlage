//! MEIN SPIEL. Engine: Mettgine (Untermodul `mettgine/`).
//!
//! `app` verdrahtet Fenster, Menues und Bild, `game` enthaelt die Spielregeln (ohne Grafik, gut
//! zu testen), `hud` zeichnet Texte, `achievements`, `settings` und `sounds` sagen, was sie tun.

// Veroeffentlichter Bau ohne Konsolenfenster
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod achievements;
mod app;
mod game;
mod hud;
mod settings;
mod sounds;

use app::App;
use mettgine::app::AppConfig;
use mettgine::asset::data;
use mettgine::text::FontAtlas;

/// Version aus der Versionsmarke (build.rs), fuer Absturzberichte und Menue
pub const VERSION: &str = env!("MEINSPIEL_VERSION");
pub const TITLE: &str = "MEIN SPIEL";
const FONT: &str = "fonts/ArchivoBlack-Regular.ttf";

fn main() -> mettgine::app::Result<()> {
    let config = AppConfig {
        max_dt: 0.05,
        ..AppConfig::new(settings::APP, TITLE, VERSION)
    };
    mettgine::app::run(config, || {
        let font = FontAtlas::bake(&data::read(FONT))?;
        Ok(App::new(font))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_is_there_and_knows_umlauts() {
        let a = FontAtlas::bake(&data::read(FONT)).unwrap();
        for c in ['A', 'z', '0', 'ä', 'Ö', 'ß', '!'] {
            assert!(a.glyph(c).advance > 0.0, "Zeichen fehlt: {c}");
        }
    }
}
