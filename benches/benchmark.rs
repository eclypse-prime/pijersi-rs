use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pijersi_rs::game::Game;
use pijersi_rs::logic::perft::perft;
use pijersi_rs::search::alphabeta::{search_node, BASE_ALPHA, BASE_BETA};

fn bench_perft(c: &mut Criterion) {
    let mut game = Game::new();
    game.init();

    c.bench_function("perft 1", |b| {
        b.iter(|| black_box(perft(&game.board, game.current_player, 1)))
    });
    c.bench_function("perft 2", |b| {
        b.iter(|| black_box(perft(&game.board, game.current_player, 2)))
    });
    c.bench_function("perft 3", |b| {
        b.iter(|| black_box(perft(&game.board, game.current_player, 3)))
    });
    c.bench_function("perft 4", |b| {
        b.iter(|| black_box(perft(&game.board, game.current_player, 4)))
    });
}

fn bench_evaluate_action(c: &mut Criterion) {
    let mut board = Game::new();
    board.init();
    c.bench_function("search_node 1", |b| {
        b.iter(|| {
            black_box(search_node(
                (&board.board, 1 - board.current_player),
                1,
                (BASE_ALPHA, BASE_BETA),
                None,
                Default::default(),
                None,
            ))
        })
    });
    c.bench_function("search_node 2", |b| {
        b.iter(|| {
            black_box(search_node(
                (&board.board, 1 - board.current_player),
                2,
                (BASE_ALPHA, BASE_BETA),
                None,
                Default::default(),
                None,
            ))
        })
    });
    c.bench_function("search_node 3", |b| {
        b.iter(|| {
            black_box(search_node(
                (&board.board, 1 - board.current_player),
                3,
                (BASE_ALPHA, BASE_BETA),
                None,
                Default::default(),
                None,
            ))
        })
    });
    c.bench_function("search_node 4", |b| {
        b.iter(|| {
            black_box(search_node(
                (&board.board, 1 - board.current_player),
                4,
                (BASE_ALPHA, BASE_BETA),
                None,
                Default::default(),
                None,
            ))
        })
    });
}

fn bench_search(c: &mut Criterion) {
    let mut board = Game::new();
    board.init();
    board.options.verbose = false;
    c.bench_function("search 1", |b| {
        b.iter(|| black_box(board.search_to_depth(1, None, None)))
    });
    c.bench_function("search 2", |b| {
        b.iter(|| black_box(board.search_to_depth(2, None, None)))
    });
    c.bench_function("search 3", |b| {
        b.iter(|| black_box(board.search_to_depth(3, None, None)))
    });
    c.bench_function("search 4", |b| {
        b.iter(|| black_box(board.search_to_depth(4, None, None)))
    });
}

criterion_group!(benches, bench_perft, bench_evaluate_action, bench_search);
criterion_main!(benches);
