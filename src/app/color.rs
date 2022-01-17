use tui::style::Color;

#[derive(Copy, Clone, Debug)]
pub enum ColorTheme {
    Bg,
    BoardBorderFg,
    BoardTitleFg,

    // Actors
    AsteroidFullHp,
    AsteroidHalfHp,
    AsteroidLowHp,
    Bullet,
    ExplosionStart,
    ExplosionMiddle,
    ExplosionEnd,
    ExplosionEnd2,
    Ship,

    // UI
    DisabledButton,
    GameOver,
    Missile,
    MissileActive,
    Rewind,
    RewindActive,
    Shield,
    ShieldActive,
}

impl From<ColorTheme> for Color {
    fn from(color_theme: ColorTheme) -> Self {
        match color_theme {
            ColorTheme::Bg => Color::Rgb(21, 21, 21),
            ColorTheme::BoardBorderFg => Color::Rgb(128, 128, 128),
            ColorTheme::BoardTitleFg => Color::Blue,

            // Actors
            ColorTheme::AsteroidFullHp => Color::Rgb(120, 99, 66),
            ColorTheme::AsteroidHalfHp => Color::Rgb(99, 66, 33),
            ColorTheme::AsteroidLowHp => Color::Rgb(66, 33, 22),
            ColorTheme::Bullet => Color::Rgb(204, 204, 0),
            ColorTheme::ExplosionStart => Color::Rgb(153, 0, 0),
            ColorTheme::ExplosionMiddle => Color::Rgb(204, 21, 21),
            ColorTheme::ExplosionEnd => Color::Rgb(255, 51, 51),
            ColorTheme::ExplosionEnd2 => Color::Rgb(255, 104, 104),
            ColorTheme::Ship => Color::Cyan,

            // UI
            ColorTheme::DisabledButton => Color::Rgb(51, 51, 51),
            ColorTheme::GameOver => Color::Rgb(204, 102, 153),
            ColorTheme::Missile => Color::Rgb(175, 102, 102),
            ColorTheme::MissileActive => Color::Rgb(255, 204, 204),
            ColorTheme::Rewind => Color::Rgb(102, 175, 102),
            ColorTheme::RewindActive => Color::Rgb(204, 255, 204),
            ColorTheme::Shield => Color::Rgb(102, 102, 175),
            ColorTheme::ShieldActive => Color::Rgb(204, 204, 255),
        }
    }
}