#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Neutral,
    Player1,
    Player2,
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Neutral => write!(f, "Neutral"),
            CellState::Player1 => write!(f, "P1"),
            CellState::Player2 => write!(f, "P2"),
        }
    }
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}
