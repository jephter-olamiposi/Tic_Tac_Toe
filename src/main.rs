mod game;

use game::{Game, Player};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();
    game.print_board();

    loop {
        // Player move
        if game.current_turn == Player::X {
            println!("Your turn (Player X)!");
            let player_move = get_user_input();
            game.make_move(player_move.0, player_move.1).unwrap();
            game.print_board();

            if let Some(winner) = game.check_winner() {
                println!("{:?} wins!", winner);
                break;
            }
        }

        // AI move (Player O)
        if game.current_turn == Player::O {
            println!("AI's turn (Player O)!");
            game.ai_move().unwrap();
            game.print_board();

            if let Some(winner) = game.check_winner() {
                println!("{:?} wins!", winner);
                break;
            }
        }
    }
}

// Function to get user input (coordinates)
fn get_user_input() -> (usize, usize) {
    loop {
        print!("Enter your move (row col): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before user input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if row < 3 && col < 3 {
                    return (row, col);
                }
            }
        }

        println!("Invalid input. Please enter two numbers between 0 and 2 (e.g., '1 1').");
    }
}
