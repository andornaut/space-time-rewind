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
    PowerUpHealth,
    PowerUpMissile,
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

const BLACK: Color = Color::Rgb(21, 21, 21);
const BLUE_DARK: Color = Color::Rgb(51, 102, 175);
const BLUE_LIGHT: Color = Color::Rgb(153, 204, 255);
const GREY: Color = Color::Rgb(104, 104, 104);
const GREY_DARK: Color = Color::Rgb(51, 51, 51);
const GREY_LIGHT: Color = Color::Rgb(204, 204, 204);
const PINK_DARK: Color = Color::Rgb(204, 104, 204);
const PINK_LIGHT: Color = Color::Rgb(175, 51, 175);
const RED: Color = Color::Rgb(204, 25, 25);

impl From<ColorTheme> for Color {
    fn from(color_theme: ColorTheme) -> Self {
        match color_theme {
            ColorTheme::Bg => BLACK,
            ColorTheme::BoardBorderFg => GREY,
            ColorTheme::BoardTitleFg => GREY,
            ColorTheme::ErrorBg => RED,
            ColorTheme::ErrorFg => BLACK,

            // Actors
            ColorTheme::AsteroidHighHpLarge => Color::Rgb(120, 130, 110),
            ColorTheme::AsteroidHighHpMedium => Color::Rgb(110, 88, 77),
            ColorTheme::AsteroidHighHpSmall => Color::Rgb(88, 77, 66),
            ColorTheme::AsteroidLowHp => Color::Rgb(44, 33, 22),
            ColorTheme::AsteroidMidHp => Color::Rgb(66, 55, 44),
            ColorTheme::Bullet => Color::Rgb(204, 204, 0),
            ColorTheme::ExplosionEnd => Color::Rgb(255, 104, 104),
            ColorTheme::ExplosionMiddle1 => RED,
            ColorTheme::ExplosionMiddle2 => Color::Rgb(255, 51, 51),
            ColorTheme::ExplosionStart => Color::Rgb(153, 0, 0),
            ColorTheme::Missile => PINK_LIGHT,
            ColorTheme::PowerUpHealth => RED,
            ColorTheme::PowerUpMissile => PINK_LIGHT,
            ColorTheme::Ship => Color::Rgb(51, 153, 204),
            ColorTheme::ShipShields => BLUE_LIGHT,

            // UI
            ColorTheme::DisabledButton => GREY_DARK,
            ColorTheme::GameOver => Color::Rgb(204, 102, 153),
            ColorTheme::HealthCurrent => RED,
            ColorTheme::HealthHeader => GREY,
            ColorTheme::HealthLost => GREY,
            ColorTheme::MissileButton => PINK_LIGHT,
            ColorTheme::MissileButtonActive => PINK_DARK,
            ColorTheme::MissilesCurrent => PINK_LIGHT,
            ColorTheme::MissilesHeader => GREY,
            ColorTheme::MissilesLost => GREY,
            ColorTheme::RewindButton => Color::Rgb(102, 175, 51),
            ColorTheme::RewindButtonActive => Color::Rgb(204, 255, 153),
            ColorTheme::ScoreHeader => GREY,
            ColorTheme::ScorePoints => GREY_LIGHT,
            ColorTheme::ShieldsButton => BLUE_DARK,
            ColorTheme::ShieldsButtonActive => BLUE_LIGHT,
        }
    }
}
