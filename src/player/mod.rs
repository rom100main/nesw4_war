use crate::constants::{PLAYER_MAX_RULES, PLAYER_SPAWN_PROBA, PLAYER_START_MONEY};
use crate::rule::Rule;
use eframe::egui;

pub struct Player {
    pub name: String,
    pub rules: Vec<Rule>,
    pub max_rules: usize,
    pub spawn_proba: f32, // spawn probability
    pub money: usize,
    pub win: usize,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            rules: Vec::new(),
            max_rules: PLAYER_MAX_RULES,
            spawn_proba: PLAYER_SPAWN_PROBA,
            money: PLAYER_START_MONEY,
            win: 0,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, cell_count: usize) {
        ui.label(egui::RichText::new(format!("Cells: {}", cell_count)).monospace());
        ui.label(egui::RichText::new(format!("Money: {}", self.money)).monospace());
        ui.label(
            egui::RichText::new(format!("Rules: {}/{}", self.rules.len(), self.max_rules))
                .monospace(),
        );
        ui.label(egui::RichText::new(format!("Score: {}", self.win)).monospace());

        ui.add_space(10.0);
        ui.heading(egui::RichText::new("Rules").size(16.0));
        ui.add_space(5.0);
        egui::Grid::new(format!("rules_grid {}", self.name))
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                for (i, rule) in self.rules.iter().enumerate() {
                    ui.vertical(|ui| {
                        rule.show(ui, i + 1);
                    });
                    if (i + 1) % 2 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
}
