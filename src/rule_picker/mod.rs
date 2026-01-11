use crate::Page;
pub use crate::rule::Rule;

use eframe::egui;

pub struct RulePicker {
    pub player1_choosing: bool,
    pub rules: Vec<Rule>,
    pub rules_available: Vec<bool>, // vrai si on peut
}

impl RulePicker {
    pub fn new() -> RulePicker {
        RulePicker {
            player1_choosing: true,
            rules: Vec::new(),
            rules_available: Vec::new(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        //rule_picked: &mut usize, // numéro de la règle achetée
        current_page: &mut Page,
    ) {
        ui.heading("ToomWar Grid Game");
        if ui.button("click me").clicked() {
            *current_page = Page::MainGame
        }
    }
}
