use crate::constants::{GRID_SIZE, MAX_ITERATIONS, PLAYER_MONEY, PLAYER_SPAWN_PROBA};
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
        let player1 = Player::new();
        let player2 = Player::new();
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
        self.player1.money = PLAYER_MONEY;
        self.player2.money = PLAYER_MONEY;
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
            self.round_result = Some(format!("Player 1 wins! {} vs {}", p1_count, p2_count));
        } else if p2_count > p1_count {
            self.player2.win += 1;
            self.round_result = Some(format!("Player 2 wins! {} vs {}", p2_count, p1_count));
        } else {
            self.round_result = Some(format!("Draw! {} - {}", p1_count, p2_count));
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        update_interval: &mut std::time::Duration,
        current_page: &mut super::Page,
    ) {
        ui.heading("ToomWar Grid Game");

        ui.horizontal(|ui| {
            if self.round_over {
                if ui.button("Shop").clicked() {
                    let p1_count = self.grid.count(CellState::Player1);
                    let p2_count = self.grid.count(CellState::Player2);

                    if p1_count > p2_count {
                        self.shop_first_player = 2;
                    } else {
                        self.shop_first_player = 1;
                    }
                    *current_page = super::Page::Shop;
                }
            }

            ui.checkbox(&mut self.grid.show_grid_lines, "Show Grid Lines");

            ui.label("Update Speed:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{}ms", update_interval.as_millis()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        update_interval,
                        std::time::Duration::from_millis(100),
                        "100ms",
                    );
                    ui.selectable_value(
                        update_interval,
                        std::time::Duration::from_millis(200),
                        "200ms",
                    );
                    ui.selectable_value(
                        update_interval,
                        std::time::Duration::from_millis(300),
                        "300ms",
                    );
                    ui.selectable_value(
                        update_interval,
                        std::time::Duration::from_millis(500),
                        "500ms",
                    );
                });
        });

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

        ui.horizontal(|ui| {
            ui.scope(|ui| {
                ui.set_min_width(p1_section_width);
                ui.set_max_width(p1_section_width);
                ui.vertical(|ui| {
                    ui.heading("Player 1");
                    self.player1.show(ui, p1_count);
                });
            });

            ui.scope(|ui| {
                ui.set_min_width(grid_section_width);
                ui.set_max_width(grid_section_width);
                ui.vertical_centered(|ui| {
                    let iter_text = if self.round_over {
                        format!("Round Over - {}/{}", self.iteration, MAX_ITERATIONS)
                    } else {
                        format!("Iteration: {}/{}", self.iteration, MAX_ITERATIONS)
                    };
                    ui.heading(iter_text);
                    self.grid.show(ui);

                    if let Some(ref result) = self.round_result {
                        ui.add_space(10.0);
                        ui.heading(result);
                    }
                });
            });

            ui.scope(|ui| {
                ui.set_min_width(p2_section_width);
                ui.set_max_width(p2_section_width);
                ui.vertical(|ui| {
                    ui.heading("Player 2");
                    self.player2.show(ui, p2_count);
                });
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
