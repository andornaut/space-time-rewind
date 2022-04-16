use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
        main::TICKS_PER_SECOND,
    },
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    game::game_item::{GameItem, GameItemKind},
    view::{
        coordinates::Coordinates,
        render::Renderable,
        renderer::Renderer,
        util::{chars_height, chars_width},
        viewport::Viewport,
    },
};

static TEXT: &str = "◄◆►";

static TEXT_SHIELDS: &str = "\
░░░░░
░◄◆►░
░░░░░";

const DISABLED_GUNS_COUNT: u16 = TICKS_PER_SECOND / 20; // 50 ms
const ENABLED_SHIELDS_COUNT: u16 = TICKS_PER_SECOND * 5; // 5 seconds
const INITIAL_MAX_HEALTH: u8 = 5;
const INITIAL_MAX_MISSILES: u8 = 5;

pub struct Ship {
    coordinates: Coordinates,
    deleted: bool,
    disabled_guns: Countdown,
    enabled_shields: Countdown,
    health: u8,
    max_y: i8,
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
                        commands.push(Command::AddExplosion(self.viewport().centered()));
                        commands.push(Command::GameOver);
                    }
                    return commands;
                }
            }
            Command::FireGuns => {
                if self.disabled_guns.off() {
                    self.disabled_guns.restart();
                    let mut coordinates = self.viewport().centered();
                    coordinates.offset_y(1);
                    return vec![Command::AddBullet(coordinates)];
                }
            }
            Command::FireMissile => {
                self.missiles = self.missiles.saturating_sub(1);
                let mut coordinates = self.viewport().centered();
                coordinates.offset_y(1);
                return vec![
                    Command::AddMissile(coordinates),
                    Command::UpdateMissiles(self.missiles, INITIAL_MAX_MISSILES),
                ];
            }
            Command::FireShields => {
                let wider_width = chars_width(TEXT_SHIELDS);
                let taller_height = chars_height(TEXT_SHIELDS);
                self.coordinates = self.viewport().expanded(wider_width, taller_height);
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
                let dx = dx * i16::from(self.width());
                self.coordinates.movement((dx, dy));

                // Don't move above the visible viewport.
                let (x, y) = self.coordinates.as_tuple();
                if y > self.max_y {
                    self.coordinates = Coordinates::new(x, self.max_y);
                }
                return vec![Command::MoveOffset((-dx, 0))];
            }
            Command::ActorsViewportChanged(viewport) => {
                self.coordinates = viewport.contained_vertically(self.viewport());
                self.update_max_y(viewport);
            }
            Command::ActorsViewportInitialized(viewport) => {
                self.coordinates = self
                    .viewport()
                    .with_coordinates(viewport.centered())
                    .centered_around_bottom_left();
                self.update_max_y(viewport);
                return vec![
                    Command::UpdateMissiles(INITIAL_MAX_MISSILES, INITIAL_MAX_MISSILES),
                    Command::UpdateHealth(INITIAL_MAX_HEALTH, INITIAL_MAX_HEALTH),
                ];
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
            GameItemKind::ShipWithShields
        } else {
            GameItemKind::Ship
        }
    }
}

impl Renderable for Ship {
    fn render(&self, renderer: &mut Renderer) {
        renderer.render_with_offset(self.viewport(), self.text(), self.color());
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Ship {
    fn handle_tick(&mut self, _: &Ticker, _: Viewport) {
        self.disabled_guns.down();

        if self.enabled_shields.current() == 1 {
            let narrower_width = chars_width(TEXT);
            let shorter_height = chars_height(TEXT);
            // Shrink before turning off the shields, because turning off the shields will reduce
            // the ship's width, which will cause the `shrink()` calculation to be incorrect.
            self.coordinates = self.viewport().shrunk(narrower_width, shorter_height);
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
            max_y: 0,
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

    fn height(&self) -> u8 {
        chars_height(self.text())
    }

    fn width(&self) -> u8 {
        chars_width(self.text())
    }

    fn update_max_y(&mut self, viewport: Viewport) {
        // Save the max visible viewport y-position to constrain the ship's movement.
        let (_, max_y) = viewport.top_right().as_tuple();
        self.max_y = max_y;
    }
}
