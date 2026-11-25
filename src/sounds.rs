//! Klaenge, beim Start im Code erzeugt. Klangdateien gehen auch: Feature `audio-files`, Datei nach
//! `assets/`, Eintrag in `spiel.toml` und `CREDITS.md`.

use crate::game::Event;
use mettgine::audio::synth::{envelope, tone, Wave};
use mettgine::audio::{Audio, Sound};

pub struct Sounds {
    audio: Audio,
    coin: Sound,
    won: Sound,
    lost: Sound,
    click: Sound,
}

fn beep(wave: Wave, from: f32, to: f32, secs: f32) -> Sound {
    let mut s = tone(wave, from, to, secs);
    envelope(&mut s, 0.005, secs * 0.5);
    Sound::new(s.into_iter().map(|v| v * 0.4).collect())
}

fn all() -> [Sound; 4] {
    [
        beep(Wave::Triangle, 660.0, 990.0, 0.15),
        beep(Wave::Square, 440.0, 880.0, 0.6),
        beep(Wave::Saw, 220.0, 80.0, 0.7),
        beep(Wave::Sine, 500.0, 500.0, 0.05),
    ]
}

impl Sounds {
    pub fn new(volume: f32) -> Self {
        let [coin, won, lost, click] = all();
        Self {
            audio: Audio::new(volume, 1.0),
            coin,
            won,
            lost,
            click,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.audio.set_volumes(volume, 1.0);
    }

    pub fn play(&mut self, e: Event) {
        let s = match e {
            Event::Coin => &self.coin,
            Event::Won => &self.won,
            Event::Lost => &self.lost,
        };
        self.audio.play(s, 1.0, 1.0);
    }

    pub fn click(&mut self) {
        self.audio.play(&self.click, 0.6, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sounds_are_clean() {
        for s in all() {
            assert!(!s.0.is_empty());
            assert!(s.0.iter().all(|v| v.is_finite() && v.abs() <= 1.0));
        }
    }
}
