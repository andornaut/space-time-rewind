use tui::layout::Rect;

pub type Coordinates = (u16, u16);
pub type Movement = (i16, i16);

#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    pub rect: Rect,
}

impl Viewport {
    pub fn new(width: u16, height: u16) -> Self {
        Self::new_from_coordinates(width, height, (0, 0))
    }

    pub fn new_from_coordinates(width: u16, height: u16, (x, y): Coordinates) -> Self {
        let rect = Rect {
            x,
            y,
            height,
            width,
        };
        Self { rect }
    }

    pub fn bottom_left(self) -> Coordinates {
        (self.rect.x, self.rect.y)
    }

    pub fn center(self) -> Coordinates {
        let (x1, y1) = self.bottom_left();
        (x1 + (self.rect.width / 2), y1 + (self.rect.height / 2))
    }

    pub fn centered_around_bottom_left(self) -> Self {
        let (x1, y1) = self.bottom_left();
        let coordinates = (x1 - (self.rect.width / 2), y1 - (self.rect.height / 2));
        Self::new_from_coordinates(self.rect.width, self.rect.height, coordinates)
    }

    pub fn contain(self, other: &Self) -> Coordinates {
        let (x_bl, y_bl) = other.bottom_left();
        let mut x_bl = x_bl;
        let mut y_bl = y_bl;
        let (x_tr, y_tr) = other.top_right();
        let (x_max, y_max) = self.top_right();
        if x_tr > x_max {
            x_bl -= x_tr.saturating_sub(x_max);
        }
        if y_tr > y_max {
            y_bl -= y_tr.saturating_sub(y_max);
        }
        (x_bl, y_bl)
    }

    pub fn out_of_bounds(self, other: &Self) -> bool {
        let (x, y) = other.top_right();
        let (max_x, max_y) = self.top_right();
        return x > max_x || y > max_y;
    }

    pub fn out_of_bounds_partial(self, other: &Self) -> bool {
        let (x, y) = other.bottom_left();
        let (max_x, max_y) = self.top_right();
        return x > max_x || y > max_y;
    }

    pub fn overlaps(self, other: &Self) -> bool {
        let self_rect = self.rect;
        let other_rect = other.rect;
        self_rect.x < other_rect.x + other_rect.width
            && self_rect.x + self_rect.width > other_rect.x
            && self_rect.y < other_rect.y + other_rect.height
            && self_rect.height + self_rect.y > other_rect.y
    }

    pub fn top_right(self) -> Coordinates {
        let rect = self.rect;
        let x = rect.x + rect.width - 1;
        let y = rect.y + rect.height - 1;
        (x, y)
    }
}
