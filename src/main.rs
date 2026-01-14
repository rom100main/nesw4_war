pub mod components;
pub mod constants;
pub mod game;
pub mod grid;
pub mod player;
pub mod rule;
pub mod rule_picker;
pub mod shop;

pub use constants::*;
pub use game::Game;
pub use grid::Grid;
pub use grid::cell::CellState;
pub use player::Player;
pub use rule::Rule;
pub use rule_picker::RulePicker;
pub use shop::Shop;

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
    current_page: Page,
    rule_picker: RulePicker,
}

impl Default for GameUI {
    fn default() -> Self {
        let mut game = Game::new();
        game.new_round();

        game.player1
            .rules
            .push(Rule::new_direction(CellState::Player1));

        game.player2
            .rules
            .push(Rule::new_direction(CellState::Player2));

        let rule_picker = RulePicker::new_with_players(&game.player1, &game.player2);

        Self {
            game,
            last_update: Instant::now(),
            current_page: Page::InitialRulePicker,
            rule_picker,
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.current_page {
            Page::LandingScreen => todo!(),

            Page::InitialRulePicker => {
                let player = if self.rule_picker.player1_choosing {
                    &mut self.game.player1
                } else {
                    &mut self.game.player2
                };

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.rule_picker.show(ui, player);
                });

                if self.game.player1.rules.len() == PLAYER_START_RULES
                    && self.game.player2.rules.len() == PLAYER_START_RULES
                {
                    self.current_page = Page::MainGame;
                }
            }

            Page::MainGame => {
                if self.last_update.elapsed()
                    >= Duration::from_millis(constants::UPDATE_INTERVAL_MS)
                    && !self.game.round_over
                {
                    self.update_game();
                    self.last_update = Instant::now();
                }

                ctx.request_repaint_after(Duration::from_millis(100));

                let mut shop_clicked = false;
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.game.show(ui, &mut shop_clicked);
                });

                if shop_clicked {
                    self.game.shop = Shop::new_with_players(&self.game.player1, &self.game.player2);
                    self.game.shop.current_player = self.game.shop_first_player;
                    self.current_page = Page::Shop;
                }
            }
            Page::Shop => {
                ctx.request_repaint_after(Duration::from_millis(100));
                egui::CentralPanel::default().show(ctx, |ui| {
                    let (player, opponent) = if self.game.shop.current_player == 1 {
                        self.game.shop.player1_shopped = true;
                        (&mut self.game.player1, &mut self.game.player2)
                    } else {
                        self.game.shop.player2_shopped = true;
                        (&mut self.game.player2, &mut self.game.player1)
                    };
                    self.game.shop.show(ui, player, opponent);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    if ui
                        .button(egui::RichText::new("Finish").size(18.0))
                        .clicked()
                    {
                        self.game.shop.current_player = if self.game.shop.current_player == 1 {
                            2
                        } else {
                            1
                        };

                        if self.game.shop.player1_shopped && self.game.shop.player2_shopped {
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

        // Check if grid is identical to previous state
        let current_grid_state = self.game.grid.values.clone();
        if let Some(ref previous_state) = self.game.previous_grid_state {
            if *previous_state == current_grid_state {
                self.game.end_round();
                return;
            }
        }
        self.game.previous_grid_state = Some(current_grid_state);

        // Advance the iteration counter
        self.game.advance_iteration();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("NESW4: New Extreme Strategical Warfare 4"),
        ..Default::default()
    };

    Ok(eframe::run_native(
        "NESW4: New Extreme Strategical Warfare 4",
        options,
        Box::new(|_cc| Ok(Box::<GameUI>::default())),
    )?)
}
