pub mod constants;
pub mod game;
pub mod grid;
pub mod player;
pub mod rule;
pub mod rule_picker;
pub mod shop;
pub mod types;

pub use constants::*;
pub use game::Game;
pub use grid::Grid;
pub use player::Player;
pub use rule::Rule;
pub use rule_picker::RulePicker;
pub use shop::Shop;
pub use types::*;

use eframe::egui;
use std::time::{Duration, Instant};

pub use constants::SHOP_NB_RULES;

pub enum Page {
    LandingScreen,
    InitialRulePicker,
    MainGame,
    Shop,
    EndScreen,
}

struct GameUI {
    game: Game,
    last_update: Instant,
    update_interval: Duration,
    current_page: Page,
    rule_picker: RulePicker,
    shop_current_player: u8,
    shop_first_player: u8,
    shop_bought_rules: Vec<bool>,
}

impl Default for GameUI {
    fn default() -> Self {
        let mut game = Game::new();
        game.new_round();

        // Add more effective rules for both players to demonstrate grid evolution
        // Player 1: Spread towards bottom
        let rule1 = Rule {
            top: CellState::Player1,
            bottom: CellState::Neutral,
            left: CellState::Neutral,
            right: CellState::Neutral,
        };
        game.player1.rules.push(rule1);

        let rule1 = Rule {
            top: CellState::Player1,
            bottom: CellState::Neutral,
            left: CellState::Player1,
            right: CellState::Neutral,
        };
        game.player1.rules.push(rule1);

        let rule1 = Rule {
            top: CellState::Player1,
            bottom: CellState::Neutral,
            left: CellState::Neutral,
            right: CellState::Player1,
        };
        game.player1.rules.push(rule1);

        let rule1 = Rule {
            top: CellState::Player1,
            bottom: CellState::Neutral,
            left: CellState::Player1,
            right: CellState::Player1,
        };
        game.player1.rules.push(rule1);

        // Player 2: Different spreading pattern towards right
        let rule2 = Rule {
            top: CellState::Neutral,
            bottom: CellState::Neutral,
            left: CellState::Neutral,
            right: CellState::Player2,
        };
        game.player2.rules.push(rule2);

        let rule2 = Rule {
            top: CellState::Player2,
            bottom: CellState::Neutral,
            left: CellState::Neutral,
            right: CellState::Player2,
        };
        game.player2.rules.push(rule2);

        let rule2 = Rule {
            top: CellState::Neutral,
            bottom: CellState::Player2,
            left: CellState::Neutral,
            right: CellState::Player2,
        };
        game.player2.rules.push(rule2);

        let rule2 = Rule {
            top: CellState::Player2,
            bottom: CellState::Player2,
            left: CellState::Neutral,
            right: CellState::Player2,
        };
        game.player2.rules.push(rule2);

        for i in 0..10 {
            let idx = i * game.grid.width + 10 + i;
            if idx < game.grid.values.len() {
                game.grid.values[idx] = CellState::Player1;
            }
            let idx2 = i * game.grid.width + 35 - i;
            if idx2 < game.grid.values.len() {
                game.grid.values[idx2] = CellState::Player2;
            }
        }

        Self {
            game,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            current_page: Page::InitialRulePicker,
            rule_picker: RulePicker::new(),
            shop_current_player: 1,
            shop_first_player: 1,
            shop_bought_rules: vec![false; SHOP_NB_RULES],
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.current_page {
            Page::LandingScreen => todo!(),
            Page::InitialRulePicker => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.rule_picker
                        .show(ui, &mut self.update_interval, &mut self.current_page);
                });
            }
            Page::MainGame => {
                if self.last_update.elapsed() >= self.update_interval {
                    self.update_game();
                    self.last_update = Instant::now();
                }

                ctx.request_repaint_after(Duration::from_millis(100));

                let mut shop_clicked = false;
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.game
                        .show(ui, &mut self.update_interval, &mut shop_clicked);
                });

                if shop_clicked {
                    self.game.shop = Shop::new_with_players(&self.game.player1, &self.game.player2);
                    self.shop_bought_rules = vec![false; SHOP_NB_RULES];
                    self.shop_current_player = self.game.shop_first_player;
                    self.current_page = Page::Shop;
                }
            }
            Page::Shop => {
                ctx.request_repaint_after(Duration::from_millis(100));
                egui::CentralPanel::default().show(ctx, |ui| {
                    let player = if self.shop_current_player == 1 {
                        &mut self.game.player1
                    } else {
                        &mut self.game.player2
                    };
                    self.game.shop.show(
                        ui,
                        player,
                        self.shop_current_player,
                        &mut self.shop_bought_rules,
                    );

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    if ui.button("Finish").clicked() {
                        self.shop_current_player =
                            if self.shop_current_player == 1 { 2 } else { 1 };

                        if self.shop_current_player == self.shop_first_player {
                            self.game.new_round();
                            self.current_page = Page::MainGame;
                        }
                    }
                });
            }
            Page::EndScreen => todo!(),
        }
    }
}

impl GameUI {
    fn update_game(&mut self) {
        // Update the grid with the rules of each player
        self.game
            .grid
            .next(&self.game.player1.rules, &self.game.player2.rules);

        // Advance the iteration counter
        self.game.advance_iteration();
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
