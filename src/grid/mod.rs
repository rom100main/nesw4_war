use crate::rule::Rule;
use crate::types::CellState;

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

    pub fn next(&mut self, state: CellState, rules: &Vec<Rule>) {
        let mut new_values = self.values.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.values[y * self.width + x] == state {
                    continue;
                }

                let current_idx = y * self.width + x;
                let current_state = self.values[current_idx];

                // TODO(Clément): check if player 2 in this case reverse rul

                // Get the cell states for checking rules
                // top: cell above
                // inner: current cell
                // right: cell to the right
                let (top_idx, right_idx) = if self.toric {
                    // Toroidal - wrap around
                    let top_y = if y == 0 { self.height - 1 } else { y - 1 };
                    let right_x = if x == self.width - 1 { 0 } else { x + 1 };
                    (top_y * self.width + x, y * self.width + right_x)
                } else {
                    // Non-toroidal - out of bounds
                    if y == 0 || x == self.width - 1 {
                        continue;
                    }
                    ((y - 1) * self.width + x, y * self.width + (x + 1))
                };

                let top_state = self.values[top_idx];
                let right_state = self.values[right_idx];

                // Check if any rule matches
                for rule in rules {
                    if rule.next(top_state, current_state, right_state) {
                        new_values[current_idx] = state;
                        continue;
                    }
                }
            }
        }

        self.values = new_values;
    }
}
