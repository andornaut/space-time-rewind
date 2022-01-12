use tui::layout::{Constraint, Direction, Layout, Rect};

const BOARD_MIN_HEIGHT: u16 = 10;
const BUTTON_PANEL_HEIGHT: u16 = 3;
const MAX_HEIGHT: u16 = 60;
const MAX_WIDTH: u16 = 79;
const MIN_HEIGHT: u16 = 1; // TODO change to 20;
const MIN_WIDTH: u16 = 1; // TODO change to 40;

pub fn split_rect(rect: Rect) -> (Rect, Rect) {
    validate(rect);

    let rect = normalize(rect);
    let constraints = [
        Constraint::Min(BOARD_MIN_HEIGHT),
        Constraint::Length(BUTTON_PANEL_HEIGHT),
    ];
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(rect);
    (rects[0], rects[1])
}

fn normalize(rect: Rect) -> Rect {
    Rect {
        height: MAX_HEIGHT,
        width: MAX_WIDTH,
        ..rect
    }
    .intersection(rect)
}

fn validate(rect: Rect) {
    let Rect { height, width, .. } = rect;
    validate_length(width, MIN_WIDTH, "width");
    validate_length(height, MIN_HEIGHT, "height");
}

fn validate_length(length: u16, min_length: u16, label: &'static str) {
    if length < min_length {
        panic!(
            "Please resize your terminal so that it is at least {min_length} characters in {label}.
            It's currently {length} characters in {label}.",
            min_length = min_length,
            label = label,
            length = length
        );
    }
}
