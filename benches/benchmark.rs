use checkers_core as core;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use r_checkers::player::{self, bot_player::Depth, Player};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let board_start = core::Board::new();
    let moves = board_start.get_possible_moves(&core::board::WHITE);

    let minimax_player_static = player::BotPlayer::minimax(Depth::Static(5), 2);
    // let minimax_player = player::BotPlayer::minimax(Depth::Dynamic, 2);
    c.bench_function("Minimax depth 5 Start layout", |b| {
        b.iter(|| minimax_player_static.get_move(black_box(&board_start), black_box(&moves)))
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(13)).warm_up_time(Duration::from_secs(1));
    targets = criterion_benchmark
}
criterion_main!(benches);
