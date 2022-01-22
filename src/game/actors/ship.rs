use crate::{
    app::{
        app::TICKS_PER_SECOND,
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    game::{
        game_item::{GameItem, GameItemKind},
        INITIAL_MAX_HEALTH, INITIAL_MAX_MISSILES,
    },
    view::{
        render::{render_text, Renderable},
        util::{chars_height, chars_width},
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

static TEXT: &str = "◄◆►";

static TEXT_SHIELDS: &str = "\
░░░░░
░◄◆►░
░░░░░";

const DISABLED_GUNS_COUNT: u16 = TICKS_PER_SECOND / 10; // 100 ms
const ENABLED_SHIELDS_COUNT: u16 = TICKS_PER_SECOND * 5; // 5 seconds

pub struct Ship {
    coordinates: Coordinates,
    deleted: bool,
    disabled_guns: Countdown,
    enabled_shields: Countdown,
    health: u8,
    missiles: u8,
}

impl CommandHandler for Ship {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::Collide(kind) => {
                if self.enabled_shields.on() {
                    return NO_COMMANDS;
                }
                if let GameItemKind::Asteroid = kind {
                    self.health = self.health.saturating_sub(1);
                    let mut commands = vec![Command::UpdateHealth(self.health, INITIAL_MAX_HEALTH)];
                    if self.health == 0 {
                        self.deleted = true;
                        commands.push(Command::AddExplosion(self.viewport().center()));
                        commands.push(Command::GameOver);
                    }
                    return commands;
                }
            }
            Command::FireGuns => {
                if self.disabled_guns.off() {
                    self.disabled_guns.restart();
                    let (cx, cy) = self.viewport().center();
                    return vec![Command::AddBullet((cx, cy + 1))];
                }
            }
            Command::FireMissile => {
                let (cx, cy) = self.viewport().center();
                self.missiles = self.missiles.saturating_sub(1);
                return vec![
                    Command::UpdateMissiles(self.missiles, INITIAL_MAX_MISSILES),
                    Command::AddMissile((cx, cy + 1)),
                ];
            }
            Command::FireShields => {
                let wider_width = chars_width(TEXT_SHIELDS);
                let taller_height = chars_height(TEXT_SHIELDS);
                self.coordinates = self.viewport().grow(wider_width, taller_height);
                self.enabled_shields.restart();
            }
            Command::IncreaseHealth(number) => {
                if self.health != INITIAL_MAX_HEALTH {
                    self.health += number;
                    return vec![Command::UpdateHealth(self.health, INITIAL_MAX_HEALTH)];
                }
            }
            Command::IncreaseMissiles(number) => {
                if self.missiles != INITIAL_MAX_MISSILES {
                    self.missiles += number;
                    return vec![Command::UpdateMissiles(self.missiles, INITIAL_MAX_MISSILES)];
                }
            }
            Command::MoveShip((dx, dy)) => {
                let (x, y) = self.coordinates;
                let width = i16::try_from(self.width()).unwrap();
                self.coordinates = (
                    u16::from(x.saturating_add_signed(dx * width)),
                    u16::from(y.saturating_add_signed(dy)),
                );
            }
            _ => (),
        }
        NO_COMMANDS
    }
}

impl GameItem for Ship {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        if self.enabled_shields.on() {
            GameItemKind::Shields
        } else {
            GameItemKind::Ship
        }
    }
}

impl Renderable for Ship {
    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        // Prevent the ship from going out of bounds when the viewport is resized.
        self.coordinates = viewport.contain(&self.viewport());
        render_text(context, self.coordinates, self.text(), self.color());
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Ship {
    fn handle_tick(&mut self, _: &Ticker) {
        self.disabled_guns.down();

        if self.enabled_shields.current() == 1 {
            let narrower_width = chars_width(TEXT);
            let shorter_height = chars_height(TEXT);
            // Shrink before turning off the shields, because turning off the shields will reduce
            // the ship's width, which will cause the `shrink()` calculation to be incorrect.
            self.coordinates = self.viewport().shrink(narrower_width, shorter_height);
        }
        self.enabled_shields.down();
    }
}

impl Ship {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            deleted: false,
            disabled_guns: Countdown::new(DISABLED_GUNS_COUNT),
            enabled_shields: Countdown::new(ENABLED_SHIELDS_COUNT),
            health: INITIAL_MAX_HEALTH,
            missiles: INITIAL_MAX_MISSILES,
        }
    }

    fn color(&self) -> ColorTheme {
        if self.enabled_shields.on() {
            ColorTheme::ShipShields
        } else {
            ColorTheme::Ship
        }
    }

    fn text(&self) -> &'static str {
        if self.enabled_shields.on() {
            TEXT_SHIELDS
        } else {
            TEXT
        }
    }

    fn height(&self) -> u16 {
        chars_height(self.text())
    }

    fn width(&self) -> u16 {
        chars_width(self.text())
    }
}
