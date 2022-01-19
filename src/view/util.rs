pub fn chars_height(s: &str) -> u16 {
    s.lines().count() as u16
}

pub fn chars_width(s: &str) -> u16 {
    s.lines().next().unwrap().chars().count() as u16
}
