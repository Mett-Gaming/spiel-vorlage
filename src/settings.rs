//! Einstellungen und Zaehler in %APPDATA%\.mettlauncher\meinspiel (Datei: mettgine::settings).
//! Nie neben die exe schreiben: Der Launcher ersetzt den Installordner.

use mettgine::settings::{self, KeyValues};

/// Kurzname fuer Ordner, Protokoll und Absturzberichte; nie wieder aendern
pub const APP: &str = "meinspiel";
const FILE: &str = "settings.txt";

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub volume: f32,
    pub fullscreen: bool,
    /// Weniger Verzoegerung (Reflex, Anti-Lag)
    pub low_latency: bool,
    /// Münzen ueber alle Runden (Erfolg "muenzsammler")
    pub total_coins: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume: 0.8,
            fullscreen: true,
            low_latency: true,
            total_coins: 0,
        }
    }
}

impl Settings {
    pub fn from_kv(kv: &KeyValues) -> Self {
        let d = Self::default();
        Self {
            volume: kv.f32("volume", d.volume, 0.0, 1.0),
            fullscreen: kv.bool("fullscreen", d.fullscreen),
            low_latency: kv.bool("low_latency", d.low_latency),
            total_coins: kv.u32("total_coins", d.total_coins, 0, u32::MAX),
        }
    }

    pub fn to_kv(&self) -> KeyValues {
        let mut kv = KeyValues::default();
        kv.set("volume", self.volume);
        kv.set("fullscreen", self.fullscreen);
        kv.set("low_latency", self.low_latency);
        kv.set("total_coins", self.total_coins);
        kv
    }

    pub fn load() -> Self {
        Self::from_kv(&settings::load(APP, FILE))
    }

    pub fn save(&self) {
        settings::save(APP, FILE, &self.to_kv());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_survive_saving_and_nonsense_is_ignored() {
        let s = Settings {
            volume: 0.3,
            fullscreen: false,
            low_latency: false,
            total_coins: 42,
        };
        assert_eq!(
            Settings::from_kv(&KeyValues::parse(&s.to_kv().to_text())),
            s
        );
        let bad = Settings::from_kv(&KeyValues::parse("volume=7\ntotal_coins=abc\n"));
        assert_eq!((bad.volume, bad.total_coins), (1.0, 0));
    }
}
