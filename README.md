
# checker_rs

Checkers game/bot writen in rust.

## Modules

This project is currently split into 2 modules:

- `logic`: Library containing the game, board and player logic.
- `cli`: Command line interface for analyzing a board state.

### Logic

The logic module contains the game logic, board logic and player logic for a checkers game.

#### Board

The board struct tracks all the pieces (man and king) on the board using 3 64-bit integers. It is responsible for moving/taking/crowning pieces and getting the possible moves for one or all pieces.

#### Game

The game struct is responsible for managing the board and the players. It determines the current player and if the game is over (the game loop is defined here).

#### Player

The player trait is implementes the necessary methods for a player to play a game.

Predifined players are:

- `HumanPlayer`: A player that asks for input from the command line.
- `MinimaxPlayer`: A player that uses the minimax algorithm with alpha-beta pruning to determine the best move.

#### Logic Usage

The libary can be used to play a game by creating a game with 2 players and calling the `play` method.

```rust
use logic;

fn main() {
    // Create a game with a human player and a minimax player.
    let mut game = logic::Game::new(
        // A human player.
        Box::new(logic::HumanPlayer::new()),

        // A minimax player with a depth of 8 and the v2 heuristic function.
        Box::new(logic::MinimaxPlayer::new(8, logic::v2)),
    );

    // Play the game.
    game.play();
}
```

But you can also use the MinimaxPlayer to analyze a board state.

```rust
use logic;

fn main() {
    // A board representation by a modified FEN string.
    let fen = "1m1m1m1m/m1m1m1m1/1m1m1m1m/8/8/M1M1M1M1/1M1M1M1M/M1M1M1M1";

    // Create a board from the FEN string.
    let board = logic::Board::from_fen(&args.fen).unwrap();

    // Create a minimax player with a depth of 10 and the v2 heuristic function.
    let mut analyser = logic::MinimaxPlayer::new(10, logic::v2);

    // Analyze the board.
    analyser.analyse(&board);
}
```

### CLI

The cli module contains a command line interface for analyzing a board state.

#### CLI Usage

The cli can be used to analyze a board state by passing a modified FEN string as an argument.

```bash
cargo run --bin cli -- [OPTIONS] --fen <FEN>

Options:
  -d, --depth <DEPTH>  The depth of the minimax algorithm [default: 7]
  -e, --eval <EVAL>    The evaluation function to use [version 1, 2...] [default: 2]
  -f, --fen <FEN>      The board to evaluate in FEN notation
                       'm': black Man
                       'M': white Man
                       'k': black King
                       'K': white King
                       '/': new row
                       '1-9': n empty squares
                       Example Starting Board: 1m1m1m1m/m1m1m1m1/1m1m1m1m/8/8/M1M1M1M1/1M1M1M1M/M1M1M1M1
  -h, --help           Print help
```
