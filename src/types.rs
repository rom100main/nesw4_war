#[derive(PartialEq, Clone, Copy)]
pub enum CellState {
    Neutral,
    Player1,
    Player2,
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}