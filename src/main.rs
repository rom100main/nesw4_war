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
use std::time::{Duration, Instant};

struct GameUI {
    game: Game,
    last_update: Instant,
    update_interval: Duration,
}

impl Default for GameUI {
    fn default() -> Self {
        let mut game = Game::new();
        game.new_round();

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
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.last_update.elapsed() >= self.update_interval {
            self.update_game();
            self.last_update = Instant::now();
        }

        ctx.request_repaint_after(Duration::from_millis(100));

        let mut new_round_clicked = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            self.game
                .show(ui, &mut new_round_clicked, &mut self.update_interval);
        });

        if new_round_clicked {
            self.game.new_round();
        }
    }
}

impl GameUI {
    fn update_game(&mut self) {
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
