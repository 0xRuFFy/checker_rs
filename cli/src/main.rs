use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
struct Cli {
    /// The depth of the minimax algorithm
    #[arg(short, long, default_value_t = 7)]
    depth: u8,
    /// The evaluation function to use [version 1, 2...]
    #[arg(short, long, default_value_t = 2)]
    eval: u8,
    /// The board to evaluate in FEN notation
    /// 'm': black Man
    /// 'M': white Man
    /// 'k': black King
    /// 'K': white King
    /// '/': new row
    /// '1-9': n empty squares
    /// Example Starting Board: 1m1m1m1m/m1m1m1m1/1m1m1m1m/8/8/M1M1M1M1/1M1M1M1M/M1M1M1M1
    #[arg(verbatim_doc_comment, short, long)]
    fen: String,
}

fn main() {
    let args = Cli::parse();

    let board = logic::Board::from_fen(&args.fen).unwrap();
    let mut analyser = logic::MinimaxPlayer::new(
        args.depth,
        match args.eval {
            1 => logic::eval::v1,
            2 => logic::eval::v2,
            _ => panic!("Invalid evaluation function"),
        },
    );

    // TODO: Make current player [W/B] a parameter ... in FEN?

    let w_1 = 40 - args.depth.to_string().len();
    let w_2 = 33 - args.eval.to_string().len();

    println!(
        "{}",
        "+-----------------------------------------------+".dimmed()
    );
    println!(
        "{} {} {}",
        "|--------------".dimmed(),
        "Checkers Analysis".green().bold(),
        "--------------|".dimmed()
    );
    println!(
        "{}",
        "|-----------------------------------------------|".dimmed()
    );
    println!(
        "{}Depth: {}{:>w_1$}",
        "| ".dimmed(),
        args.depth.to_string().bold().cyan(),
        "|".dimmed()
    );
    println!(
        "{}Eval Version: {}{:>w_2$}",
        "| ".dimmed(),
        args.eval.to_string().bold().cyan(),
        "|".dimmed()
    );
    println!(
        "{}",
        "+-----------------------------------------------+".dimmed()
    );
    println!("\n{}\n", "Board:".bold().cyan());
    println!("{}\n", board);
    analyser.analyse(&board);
    println!(
        "{}",
        "+-----------------------------------------------+".dimmed()
    );
}
