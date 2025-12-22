pub const PLAYER_MAX_RULES: usize = 5;
pub const PLAYER_START_MONEY: usize = 5;
pub const PLAYER_MONEY: usize = 3;

pub const SHOP_NB_RULES: usize = 9;
pub const SHOP_PRICE_RULE: usize = 1;

pub const GRID_SIZE: usize = 50;

#[derive(PartialEq, Clone)]
pub enum CellState {
    Neutral,
    Player1,
    Player2,
}

pub struct Pos {
    x: usize,
    y: usize,
}

pub struct Grid {
    values: Vec<CellState>,
    toric: bool,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            values: vec![CellState::Neutral; size],
            toric: true,
        }
    }

    pub fn count(&self, value: CellState) -> usize {
        self.values.iter().filter(|v| **v == value).count()
    }

    pub fn next(&mut self, rules: Vec<Rule>) {
        // TODO
        // // Player1
        // for values.size
        //      for rules
        //          if rule.next(values)
        //          change state
        // // Player2
        // // /!\ same but becareful direction !
    }
}

#[derive(PartialEq)]
pub struct Rule {
    top: CellState,
    inner: CellState,
    right: CellState,
}

impl Rule {
    /*
    pub fn new() -> Rule {
        Rule {
            top:, // TODO: random
            inner:,
            right:,
        }
    }
    */

    pub fn next(&self, top: CellState, inner: CellState, right: CellState) -> bool {
        *self == Rule { top, inner, right }
    }
}

pub struct Player {
    rules: Vec<Rule>,
    max_rules: usize,
    spawn: Vec<Pos>,
    money: usize,
    win: usize,
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

pub struct Shop {
    rules: Vec<Rule>,
}

impl Shop {
    /*
    pub fn new() -> Shop {
        let shop: Shop;
        // TODO: add SHOP_NB_RULES new rule (which are randomly generate)
    }
    */

    pub fn buy_rule(&mut self, mut player: Player, number: usize) -> Result<(), ()> {
        if player.money < SHOP_PRICE_RULE {
            // Err(()) type error?
        }
        if player.rules.len() >= PLAYER_MAX_RULES {
            // Err(())
        }
        player.rules.push(self.rules.remove(number));
        player.money -= SHOP_PRICE_RULE;
        Ok(())
    }
}

pub struct Game {
    player1: Player,
    player2: Player,
    grid: Grid,
    size_grid: usize,
    shop: Shop,
}

impl Game {
    /*
    pub fn new() -> Game {
        let size_grid = GRID_SIZE;
        Game {
            player1: Player::new(),
            player2: Player::new_p2(size_grid),
            grid: Grid::new(size_grid),
            size_grid,
            shop: Shop::new()
        }
    }
    */

    pub fn new_round(&mut self) {
        self.new_grid();
        self.new_shop();
        self.player1.money = PLAYER_MONEY;
        self.player2.money = PLAYER_MONEY;
    }

    fn new_grid(&mut self) {
        self.grid = Grid::new(self.size_grid);
        // TODO: set player spawn
    }

    fn new_shop(&mut self) {
        // self.shop = Shop::new();
    }

    pub fn next_p1(mut self) {
        // CHECK: mut and not &mut?
        self.grid.next(self.player1.rules);
    }

    pub fn next_p2(mut self) {
        self.grid.next(self.player2.rules);
    }
}

fn main() {
    println!("Hello, world!");
}
