pub mod constants;
pub mod game;
pub mod grid;
pub mod player;
pub mod rule;
pub mod shop;
pub mod types;

pub use constants::*;
pub use game::Game;
pub use grid::Grid;
pub use player::Player;
pub use rule::Rule;
pub use shop::Shop;
pub use types::*;

use eframe::egui;
use egui::{Color32, Rect, pos2, vec2};
use std::time::{Duration, Instant};

struct GameUI {
    game: Game,
    last_update: Instant,
    update_interval: Duration,
    show_grid_lines: bool,
}

impl Default for GameUI {
    fn default() -> Self {
        let mut game = Game::new();
        game.new_round(); // Initialize the game with a new round

        // Add more effective rules for both players to demonstrate grid evolution
        // Player 1: Spread to adjacent neutral cells
        let rule1 = Rule {
            top: CellState::Neutral,
            inner: CellState::Player1,
            right: CellState::Neutral,
        };
        game.player1.rules.push(rule1);

        let rule1_alt = Rule {
            top: CellState::Player1,
            inner: CellState::Neutral,
            right: CellState::Neutral,
        };
        game.player1.rules.push(rule1_alt);

        // Player 2: Different spreading pattern
        let rule2 = Rule {
            top: CellState::Neutral,
            inner: CellState::Player2,
            right: CellState::Neutral,
        };
        game.player2.rules.push(rule2);

        let rule2_alt = Rule {
            top: CellState::Neutral,
            inner: CellState::Neutral,
            right: CellState::Player2,
        };
        game.player2.rules.push(rule2_alt);

        // Add some initial player cells in different locations for visibility
        for i in 0..10 {
            let idx = i * game.grid.width + 10 + i; // Diagonal pattern for P1
            if idx < game.grid.values.len() {
                game.grid.values[idx] = CellState::Player1;
            }
            let idx2 = i * game.grid.width + 35 - i; // Opposite diagonal for P2
            if idx2 < game.grid.values.len() {
                game.grid.values[idx2] = CellState::Player2;
            }
        }

        Self {
            game,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            show_grid_lines: true, // Enable by default for better visibility
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle timing for updates
        if self.last_update.elapsed() >= self.update_interval {
            self.update_game();
            self.last_update = Instant::now();
        }

        // Request continuous repainting for smooth animation
        ctx.request_repaint_after(Duration::from_millis(100));

        // Create the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_ui(ui);
            self.draw_grid(ui);
        });
    }
}

impl GameUI {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("ToomWar Grid Game");
            ui.separator();

            // Controls
            if ui.button("New Round").clicked() {
                self.game.new_round();
            }

            ui.checkbox(&mut self.show_grid_lines, "Show Grid Lines");

            // Player stats
            let p1_count = self.game.grid.count(CellState::Player1);
            let p2_count = self.game.grid.count(CellState::Player2);

            ui.label(format!("Player 1: {} cells", p1_count));
            ui.label(format!("Player 2: {} cells", p2_count));

            // Update speed control
            ui.label("Update Speed:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{}ms", self.update_interval.as_millis()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.update_interval,
                        Duration::from_millis(100),
                        "100ms",
                    );
                    ui.selectable_value(
                        &mut self.update_interval,
                        Duration::from_millis(200),
                        "200ms",
                    );
                    ui.selectable_value(
                        &mut self.update_interval,
                        Duration::from_millis(300),
                        "300ms",
                    );
                    ui.selectable_value(
                        &mut self.update_interval,
                        Duration::from_millis(500),
                        "500ms",
                    );
                });
        });
    }

    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        // Get the available size in the current panel
        let available_size = ui.available_size();

        // Calculate optimal cell size to fit the entire grid
        let padding = 20.0; // Padding around the grid
        let max_width = available_size.x - padding * 2.0;
        let max_height = available_size.y - padding * 2.0;

        // Calculate cell size that allows the entire grid to fit
        let cell_size_w = max_width / self.game.grid.width as f32;
        let cell_size_h = max_height / self.game.grid.height as f32;
        let cell_size = cell_size_w.min(cell_size_h).min(50.0); // Cap at 50px for visibility

        // Ensure minimum cell size for visibility
        let cell_size = cell_size.max(8.0);

        let grid_width_px = self.game.grid.width as f32 * cell_size;
        let grid_height_px = self.game.grid.height as f32 * cell_size;

        // Add some space at the top
        ui.add_space(10.0);

        // Calculate position to center the grid
        let x_pos = (available_size.x - grid_width_px) / 2.0;
        let y_pos = ui.cursor().min.y;

        // Create a custom response to control the space allocation
        let (_, painter) = ui.allocate_painter(
            egui::vec2(available_size.x, grid_height_px + padding * 2.0),
            egui::Sense::hover(),
        );

        // Draw the grid directly using the painter
        for row in 0..self.game.grid.height {
            for col in 0..self.game.grid.width {
                let x = x_pos + col as f32 * cell_size;
                let y = y_pos + row as f32 * cell_size;

                let cell_rect = Rect::from_min_size(pos2(x, y), vec2(cell_size, cell_size));

                let cell_idx = row * self.game.grid.width + col;
                let cell_state = self.game.grid.values[cell_idx];

                // Draw filled rectangle based on cell state
                match cell_state {
                    CellState::Neutral => {
                        painter.rect_filled(cell_rect, 0.0, Color32::from_gray(200));
                    }
                    CellState::Player1 => {
                        painter.rect_filled(
                            cell_rect,
                            0.0,
                            Color32::from_rgb(255, 100, 100), // Red
                        );
                    }
                    CellState::Player2 => {
                        painter.rect_filled(
                            cell_rect,
                            0.0,
                            Color32::from_rgb(100, 100, 255), // Blue
                        );
                    }
                }

                // Draw grid lines if enabled
                if self.show_grid_lines {
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(0.5, Color32::from_gray(150)),
                        egui::StrokeKind::Inside,
                    );
                }
            }
        }

        // Draw border around the entire grid
        painter.rect_stroke(
            Rect::from_min_size(pos2(x_pos, y_pos), vec2(grid_width_px, grid_height_px)),
            2.0,
            egui::Stroke::new(2.0, Color32::BLACK),
            egui::StrokeKind::Inside,
        );
    }

    fn update_game(&mut self) {
        // Update both players' grids
        self.game.next_p1();
        self.game.next_p2();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("ToomWar - Grid Game"),
        ..Default::default()
    };

    Ok(eframe::run_native(
        "ToomWar",
        options,
        Box::new(|_cc| Ok(Box::<GameUI>::default())),
    )?)
}
