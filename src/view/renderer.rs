use super::{coordinates::Coordinates, factory::WORLD_WIDTH, util::chars_width};
use crate::app::color::ColorTheme;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::canvas::Context,
};

pub struct Renderer<'a, 'b> {
    ctx: &'a mut Context<'b>,
    offset: Coordinates,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(ctx: &'a mut Context<'b>, offset: Coordinates) -> Self {
        Self { ctx, offset }
    }

    pub fn render(&mut self, coordinates: Coordinates, text: &'static str, color: ColorTheme) {
        let style = Style::default().fg(Color::from(color));
        let (x, y) = coordinates.as_tuple();
        let x = f64::from(x);
        let y = f64::from(y);

        // Reverse the string, because it is stored top->down, but is rendered bottom->up.
        for (y_offset, line) in text.lines().rev().enumerate() {
            let span = Span::styled(line, style);
            self.ctx.print(x, y + y_offset as f64, span);
        }
    }

    pub fn render_with_offset(
        &mut self,
        mut coordinates: Coordinates,
        text: &'static str,
        color: ColorTheme,
    ) {
        coordinates.offset(self.offset);

        let style = Style::default().fg(Color::from(color));
        let (x, y) = coordinates.as_tuple();
        let x_offset = x_offset(x, text);
        let mut x = f64::from(x);
        let y = f64::from(y);

        // Reverse the string, because it is stored top->down, but is rendered bottom->up.
        for (y_offset, line) in text.lines().rev().enumerate() {
            let mut line = line;
            if x_offset != 0 {
                line = crop_left_offset(line, x_offset);
                x = 0.;
            }
            let span = Span::styled(line, style);
            self.ctx.print(x, y + y_offset as f64, span);
        }
    }

    pub fn render_spans(&mut self, coordinates: Coordinates, spans: Vec<Span<'b>>) {
        let (x, y) = coordinates.as_tuple();
        let x = f64::from(x);
        let y = f64::from(y);
        self.ctx.print(x, y, Spans::from(spans));
    }
}

fn crop_left_offset(s: &str, offset: usize) -> &str {
    match s.char_indices().nth(offset) {
        Some((idx, _)) => &s[idx..],
        None => "",
    }
}

fn x_offset(x: u8, s: &str) -> usize {
    let width = chars_width(s.lines().next().unwrap());
    let x_offset = WORLD_WIDTH - x;
    if x_offset < width {
        usize::try_from(x_offset).unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn crop_left_offset_returns_midway_split() {
        let actual = crop_left_offset("abcd", 2);

        assert_eq!(actual, "cd");
    }

    #[test]
    fn crop_left_offset_returns_empty_when_offset_equals_length() {
        let actual = crop_left_offset("a", 1);

        assert_eq!(actual, "");
    }

    #[test]
    fn crop_left_offset_returns_empty_when_offset_greater_than_length() {
        let actual = crop_left_offset("a", 2);

        assert_eq!(actual, "");
    }

    #[test]
    fn x_offset_returns_0_when_x_is_positive() {
        let offset = x_offset(0, "a");

        assert_eq!(offset, 0);
    }

    #[test]
    fn x_offset_returns_0_when_just_before_wrap_boundary_with_overlap() {
        let offset = x_offset(199, "ab");

        assert_eq!(offset, 1);
    }
    #[test]
    fn x_offset_returns_0_when_just_before_wrap_boundary_without_overlap() {
        let offset = x_offset(199, "a");

        assert_eq!(offset, 0);
    }
}
