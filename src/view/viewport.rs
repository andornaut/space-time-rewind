use super::{
    coordinates::Coordinates,
    factory::{WORLD_HEIGHT, WORLD_WIDTH},
};

#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    x: u8,
    y: i8,
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
        let (x, y) = bottom_left.as_tuple();
        assert!(height > 0);
        assert!(width > 0);
        Self {
            x,
            y,
            height,
            width,
        }
    }

    pub fn bottom_left(&self) -> (u8, i8) {
        (self.x, self.y)
    }

    pub fn centered(&self) -> Coordinates {
        let (x1, y1) = self.bottom_left();
        let (x2, y2) = self.top_right();
        let x1 = u16::from(x1);
        let x2 = u16::from(x2);
        let y1 = i16::from(y1);
        let y2 = i16::from(y2);
        Coordinates::new_wrapped_and_saturated(
            u8::try_from((x1 + x2) / 2).unwrap(),
            i8::try_from((y1 + y2) / 2).unwrap(),
        )
    }

    pub fn centered_around_bottom_left(&self) -> Coordinates {
        let (x1, y1) = self.bottom_left();
        let mut x = i16::from(x1) - (i16::from(self.width) / 2);
        if x.is_negative() {
            x += i16::from(WORLD_WIDTH);
        }
        let x = u8::try_from(x).unwrap();
        let y = y1 - (i8::try_from(self.height).unwrap() / 2);
        Coordinates::new(x, y)
    }

    pub fn contained_vertically(&self, other: Viewport) -> Coordinates {
        let (_, y_max) = self.top_right();
        let (_, y_min) = self.bottom_left();
        let (x, y) = other.bottom_left();
        let mut y = y;
        if y < y_min {
            y = y_min;
        } else if y > y_max {
            y = y_max;
        }
        Coordinates::new(x, y)
    }

    pub fn expanded(&self, wider_width: u8, taller_height: u8) -> Coordinates {
        let (x, y) = self.bottom_left();
        let x_offset = wider_width.saturating_sub(self.width) / 2;
        let y_offset = taller_height.saturating_sub(self.height) / 2;
        let y_offset = i8::try_from(y_offset).unwrap();
        Coordinates::new(x - x_offset, y - y_offset)
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.intersects_horizontally(other) && self.intersects_vertically(other)
    }

    pub fn intersects_horizontally(&self, other: Self) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x
    }

    pub fn intersects_vertically(&self, other: Self) -> bool {
        let o_h = i8::try_from(other.height).unwrap();
        let s_h = i8::try_from(self.height).unwrap();
        self.y < other.y + o_h && self.y + s_h > other.y
    }

    pub fn shrunk(&self, narrower_width: u8, shorter_height: u8) -> Coordinates {
        let (x, y) = self.bottom_left();
        let x_offset = self.width.saturating_sub(narrower_width) / 2;
        let y_offset = self.height.saturating_sub(shorter_height) / 2;
        let y_offset = i8::try_from(y_offset).unwrap();
        Coordinates::new(x + x_offset, y + y_offset)
    }

    pub fn top_right(&self) -> (u8, i8) {
        let x = self.x + self.width - 1;
        let y = self.y + i8::try_from(self.height).unwrap() - 1;
        // n.b. The x-position may sometimes be larger than 200 (WORLD_WIDTH), for example when
        // firing a gun at x:199, after calling centered() the top-right x-position will be 201,
        // which would cause a panic
        (x, y)
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn with_coordinates(&self, coordinates: Coordinates) -> Self {
        Self::new_with_coordinates(self.width, self.height, coordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_returns_true_when_overlapping() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(1, 1));

        assert!(bl.intersects(tr));
    }

    #[test]
    fn intersects_returns_false_when_bottom_adjacent() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 2));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 0));

        assert!(!bl.intersects(tr));
    }

    #[test]
    fn intersects_returns_false_when_right_adjacent() {
        let bl = Viewport::new_with_coordinates(2, 2, Coordinates::new(0, 0));
        let tr = Viewport::new_with_coordinates(2, 2, Coordinates::new(2, 0));

        assert!(!bl.intersects(tr));
    }
}
