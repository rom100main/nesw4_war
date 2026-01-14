use crate::constants::{
    COLOR_PLAYER1, COLOR_PLAYER2, PLAYER_MAX_RULES, SHOP_ADD_SPAWN_PROBA, SHOP_NB_RULES,
    SHOP_PRICE_DELETE_RULE, SHOP_PRICE_RULE, SHOP_PRICE_SPAWN,
};
use crate::player::Player;
use crate::rule::Rule;
use crate::{CELL_SIZE, components};
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

    pub fn delete_rule(&mut self, player: &mut Player, index: usize) -> Result<(), ()> {
        if player.money < SHOP_PRICE_DELETE_RULE {
            return Err(());
        }
        if index >= player.rules.len() {
            return Err(());
        }
        player.rules.remove(index);
        player.money += SHOP_PRICE_DELETE_RULE;
        Ok(())
    }

    pub fn show(&mut self, ui: &mut egui::Ui, player: &mut Player, opponent: &mut Player) -> bool {
        let mut finish_clicked = false;
        components::text::title(ui);

        let player_color = if self.current_player == 1 {
            COLOR_PLAYER1
        } else {
            COLOR_PLAYER2
        };
        ui.heading(
            egui::RichText::new(format!("Player {} Shopping", self.current_player))
                .color(player_color)
                .size(18.0)
                .strong(),
        );
        ui.add_space(5.0);
        ui.label(format!("Money: {}", player.money));

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            components::text::heading(ui, "Rules");

            ui.label(format!("Cost: ${}", SHOP_PRICE_RULE));
            ui.add_space(5.0);

            let can_buy = player.money >= SHOP_PRICE_RULE && player.rules.len() < PLAYER_MAX_RULES;
            if !can_buy {
                ui.label("Can't afford or full");
                ui.add_space(5.0);
            }

            egui::ScrollArea::horizontal()
                .id_salt("shop_rules")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for i in 0..SHOP_NB_RULES {
                            ui.vertical(|ui| {
                                ui.set_max_width(CELL_SIZE * 3.0 + 10.0);
                                self.rules[i].show(ui);
                                ui.add_space(10.0);

                                if self.bought_rules[i] {
                                    ui.label(
                                        egui::RichText::new("bought")
                                            .color(egui::Color32::DARK_GREEN),
                                    );
                                } else {
                                    if can_buy {
                                        if ui.button("Buy").clicked() {
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
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            components::text::heading(ui, "Delete Rules");

            ui.label(format!("Cost: ${}", SHOP_PRICE_DELETE_RULE));
            ui.add_space(5.0);

            let can_delete = player.money >= SHOP_PRICE_DELETE_RULE;

            if !can_delete {
                ui.label("Can't afford");
                ui.add_space(5.0);
            }

            components::text::heading_small(ui, "Your Rules");

            egui::ScrollArea::horizontal()
                .id_salt("player_rules")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for i in 0..player.rules.len() {
                            ui.vertical(|ui| {
                                ui.set_max_width(CELL_SIZE * 3.0 + 10.0);
                                player.rules[i].show(ui);
                                ui.add_space(10.0);

                                if can_delete {
                                    if ui.button("Delete").clicked() {
                                        let _ = self.delete_rule(player, i);
                                    }
                                }
                            });
                            ui.add_space(5.0);
                        }
                    });
                });

            ui.add_space(10.0);

            components::text::heading_small(ui, "Opponent's Rules");

            egui::ScrollArea::horizontal()
                .id_salt("opponent_rules")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for i in 0..opponent.rules.len() {
                            ui.vertical(|ui| {
                                ui.set_max_width(CELL_SIZE * 3.0 + 10.0);
                                opponent.rules[i].show(ui);
                                ui.add_space(10.0);

                                if can_delete {
                                    if ui.button("Delete").clicked() {
                                        let _ = self.delete_rule(opponent, i);
                                    }
                                }
                            });
                            ui.add_space(5.0);
                        }
                    });
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            components::text::heading(ui, "Spawn Probability");

            ui.label(format!("Cost: ${}", SHOP_PRICE_SPAWN));
            ui.add_space(5.0);

            let can_buy_spawn = player.money >= SHOP_PRICE_SPAWN;
            if !can_buy_spawn {
                ui.label("Can't afford");
                ui.add_space(5.0);
            }

            ui.label(format!("Current: {:.4}", player.spawn_proba));

            if can_buy_spawn {
                if ui.button("Upgrade Spawn").clicked() {
                    let _ = self.buy_spawn(player);
                }
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            if ui
                .button(egui::RichText::new("Finish").size(18.0))
                .clicked()
            {
                finish_clicked = true;
            }
        });

        finish_clicked
    }
}
