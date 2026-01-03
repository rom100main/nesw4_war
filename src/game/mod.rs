use crate::constants::{GRID_SIZE, PLAYER_MONEY};
use crate::grid::Grid;
use crate::player::Player;
use crate::shop::Shop;
use crate::types::CellState;

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
            player2: Player::new_p2(size_grid),
            grid: Grid::new(size_grid),
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
        self.grid = Grid::new(self.size_grid);
        for spawn in &self.player1.spawn {
            let idx = spawn.y * self.size_grid + spawn.x;
            self.grid.values[idx] = CellState::Player1;
        }
        for spawn in &self.player2.spawn {
            let idx = spawn.y * self.size_grid + spawn.x;
            self.grid.values[idx] = CellState::Player2;
        }
    }

    fn new_shop(&mut self) {
        self.shop = Shop::new();
    }

    pub fn next_p1(&mut self) {
        self.grid.next(CellState::Player1, &self.player1.rules);
    }

    pub fn next_p2(&mut self) {
        self.grid.next(CellState::Player2, &self.player2.rules);
    }
}
