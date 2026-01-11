pub use crate::rule::Rule;
use crate::{COLOR_PLAYER1, COLOR_PLAYER2, Player, RULE_PICKER_NB_RULES};

use eframe::egui::{self, vec2};

pub struct RulePicker {
    pub player1_choosing: bool,
    pub rules: Vec<Rule>,
    pub rules_available: Vec<bool>, // vrai si on peut sélectionner la règle
}

impl RulePicker {
    pub fn new_with_players(player1: &Player, player2: &Player) -> RulePicker {
        let mut rules = Vec::new();
        while rules.len() < RULE_PICKER_NB_RULES {
            let new_rule = Rule::new();
            if !rules.contains(&new_rule)
                && !player1.rules.contains(&new_rule)
                && !player2.rules.contains(&new_rule)
            {
                rules.push(new_rule);
            }
        }
        RulePicker {
            player1_choosing: rand::random(),
            rules,
            rules_available: vec![true; RULE_PICKER_NB_RULES],
        }
    }

    // ajoute la règle à un joueur et la rend non disponible
    pub fn add_rule(&mut self, player: &mut Player, index: usize) {
        self.rules_available[index] = false;
        player.rules.push(self.rules[index].clone());
    }

    pub fn show(&mut self, ui: &mut egui::Ui, player: &mut Player) {
        ui.heading(egui::RichText::new("New Extreme Strategical Warfare").size(24.0));
        ui.add_space(20.0);

        let (player_num, player_color) = if self.player1_choosing {
            (1, COLOR_PLAYER1)
        } else {
            (2, COLOR_PLAYER2)
        };
        ui.label(
            egui::RichText::new(format!("Time for player {} to select", player_num))
                .color(player_color)
                .size(18.0)
                .strong(),
        );
        ui.add_space(10.0);

        const NB_RULES_PER_LINE: usize = 5;
        egui::Grid::new("foobaz")
            .spacing(vec2(20., 20.))
            .show(ui, |ui| {
                for (i, rule) in self.rules.clone().iter().enumerate() {
                    ui.vertical_centered(|ui| {
                        rule.show(ui, i);

                        if self.rules_available[i] {
                            if ui.button("Select").clicked() {
                                self.add_rule(player, i);
                                self.player1_choosing = !self.player1_choosing;
                            };
                        } else {
                            ui.label("Chosen");
                        }
                    });
                    if (i + 1) % NB_RULES_PER_LINE == 0 {
                        ui.end_row()
                    }
                }
            });
    }
}
