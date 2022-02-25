use std::cmp::{max, min};

use super::{
    coordinates::Coordinates,
    factory::{WORLD_HEIGHT, WORLD_WIDTH},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
    bottom_left: Coordinates,
    width: u8,
    height: u8,
}

impl Viewport {
    pub fn new(width: u8, height: u8) -> Self {
        Self::new_with_coordinates(width, height, Coordinates::default())
    }

    pub fn new_for_world() -> Self {
        Self::new(WORLD_WIDTH, WORLD_HEIGHT)
    }

    pub fn new_with_coordinates(width: u8, height: u8, bottom_left: Coordinates) -> Self {
        assert!(height > 0);
        assert!(width > 0);
        Self {
            bottom_left,
            height,
            width,
        }
    }

    pub fn bottom_left(&self) -> Coordinates {
        self.bottom_left
    }

    pub fn centered(&self) -> Coordinates {
        let (x, y) = self.bottom_left().as_tuple();

        // x
        let x = u16::from(x);
        let w = u16::from(self.width);
        let x = u8::try_from((2 * x + w) / 2).unwrap();

        // y
        let y = i16::from(y);
        let h = i16::from(self.height);
        let y = i8::try_from((2 * y + h) / 2).unwrap();
        Coordinates::new_wrapped_and_saturated(x, y)
    }

    pub fn centered_around_bottom_left(&self) -> Coordinates {
        let (x, y) = self.bottom_left().as_tuple();

        // x
        let x = i16::from(x);
        let w = i16::from(self.width);
        let mut x = x - (w / 2);
        if x.is_negative() {
            x += i16::from(WORLD_WIDTH);
        }
        let x = u8::try_from(x).unwrap();

        // y
        let h = i8::try_from(self.height).unwrap();
        let y = y - (h / 2);
        Coordinates::new(x, y)
    }

    pub fn contained_vertically(&self, other: Viewport) -> Coordinates {
        let (_, y_max) = self.top_right().as_tuple();
        let (_, y_min) = self.bottom_left().as_tuple();
        let (x, y) = other.bottom_left().as_tuple();
        let mut y = y;
        y = max(y, y_min);
        y = min(y, y_max);
        Coordinates::new(x, y)
    }

    pub fn expanded(&self, wider_width: u8, taller_height: u8) -> Coordinates {
        let (x, y) = self.bottom_left().as_tuple();
        let x_offset = wider_width.saturating_sub(self.width) / 2;
        let y_offset = taller_height.saturating_sub(self.height) / 2;
        let y_offset = i8::try_from(y_offset).unwrap();
        Coordinates::new(x - x_offset, y - y_offset)
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.intersects_horizontally(other) && self.intersects_vertically(other)
    }

    pub fn intersects_horizontally(&self, other: Self) -> bool {
        let world_width = i16::from(WORLD_WIDTH);
        let (x1, _) = self.bottom_left().as_tuple();
        let (x2, _) = other.bottom_left().as_tuple();
        let mut x1 = i16::from(x1);
        let mut x2 = i16::from(x2);
        let w1 = i16::from(self.width);
        let w2 = i16::from(other.width);
        // Account for x-wrapping
        if x1 + w1 > world_width {
            x1 -= world_width;
        }
        if x2 + w2 > world_width {
            x2 -= world_width;
        }
        intersects(x1, x2, w1, w2)
    }

    pub fn intersects_vertically(&self, other: Self) -> bool {
        let (_, y1) = self.bottom_left().as_tuple();
        let (_, y2) = other.bottom_left().as_tuple();
        let y1 = i16::from(y1);
        let y2 = i16::from(y2);
        let h1 = i16::from(self.height);
        let h2 = i16::from(other.height);
        intersects(y1, y2, h1, h2)
    }

    pub fn offset(&mut self, offset: Coordinates) {
        let (x, y) = self.bottom_left().as_tuple();
        let mut coordinates = Coordinates::new(x, y);
        coordinates.offset(offset);
        let (x, y) = coordinates.as_tuple();
        self.bottom_left = Coordinates::new(x, y);
    }

    pub fn shrunk(&self, narrower_width: u8, shorter_height: u8) -> Coordinates {
        let (x, y) = self.bottom_left().as_tuple();
        let x_offset = self.width.saturating_sub(narrower_width) / 2;
        let y_offset = self.height.saturating_sub(shorter_height) / 2;
        let y_offset = i8::try_from(y_offset).unwrap();
        Coordinates::new(x + x_offset, y + y_offset)
    }

    pub fn top_right(&self) -> Coordinates {
        let (x, y) = self.bottom_left().as_tuple();
        let x = (x + self.width - 1) % WORLD_WIDTH;
        let y = y + i8::try_from(self.height).unwrap() - 1;
        Coordinates::new(x, y)
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn with_coordinates(&self, coordinates: Coordinates) -> Self {
        Self::new_with_coordinates(self.width, self.height, coordinates)
    }
}

fn intersects(pos_1: i16, pos_2: i16, len_1: i16, len_2: i16) -> bool {
    pos_1 < pos_2 + len_2 && pos_1 + len_1 > pos_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_around_bottom_left_handles_no_wrap() {
        let v = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 2));

        let (x, y) = v.centered_around_bottom_left().as_tuple();

        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn centered_around_bottom_left_handles_x_wrapping() {
        let v = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));

        let (x, y) = v.centered_around_bottom_left().as_tuple();

        assert_eq!(x, 199);
        assert_eq!(y, -1);
    }

    #[test]
    fn centered_returns_0x0_for_1_length_from_origin() {
        let v = Viewport::new_with_coordinates(1, 1, Coordinates::new(0, 0));

        let (x, y) = v.centered().as_tuple();

        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn centered_returns_1x1_for_2_length_from_origin() {
        let v = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));

        let (x, y) = v.centered().as_tuple();

        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn centered_handles_negative_y() {
        let v = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, -1));

        let (x, y) = v.centered().as_tuple();

        assert_eq!(x, 1);
        assert_eq!(y, 0);
    }
    #[test]
    fn centered_handles_x_wrapping() {
        let v = Viewport::new_with_coordinates(2, 2, Coordinates::new(199, 0));

        let (x, y) = v.centered().as_tuple();

        assert_eq!(x, 0);
        assert_eq!(y, 1);
    }

    #[test]
    fn intersects_returns_true_when_overlapping() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(1, 1));

        assert!(bl.intersects(tr));
        assert!(tr.intersects(bl));
        assert!(bl.intersects_horizontally(tr));
        assert!(tr.intersects_horizontally(bl));
        assert!(bl.intersects_vertically(tr));
        assert!(tr.intersects_vertically(bl));
    }

    #[test]
    fn intersects_horizontally_returns_false_when_adjacent() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 0));

        assert!(!bl.intersects_horizontally(tr));
        assert!(!tr.intersects_horizontally(bl));
    }

    #[test]
    fn intersects_horizontally_returns_false_when_adjacent_and_wrapped() {
        let l = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let r = Viewport::new_with_coordinates(2, 2, Coordinates::new(198, 0));

        assert!(!l.intersects_horizontally(r));
        assert!(!r.intersects_horizontally(l));
    }

    #[test]
    fn intersects_horizontally_returns_true_when_wrapped() {
        let l = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let r = Viewport::new_with_coordinates(2, 2, Coordinates::new(199, 0));

        assert!(l.intersects_horizontally(r));
        assert!(r.intersects_horizontally(l));
    }

    #[test]
    fn intersects_vertically_returns_false_when_adjacent() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 2));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 0));

        assert!(!bl.intersects_vertically(tr));
        assert!(!tr.intersects_vertically(bl));
    }
}
