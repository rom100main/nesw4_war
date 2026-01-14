use crate::CELL_SIZE;
use crate::grid::cell::CellState;
use eframe::egui;
use rand::Rng;

#[derive(PartialEq, Clone, Debug)]
pub struct Rule {
    pub top: CellState,
    pub right: CellState,
    pub bottom: CellState,
    pub left: CellState,
}

impl Rule {
    pub fn new() -> Rule {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let rule = Rule {
                top: match rng.gen_range(0..3) {
                    0 => CellState::Neutral,
                    1 => CellState::Player1,
                    _ => CellState::Player2,
                },
                right: match rng.gen_range(0..3) {
                    0 => CellState::Neutral,
                    1 => CellState::Player1,
                    _ => CellState::Player2,
                },
                bottom: match rng.gen_range(0..3) {
                    0 => CellState::Neutral,
                    1 => CellState::Player1,
                    _ => CellState::Player2,
                },
                left: match rng.gen_range(0..3) {
                    0 => CellState::Neutral,
                    1 => CellState::Player1,
                    _ => CellState::Player2,
                },
            };
            if rule.top != CellState::Neutral
                || rule.right != CellState::Neutral
                || rule.bottom != CellState::Neutral
                || rule.left != CellState::Neutral
            {
                return rule;
            }
        }
    }

    pub fn new_direction(cell: CellState) -> Rule {
        let mut rng = rand::thread_rng();
        let pattern: usize = rng.gen_range(0..4);
        match pattern {
            0 => Rule {
                top: cell,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            1 => Rule {
                top: CellState::Neutral,
                right: cell,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            2 => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: cell,
                left: CellState::Neutral,
            },
            _ => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: cell,
            },
        }
    }

    pub fn next(
        &self,
        top: CellState,
        right: CellState,
        bottom: CellState,
        left: CellState,
    ) -> bool {
        *self
            == Rule {
                top,
                right,
                bottom,
                left,
            }
    }

    pub fn show(&self, ui: &mut ::eframe::egui::Ui, index: usize) {
        let grid_size = CELL_SIZE * 3.0;

        ui.label(format!("Rule {}", index));

        let cursor = ui.cursor();
        let x_offset = cursor.min.x;
        let y_offset = cursor.min.y;

        let painter = ui.painter();

        let cells = [
            (1, 0, self.top),
            (2, 1, self.right),
            (1, 2, self.bottom),
            (0, 1, self.left),
        ];

        for (col, row, cell_state) in cells {
            let x = x_offset + col as f32 * CELL_SIZE;
            let y = y_offset + row as f32 * CELL_SIZE;

            let cell_rect =
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(CELL_SIZE, CELL_SIZE));

            cell_state.show(&painter, cell_rect);
        }

        ui.allocate_space(egui::vec2(grid_size, grid_size));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::cell::CellState;

    #[test]
    fn test_rule_next_true() {
        let rule = Rule {
            top: CellState::Player1,
            right: CellState::Neutral,
            bottom: CellState::Player2,
            left: CellState::Player1,
        };
        assert!(rule.next(
            CellState::Player1,
            CellState::Neutral,
            CellState::Player2,
            CellState::Player1,
        ));
    }

    #[test]
    fn test_rule_next_false() {
        let rule = Rule {
            top: CellState::Player1,
            right: CellState::Neutral,
            bottom: CellState::Player2,
            left: CellState::Player1,
        };
        assert!(!rule.next(
            CellState::Player2,
            CellState::Neutral,
            CellState::Player2,
            CellState::Player1,
        ));
    }
}
