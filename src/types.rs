#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Neutral,
    Player1,
    Player2,
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}
