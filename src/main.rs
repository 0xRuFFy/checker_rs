mod game;
mod player;

use game::CheckersGame;

fn main() {
    println!("Hello, world!");

    let mut game = CheckersGame::new(
        Box::new(player::HumanPlayer::new()),
        // Box::new(player::HumanPlayer::new()),
        Box::new(player::BotPlayer::minimax()),
    );

    game.play();
}
