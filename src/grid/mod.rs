use crate::rule::Rule;
use crate::types::CellState;
use eframe::egui;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub values: Vec<CellState>,
    pub toric: bool,
    pub show_grid_lines: bool,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            width: size,
            height: size,
            values: vec![CellState::Neutral; size * size],
            toric: true,
            show_grid_lines: true,
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

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();

        let padding = 20.0;
        let max_width = available_size.x - padding * 2.0;
        let max_height = available_size.y - padding * 2.0;

        let cell_size_w = max_width / self.width as f32;
        let cell_size_h = max_height / self.height as f32;
        let cell_size = cell_size_w.min(cell_size_h).min(50.0);
        let cell_size = cell_size.max(8.0);

        let grid_width_px = self.width as f32 * cell_size;
        let grid_height_px = self.height as f32 * cell_size;

        ui.add_space(10.0);

        let x_pos = (available_size.x - grid_width_px) / 2.0;
        let y_pos = ui.cursor().min.y;

        let (_, painter) = ui.allocate_painter(
            egui::vec2(available_size.x, grid_height_px + padding * 2.0),
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
                        painter.rect_filled(cell_rect, 0.0, egui::Color32::from_gray(200));
                    }
                    CellState::Player1 => {
                        painter.rect_filled(cell_rect, 0.0, egui::Color32::from_rgb(255, 100, 100));
                    }
                    CellState::Player2 => {
                        painter.rect_filled(cell_rect, 0.0, egui::Color32::from_rgb(100, 100, 255));
                    }
                }

                if self.show_grid_lines {
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(0.5, egui::Color32::from_gray(150)),
                        egui::StrokeKind::Inside,
                    );
                }
            }
        }

        painter.rect_stroke(
            egui::Rect::from_min_size(
                egui::pos2(x_pos, y_pos),
                egui::vec2(grid_width_px, grid_height_px),
            ),
            2.0,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
            egui::StrokeKind::Inside,
        );
    }
}
