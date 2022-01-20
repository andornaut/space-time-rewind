use tui::style::Color;

#[derive(Copy, Clone)]
pub enum ColorTheme {
    Bg,
    BoardBorderFg,
    BoardTitleFg,
    ErrorBg,
    ErrorFg,

    // Actors
    AsteroidHighHpLarge,
    AsteroidHighHpMedium,
    AsteroidHighHpSmall,
    AsteroidLowHp,
    AsteroidMidHp,
    Bullet,
    ExplosionEnd,
    ExplosionMiddle1,
    ExplosionMiddle2,
    ExplosionStart,
    Missile,
    Ship,
    ShipShields,

    // UI
    DisabledButton,
    GameOver,
    HealthCurrent,
    HealthHeader,
    HealthLost,
    MissileButton,
    MissileButtonActive,
    MissilesCurrent,
    MissilesHeader,
    MissilesLost,
    RewindButton,
    RewindButtonActive,
    ScoreHeader,
    ScorePoints,
    ShieldsButton,
    ShieldsButtonActive,
}

impl From<ColorTheme> for Color {
    fn from(color_theme: ColorTheme) -> Self {
        let dark_grey = Color::Rgb(51, 51, 51);
        let grey = Color::Rgb(104, 104, 104);
        let red = Color::Rgb(204, 25, 25);
        let black = Color::Rgb(21, 21, 21);
        let missiles_on = Color::Rgb(255, 153, 204);
        let missiles_off = Color::Rgb(175, 51, 102);
        let shields_on = Color::Rgb(153, 204, 255);
        match color_theme {
            ColorTheme::Bg => black,
            ColorTheme::BoardBorderFg => Color::Rgb(128, 128, 128),
            ColorTheme::BoardTitleFg => Color::Blue,
            ColorTheme::ErrorBg => red,
            ColorTheme::ErrorFg => black,

            // Actors
            ColorTheme::AsteroidHighHpLarge => Color::Rgb(120, 130, 110),
            ColorTheme::AsteroidHighHpMedium => Color::Rgb(110, 88, 77),
            ColorTheme::AsteroidHighHpSmall => Color::Rgb(88, 77, 66),
            ColorTheme::AsteroidLowHp => Color::Rgb(44, 33, 22),
            ColorTheme::AsteroidMidHp => Color::Rgb(66, 55, 44),
            ColorTheme::Bullet => Color::Rgb(204, 204, 0),
            ColorTheme::Missile => missiles_off,
            ColorTheme::ExplosionEnd => Color::Rgb(255, 104, 104),
            ColorTheme::ExplosionMiddle1 => red,
            ColorTheme::ExplosionMiddle2 => Color::Rgb(255, 51, 51),
            ColorTheme::ExplosionStart => Color::Rgb(153, 0, 0),
            ColorTheme::Ship => Color::Rgb(51, 153, 204),
            ColorTheme::ShipShields => shields_on,

            // UI
            ColorTheme::DisabledButton => dark_grey,
            ColorTheme::GameOver => Color::Rgb(204, 102, 153),
            ColorTheme::HealthCurrent => red,
            ColorTheme::HealthHeader => grey,
            ColorTheme::HealthLost => grey,
            ColorTheme::MissileButton => missiles_off,
            ColorTheme::MissileButtonActive => missiles_on,
            ColorTheme::MissilesCurrent => missiles_off,
            ColorTheme::MissilesHeader => grey,
            ColorTheme::MissilesLost => grey,
            ColorTheme::RewindButton => Color::Rgb(102, 175, 51),
            ColorTheme::RewindButtonActive => Color::Rgb(204, 255, 153),
            ColorTheme::ScoreHeader => grey,
            ColorTheme::ScorePoints => Color::Rgb(204, 204, 204),
            ColorTheme::ShieldsButton => Color::Rgb(51, 102, 175),
            ColorTheme::ShieldsButtonActive => shields_on,
        }
    }
}
