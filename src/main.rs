mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.print_board();

    // Example moves
    game.make_move(0, 0).unwrap();
    game.print_board();

    game.make_move(1, 1).unwrap();
    game.print_board();

    game.make_move(0, 1).unwrap();
    game.print_board();

    if let Some(winner) = game.check_winner() {
        println!("{:?} wins!", winner);
    } else {
        println!("No winner yet.");
    }
}
