# A Fast Chess Library In Rust

<div align="center">

[![Build Status](https://github.com/icodesometime/candidate/actions/workflows/test.yml/badge.svg)](https://github.com/iCodeSometime/candidate/actions/workflows/test.yml)
[![docs.rs](https://docs.rs/candidate/badge.svg)](https://docs.rs/candidate/)
[![DeepSource](https://deepsource.io/gh/iCodeSometime/candidate.svg/?label=resolved+issues&show_trend=true&token=jnTOWlnEoudfrav1yqDcRAT7)](https://deepsource.io/gh/iCodeSometime/candidate/?ref=repository-badge)
[![crates.io](https://img.shields.io/crates/v/candidate.svg)](https://crates.io/crates/candidate)

</div>

This library handles the process of move generation within a chess engine or chess UI.

This is a library to manage chess game state and move generation.

## Requires Rust 1.31 or Greater

This library requires rust version 1.27 or greater in order to check for the BMI2 instruction-set at compile-time.  Additionally, this build is compatible with rust 2018 which, I believe, requires rust 1.31.

> Note: bmi2 has been disabled due to horrible performance on AMD architectures.  I have instead opted to expose the two relevant functions publicly if on a bmi2 CPU.

## Examples

### Incremental Move Generation With Capture/Non-Capture Sorting

Here we iterate over all moves with incremental move generation.  The iterator below will generate moves as you are going through the list, which is ideal for situations where not all moves will be looked at (such as in an engine search function).

```rust
  use candidate::MoveGen;
  use candidate::Board;
  use candidate::EMPTY;

  // create a board with the initial position
  let board = Board::default();

  // create an iterable
  let mut iterable = MoveGen::new_legal(&board);

  // make sure .len() works.
  assert_eq!(iterable.len(), 20); // the .len() function does *not* consume the iterator

  // lets iterate over targets.
  let targets = board.color_combined(!board.side_to_move());
  iterable.set_iterator_mask(targets);

  // count the number of targets
  let mut count = 0;
  for _ in &mut iterable {
      count += 1;
      // This move captures one of my opponents pieces (with the exception of en passant)
  }

  // now, iterate over the rest of the moves
  iterable.set_iterator_mask(!EMPTY);
  for _ in &mut iterable {
      count += 1;
      // This move does not capture anything
  }

  // make sure it works
  assert_eq!(count, 20);
```

### Setting up a position

The `Board` structure tries to keep the position legal at all times.  This can be annoying when setting up a board, for example via user input.

To deal with this, the `BoardBuilder` structure was introduced in 3.1.0. `BoardBuilder` structure follows a non-consuming builder pattern and can be converted to a `Result<Board, Error>` via `Board::try_from(...)` or `board_builder.try_into()`.

```rust
  use candidate::{Board, BoardBuilder, Piece, Square, Color};
  use std::convert::TryInto;

  let mut board_builder = BoardBuilder::new();
  board_builder.piece(Square::A1, Piece::King, Color::White)
               .piece(Square::A8, Piece::Rook, Color::Black)
               .piece(Square::D1, Piece::King, Color::Black);
  
  let board: Board = board_builder.try_into()?;
```

### Making a Move

Here we make a move on the chess board.  The board is a copy-on-make structure, meaning every time you make a move, you create a new chess board.  You can use `board.make_move()` to update the current position, but you cannot unmake the move.  The board structure is optimized for size to reduce copy-time.

```rust
  use candidate::{Board, ChessMove, Square, Color};

  let m = ChessMove::new(Square::D2, Square::D4, None);

  let board = Board::default();
  assert_eq!(board.make_move_new(m).side_to_move(), Color::Black);
```

### Representing a Full Game

There is more to chess than just what is on the board.  The `Game` object keeps track of the history of the game to allow draw offers, resignations, draw by 50 move rule, draw by repetition, and in general anything that needs the history of the game.

```rust
  use candidate::{Game, Square, ChessMove};

  let b1c3 = ChessMove::new(Square::B1, Square::C3, None);
  let c3b1 = ChessMove::new(Square::C3, Square::B1, None);
  
  let b8c6 = ChessMove::new(Square::B8, Square::C6, None);
  let c6b8 = ChessMove::new(Square::C6, Square::B8, None);
  
  let mut game = Game::new();
  assert_eq!(game.can_declare_draw(), false);
  
  game.make_move(b1c3);
  game.make_move(b8c6);
  game.make_move(c3b1);
  game.make_move(c6b8);
  
  assert_eq!(game.can_declare_draw(), false); // position has shown up twice
  
  game.make_move(b1c3);
  game.make_move(b8c6);
  game.make_move(c3b1);
  game.make_move(c6b8);
  assert_eq!(game.can_declare_draw(), true); // position has shown up three times
```

### FEN Strings

`BoardBuilder`, `Board`, and `Game` all implement `FromStr` to allow you to convert an FEN string into the object.  Additionally, `BoardBuilder` and `Board` implement `std::fmt::Display` to convert them into an FEN string.

```rust
  use candidate::Board;
  use std::str::FromStr;
  
  assert_eq!(
      Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
              .expect("Valid Position"),
    Board::default()
  );
```

## Compile-time Options

When compiling, I definitely recommend using RUSTFLAGS="-C target-cpu=native", specifically to gain access to the popcnt and ctzl instruction available on almost all modern CPUs.  This is used internally to figure out how many pieces are on a bitboard, and what square a piece is on respectively.  Because of the type system used here, these tasks become literally a single instruction.  Additionally, BMI2 is enabled on machines with the instructions by using this flag.

## BMI2

The BMI2 instruction-set is used on machines that support it.  This speeds up the logic in two ways:
* It uses built-in instructions to do the same logic that magic bitboards do.
* It reduces cache load by storing moves in a u16 rather than a u64, which can be decompressed to a u64 with a single instruction.

On targets without BMI2, the library falls back on magic bitboards.  This is checked at compile-time.

## Shakmaty

Another rust chess library is in the 'shakmaty' crate.  This is a great library, with many more features than this one.  It supports various chess variants, as well as the UCI protocol.  However, those features come at a cost, and this library performs consistently faster in all test cases I can throw at it.  To compare the two, I have added 'shakmaty' support to the 'chess_perft' application, and moved a bunch of benchmarks to that crate.  You can view the results at
https://github.com/jordanbray/chess_perft.

## What It Does

This library allows you to create a chess board from a FEN-formatted string, list all legal moves for the chess board and make moves.

This library also allows you to view various pieces of board-state information such as castle rights.

This library has very fast move generation (the primary purposes of its existance), which will be optimized more.  All the tricks to make chess move generation fast are used.

## What It Does Not Do

This is not a chess engine, just the move generator.  This is not a chess UI, just the move generator.  This is not a chess PGN parser, database, UCI communicator, XBOARD/WinBoard protocol, website or grandmaster.  Just a humble move generator.

More coming soon

## API Documentation

 https://jordanbray.github.io/chess/chess/.

## History
This project was forked from https://github.com/jordanbray/chess, as of `91fe8e2`.
Several unmerged PRs have been added into this version - the author on these commits has been set to reflect the actual author.
Maintaining backwards compatibility with the `chess` crate is a non-goal. Expect breaking changes until 1.0.
Check CHANGELOG.md for details - all breaking changes should be marked as *BREAKING*

Some of the improvements made since the fork:
 - Build times are _drastically_ improved. rust-analyzer actually works now (thanks KarelPeeters)
 - Checking the `status` of the `Board` is 2-3x faster for a fully populated board (thanks AlexanderHarrison)
 - Using `Game` is 10-20x faster for reasonably sized games when using the new `cache_game_state` feature (more for larger games). If you're in an embedded environment and can't tolerate the extra KB of memory, you can disable this feature.
 - Legality checking of unsanitized moves is 4-5x faster
 - `Game::make_move` now returns `Option<String>` with the SAN representation of the move. `Board::make_move` still returns a bool to avoid overhead in the hot path
 - Optional instrumentation added to `Game`, using [tracing](https://github.com/tokio-rs/tracing) - just use the `instrument_game` feature.
 - `Board::en_passant_target` and `Board::has_checkers` added as convenience methods. `Board::en_passant` is slightly faster than `Board::en_passant_target` for now.
 - Performance benchmarks added

## Is it any good?

[Yes](https://news.ycombinator.com/item?id=3067434)

