#![allow(dead_code, unused_imports, unused_variables)]
mod game;
mod player;

use checkers_core::board::{BLACK, WHITE};
use game::CheckersGame;
use player::bot_player;

fn main() {
    println!("Hello, world!");

    let mut game = CheckersGame::new(
        Box::new(player::HumanPlayer::new()),
        // Box::new(player::BotPlayer::minimax(bot_player::Depth::Dynamic, 2)),
        // Box::new(player::BotPlayer::alpha_beta(bot_player::Depth::Static(6), 2)),
        Box::new(player::BotPlayer::minimax(bot_player::Depth::Static(6), 2)),
        // Box::new(player::BotPlayer::minimax(bot_player::Depth::Static(5), 1)),
        // Box::new(player::HumanPlayer::new()),
        // Box::new(player::BotPlayer::minimax(bot_player::Depth::Dynamic, 2)),
        // Box::new(player::BotPlayer::minimax(4, 2)),
    );

    game.play();

    // let board = checkers_core::Board::new();
    // println!("{}", board);
    // println!("White 1:{}", board.eval_v1(WHITE));
    // println!("White 2:{}", board.eval_v2(WHITE));
    // println!("Black 1:{}", board.eval_v1(BLACK));
    // println!("Black 2:{}", board.eval_v2(BLACK));
}
