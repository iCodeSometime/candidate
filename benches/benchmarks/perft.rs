use candidate::{Board, BoardBuilder, MoveGen};
use criterion::{criterion_group, Criterion};
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Duration;

fn movegen_perft_test(fen: String, depth: usize, result: usize) {
    let board: Board = BoardBuilder::from_str(&fen).unwrap().try_into().unwrap();

    assert_eq!(MoveGen::movegen_perft_test(&board, depth), result);
    assert_eq!(MoveGen::movegen_perft_test_piecewise(&board, depth), result);
}

fn movegen_perft_kiwipete() {
    movegen_perft_test(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_owned(),
        5,
        193690690,
    );
}

fn movegen_perft_1() {
    movegen_perft_test("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1".to_owned(), 6, 824064);
    // Invalid FEN
}

fn movegen_perft_2() {
    movegen_perft_test("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1".to_owned(), 6, 824064);
    // Invalid FEN
}

fn movegen_perft_3() {
    movegen_perft_test("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1".to_owned(), 6, 1440467);
}

fn movegen_perft_4() {
    movegen_perft_test("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1".to_owned(), 6, 1440467);
}

fn movegen_perft_5() {
    movegen_perft_test("5k2/8/8/8/8/8/8/4K2R w K - 0 1".to_owned(), 6, 661072);
}

fn movegen_perft_6() {
    movegen_perft_test("4k2r/8/8/8/8/8/8/5K2 b k - 0 1".to_owned(), 6, 661072);
}

fn movegen_perft_7() {
    movegen_perft_test("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1".to_owned(), 6, 803711);
}

fn movegen_perft_8() {
    movegen_perft_test("r3k3/8/8/8/8/8/8/3K4 b q - 0 1".to_owned(), 6, 803711);
}

fn movegen_perft_9() {
    movegen_perft_test(
        "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".to_owned(),
        4,
        1274206,
    );
}

fn movegen_perft_10() {
    movegen_perft_test(
        "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1".to_owned(),
        4,
        1274206,
    );
}

fn movegen_perft_11() {
    movegen_perft_test(
        "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1".to_owned(),
        4,
        1720476,
    );
}

fn movegen_perft_12() {
    movegen_perft_test(
        "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1".to_owned(),
        4,
        1720476,
    );
}

fn movegen_perft_13() {
    movegen_perft_test("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1".to_owned(), 6, 3821001);
}

fn movegen_perft_14() {
    movegen_perft_test("3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1".to_owned(), 6, 3821001);
}

fn movegen_perft_15() {
    movegen_perft_test("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1".to_owned(), 5, 1004658);
}

fn movegen_perft_16() {
    movegen_perft_test("5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1".to_owned(), 5, 1004658);
}

fn movegen_perft_17() {
    movegen_perft_test("4k3/1P6/8/8/8/8/K7/8 w - - 0 1".to_owned(), 6, 217342);
}

fn movegen_perft_18() {
    movegen_perft_test("8/k7/8/8/8/8/1p6/4K3 b - - 0 1".to_owned(), 6, 217342);
}

fn movegen_perft_19() {
    movegen_perft_test("8/P1k5/K7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 92683);
}

fn movegen_perft_20() {
    movegen_perft_test("8/8/8/8/8/k7/p1K5/8 b - - 0 1".to_owned(), 6, 92683);
}

fn movegen_perft_21() {
    movegen_perft_test("K1k5/8/P7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 2217);
}

fn movegen_perft_22() {
    movegen_perft_test("8/8/8/8/8/p7/8/k1K5 b - - 0 1".to_owned(), 6, 2217);
}

fn movegen_perft_23() {
    movegen_perft_test("8/k1P5/8/1K6/8/8/8/8 w - - 0 1".to_owned(), 7, 567584);
}

fn movegen_perft_24() {
    movegen_perft_test("8/8/8/8/1k6/8/K1p5/8 b - - 0 1".to_owned(), 7, 567584);
}

fn movegen_perft_25() {
    movegen_perft_test("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1".to_owned(), 4, 23527);
}

fn movegen_perft_26() {
    movegen_perft_test("8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1".to_owned(), 4, 23527);
}

fn run_perft(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft");
    group.measurement_time(Duration::new(10, 0));
    group.sample_size(10);

    group.bench_function("kiwipete", |b| b.iter(|| movegen_perft_kiwipete()));
    group.bench_function("1", |b| b.iter(|| movegen_perft_1()));
    group.bench_function("2", |b| b.iter(|| movegen_perft_2()));
    group.bench_function("3", |b| b.iter(|| movegen_perft_3()));
    group.bench_function("4", |b| b.iter(|| movegen_perft_4()));
    group.bench_function("5", |b| b.iter(|| movegen_perft_5()));
    group.bench_function("6", |b| b.iter(|| movegen_perft_6()));
    group.bench_function("7", |b| b.iter(|| movegen_perft_7()));
    group.bench_function("8", |b| b.iter(|| movegen_perft_8()));
    group.bench_function("9", |b| b.iter(|| movegen_perft_9()));
    group.bench_function("10", |b| b.iter(|| movegen_perft_10()));
    group.bench_function("11", |b| b.iter(|| movegen_perft_11()));
    group.bench_function("12", |b| b.iter(|| movegen_perft_12()));
    group.bench_function("13", |b| b.iter(|| movegen_perft_13()));
    group.bench_function("14", |b| b.iter(|| movegen_perft_14()));
    group.bench_function("15", |b| b.iter(|| movegen_perft_15()));
    group.bench_function("16", |b| b.iter(|| movegen_perft_16()));
    group.bench_function("17", |b| b.iter(|| movegen_perft_17()));
    group.bench_function("18", |b| b.iter(|| movegen_perft_18()));
    group.bench_function("19", |b| b.iter(|| movegen_perft_19()));
    group.bench_function("20", |b| b.iter(|| movegen_perft_20()));
    group.bench_function("21", |b| b.iter(|| movegen_perft_21()));
    group.bench_function("22", |b| b.iter(|| movegen_perft_22()));
    group.bench_function("23", |b| b.iter(|| movegen_perft_23()));
    group.bench_function("24", |b| b.iter(|| movegen_perft_24()));
    group.bench_function("25", |b| b.iter(|| movegen_perft_25()));
    group.bench_function("26", |b| b.iter(|| movegen_perft_26()));
}

criterion_group!(all, run_perft);
