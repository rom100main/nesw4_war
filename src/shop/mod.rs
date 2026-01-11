use crate::constants::{
    PLAYER_MAX_RULES, SHOP_ADD_SPAWN_PROBA, SHOP_NB_RULES, SHOP_PRICE_RULE, SHOP_PRICE_SPAWN,
};
use crate::player::Player;
use crate::rule::Rule;
use eframe::egui;

pub struct Shop {
    pub rules: Vec<Rule>,
}

impl Shop {
    pub fn new() -> Shop {
        let mut rules = Vec::new();
        for _ in 0..SHOP_NB_RULES {
            rules.push(Rule::new());
        }
        Shop { rules }
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

    pub fn show(&self, _ui: &mut egui::Ui) {}
}
