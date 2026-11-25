//! Erfolge melden (Beschreibung: `erfolge/erfolge.json`). Speichern, Einblenden und Liste macht der
//! Launcher; ohne Launcher passiert nichts.

use crate::game::Event;
use mettgine::launcher::Achievements;

pub const FIRST_COIN: &str = "erste_muenze";
pub const WON: &str = "geschafft";
pub const COLLECTOR: &str = "muenzsammler";

pub fn on_event(a: &Achievements, e: Event) {
    match e {
        Event::Coin => a.unlock(FIRST_COIN),
        Event::Won => a.unlock(WON),
        Event::Lost => {}
    }
}

/// Stand ueber alle Runden; auch einmal beim Start, damit nichts verloren geht, was ohne Launcher
/// gespielt wurde.
pub fn totals(a: &Achievements, coins: u32) {
    if coins > 0 {
        a.unlock(FIRST_COIN);
        a.progress(COLLECTOR, coins);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_achievement_in_the_code_is_described() {
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("erfolge/erfolge.json");
        let text = std::fs::read_to_string(file).unwrap();
        for id in [FIRST_COIN, WON, COLLECTOR] {
            assert!(text.contains(&format!("\"id\": \"{id}\"")), "{id} fehlt");
        }
    }
}
