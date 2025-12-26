use crate::types::Pos;
use crate::constants::{PLAYER_MAX_RULES, PLAYER_START_MONEY};
use crate::rule::Rule;

pub struct Player {
    pub rules: Vec<Rule>,
    pub max_rules: usize,
    pub spawn: Vec<Pos>,
    pub money: usize,
    pub win: usize,
}

impl Player {
    pub fn new() -> Player {
        Player {
            rules: Vec::new(),
            max_rules: PLAYER_MAX_RULES,
            spawn: vec![Pos { x: 0, y: 0 }],
            money: PLAYER_START_MONEY,
            win: 0,
        }
    }

    pub fn new_p2(size_grid: usize) -> Player {
        Player {
            rules: Vec::new(),
            max_rules: PLAYER_MAX_RULES,
            spawn: vec![Pos {
                x: size_grid - 1,
                y: size_grid - 1,
            }],
            money: PLAYER_START_MONEY,
            win: 0,
        }
    }
}