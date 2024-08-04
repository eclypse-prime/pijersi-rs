use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pijersi_rs::board::Board;
use pijersi_rs::logic::perft::perft;

fn bench_perft(c: &mut Criterion) {
    let mut board = Board::new();
    board.init();
    c.bench_function("perft 1", |b| {
        b.iter(|| black_box(perft(&board.cells, board.current_player, 1)))
    });
    c.bench_function("perft 2", |b| {
        b.iter(|| black_box(perft(&board.cells, board.current_player, 2)))
    });
    c.bench_function("perft 3", |b| {
        b.iter(|| black_box(perft(&board.cells, board.current_player, 3)))
    });
    c.bench_function("perft 4", |b| {
        b.iter(|| black_box(perft(&board.cells, board.current_player, 4)))
    });
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
