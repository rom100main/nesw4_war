use crate::constants::{PLAYER_MAX_RULES, PLAYER_START_MONEY};
use crate::rule::Rule;
use crate::types::Pos;
use eframe::egui;

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

    pub fn show(&self, ui: &mut egui::Ui, player_num: i32, cell_count: usize) {
        ui.label(format!(
            "Player {}: {} cells | Money: {} | Rules: {}/{}",
            player_num,
            cell_count,
            self.money,
            self.rules.len(),
            self.max_rules
        ));
    }
}
