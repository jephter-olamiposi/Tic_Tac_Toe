#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug)]
pub struct Game {
    board: [[Option<Player>; 3]; 3],
    current_turn: Player,
}

impl Game {
    // Initialize a new game with an empty board
    pub fn new() -> Self {
        Game {
            board: [[None; 3]; 3],
            current_turn: Player::X,
        }
    }

    // Print the board to the console
    pub fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    // Make a move on the board
    pub fn make_move(&mut self, x: usize, y: usize) -> Result<(), String> {
        if x >= 3 || y >= 3 {
            return Err("Invalid move: out of bounds".to_string());
        }

        if self.board[x][y].is_some() {
            return Err("Invalid move: cell already taken".to_string());
        }

        self.board[x][y] = Some(self.current_turn);
        self.current_turn = match self.current_turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(())
    }

    // Check if a player has won
    pub fn check_winner(&self) -> Option<Player> {
        // Check rows, columns, and diagonals for a winner
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
                if let Some(player) = self.board[i][0] {
                    return Some(player);
                }
            }

            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                if let Some(player) = self.board[0][i] {
                    return Some(player);
                }
            }
        }

        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if let Some(player) = self.board[0][0] {
                return Some(player);
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if let Some(player) = self.board[0][2] {
                return Some(player);
            }
        }

        None
    }
}
