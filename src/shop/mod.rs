use crate::constants::{PLAYER_MAX_RULES, SHOP_NB_RULES, SHOP_PRICE_RULE};
use crate::player::Player;
use crate::rule::Rule;

pub struct Shop {
    rules: Vec<Rule>,
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
        player.rules.push(self.rules.remove(index));
        player.money -= SHOP_PRICE_RULE;
        Ok(())
    }
}
