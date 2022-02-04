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
    ExplosionA,
    ExplosionB,
    ExplosionC,
    ExplosionD,
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
const BLUE: Color = Color::Rgb(51, 102, 175);
const BLUE_LIGHT: Color = Color::Rgb(153, 204, 255);
const GREEN: Color = Color::Rgb(102, 175, 51);
const GREEN_LIGHT: Color = Color::Rgb(153, 255, 153);
const GREY: Color = Color::Rgb(104, 104, 104);
const GREY_DARK: Color = Color::Rgb(51, 51, 51);
const GREY_LIGHT: Color = Color::Rgb(204, 204, 204);
const GREY_MEDIUM_LIGHT: Color = Color::Rgb(153, 153, 153);
const PINK: Color = Color::Rgb(175, 51, 175);
const PINK_LIGHT: Color = Color::Rgb(255, 153, 255);
const RED: Color = Color::Rgb(204, 25, 25);

impl From<ColorTheme> for Color {
    fn from(color_theme: ColorTheme) -> Self {
        match color_theme {
            ColorTheme::Bg => BLACK,
            ColorTheme::BoardBorderFg => GREY,
            ColorTheme::BoardTitleFg => GREY_MEDIUM_LIGHT,
            ColorTheme::ErrorBg => RED,
            ColorTheme::ErrorFg => BLACK,

            // Actors
            ColorTheme::AsteroidHighHpLarge => Color::Rgb(120, 130, 110),
            ColorTheme::AsteroidHighHpMedium => Color::Rgb(110, 88, 77),
            ColorTheme::AsteroidHighHpSmall => Color::Rgb(88, 77, 66),
            ColorTheme::AsteroidLowHp => Color::Rgb(44, 33, 22),
            ColorTheme::AsteroidMidHp => Color::Rgb(66, 55, 44),
            ColorTheme::Bullet => Color::Rgb(204, 204, 0),
            ColorTheme::ExplosionA => Color::Rgb(153, 0, 0),
            ColorTheme::ExplosionB => RED,
            ColorTheme::ExplosionC => Color::Rgb(255, 51, 51),
            ColorTheme::ExplosionD => Color::Rgb(255, 104, 104),
            ColorTheme::Missile => PINK,
            ColorTheme::PowerUpHealth => RED,
            ColorTheme::PowerUpMissile => PINK,
            ColorTheme::Ship => Color::Rgb(51, 153, 204),
            ColorTheme::ShipShields => BLUE_LIGHT,

            // UI
            ColorTheme::DisabledButton => GREY_DARK,
            ColorTheme::GameOver => Color::Rgb(204, 102, 153),
            ColorTheme::HealthCurrent => RED,
            ColorTheme::HealthHeader => GREY,
            ColorTheme::HealthLost => GREY,
            ColorTheme::MissileButton => PINK,
            ColorTheme::MissileButtonActive => PINK_LIGHT,
            ColorTheme::MissilesCurrent => PINK,
            ColorTheme::MissilesHeader => GREY,
            ColorTheme::MissilesLost => GREY,
            ColorTheme::RewindButton => GREEN,
            ColorTheme::RewindButtonActive => GREEN_LIGHT,
            ColorTheme::ScoreHeader => GREY,
            ColorTheme::ScorePoints => GREY_LIGHT,
            ColorTheme::ShieldsButton => BLUE,
            ColorTheme::ShieldsButtonActive => BLUE_LIGHT,
        }
    }
}
