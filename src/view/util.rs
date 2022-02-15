pub fn chars_height(s: &str) -> u8 {
    s.lines().count() as u8
}

pub fn chars_width(s: &str) -> u8 {
    u8::try_from(s.lines().next().unwrap().chars().count()).unwrap()
}
