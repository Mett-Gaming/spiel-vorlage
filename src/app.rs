//! Fenster, Menues, Eingabe und Bild. Die Regeln stehen in `game`.

use crate::game::{
    Event as GameEvent, Game as Round, Input, Outcome, FIELD, RADIUS, RING_RADIUS, STEP,
};
use crate::settings::Settings;
use crate::sounds::Sounds;
use crate::{achievements, hud, TITLE};
use glam::{vec2, vec3, Mat4, Vec2, Vec3};
use mettgine::app::{Game, Result, Stage};
use mettgine::canvas::Canvas;
use mettgine::input::{Bind, Key, Pad};
use mettgine::menu::{Event, Item, Kind, Menu, Style};
use mettgine::presence::Status;
use mettgine::render::{Config, FrameInput, Lighting, LowLatency, MeshDraw, Post, Renderer, Sun};
use mettgine::shapes;
use mettgine::text::FontAtlas;
use mettgine::time::FixedStep;
use winit::event::WindowEvent;
use winit::window::Window;

const LEFT: &[Bind] = &[
    Bind::Key(Key::ArrowLeft),
    Bind::Key(Key::KeyA),
    Bind::Pad(Pad::Left),
];
const RIGHT: &[Bind] = &[
    Bind::Key(Key::ArrowRight),
    Bind::Key(Key::KeyD),
    Bind::Pad(Pad::Right),
];
const UP: &[Bind] = &[
    Bind::Key(Key::ArrowUp),
    Bind::Key(Key::KeyW),
    Bind::Pad(Pad::Up),
];
const DOWN: &[Bind] = &[
    Bind::Key(Key::ArrowDown),
    Bind::Key(Key::KeyS),
    Bind::Pad(Pad::Down),
];
const PAUSE: &[Bind] = &[Bind::Key(Key::Escape), Bind::Pad(Pad::Start)];
const FULLSCREEN: &[Bind] = &[Bind::Key(Key::F11)];

/// Modelle in der Reihenfolge von `renderer`
const CUBE: usize = 0;
const BALL: usize = 1;
const RING: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Screen {
    Title,
    Options,
    Playing,
    Paused,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entry {
    Play,
    Resume,
    Options,
    Title,
    Quit,
    Volume,
    Fullscreen,
    Latency,
    Back,
}

pub struct App {
    settings: Settings,
    sounds: Sounds,
    canvas: Canvas,
    menu: Menu<Entry>,
    screen: Screen,
    /// wohin "Zurueck" aus den Optionen fuehrt
    came_from: Screen,
    round: Round,
    takt: FixedStep,
    time: f32,
    next_seed: u64,
}

fn items(screen: Screen, s: &Settings) -> Vec<Item<Entry>> {
    let b = |e, t: &str| Item::button(e, t);
    match screen {
        Screen::Title => vec![
            b(Entry::Play, "Spielen"),
            b(Entry::Options, "Optionen"),
            b(Entry::Quit, "Beenden"),
        ],
        Screen::Paused => vec![
            b(Entry::Resume, "Weiter"),
            b(Entry::Options, "Optionen"),
            b(Entry::Title, "Hauptmenü"),
        ],
        Screen::End => vec![b(Entry::Play, "Noch einmal"), b(Entry::Title, "Hauptmenü")],
        Screen::Options => vec![
            Item::new(
                Entry::Volume,
                "Lautstärke",
                Kind::Slider {
                    value: s.volume,
                    min: 0.0,
                    max: 1.0,
                    step: 0.05,
                },
            ),
            Item::new(Entry::Fullscreen, "Vollbild", Kind::Toggle(s.fullscreen)),
            Item::new(
                Entry::Latency,
                "Weniger Verzögerung",
                Kind::Toggle(s.low_latency),
            ),
            b(Entry::Back, "Zurück"),
        ],
        Screen::Playing => Vec::new(),
    }
}

impl App {
    pub fn new(font: FontAtlas) -> Self {
        let settings = Settings::load();
        Self {
            sounds: Sounds::new(settings.volume),
            canvas: Canvas::new(font),
            menu: Menu::new(items(Screen::Title, &settings)),
            settings,
            screen: Screen::Title,
            came_from: Screen::Title,
            round: Round::new(0),
            takt: FixedStep::new(STEP),
            time: 0.0,
            next_seed: mettgine::random::seed_from_clock(),
        }
    }

    fn go(&mut self, screen: Screen) {
        self.screen = screen;
        self.menu = Menu::new(items(screen, &self.settings));
    }

    fn start(&mut self) {
        self.round = Round::new(self.next_seed);
        self.next_seed = self.next_seed.wrapping_add(1);
        self.takt.reset();
        self.go(Screen::Playing);
    }

    fn apply(&mut self, stage: &mut Stage) {
        self.sounds.set_volume(self.settings.volume);
        stage.set_fullscreen(self.settings.fullscreen);
        stage
            .renderer
            .set_low_latency(if self.settings.low_latency {
                LowLatency::On
            } else {
                LowLatency::Off
            });
        self.settings.save();
    }

    fn menu_event(&mut self, stage: &mut Stage, e: Event<Entry>) {
        self.sounds.click();
        match e {
            Event::Moved => {}
            Event::Pressed(Entry::Play) => self.start(),
            Event::Pressed(Entry::Resume) => self.go(Screen::Playing),
            Event::Pressed(Entry::Options) => {
                self.came_from = self.screen;
                self.go(Screen::Options);
            }
            Event::Pressed(Entry::Title) => self.go(Screen::Title),
            Event::Pressed(Entry::Quit) => stage.quit(),
            Event::Slid(Entry::Volume, v) => self.settings.volume = v,
            Event::Toggled(Entry::Fullscreen, on) => self.settings.fullscreen = on,
            Event::Toggled(Entry::Latency, on) => self.settings.low_latency = on,
            Event::Pressed(Entry::Back) | Event::Back => match self.screen {
                Screen::Options => self.go(self.came_from),
                Screen::Paused => self.go(Screen::Playing),
                _ => {}
            },
            _ => {}
        }
        if self.screen == Screen::Options {
            self.apply(stage);
        }
    }

    fn play(&mut self, stage: &mut Stage, dt: f32) {
        // im Einzelspieler anhalten, solange das Overlay des Launchers offen ist
        if stage.overlay_open() || stage.input.pressed(PAUSE) {
            self.go(Screen::Paused);
            return;
        }
        let key = |b: &[Bind]| stage.input.held(b) as u8 as f32;
        let (sx, sy) = stage.input.sticks().walk();
        let keys = vec2(key(RIGHT) - key(LEFT), key(DOWN) - key(UP));
        let input = Input {
            dir: (keys + vec2(sx, -sy)).clamp_length_max(1.0),
        };
        self.takt.add(dt);
        while self.takt.step() {
            for e in self.round.update(&input) {
                self.sounds.play(e);
                achievements::on_event(&stage.achievements, e);
                if e == GameEvent::Ring {
                    self.settings.total_rings += 1;
                    achievements::totals(&stage.achievements, self.settings.total_rings);
                    stage.input.rumble(0.3, 60);
                }
            }
        }
        if self.round.outcome != Outcome::Running {
            self.settings.save();
            self.go(Screen::End);
        }
    }

    fn draw(&mut self, stage: &mut Stage) {
        let (w, h) = stage.renderer.ui_area();
        self.canvas.begin(w, h);
        let c = &mut self.canvas;
        if self.screen == Screen::Playing {
            hud::game(c, &self.round);
        } else {
            let (title, below) = match self.screen {
                Screen::Paused => ("PAUSE", ""),
                Screen::End => ("FERTIG", hud::result(self.round.outcome)),
                Screen::Options => ("OPTIONEN", ""),
                _ => (TITLE, "Sammle alle Ringe, bevor die Zeit um ist."),
            };
            hud::menu_head(c, title, below);
            hud::version(c);
            let x = w * 0.5 - 260.0;
            self.menu.draw(c, x, 290.0, 520.0, 52.0, &Style::default());
        }
        let meshes = scene(&self.round, self.time);
        let eye = vec3(0.0, 17.0, 13.0);
        stage.draw(&FrameInput {
            time: self.time,
            lighting: Lighting::Sun(Sun {
                shadow_radius: FIELD + 4.0,
                shadow_strength: 0.7,
                ..Sun::default()
            }),
            post: Post::Plain(0.0),
            meshes: &meshes,
            ui: &self.canvas.verts,
            ..FrameInput::looking(eye, vec3(0.0, 0.0, 1.0), stage.aspect())
        });
    }
}

fn at(p: Vec2, y: f32) -> Mat4 {
    Mat4::from_translation(vec3(p.x, y, p.y))
}

/// Spielfeld, Kugel und Ringe.
fn scene(g: &Round, time: f32) -> Vec<MeshDraw> {
    let mut out = vec![
        MeshDraw::new(
            CUBE,
            Mat4::from_translation(vec3(0.0, -0.5, 0.0))
                * Mat4::from_scale(vec3(FIELD * 2.0, 1.0, FIELD * 2.0)),
        )
        .tint(vec3(0.35, 0.5, 0.4)),
        MeshDraw::new(
            BALL,
            at(g.pos, RADIUS) * Mat4::from_scale(Vec3::splat(RADIUS)),
        )
        .tint(vec3(0.9, 0.35, 0.3)),
    ];
    out.extend(g.rings.iter().enumerate().map(|(i, r)| {
        let spin = Mat4::from_rotation_y(time * 2.0 + i as f32)
            * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let size = vec3(RING_RADIUS, 0.12, RING_RADIUS);
        MeshDraw::new(RING, at(*r, 0.6) * spin * Mat4::from_scale(size))
            .tint(vec3(1.0, 0.8, 0.25))
            .glow(0.25)
    }));
    out
}

impl Game for App {
    fn starts_fullscreen(&self) -> bool {
        self.settings.fullscreen
    }

    fn renderer(&mut self, window: &Window) -> Result<Renderer> {
        let font = &self.canvas.font;
        let meshes = [shapes::cube(), shapes::sphere(16), shapes::cylinder(24)];
        Renderer::new(
            window,
            &Config {
                low_latency: true,
                diagnostics: true,
                ..Config::new(&meshes, &[], (&font.pixels, font.width, font.height))
            },
        )
    }

    fn ready(&mut self, stage: &mut Stage) {
        stage.renderer.set_msaa(4);
        self.apply(stage);
        achievements::totals(&stage.achievements, self.settings.total_rings);
    }

    fn frame(&mut self, stage: &mut Stage, dt: f32) {
        self.time += dt;
        if stage.input.pressed(FULLSCREEN) {
            self.settings.fullscreen = !self.settings.fullscreen;
            self.apply(stage);
        }
        if !stage.splash_active() {
            if self.screen == Screen::Playing {
                self.play(stage, dt);
            } else {
                let cursor = stage.cursor();
                for e in self.menu.update(&stage.input, cursor, dt) {
                    self.menu_event(stage, e);
                }
            }
        }
        self.draw(stage);
    }

    fn event(&mut self, _stage: &mut Stage, event: &WindowEvent) {
        if matches!(event, WindowEvent::Focused(false)) && self.screen == Screen::Playing {
            self.go(Screen::Paused);
        }
    }

    /// Was der Launcher in Discord zeigt.
    fn status(&self) -> Option<Status> {
        Some(match self.screen {
            Screen::Playing | Screen::Paused => Status::new(
                "Sammelt Ringe",
                format!("{} von {}", self.round.collected, crate::game::RINGS),
            ),
            _ => Status::new("Im Menü", ""),
        })
    }

    fn crash_context(&self) {
        mettgine::crash::context("Bildschirm", format!("{:?}", self.screen));
        mettgine::crash::context("Runde", format!("{:?}", self.round.outcome));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_screen_has_a_way_on() {
        let s = Settings::default();
        for screen in [Screen::Title, Screen::Options, Screen::Paused, Screen::End] {
            assert!(!items(screen, &s).is_empty(), "{screen:?} ohne Eintraege");
        }
    }

    #[test]
    fn the_scene_shows_every_ring() {
        let g = Round::new(3);
        assert_eq!(scene(&g, 0.0).len(), 2 + g.rings.len());
    }
}
