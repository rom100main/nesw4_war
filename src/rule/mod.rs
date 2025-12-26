use crate::types::CellState;

#[derive(PartialEq)]
pub struct Rule {
    top: CellState,
    inner: CellState,
    right: CellState,
}

impl Rule {
    pub fn new() -> Rule {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Rule {
            top: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
            inner: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
            right: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
        }
    }

    pub fn next(&self, top: CellState, inner: CellState, right: CellState) -> bool {
        *self == Rule { top, inner, right }
    }
}
