//! Spielregeln des Beispiels: eine Kugel sammelt Münzen, bevor die Zeit um ist. Keine Grafik, kein
//! Ton, keine Eingabegeraete: So laesst sich alles testen. Hier faengt dein eigenes Spiel an.

use glam::{vec2, Vec2};
use mettgine::random::Random;

/// Schritte pro Sekunde
pub const STEP: f32 = 1.0 / 60.0;
/// Halbe Kantenlaenge des Spielfelds (Meter)
pub const FIELD: f32 = 10.0;
/// Radius der Kugel und der Münzen (Meter)
pub const RADIUS: f32 = 0.5;
pub const COIN_RADIUS: f32 = 0.4;
/// Münzen je Runde
pub const COINS: usize = 12;
/// Dauer einer Runde (Sekunden)
pub const ROUND: f32 = 30.0;
/// Beschleunigung (m/s^2) und Bremsen (1/s)
const PUSH: f32 = 40.0;
const DRAG: f32 = 4.0;

/// Wunsch des Spielers fuer einen Schritt.
#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    /// Richtung: x nach rechts, y nach hinten, hoechstens Laenge 1
    pub dir: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Running,
    Won,
    Lost,
}

/// Was in einem Schritt passiert ist (fuer Ton und Erfolge).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Coin,
    Won,
    Lost,
}

/// Eine Runde.
#[derive(Clone, Debug)]
pub struct Game {
    pub pos: Vec2,
    pub vel: Vec2,
    pub coins: Vec<Vec2>,
    pub collected: u32,
    pub time_left: f32,
    pub outcome: Outcome,
}

impl Game {
    /// Gleicher Startwert = gleiche Runde.
    pub fn new(seed: u64) -> Self {
        let mut rng = Random::new(seed);
        let edge = FIELD - 1.0;
        let mut coins: Vec<Vec2> = Vec::new();
        while coins.len() < COINS {
            let p = vec2(rng.range(-edge, edge), rng.range(-edge, edge));
            // nicht auf dem Start und nicht ineinander
            if p.length() > 2.0 && coins.iter().all(|r| r.distance(p) > 1.5) {
                coins.push(p);
            }
        }
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            coins,
            collected: 0,
            time_left: ROUND,
            outcome: Outcome::Running,
        }
    }

    /// Ein Schritt von [`STEP`] Sekunden.
    pub fn update(&mut self, input: &Input) -> Vec<Event> {
        let mut events = Vec::new();
        if self.outcome != Outcome::Running {
            return events;
        }
        self.vel += input.dir.clamp_length_max(1.0) * PUSH * STEP;
        self.vel *= (-DRAG * STEP).exp();
        self.pos += self.vel * STEP;
        let limit = FIELD - RADIUS;
        let inside = self.pos.clamp(Vec2::splat(-limit), Vec2::splat(limit));
        if inside.x != self.pos.x {
            self.vel.x = 0.0;
        }
        if inside.y != self.pos.y {
            self.vel.y = 0.0;
        }
        self.pos = inside;

        let before = self.coins.len();
        let pos = self.pos;
        self.coins
            .retain(|r| r.distance(pos) > RADIUS + COIN_RADIUS);
        for _ in self.coins.len()..before {
            self.collected += 1;
            events.push(Event::Coin);
        }
        self.time_left = (self.time_left - STEP).max(0.0);
        if self.coins.is_empty() {
            self.outcome = Outcome::Won;
            events.push(Event::Won);
        } else if self.time_left <= 0.0 {
            self.outcome = Outcome::Lost;
            events.push(Event::Lost);
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn towards(g: &Game) -> Input {
        let near = g
            .coins
            .iter()
            .min_by(|a, b| a.distance(g.pos).total_cmp(&b.distance(g.pos)));
        Input {
            dir: near.map_or(Vec2::ZERO, |r| (*r - g.pos).normalize_or_zero()),
        }
    }

    #[test]
    fn same_seed_gives_the_same_round() {
        assert_eq!(Game::new(7).coins, Game::new(7).coins);
        assert_ne!(Game::new(7).coins, Game::new(8).coins);
    }

    #[test]
    fn collecting_everything_wins_in_time() {
        for seed in 0..20 {
            let mut g = Game::new(seed);
            let mut coins = 0;
            while g.outcome == Outcome::Running {
                let input = towards(&g);
                let events = g.update(&input);
                coins += events.iter().filter(|e| **e == Event::Coin).count();
            }
            assert_eq!(
                g.outcome,
                Outcome::Won,
                "Runde {seed} ist nicht zu schaffen"
            );
            assert_eq!(coins, COINS);
        }
    }

    #[test]
    fn standing_still_loses_and_then_nothing_happens_anymore() {
        let mut g = Game::new(1);
        let mut lost = 0;
        for _ in 0..(ROUND / STEP) as usize + 10 {
            let events = g.update(&Input::default());
            lost += events.iter().filter(|e| **e == Event::Lost).count();
        }
        assert_eq!((g.outcome, lost), (Outcome::Lost, 1));
    }

    #[test]
    fn the_ball_stays_on_the_field() {
        let mut g = Game::new(2);
        for k in 0..600 {
            let dir = if k % 200 < 100 {
                vec2(1.0, 1.0)
            } else {
                vec2(-1.0, 1.0)
            };
            g.update(&Input { dir });
            assert!(g.pos.abs().max_element() <= FIELD - RADIUS + 1e-4);
        }
    }
}
