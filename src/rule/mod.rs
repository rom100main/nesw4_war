use crate::constants::{COLOR_GRID_LINE, COLOR_NEUTRAL, COLOR_PLAYER1, COLOR_PLAYER2};
use crate::types::CellState;
use eframe::egui;

#[derive(PartialEq, Clone)]
pub struct Rule {
    pub top: CellState,
    pub bottom: CellState,
    pub left: CellState,
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
            right: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
        }
    }

    pub fn next(
        &self,
        top: CellState,
        bottom: CellState,
        left: CellState,
        right: CellState,
    ) -> bool {
        *self
            == Rule {
                top,
                bottom,
                left,
                right,
            }
    }

    pub fn show(&self, ui: &mut ::eframe::egui::Ui, index: usize) {
        let cell_size = 15.0;
        let grid_size = cell_size * 3.0;

        ui.label(format!("Rule {}", index));

        let cursor = ui.cursor();
        let x_offset = cursor.min.x;
        let y_offset = cursor.min.y;

        let painter = ui.painter();

        let cells = [
            (0, 0, CellState::Neutral),
            (1, 0, self.top),
            (2, 0, CellState::Neutral),
            (0, 1, self.left),
            (1, 1, CellState::Neutral),
            (2, 1, self.right),
            (0, 2, CellState::Neutral),
            (1, 2, self.bottom),
            (2, 2, CellState::Neutral),
        ];

        for (col, row, cell_state) in cells {
            let x = x_offset + col as f32 * cell_size;
            let y = y_offset + row as f32 * cell_size;

            let cell_rect =
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(cell_size, cell_size));

            match cell_state {
                CellState::Neutral => {
                    painter.rect_filled(cell_rect, 0.0, COLOR_NEUTRAL);
                }
                CellState::Player1 => {
                    painter.rect_filled(cell_rect, 0.0, COLOR_PLAYER1);
                }
                CellState::Player2 => {
                    painter.rect_filled(cell_rect, 0.0, COLOR_PLAYER2);
                }
            }

            painter.rect_stroke(
                cell_rect,
                0.0,
                egui::Stroke::new(0.5, COLOR_GRID_LINE),
                egui::StrokeKind::Inside,
            );
        }

        let grid_rect = egui::Rect::from_min_size(
            egui::pos2(x_offset, y_offset),
            egui::vec2(grid_size, grid_size),
        );
        painter.rect_stroke(
            grid_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::BLACK),
            egui::StrokeKind::Inside,
        );

        ui.allocate_space(egui::vec2(grid_size, grid_size));
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
            bottom: CellState::Player2,
            left: CellState::Player1,
            right: CellState::Neutral,
        };
        assert!(rule.next(
            CellState::Player1,
            CellState::Player2,
            CellState::Player1,
            CellState::Neutral
        ));
    }

    #[test]
    fn test_rule_next_false() {
        let rule = Rule {
            top: CellState::Player1,
            bottom: CellState::Player2,
            left: CellState::Player1,
            right: CellState::Neutral,
        };
        assert!(!rule.next(
            CellState::Player2,
            CellState::Player2,
            CellState::Player1,
            CellState::Neutral
        ));
    }
}
