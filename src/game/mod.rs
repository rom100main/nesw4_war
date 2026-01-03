use crate::constants::{GRID_SIZE, PLAYER_MONEY, PLAYER_SPAWN_PROBA};
use crate::grid::Grid;
use crate::player::Player;
use crate::shop::Shop;

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub grid: Grid,
    pub size_grid: usize,
    pub shop: Shop,
}

impl Game {
    pub fn new() -> Game {
        let size_grid = GRID_SIZE;
        Game {
            player1: Player::new(),
            player2: Player::new(),
            grid: Grid::new(size_grid, PLAYER_SPAWN_PROBA, PLAYER_SPAWN_PROBA),
            size_grid,
            shop: Shop::new(),
        }
    }

    pub fn new_round(&mut self) {
        self.new_grid();
        self.new_shop();
        self.player1.money = PLAYER_MONEY;
        self.player2.money = PLAYER_MONEY;
    }

    fn new_grid(&mut self) {
        self.grid = Grid::new(
            self.size_grid,
            self.player1.spawn_proba,
            self.player2.spawn_proba,
        );
    }

    fn new_shop(&mut self) {
        self.shop = Shop::new();
    }
}
