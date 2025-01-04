use eframe::egui;
mod game; // Import the game module
use game::{Game, Player};

fn main() -> Result<(), eframe::Error> {
    // Run the application with default native options
    eframe::run_native(
        "Tic-Tac-Toe", // Title of the window
        eframe::NativeOptions {
            ..Default::default() // Let eframe decide the window size
        },
        Box::new(|_cc| {
            // Create and return an instance of the game app
            Ok(Box::new(GameApp::default()))
        }),
    )
}

#[derive(Default)]
struct GameApp {
    game: Game,      // Holds the game state (board, current_turn)
    game_over: bool, // Track if the game is over (win/draw)
    draw: bool,      // Track if the game ended in a draw
}

impl GameApp {
    // Initialize the app (we can modify egui settings if needed in the future)
    fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Begin rendering UI in the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Vertical centering for labels and reset button
            ui.vertical_centered(|ui| {
                // Add horizontal space before and after the game grid (will be equally distributed)
                ui.horizontal(|ui| {
                    // Dynamically calculate available space to ensure equal padding
                    let available_space = ui.available_width(); // Get the available width
                    let padding = (available_space - 360.0) / 2.0; // Calculate padding for the left and right sides

                    ui.add_space(padding); // Add space on the left side
                    ui.centered_and_justified(|ui| {
                        self.render_board(ui);
                    });
                    ui.add_space(padding); // Add space on the right side
                });

                ui.add_space(10.0); // Adjust the space between grid and status/controls

                // Check for a winner
                if let Some(winner) = self.game.check_winner() {
                    self.game_over = true;
                    ui.label(egui::RichText::new(format!("{:?} wins!", winner)).size(50.0)); // Make the winner text larger

                    if ui
                        .button(egui::RichText::new("Reset Game").size(50.0))
                        .clicked()
                    {
                        self.reset_game();
                    }
                } else if self.game.is_full() {
                    self.game_over = true;
                    self.draw = true;
                    ui.label(egui::RichText::new("It's a draw!").size(50.0)); // Make the draw text larger
                    if ui
                        .button(egui::RichText::new("Reset Game").size(50.0))
                        .clicked()
                    {
                        self.reset_game();
                    }
                } else {
                    ui.label(
                        egui::RichText::new(format!("{:?}'s turn", self.game.current_turn))
                            .size(50.0),
                    ); // Make the turn text larger
                }
            });
        });
    }
}

impl GameApp {
    fn render_board(&mut self, ui: &mut egui::Ui) {
        let mut grid: [[Option<Player>; 3]; 3] = self.game.board;

        // Center the board with equal padding around it
        ui.vertical(|ui| {
            for row in 0..3 {
                ui.horizontal(|ui| {
                    for col in 0..3 {
                        let cell = &mut grid[row][col];

                        // Set the button size to make the game board larger and more visible
                        let button_size = egui::vec2(120.0, 120.0); // Larger button size

                        // Add buttons side by side with minimal gap
                        let button = ui.add_enabled(
                            !self.game_over, // Disable if the game is over
                            egui::Button::new(match cell {
                                Some(Player::X) => egui::RichText::new("X").size(80.0), // Make "X" bigger
                                Some(Player::O) => egui::RichText::new("O").size(80.0), // Make "O" bigger
                                None => egui::RichText::new(" ").size(80.0), // Ensure None returns RichText
                            })
                            .min_size(button_size), // Set the button size
                        );

                        // If the cell is clicked, make a move
                        if button.clicked() && cell.is_none() {
                            let _ = self.game.make_move(row, col); // Make the move
                        }
                    }
                });
            }
        });

        // AI move if it's O's turn
        if self.game.current_turn == Player::O && !self.game_over {
            if let Err(e) = self.game.ai_move() {
                println!("AI Error: {}", e);
            }
        }
    }

    fn reset_game(&mut self) {
        self.game = Game::new();
        self.game_over = false;
        self.draw = false;
    }
}
