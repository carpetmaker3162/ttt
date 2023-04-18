use std::io;

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O
}

pub struct Board {
    pub squares: [[Option<Player>; 3]; 3],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [[None; 3]; 3],
            turn: Player::X
        }
    }

    pub fn print(&self) {
        for row in self.squares {
            for cell in row {
                if let Some(player) = cell {
                    if player == Player::X {
                        print!("X ");
                    } else if player == Player::O {
                        print!("O ");
                    }
                } else {
                    print!("_ ");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), &str> {
        if row >= 3 || col >= 3 {
            return Err("out of bounds");
        }
        if self.squares[row][col].is_some() {
            return Err("the");
        }
        self.squares[row][col] = Some(self.turn);
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        Ok(())
    }

    // whether or not an array has 2 squares filled by the same player, with the 3rd unoccupied
    pub fn arr_has_2filled(&self, row: [Option<Player>; 3], player: Player) -> Option<usize> {
        let mut filled_squares = vec![];
        for &x in row.iter() {
            if x.is_some() {
                filled_squares.push(x);
            }
        }
        if filled_squares.len() != 2 {
            return None;
        }
        for (i, &x) in row.iter().enumerate() {
            if x.is_some() && x != Some(player) {
                return Some(i);
            } else if x.is_none() {
                return Some(i);
            }
        }
        None
    }

    pub fn get_move() -> (usize, usize) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("fail lmao");

        let coordinates: Vec<i32> = input
            .split(" ")
            .map(|x| x.trim().parse().expect("not int"))
            .collect();

        let r = coordinates[0] as usize;
        let c = coordinates[1] as usize;

        (r, c)
    }

    pub fn get_board_with_placement(&self, player: Player, r: usize, c: usize) -> [[Option<Player>; 3]; 3] {
        let mut grid = self.squares.clone();
        grid[r][c] = Some(player);
        grid
    }

    pub fn has_won(&self, player: Player) -> bool {
        for i in 0..3 {
            if self.squares[0][i] == Some(player)
                && self.squares[1][i] == Some(player)
                && self.squares[2][i] == Some(player) {
                return true;
            }
            if self.squares[i][0] == Some(player)
                && self.squares[i][1] == Some(player)
                && self.squares[i][2] == Some(player) {
                return true;
            }
        }
        if self.squares[0][0] == Some(player)
            && self.squares[1][1] == Some(player)
            && self.squares[2][2] == Some(player)
        {
            return true;
        }
        if self.squares[0][2] == Some(player)
            && self.squares[1][1] == Some(player)
            && self.squares[2][0] == Some(player)
        {
            return true;
        }
        false
    }
}
