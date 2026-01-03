use crate::constants::{GRID_SIZE, PLAYER_MONEY};
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
}

impl Game {
    pub fn new() -> Game {
        let size_grid = GRID_SIZE;
        Game {
            player1: Player::new(),
            player2: Player::new_p2(size_grid),
            grid: Grid::new(size_grid),
            size_grid,
            shop: Shop::new(),
        }
    }

    pub fn new_round(&mut self) {
        self.new_grid();
        self.new_shop();
        self.player1.money = PLAYER_MONEY;
        self.player2.money = PLAYER_MONEY;
    }

    fn new_grid(&mut self) {
        self.grid = Grid::new(self.size_grid);
        for spawn in &self.player1.spawn {
            let idx = spawn.y * self.size_grid + spawn.x;
            self.grid.values[idx] = CellState::Player1;
        }
        for spawn in &self.player2.spawn {
            let idx = spawn.y * self.size_grid + spawn.x;
            self.grid.values[idx] = CellState::Player2;
        }
    }

    fn new_shop(&mut self) {
        self.shop = Shop::new();
    }

    pub fn next_p1(&mut self) {
        self.grid.next(CellState::Player1, &self.player1.rules);
    }

    pub fn next_p2(&mut self) {
        self.grid.next(CellState::Player2, &self.player2.rules);
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        new_round_clicked: &mut bool,
        update_interval: &mut std::time::Duration,
    ) {
        ui.horizontal(|ui| {
            ui.heading("ToomWar Grid Game");
            ui.separator();

            if ui.button("New Round").clicked() {
                *new_round_clicked = true;
            }

            ui.checkbox(&mut self.grid.show_grid_lines, "Show Grid Lines");

            let p1_count = self.grid.count(CellState::Player1);
            let p2_count = self.grid.count(CellState::Player2);

            self.player1.show(ui, 1, p1_count);
            self.player2.show(ui, 2, p2_count);

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

        self.grid.show(ui);
        self.shop.show(ui);
    }
}
