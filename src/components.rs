#[derive(Default)]
pub(crate) struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

#[derive(Default)]
pub(crate) struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Clone)]
pub(crate) struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Default)]
pub(crate) struct CharacterView {
    pub position: Position,
    pub dir: f64,
    pub radius: f64,
}
