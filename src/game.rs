use rand::seq::SliceRandom; // Importing random selection for AI moves

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug)]
pub struct Game {
    pub board: [[Option<Player>; 3]; 3],
    pub current_turn: Player,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: [[None; 3]; 3],   // Initialize an empty board
            current_turn: Player::X, // Player X starts
        }
    }
}

impl Game {
    // Initialize a new game with an empty board
    pub fn new() -> Self {
        Self::default()
    }
    // Check if the board is full (no empty spaces)
    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
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

    // Check for a winner
    pub fn check_winner(&self) -> Option<Player> {
        // Check rows, columns, and diagonals for a winner
        for i in 0..3 {
            // Check rows
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
                if let Some(player) = self.board[i][0] {
                    return Some(player); // Winner found
                }
            }
            // Check columns
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                if let Some(player) = self.board[0][i] {
                    return Some(player); // Winner found
                }
            }
        }

        // Check diagonals
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if let Some(player) = self.board[0][0] {
                return Some(player); // Winner found
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if let Some(player) = self.board[0][2] {
                return Some(player); // Winner found
            }
        }

        None // No winner
    }

    // AI makes a move (blocking or winning)
    pub fn ai_move(&mut self) -> Result<(), String> {
        // Try to win
        if let Some((x, y)) = self.find_winning_move(Player::O) {
            self.make_move(x, y)?;
            return Ok(());
        }

        // Try to block the opponent
        if let Some((x, y)) = self.find_winning_move(Player::X) {
            self.make_move(x, y)?;
            return Ok(());
        }

        // Random move if no winning or blocking move
        let empty_cells: Vec<(usize, usize)> = (0..3)
            .flat_map(|x| (0..3).map(move |y| (x, y)))
            .filter(|&(x, y)| self.board[x][y].is_none())
            .collect();

        if empty_cells.is_empty() {
            return Err("No available moves".to_string());
        }

        let selected_cell = empty_cells.choose(&mut rand::thread_rng()).unwrap();
        self.make_move(selected_cell.0, selected_cell.1)
    }

    // Find a winning move for the given player
    fn find_winning_move(&self, player: Player) -> Option<(usize, usize)> {
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j].is_none() {
                    let mut board_copy = self.board.clone();
                    board_copy[i][j] = Some(player);

                    if self.check_winner_with_board(&board_copy, player).is_some() {
                        return Some((i, j));
                    }
                }
            }
        }
        None
    }

    // Check if a player has won on a given board state
    fn check_winner_with_board(
        &self,
        board: &[[Option<Player>; 3]; 3],
        player: Player,
    ) -> Option<Player> {
        for i in 0..3 {
            if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
                if let Some(p) = board[i][0] {
                    if p == player {
                        return Some(player);
                    }
                }
            }
            if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                if let Some(p) = board[0][i] {
                    if p == player {
                        return Some(player);
                    }
                }
            }
        }

        if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            if let Some(p) = board[0][0] {
                if p == player {
                    return Some(player);
                }
            }
        }

        if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            if let Some(p) = board[0][2] {
                if p == player {
                    return Some(player);
                }
            }
        }

        None
    }
}
