use criterion::criterion_main;

mod benchmarks;

criterion_main! {
  benchmarks::perft::perft,
  benchmarks::board::board,
  benchmarks::game::game,
}
