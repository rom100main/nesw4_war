use crate::constants::{PLAYER_MAX_RULES, PLAYER_SPAWN_PROBA, PLAYER_START_MONEY};
use crate::rule::Rule;

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
}
