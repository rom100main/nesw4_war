use crate::types::CellState;

#[derive(PartialEq)]
pub struct Rule {
    pub top: CellState,
    pub inner: CellState,
    pub right: CellState,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CellState;

    #[test]
    fn test_rule_next_true() {
        let rule = Rule {
            top: CellState::Player1,
            inner: CellState::Player2,
            right: CellState::Neutral,
        };
        assert!(rule.next(CellState::Player1, CellState::Player2, CellState::Neutral));
    }

    #[test]
    fn test_rule_next_false() {
        let rule = Rule {
            top: CellState::Player1,
            inner: CellState::Player2,
            right: CellState::Neutral,
        };
        assert!(!rule.next(CellState::Player2, CellState::Player2, CellState::Neutral));
    }
}
