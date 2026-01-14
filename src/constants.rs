use eframe::egui::Color32;
use std::usize;

pub const PLAYER_MAX_RULES: usize = 16;
pub const PLAYER_START_RULES: usize = 4;
pub const PLAYER_START_MONEY: usize = 0;
pub const PLAYER_ADD_MONEY: usize = 2;
pub const PLAYER_SPAWN_PROBA: f32 = 0.001;

pub const RULE_PICKER_NB_RULES: usize = 20;

pub const SHOP_NB_RULES: usize = 8;
pub const SHOP_PRICE_RULE: usize = 1;
pub const SHOP_PRICE_SPAWN: usize = 1;
pub const SHOP_ADD_SPAWN_PROBA: f32 = 0.0001;

pub const GRID_SIZE: usize = 64;
pub const CELL_SIZE: f32 = 16.0;
pub const MAX_ITERATIONS: usize = 128;
pub const UPDATE_INTERVAL_MS: u64 = 100;

pub const COLOR_NEUTRAL: Color32 = Color32::from_gray(200);
pub const COLOR_PLAYER1: Color32 = Color32::from_rgb(100, 100, 255);
pub const COLOR_PLAYER2: Color32 = Color32::from_rgb(255, 100, 100);
pub const COLOR_GRID_LINE: Color32 = Color32::from_gray(150);
