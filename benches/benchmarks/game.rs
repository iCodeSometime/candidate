use candidate::{ChessMove, Game, MoveGen};
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

// meant to emulate a somewhat typical usage (checking side to move, etc)
// note: checking the side didn't seem to make a difference in the benchmarks
fn game_playthrough_test(pgn: &str) -> Game {
    let mut game = Game::new();
    let moves = pgn.split_whitespace().filter(|s| !s.ends_with("."));

    for m in moves {
        let mv = ChessMove::from_san(&game.current_position(), m).unwrap();
        let side_to_move = game.side_to_move();
        black_box(side_to_move);

        game.make_move(mv);
    }
    game
}
#[cfg(feature = "cache_game_state")]
fn legality_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("legality");
    let pgn = "1. d4 Nf6 2. c4 g6 3. Nc3 Bg7 4. e4 d6 5. Nf3 O-O 6. Be2 e5 7. O-O Nc6 8. d5 Ne7 9. Nd2 a5 10. Rb1 Nd7 11. a3 f5 12. b4 Kh8 13. f3 Ng8 14. Qc2 Ngf6 15. Nb5 axb4 16. axb4 Nh5 17. g3 Ndf6 18. c5 Bd7 19. Rb3 Nxg3 20. hxg3 Nh5 21. f4 exf4 22. c6 bxc6 23. dxc6 Nxg3 24. Rxg3 fxg3 25. cxd7 g2 26. Rf3 Qxd7 27. Bb2 fxe4 28. Rxf8+ Rxf8 29. Bxg7+ Qxg7 30. Qxe4 Qf6 31. Nf3 Qf4 32. Qe7 Rf7 33. Qe6 Rf6 34. Qe8+ Rf8 35. Qe7 Rf7 36. Qe6 Rf6 37. Qb3 g5 38. Nxc7 g4 39. Nd5 Qc1+ 40. Qd1 Qxd1+ 41. Bxd1 Rf5 42. Ne3 Rf4 43. Ne1 Rxb4 44. Bxg4 h5 45. Bf3 d5 46. N3xg2 h4 47. Nd3 Ra4 48. Ngf4 Kg7 49. Kg2 Kf6 50. Bxd5 Ra5 51. Bc6 Ra6 52. Bb7 Ra3 53. Be4 Ra4 54. Bd5 Ra5 55. Bc6 Ra6 56. Bf3 Kg5 57. Bb7 Ra1 58. Bc8 Ra4 59. Kf3 Rc4 60. Bd7 Kf6 61. Kg4 Rd4 62. Bc6 Rd8 63. Kxh4 Rg8 64. Be4 Rg1 65. Nh5+ Ke6 66. Ng3 Kf6 67. Kg4 Ra1 68. Bd5 Ra5 69. Bf3 Ra1 70. Kf4 Ke6 71. Nc5+ Kd6 72. Nge4+ Ke7 73. Ke5 Rf1 74. Bg4 Rg1 75. Be6 Re1 76. Bc8 Rc1 77. Kd4 Rd1+ 78. Nd3 Kf7 79. Ke3 Ra1 80. Kf4 Ke7 81. Nb4 Rc1 82. Nd5+ Kf7 83. Bd7 Rf1+ 84. Ke5 Ra1 85. Ng5+ Kg6 86. Nf3 Kg7 87. Bg4 Kg6 88. Nf4+ Kg7 89. Nd4 Re1+ 90. Kf5 Rc1 91. Be2 Re1 92. Bh5 Ra1 93. Nfe6+ Kh6 94. Be8 Ra8 95. Bc6 Ra1 96. Kf6 Kh7 97. Ng5+ Kh8 98. Nde6 Ra6 99. Be8 Ra8 100. Bh5 Ra1 101. Bg6 Rf1+ 102. Ke7 Ra1 103. Nf7+ Kg8 104. Nh6+ Kh8 105. Nf5 Ra7+ 106. Kf6 Ra1 107. Ne3 Re1 108. Nd5 Rg1 109. Bf5 Rf1 110. Ndf4 Ra1 111. Ng6+ Kg8 112. Ne7+ Kh8 113. Ng5";
    let game = game_playthrough_test(pgn);
    let san_moves: Vec<&str> = pgn
        .split_whitespace()
        .filter(|s| !s.ends_with("."))
        .collect();
    let mut moves: Vec<ChessMove> = vec![];
    for (index, san) in san_moves.iter().enumerate() {
        moves.push(ChessMove::from_san(&game.get_boards()[index], san_moves[index]).unwrap());
    }

    group.bench_function(BenchmarkId::new("legality_check", "current"), |b| {
        b.iter(|| {
            for (index, m) in moves.iter().enumerate() {
                let board = game.get_boards()[index];
                assert!(board.legal(*m));
            }
        })
    });
    group.bench_function(BenchmarkId::new("legality_check", "old_gen_all"), |b| {
        b.iter(|| {
            for (index, m) in moves.iter().enumerate() {
                let board = game.get_boards()[index];
                assert!(MoveGen::new_legal(&board).any(|mv| mv == *m));
            }
        })
    });
}

fn game_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("game");

    group.bench_function("1", |b| b.iter(|| game_playthrough_test("1. Nc3 d5 2. e3 Nc6 3. Nf3 Nf6 4. Bb5 a6 5. Bxc6+ bxc6 6. Ne5 Qd6 7. d4 Nd7 8. f4 Nxe5 9. dxe5 Qg6 10. O-O Bf5 11. e4 Bxe4 12. Nxe4 Qxe4 13. Re1 Qb4 14. e6 f6 15. Be3 g6 16. Qd4 Qxd4 17. Bxd4 Bh6 18. g3 g5 19. f5 g4 20. Rad1 Rg8 21. b3 Rb8 22. c4 dxc4 23. bxc4 Rd8 24. Kg2 Rc8 25. Bc5 Rg5 26. Rd7 Bf8 27. Rf1 a5 28. Kg1 a4 29. Bb4 Rh5 30. Rf4 Rg5 31. Rf1 Rh5 32. Rf4 Rg5 33. Ba5")));
    group.bench_function("2", |b| b.iter(|| game_playthrough_test("1. d4 Nf6 2. c4 g6 3. Nc3 Bg7 4. e4 d6 5. Nf3 O-O 6. Be2 e5 7. O-O Nc6 8. d5 Ne7 9. Nd2 a5 10. Rb1 Nd7 11. a3 f5 12. b4 Kh8 13. f3 Ng8 14. Qc2 Ngf6 15. Nb5 axb4 16. axb4 Nh5 17. g3 Ndf6 18. c5 Bd7 19. Rb3 Nxg3 20. hxg3 Nh5 21. f4 exf4 22. c6 bxc6 23. dxc6 Nxg3 24. Rxg3 fxg3 25. cxd7 g2 26. Rf3 Qxd7 27. Bb2 fxe4 28. Rxf8+ Rxf8 29. Bxg7+ Qxg7 30. Qxe4 Qf6 31. Nf3 Qf4 32. Qe7 Rf7 33. Qe6 Rf6 34. Qe8+ Rf8 35. Qe7 Rf7 36. Qe6 Rf6 37. Qb3 g5 38. Nxc7 g4 39. Nd5 Qc1+ 40. Qd1 Qxd1+ 41. Bxd1 Rf5 42. Ne3 Rf4 43. Ne1 Rxb4 44. Bxg4 h5 45. Bf3 d5 46. N3xg2 h4 47. Nd3 Ra4 48. Ngf4 Kg7 49. Kg2 Kf6 50. Bxd5 Ra5 51. Bc6 Ra6 52. Bb7 Ra3 53. Be4 Ra4 54. Bd5 Ra5 55. Bc6 Ra6 56. Bf3 Kg5 57. Bb7 Ra1 58. Bc8 Ra4 59. Kf3 Rc4 60. Bd7 Kf6 61. Kg4 Rd4 62. Bc6 Rd8 63. Kxh4 Rg8 64. Be4 Rg1 65. Nh5+ Ke6 66. Ng3 Kf6 67. Kg4 Ra1 68. Bd5 Ra5 69. Bf3 Ra1 70. Kf4 Ke6 71. Nc5+ Kd6 72. Nge4+ Ke7 73. Ke5 Rf1 74. Bg4 Rg1 75. Be6 Re1 76. Bc8 Rc1 77. Kd4 Rd1+ 78. Nd3 Kf7 79. Ke3 Ra1 80. Kf4 Ke7 81. Nb4 Rc1 82. Nd5+ Kf7 83. Bd7 Rf1+ 84. Ke5 Ra1 85. Ng5+ Kg6 86. Nf3 Kg7 87. Bg4 Kg6 88. Nf4+ Kg7 89. Nd4 Re1+ 90. Kf5 Rc1 91. Be2 Re1 92. Bh5 Ra1 93. Nfe6+ Kh6 94. Be8 Ra8 95. Bc6 Ra1 96. Kf6 Kh7 97. Ng5+ Kh8 98. Nde6 Ra6 99. Be8 Ra8 100. Bh5 Ra1 101. Bg6 Rf1+ 102. Ke7 Ra1 103. Nf7+ Kg8 104. Nh6+ Kh8 105. Nf5 Ra7+ 106. Kf6 Ra1 107. Ne3 Re1 108. Nd5 Rg1 109. Bf5 Rf1 110. Ndf4 Ra1 111. Ng6+ Kg8 112. Ne7+ Kh8 113. Ng5")));
}

criterion_group!(game, game_benchmarks);
