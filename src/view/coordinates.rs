use super::factory::{WORLD_HEIGHT, WORLD_WIDTH};

pub type Movement = (i16, i16);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        let x = wrap_x(i16::from(x));
        let y = saturate_y(i16::from(y));
        Self(x, y)
    }

    pub fn movement(&mut self, (dx, dy): Movement) {
        let Coordinates(x, y) = *self;
        let x = i16::from(x);
        let y = i16::from(y);
        self.0 = wrap_x(x + dx);
        self.1 = saturate_y(y + dy);
        self.validate();
    }

    pub fn offset(&mut self, Coordinates(x, y): Coordinates) {
        self.offset_((i16::from(x), i16::from(y)));
    }

    pub fn offset_x(&mut self, dx: i16) {
        self.offset_((dx, 0));
    }

    pub fn offset_y(&mut self, dy: i16) {
        self.offset_((0, dy));
    }

    pub fn as_tuple(&self) -> (u8, i8) {
        (self.0, self.1)
    }

    fn offset_(&mut self, (dx, dy): (i16, i16)) {
        let Coordinates(x, _) = *self;
        let x = i16::from(x);
        self.0 = wrap_x(x + dx);
        self.1 += i8::try_from(dy).unwrap();
        self.validate();
    }

    fn validate(&self) {
        let Coordinates(x, y) = *self;
        assert!(x < WORLD_WIDTH);

        // Actors that are spawned at the top of the WORLD_HEIGHT may have top_right() Coordinates that
        // are a bit higher, and actors may be deleted only after dipping below y=0 by a few positions,
        // so double the WORLD_HEIGHT in the assertion to compensate while still providing a sanity check.
        let y_abs = u8::try_from(y.abs()).unwrap();
        assert!(y_abs < 2 * WORLD_HEIGHT)
    }
}

fn saturate_y(mut y: i16) -> i8 {
    let max_y = i16::from(WORLD_HEIGHT) - 1;
    if y.is_negative() {
        y = 0
    } else if y > max_y {
        y = max_y;
    }
    i8::try_from(y).unwrap()
}

fn wrap_x(mut x: i16) -> u8 {
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
        coordinates.offset_x(-2);

        assert_eq!(coordinates.as_tuple(), (199, 1));
    }

    #[test]
    fn x_offset_wraps_right() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.offset_x(200);

        assert_eq!(coordinates.as_tuple(), (1, 1))
    }

    #[test]
    fn y_offset_goes_into_negative() {
        let mut coordinates = Coordinates::new(1, 1);
        coordinates.offset_y(-2);

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
