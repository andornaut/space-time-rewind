use super::factory::{WORLD_HEIGHT, WORLD_WIDTH};

pub type Movement = (i16, i16);

#[derive(Clone, Copy, Debug, Default)]
pub struct Coordinates(u8, i8);

impl Coordinates {
    pub fn new(x: u8, y: i8) -> Self {
        let coordinates = Self(x, y);
        coordinates.validate();
        coordinates
    }

    pub fn new_wrapped_and_saturated(x: u8, y: i8) -> Self {
        // This constructor avoids panics by wrapping and saturing the coordinates,
        // which is useful in some cases, such as:
        //   Firing a gun from a Ship at (199, y) will attempt to add a Bullet
        //   at (200, 0), which would otherwise panic due being wider than the world.
        let mut coordinates = Self(x, y);
        coordinates.movement((0, 0));
        coordinates
    }

    pub fn offset(&mut self, Coordinates(x, y): Coordinates) {
        self.offset_((i16::from(x), i16::from(y)));
    }

    pub fn movement(&mut self, (dx, dy): Movement) {
        let Coordinates(x, y) = *self;
        self.0 = wrap_x(x, dx);
        self.1 = saturate_y(i16::from(y), dy);
        self.validate();
    }

    pub fn x_offset(&mut self, dx: i16) {
        self.offset_((dx, 0));
    }

    pub fn y_offset(&mut self, dy: i16) {
        self.offset_((0, dy));
    }

    pub fn as_tuple(&self) -> (u8, i8) {
        (self.0, self.1)
    }

    fn offset_(&mut self, (dx, dy): (i16, i16)) {
        let Coordinates(x, _) = *self;
        self.0 = wrap_x(x, dx);
        self.1 += i8::try_from(dy).unwrap();
        self.validate();
    }

    fn validate(&self) {
        let Coordinates(x, y) = *self;
        assert!(x < WORLD_WIDTH);

        // Sanity check y: Actors should be deleted once they no longer intersect vertically with the world.
        let y_abs = u8::try_from(y.abs()).unwrap();
        assert!(y_abs <= WORLD_HEIGHT)
    }
}

fn saturate_y(y1: i16, y2: i16) -> i8 {
    let mut y = y1 + y2;
    let max_y = i16::from(WORLD_HEIGHT) - 1;
    if y.is_negative() {
        y = 0
    } else if y > max_y {
        y = max_y;
    }
    i8::try_from(y).unwrap()
}

fn wrap_x(x1: u8, x2: i16) -> u8 {
    let mut x = i16::from(x1) + x2;
    if x.is_negative() {
        x += i16::from(WORLD_WIDTH);
    } else {
        x %= i16::from(WORLD_WIDTH);
    }
    u8::try_from(x).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_offset_wraps_left() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.x_offset(-2);

        assert_eq!(coordinates.as_tuple(), (199, 1));
    }

    #[test]
    fn x_offset_wraps_right() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.x_offset(200);

        assert_eq!(coordinates.as_tuple(), (1, 1))
    }

    #[test]
    fn y_offset_goes_into_negative() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.y_offset(-2);

        assert_eq!(coordinates.as_tuple(), (1, -1))
    }

    #[test]
    fn movement_saturates_min_y() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.movement((0, -300));

        assert_eq!(coordinates.as_tuple(), (1, 0));
    }

    #[test]
    fn movement_saturates_max_y() {
        let max_y = i8::try_from(WORLD_HEIGHT - 1).unwrap();
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.movement((0, 300));

        assert_eq!(coordinates.as_tuple(), (1, max_y));
    }

    #[test]
    #[should_panic]
    fn new_panics_when_above_world_height() {
        Coordinates::new(1, 100);
    }

    #[test]
    #[should_panic]
    fn new_panics_when_below_negative_world_height() {
        Coordinates::new(1, -100);
    }

    #[test]
    #[should_panic]
    fn new_panics_when_wider_than_world_width() {
        Coordinates::new(WORLD_WIDTH, 1);
    }
}
