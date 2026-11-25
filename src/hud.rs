//! Texte ueber dem Bild (virtuelle Flaeche, 720 hoch).

use crate::game::{Game, Outcome, RINGS};
use mettgine::canvas::{Align, Canvas};

const WHITE: [f32; 4] = [0.95, 0.95, 0.92, 1.0];
const DIM: [f32; 4] = [0.7, 0.7, 0.68, 1.0];
const GOLD: [f32; 4] = [1.0, 0.8, 0.25, 1.0];

/// Stand der laufenden Runde. Unten rechts und oben links frei lassen: Dort blendet der Launcher
/// Erfolge und seine Leiste ein.
pub fn game(c: &mut Canvas, g: &Game) {
    let mid = c.w * 0.5;
    let time = format!("{:.0}", g.time_left.ceil());
    c.text_s(mid, 20.0, &time, 46.0, WHITE, Align::Center);
    let rings = format!("{} / {RINGS}", g.collected);
    c.text_s(mid, 72.0, &rings, 28.0, GOLD, Align::Center);
}

/// Kopf eines Menues, dahinter abgedunkelt.
pub fn menu_head(c: &mut Canvas, title: &str, below: &str) {
    let (w, h) = (c.w, c.h);
    c.rect(0.0, 0.0, w, h, [0.0, 0.0, 0.0, 0.55]);
    c.text_s(w * 0.5, 110.0, title, 72.0, WHITE, Align::Center);
    c.text_s(w * 0.5, 200.0, below, 24.0, DIM, Align::Center);
}

pub fn version(c: &mut Canvas) {
    let h = c.h;
    c.text(24.0, h - 36.0, crate::VERSION, 18.0, DIM, Align::Left);
}

pub fn result(o: Outcome) -> &'static str {
    match o {
        Outcome::Won => "Alle Ringe gesammelt!",
        Outcome::Lost => "Die Zeit ist um.",
        Outcome::Running => "",
    }
}
