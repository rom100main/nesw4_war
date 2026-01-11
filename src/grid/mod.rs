use crate::constants::{COLOR_GRID_LINE, COLOR_NEUTRAL, COLOR_PLAYER1, COLOR_PLAYER2};
use crate::rule::Rule;
use crate::types::CellState;
use eframe::egui;

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
    /// Create a new grid. \
    /// `pX_spawn_p` is the spawn probability of the player X. \
    /// `p1_spawn_p + p2_spawn_p` should be smaller than 1.0.
    pub fn new(size: usize, p1_spawn_p: f32, p2_spawn_p: f32) -> Grid {
        Grid {
            width: size,
            height: size,
            values: {
                use rand::Rng;
                let mut values: Vec<CellState> = vec![];
                for _ in 0..size * size {
                    let mut rng = rand::thread_rng();
                    let x: f32 = rng.r#gen();
                    if x <= p1_spawn_p {
                        values.push(CellState::Player1);
                    } else if x <= p1_spawn_p + p2_spawn_p {
                        values.push(CellState::Player2);
                    } else {
                        values.push(CellState::Neutral);
                    }
                }
                values
            },
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

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let available_rect = ui.available_rect_before_wrap();
        let padding = 10.0;

        let max_width = available_rect.width() - padding * 2.0;
        let max_height = available_rect.height() - padding * 2.0;

        if max_width.is_nan() || max_height.is_nan() || max_width <= 0.0 || max_height <= 0.0 {
            return;
        }

        let cell_size_w = max_width / self.width as f32;
        let cell_size_h = max_height / self.height as f32;
        let cell_size = cell_size_w.min(cell_size_h).min(50.0);
        let cell_size = cell_size.max(8.0);

        let grid_width_px = self.width as f32 * cell_size;
        let grid_height_px = self.height as f32 * cell_size;

        let x_pos = available_rect.min.x + (available_rect.width() - grid_width_px) / 2.0;
        let y_pos = available_rect.min.y;

        let painter_rect = egui::Rect::from_min_size(
            egui::pos2(x_pos, y_pos),
            egui::vec2(grid_width_px, grid_height_px),
        );

        let (_, painter) = ui.allocate_painter(
            egui::vec2(grid_width_px, grid_height_px),
            egui::Sense::hover(),
        );

        for row in 0..self.height {
            for col in 0..self.width {
                let x = x_pos + col as f32 * cell_size;
                let y = y_pos + row as f32 * cell_size;

                let cell_rect =
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(cell_size, cell_size));

                let cell_idx = row * self.width + col;
                let cell_state = self.values[cell_idx];

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
        }

        let p1_count = self.count(CellState::Player1);
        let p2_count = self.count(CellState::Player2);
        let border_color = if p1_count > p2_count {
            COLOR_PLAYER1
        } else if p2_count > p1_count {
            COLOR_PLAYER2
        } else {
            egui::Color32::BLACK
        };

        painter.rect_stroke(
            painter_rect,
            0.0,
            egui::Stroke::new(10.0, border_color),
            egui::StrokeKind::Inside,
        );
    }
    fn get_idx(&self, x: usize, y: usize) -> usize {
        // TODO: return correct if toroidal or error if out of bounds
        y * self.width + x
    }
}
