v0.0.6

---

v0.0.5
 - Fixed castling san generation

---

v0.0.4
 - Added `Board::legal_destinations_from` to get the bitboard representing all legal destinations from a given square.

---

v0.0.3
 - Added optional `tracing` instrumentation to Game - use the `instrument_game` feature if you're collecting telemetry.
 - Additional 3x increase in build speed, by only rebuilding magic bitboards when necessary.
 - Added `cache_game_state` default feature to improve performance when using `Game` by 10-20x for reasonably sized games (more for larger games).
 - Legality checking for unsanitized inputs is 4-5x faster.

v0.0.2
 - Added `Board::en_passant_target` to match standard meaning - for now, `Board::en_passant` should still be preferred in the hot path.
 - Added `Board::has_checkers` as convenience function.
 - *BREAKING* - `Game::make_move` now returns the SAN notation for the move made. `Board::make_move` (hot path) is unchanged.

---

v0.0.1
 - Build speed (KarelPeeters)
 - Faster status checking (AlexanderHarrison)
 - Additional safety guarantees (terrorfisch)
 - Minor performance enhancement when creating a `Square` (vitorsvt)
 - Edition bumped to 2021
 - Performance benchmarks added

---

Prior to 0.0.1 - see https://github.com/jordanbray/chess