pub const PLAYER_MAX_RULES: usize = 5;
pub const PLAYER_START_MONEY: usize = 5;
pub const PLAYER_MONEY: usize = 3;

pub const SHOP_NB_RULES: usize = 9;
pub const SHOP_PRICE_RULE: usize = 1;

pub const GRID_SIZE: usize = 50;

#[derive(PartialEq, Clone, Copy)]
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
    width: usize,
    height: usize,
    values: Vec<CellState>,
    toric: bool,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            width: size,
            height: size,
            values: vec![CellState::Neutral; size * size],
            toric: true,
        }
    }

    pub fn count(&self, value: CellState) -> usize {
        self.values.iter().filter(|v| **v == value).count()
    }

    pub fn next(&mut self, rules: Vec<Rule>) {
        let mut new_values = self.values.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let current_idx = y * self.width + x;
                let current_state = self.values[current_idx];

                // Get the cell states for checking rules
                // top: cell above
                // inner: current cell
                // right: cell to the right
                let (top_idx, right_idx) = if self.toric {
                    // Toroidal - wrap around
                    let top_y = if y == 0 { self.height - 1 } else { y - 1 };
                    let right_x = if x == self.width - 1 { 0 } else { x + 1 };
                    (top_y * self.width + x, y * self.width + right_x)
                } else {
                    // Non-toroidal - out of bounds
                    if y == 0 || x == self.width - 1 {
                        continue;
                    }
                    ((y - 1) * self.width + x, y * self.width + (x + 1))
                };

                let top_state = self.values[top_idx];
                let right_state = self.values[right_idx];

                // Check if any rule matches
                for rule in &rules {
                    if rule.next(top_state, current_state, right_state) {
                        // Change current cell to Player1 if it's currently neutral or Player2
                        if current_state == CellState::Neutral {
                            new_values[current_idx] = CellState::Player1;
                        }
                        break;
                    }
                }
            }
        }

        self.values = new_values;
    }
}

#[derive(PartialEq)]
pub struct Rule {
    top: CellState,
    inner: CellState,
    right: CellState,
}

impl Rule {
    pub fn new() -> Rule {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Rule {
            top: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
            inner: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
            right: match rng.gen_range(0..3) {
                0 => CellState::Neutral,
                1 => CellState::Player1,
                _ => CellState::Player2,
            },
        }
    }

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
    pub fn new() -> Shop {
        let mut rules = Vec::new();
        for _ in 0..SHOP_NB_RULES {
            rules.push(Rule::new());
        }
        Shop { rules }
    }

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
