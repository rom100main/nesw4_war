use crate::rule::Rule;
use crate::types::CellState;

/// Coordinates:
/// x from left (0) to right (width - 1)
/// y from top (0) to bottom (height - 1)
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub values: Vec<CellState>,
    pub toric: bool,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            width: size,
            height: size,
            values: vec![CellState::Neutral; size * size],
            toric: true,
        }
    }

    pub fn count(&self, value: CellState) -> usize {
        self.values.iter().filter(|v| **v == value).count()
    }

    /// Change grid state by applying all the rules of the players.
    /// `rules_p1` and `rules_p2` should not have any rule in common.
    pub fn next(&mut self, rules_p1: &Vec<Rule>, rules_p2: &Vec<Rule>) {
        let mut new_values = self.values.clone();

        for y in 0..self.height {
            'cell: for x in 0..self.width {
                let current_idx = self.get_idx(x, y);

                // Get the cell states for checking rules.
                // top, bottom, left, right refer to the cells relative to the current cell
                let (top_idx, bottom_idx, left_idx, right_idx) = if self.toric {
                    // Toroidal - wrap around
                    let top_y = if y == 0 { self.height - 1 } else { y - 1 };
                    let bottom_y = if y == self.height - 1 { 0 } else { y + 1 };
                    let left_x = if x == 0 { self.width - 1 } else { x - 1 };
                    let right_x = if x == self.width - 1 { 0 } else { x + 1 };
                    (
                        self.get_idx(x, top_y),
                        self.get_idx(x, bottom_y),
                        self.get_idx(left_x, y),
                        self.get_idx(right_x, y),
                    )
                } else {
                    // Non-toroidal - out of bounds
                    if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                        continue;
                    }
                    (
                        self.get_idx(x, y - 1),
                        self.get_idx(x, y + 1),
                        self.get_idx(x - 1, y),
                        self.get_idx(x + 1, y),
                    )
                };

                let top_state = self.values[top_idx];
                let bottom_state = self.values[bottom_idx];
                let left_state = self.values[left_idx];
                let right_state = self.values[right_idx];

                // Check if any rule matches
                for rule in rules_p1 {
                    if rule.next(top_state, bottom_state, left_state, right_state) {
                        new_values[current_idx] = CellState::Player1;
                        continue 'cell;
                    }
                }
                for rule in rules_p2 {
                    if rule.next(top_state, bottom_state, left_state, right_state) {
                        new_values[current_idx] = CellState::Player2;
                        continue 'cell;
                    }
                }
            }
        }

        self.values = new_values;
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        // TODO: return correct if toroidal or error if out of bounds
        y * self.width + x
    }
}
