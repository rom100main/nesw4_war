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
pub use types::CellState;
pub use types::*;

use eframe::egui;
use rand::Rng;
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

        let mut rng = rand::thread_rng();
        let pattern: usize = rng.gen_range(0..4);
        let rule_for_player1 = match pattern {
            0 => Rule {
                top: CellState::Player1,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            1 => Rule {
                top: CellState::Neutral,
                right: CellState::Player1,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            2 => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: CellState::Player1,
                left: CellState::Neutral,
            },
            _ => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: CellState::Player1,
            },
        };
        game.player1.rules.push(rule_for_player1);

        let pattern2: usize = rng.gen_range(0..4);
        let rule_for_player2 = match pattern2 {
            0 => Rule {
                top: CellState::Player2,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            1 => Rule {
                top: CellState::Neutral,
                right: CellState::Player2,
                bottom: CellState::Neutral,
                left: CellState::Neutral,
            },
            2 => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: CellState::Player2,
                left: CellState::Neutral,
            },
            _ => Rule {
                top: CellState::Neutral,
                right: CellState::Neutral,
                bottom: CellState::Neutral,
                left: CellState::Player2,
            },
        };
        game.player2.rules.push(rule_for_player2);

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
                    let player = if self.game.shop.current_player == 1 {
                        self.game.shop.player1_shopped = true;
                        &mut self.game.player1
                    } else {
                        self.game.shop.player2_shopped = true;
                        &mut self.game.player2
                    };
                    self.game.shop.show(ui, player);

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
