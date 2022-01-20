use tui::layout::Rect;

pub type Coordinates = (u16, u16);
pub type Movement = (i16, i16);

#[derive(Copy, Clone)]
pub struct Viewport {
    pub rect: Rect,
}

impl Viewport {
    pub fn new(width: u16, height: u16) -> Self {
        Self::new_from_coordinates(width, height, (0, 0))
    }

    pub fn new_from_coordinates(width: u16, height: u16, bottom_left: Coordinates) -> Self {
        let (x, y) = bottom_left;
        let rect = Rect {
            x,
            y,
            height,
            width,
        };
        Self { rect }
    }

    pub fn bottom_left(&self) -> Coordinates {
        (self.rect.x, self.rect.y)
    }

    pub fn center(&self) -> Coordinates {
        let (x1, y1) = self.bottom_left();
        let (x2, y2) = self.top_right();
        ((x1 + x2) / 2, (y1 + y2) / 2)
    }

    pub fn centered_around_bottom_left(&self) -> Coordinates {
        let (x1, y1) = self.bottom_left();
        (
            x1.saturating_sub(self.rect.width / 2),
            y1.saturating_sub(self.rect.height / 2),
        )
    }

    pub fn contain(&self, other: &Self) -> Coordinates {
        let (x_bl, y_bl) = other.bottom_left();
        let mut x_bl = x_bl;
        let mut y_bl = y_bl;
        let (x_tr, y_tr) = other.top_right();
        let (x_max, y_max) = self.top_right();
        if x_tr > x_max {
            x_bl = x_bl.saturating_sub(x_tr.saturating_sub(x_max));
        }
        if y_tr > y_max {
            y_bl = y_bl.saturating_sub(y_tr.saturating_sub(y_max));
        }
        (x_bl, y_bl)
    }

    pub fn out_of_bounds_completely(&self, other: &Self) -> bool {
        let (x, y) = other.bottom_left();
        let (max_x, max_y) = self.top_right();
        return x > max_x || y > max_y;
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.rect.intersects(other.rect)
    }

    pub fn top_right(&self) -> Coordinates {
        let rect = self.rect;
        let x = rect.x + rect.width.saturating_sub(1);
        let y = rect.y + rect.height.saturating_sub(1);
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contain_handles_complete_overlap() {
        let bl = Viewport::new(2, 2);
        let tr = Viewport::new_from_coordinates(2, 2, (2, 2));

        let (x, y) = bl.contain(&tr);

        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn contain_handles_partial_overlap() {
        let bl = Viewport::new(2, 2);
        let tr = Viewport::new_from_coordinates(2, 2, (1, 1));

        let (x, y) = bl.contain(&tr);

        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn contain_translates_minimally() {
        let bl = Viewport::new(2, 2);
        let tr = Viewport::new_from_coordinates(1, 1, (2, 2));

        let (x, y) = bl.contain(&tr);

        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn contain_handles_larger() {
        let bl = Viewport::new(2, 2);
        let tr = Viewport::new_from_coordinates(3, 3, (2, 2));

        let (x, y) = bl.contain(&tr);

        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn out_of_bounds_completely_returns_false_when_overlapping() {
        let bl = Viewport::new_from_coordinates(2, 2, (0, 0));
        let tr = Viewport::new_from_coordinates(2, 2, (1, 1));
        assert!(!bl.out_of_bounds_completely(&tr));
    }

    #[test]
    fn out_of_bounds_completely_returns_true_when_right_adjacent() {
        let bl = Viewport::new_from_coordinates(2, 2, (0, 0));
        let tr = Viewport::new_from_coordinates(2, 2, (2, 0));
        assert!(!bl.out_of_bounds_completely(&tr));
    }

    #[test]
    fn out_of_bounds_completely_returns_true_when_bottom_adjacent() {
        let bl = Viewport::new_from_coordinates(2, 2, (2, 2));
        let tr = Viewport::new_from_coordinates(2, 2, (2, 0));
        assert!(!bl.out_of_bounds_completely(&tr));
    }
}
