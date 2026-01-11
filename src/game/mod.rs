use crate::PLAYER_ADD_MONEY;
use crate::constants::{
    COLOR_NEUTRAL, COLOR_PLAYER1, COLOR_PLAYER2, GRID_SIZE, MAX_ITERATIONS, PLAYER_SPAWN_PROBA,
};
use crate::grid::Grid;
use crate::player::Player;
use crate::shop::Shop;
use crate::types::CellState;
use eframe::egui;

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub grid: Grid,
    pub size_grid: usize,
    pub shop: Shop,
    pub iteration: usize,
    pub round_over: bool,
    pub round_result: Option<String>,
    pub shop_first_player: u8,
}

impl Game {
    pub fn new() -> Game {
        let size_grid = GRID_SIZE;
        let player1 = Player::new(format!("Player 1"));
        let player2 = Player::new(format!("Player 2"));
        let shop = Shop::new_with_players(&player1, &player2);
        Game {
            player1,
            player2,
            grid: Grid::new(size_grid, PLAYER_SPAWN_PROBA, PLAYER_SPAWN_PROBA),
            size_grid,
            shop,
            iteration: 0,
            round_over: false,
            round_result: None,
            shop_first_player: 1,
        }
    }

    pub fn new_round(&mut self) {
        self.new_grid();
        self.new_shop();
        self.player1.money += PLAYER_ADD_MONEY;
        self.player2.money += PLAYER_ADD_MONEY;
        self.iteration = 0;
        self.round_over = false;
        self.round_result = None;
    }

    fn new_shop(&mut self) {
        self.shop = Shop::new_with_players(&self.player1, &self.player2);
    }

    pub fn advance_iteration(&mut self) {
        if self.round_over {
            return;
        }
        self.iteration += 1;
        if self.iteration >= MAX_ITERATIONS {
            self.end_round();
        }
    }

    fn end_round(&mut self) {
        self.round_over = true;
        let p1_count = self.grid.count(CellState::Player1);
        let p2_count = self.grid.count(CellState::Player2);

        if p1_count > p2_count {
            self.player1.win += 1;
            self.round_result = Some(format!("Player 1 wins!\n{} vs {}", p1_count, p2_count));
            self.shop_first_player = 2;
        } else if p2_count > p1_count {
            self.player2.win += 1;
            self.round_result = Some(format!("Player 2 wins!\n{} vs {}", p2_count, p1_count));
            self.shop_first_player = 1;
        } else {
            self.round_result = Some(format!("Draw!\n{} - {}", p1_count, p2_count));
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, shop_clicked: &mut bool) {
        ui.heading(egui::RichText::new("New Extreme Strategical Warfare").size(24.0));
        ui.add_space(20.0);

        let p1_count = self.grid.count(CellState::Player1);
        let p2_count = self.grid.count(CellState::Player2);

        let available_rect = ui.available_rect_before_wrap();

        if available_rect.width().is_nan() || available_rect.width() <= 0.0 {
            return;
        }

        let total_width = available_rect.width();
        let p1_section_width = total_width / 5.0;
        let grid_section_width = total_width * 3.0 / 5.0;
        let p2_section_width = total_width / 5.0;

        let bar_height = 50.0;
        let mut game_rect = available_rect;
        game_rect.max.y -= bar_height;

        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(available_rect.min.x, game_rect.max.y),
            available_rect.max,
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(game_rect), |ui| {
            ui.horizontal(|ui| {
                ui.scope(|ui| {
                    ui.set_min_width(p1_section_width);
                    ui.set_max_width(p1_section_width);
                    ui.vertical(|ui| {
                        ui.heading(
                            egui::RichText::new("Player 1")
                                .color(COLOR_PLAYER1)
                                .size(18.0)
                                .strong(),
                        );
                        ui.add_space(10.0);
                        self.player1.show(ui, p1_count);
                    });
                });

                ui.scope(|ui| {
                    ui.set_min_width(grid_section_width);
                    ui.set_max_width(grid_section_width);
                    ui.vertical_centered(|ui| {
                        let iter_text = if self.round_over {
                            format!("Round Over: {}/{}", self.iteration, MAX_ITERATIONS)
                        } else {
                            format!("Iteration: {}/{}", self.iteration, MAX_ITERATIONS)
                        };
                        ui.heading(egui::RichText::new(iter_text).size(18.0));
                        ui.add_space(10.0);

                        self.grid.show(ui);

                        if let Some(ref result) = self.round_result {
                            ui.add_space(10.0);
                            ui.heading(result);
                        }

                        if self.round_over {
                            ui.add_space(5.0);
                            if ui.button(egui::RichText::new("Shop").size(18.0)).clicked() {
                                *shop_clicked = true;
                            }
                        }
                    });
                });

                ui.scope(|ui| {
                    ui.set_min_width(p2_section_width);
                    ui.set_max_width(p2_section_width);
                    ui.vertical(|ui| {
                        ui.heading(
                            egui::RichText::new("Player 2")
                                .color(COLOR_PLAYER2)
                                .size(18.0)
                                .strong(),
                        );
                        ui.add_space(10.0);
                        self.player2.show(ui, p2_count);
                    });
                });
            });
        });

        ui.scope_builder(egui::UiBuilder::new().max_rect(bar_rect), |ui| {
            let neutral_count = self.grid.count(CellState::Neutral);
            let total_cells = (p1_count + p2_count + neutral_count) as f32;

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(grid_section_width, 30.0),
                    egui::Sense::hover(),
                );

                let painter = ui.painter();
                let width = rect.width();
                let p1_width = (p1_count as f32 / total_cells) * width;
                let neutral_width = (neutral_count as f32 / total_cells) * width;
                let p2_width = width - p1_width - neutral_width;

                let p1_rect =
                    egui::Rect::from_min_size(rect.min, egui::vec2(p1_width, rect.height()));
                let neutral_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(p1_width, 0.0),
                    egui::vec2(neutral_width, rect.height()),
                );
                let p2_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(p1_width + neutral_width, 0.0),
                    egui::vec2(p2_width, rect.height()),
                );

                painter.rect_filled(p1_rect, 0.0, COLOR_PLAYER1);
                painter.rect_filled(neutral_rect, 0.0, COLOR_NEUTRAL);
                painter.rect_filled(p2_rect, 0.0, COLOR_PLAYER2);
            });
        });
    }

    fn new_grid(&mut self) {
        self.grid = Grid::new(
            self.size_grid,
            self.player1.spawn_proba,
            self.player2.spawn_proba,
        );
    }
}
