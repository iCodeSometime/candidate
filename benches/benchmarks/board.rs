use std::time::Duration;

use candidate::{BitBoard, Board, BoardStatus, ChessMove, Game, MoveGen};
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

const EMPTY: &BitBoard = &BitBoard(0);

fn movegen_len_status(board: &Board) {
    let moves = MoveGen::new_legal(board).len();
    match moves {
        0 => {
            if board.checkers() == EMPTY {
                black_box(BoardStatus::Stalemate);
            } else {
                black_box(BoardStatus::Checkmate);
            }
        }
        _ => {
            black_box(BoardStatus::Checkmate);
        }
    }
}

fn current_status(board: &Board) {
    black_box(board.status());
}

// no need for a super long game - calculation will take longest with more pieces on the board
const TEST_PGN: &str = "1. d4 Nf6 2. c4 g6 3. Nc3 Bg7 4. e4 d6 5. Nf3 O-O 6. Be2 e5 7. O-O Nc6 8. d5 Ne7 9. Nd2 a5 10. Rb1 Nd7 11. a3 f5 12. b4 Kh8 13. f3 Ng8 14. Qc2 Ngf6 15. Nb5 axb4 16. axb4 Nh5 17. g3 Ndf6 18. c5 Bd7 19. Rb3 Nxg3 20. hxg3 Nh5 21. f4 exf4 22. c6 bxc6 23. dxc6 Nxg3 24. Rxg3 fxg3 25. cxd7 g2 26. Rf3 Qxd7 27. Bb2 fxe4 28. Rxf8+ Rxf8 29. Bxg7+ Qxg7 30. Qxe4 Qf6";

fn build_boards_from_pgn(pgn: &str) -> Vec<Board> {
    let mut boards = Vec::new();
    boards.push(Board::default());

    pgn.split_whitespace()
        .filter(|s| !s.ends_with("."))
        .fold(Game::new(), |mut g, m| {
            g.make_move(ChessMove::from_san(&g.current_position(), m).expect("Valid SAN Move"));
            boards.push(g.current_position());
            g
        });

    boards
}

#[allow(dead_code)]
fn status_comparison_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("status");
    let boards: Vec<Board> = build_boards_from_pgn(TEST_PGN);
    group.measurement_time(Duration::from_millis(500));
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(500));

    for (index, board) in boards.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("old_movegen_len", index),
            board,
            |b, board| {
                b.iter(|| movegen_len_status(board));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("current_status", index),
            board,
            |b, board| {
                b.iter(|| current_status(board));
            },
        );
    }
}

fn status_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("status");
    let boards: Vec<Board> = build_boards_from_pgn(TEST_PGN);
    group.measurement_time(Duration::from_millis(500));
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(500));

    for (index, board) in boards.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(index), board, |b, board| {
            b.iter(|| current_status(board));
        });
    }
}

criterion_group!(compare_status, status_comparison_benchmarks);
criterion_group!(board, status_benchmarks);
