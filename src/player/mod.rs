use crate::constants::{PLAYER_MAX_RULES, PLAYER_SPAWN_PROBA, PLAYER_START_MONEY};
use crate::rule::Rule;
use eframe::egui;

pub struct Player {
    pub rules: Vec<Rule>,
    pub max_rules: usize,
    pub spawn_proba: f32, // spawn probability
    pub money: usize,
    pub win: usize,
}

impl Player {
    pub fn new() -> Player {
        Player {
            rules: Vec::new(),
            max_rules: PLAYER_MAX_RULES,
            spawn_proba: PLAYER_SPAWN_PROBA,
            money: PLAYER_START_MONEY,
            win: 0,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, cell_count: usize) {
        ui.label(format!("Cells: {}", cell_count));
        ui.label(format!("Money: {}", self.money));
        ui.label(format!("Rules: {}/{}", self.rules.len(), self.max_rules));
        ui.label(format!("Score (wins): {}", self.win));

        ui.add_space(10.0);
        ui.label("Rules:");
        for (i, rule) in self.rules.iter().enumerate() {
            ui.label(format!(
                "{}. {} | {} | {} | {}",
                i + 1,
                rule.top,
                rule.right,
                rule.bottom,
                rule.left
            ));
        }
    }
}
