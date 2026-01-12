use crate::constants::{
    COLOR_PLAYER1, COLOR_PLAYER2, PLAYER_MAX_RULES, SHOP_ADD_SPAWN_PROBA, SHOP_NB_RULES,
    SHOP_PRICE_RULE, SHOP_PRICE_SPAWN,
};
use crate::player::Player;
use crate::rule::Rule;
use eframe::egui;

pub struct Shop {
    pub rules: Vec<Rule>,
    pub bought_rules: Vec<bool>,
    pub current_player: u8,
    pub player1_shopped: bool,
    pub player2_shopped: bool,
}

impl Shop {
    pub fn new_with_players(player1: &Player, player2: &Player) -> Shop {
        let mut rules = Vec::new();
        while rules.len() < SHOP_NB_RULES {
            let new_rule = Rule::new();
            if !rules.contains(&new_rule)
                && !player1.rules.contains(&new_rule)
                && !player2.rules.contains(&new_rule)
            {
                rules.push(new_rule);
            }
        }
        Shop {
            rules,
            current_player: 1,
            bought_rules: vec![false; SHOP_NB_RULES],
            player1_shopped: false,
            player2_shopped: false,
        }
    }

    pub fn buy_rule(&mut self, player: &mut Player, index: usize) -> Result<(), ()> {
        if player.money < SHOP_PRICE_RULE {
            return Err(());
        }
        if player.rules.len() >= PLAYER_MAX_RULES {
            return Err(());
        }
        if index >= self.rules.len() {
            return Err(());
        }
        player.rules.push(self.rules[index].clone());
        player.money -= SHOP_PRICE_RULE;
        Ok(())
    }

    pub fn buy_spawn(&mut self, player: &mut Player) -> Result<(), ()> {
        if player.money < SHOP_PRICE_SPAWN {
            return Err(());
        }
        if player.spawn_proba >= 50.0 {
            return Err(());
        }
        player.spawn_proba += SHOP_ADD_SPAWN_PROBA;
        player.money -= SHOP_PRICE_SPAWN;
        Ok(())
    }

    pub fn show(&mut self, ui: &mut egui::Ui, player: &mut Player) {
        ui.heading(egui::RichText::new("NESW4: New Extreme Strategical Warfare 4").size(24.0));
        ui.add_space(20.0);

        let player_color = if self.current_player == 1 {
            COLOR_PLAYER1
        } else {
            COLOR_PLAYER2
        };
        let player_name = format!("Player {} Shopping", self.current_player);
        ui.heading(
            egui::RichText::new(player_name)
                .color(player_color)
                .size(18.0)
                .strong(),
        );
        ui.add_space(5.0);
        ui.label(format!("Money: {}", player.money));

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(egui::RichText::new("Rules").size(18.0));
        ui.add_space(5.0);

        let can_buy = player.money >= SHOP_PRICE_RULE && player.rules.len() < PLAYER_MAX_RULES;
        if !can_buy {
            ui.label(format!("Can't afford (${}) or full", SHOP_PRICE_RULE));
            ui.add_space(5.0);
        }

        ui.horizontal(|ui| {
            for i in 0..SHOP_NB_RULES {
                ui.vertical(|ui| {
                    self.rules[i].show(ui, i + 1);
                    ui.add_space(10.0);

                    if self.bought_rules[i] {
                        ui.label(egui::RichText::new("bought").color(egui::Color32::DARK_GREEN));
                    } else {
                        if can_buy {
                            if ui.button(format!("Buy (${})", SHOP_PRICE_RULE)).clicked() {
                                if self.buy_rule(player, i).is_ok() {
                                    self.bought_rules[i] = true;
                                }
                            }
                        }
                    }
                });
                ui.add_space(5.0);
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(egui::RichText::new("Spawn Probability").size(18.0));
        ui.add_space(5.0);

        let can_buy_spawn = player.money >= SHOP_PRICE_SPAWN;
        if !can_buy_spawn {
            ui.label(format!("Can't afford (${})", SHOP_PRICE_SPAWN));
            ui.add_space(5.0);
        }

        ui.label(format!("Current: {:.4}", player.spawn_proba));

        if can_buy_spawn {
            if ui
                .button(format!("Upgrade Spawn (${})", SHOP_PRICE_SPAWN))
                .clicked()
            {
                let _ = self.buy_spawn(player);
            }
        }
    }
}
