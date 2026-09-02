# Scope

## Goal

A chess engine written in Rust, with a neural network evaluation trained using
[bullet](https://github.com/jw1912/bullet), that plays at or above 1000 Elo and
can be played against from a terminal UI. Everything, including tooling, is Rust.

## Definition of done

- The engine loads a `bullet`-trained NNUE and uses it as its evaluation.
- Search runs on multiple threads and gains measurable strength from them.
- The arena reports the engine scoring above 50% against Stockfish at
  `UCI_LimitStrength true, UCI_Elo 1320` with error bars that exclude 50%.
  Stockfish cannot be limited below 1320, so this proves a comfortable margin over 1000.
- The TUI supports a full game against the engine from the terminal.

Stronger than 1000 is fine. There is no strength limiter in scope.

## Decisions

- Elo is measured locally against Stockfish anchors. No Lichess integration.
- The reference time control for all Elo measurements is 10 seconds plus 0.1 second increment per side.
- The TUI talks to the engine in-process. UCI is a separate binary for testing and GUIs.
- The trained network is embedded in the engine binary with `include_bytes!`.
- The engine crate compiles for `wasm32-unknown-unknown`. Threads and wall-clock time live
  behind the default `threads` feature and the wasm build disables it. CI runs clippy for the
  wasm target on every PR so this cannot drift. A browser build is a stretch milestone.
- `bullet_lib` is pinned to a git revision and built with the `metal` feature.
  Development machine: Apple M5 Max, 18 CPU cores, 40 GPU cores, 48 GB RAM.

## Crates

| Crate | Kind | Purpose |
| --- | --- | --- |
| `engine` | lib | Board, move generation, search, evaluation, threading |
| `uci` | bin | UCI protocol over stdin/stdout |
| `tui` | bin | Terminal UI built on ratatui and crossterm |
| `arena` | bin | Parallel UCI match runner with Elo estimates and error bars |
| `datagen` | bin | Parallel self-play data generation in a bullet-readable format |
| `trainer` | bin | bullet training run definition, emits the quantised network |
| `xtask` | bin | Repository tooling and CI checks |

## Engine design

- Bitboards with magic bitboard sliding attacks, Zobrist hashing, FEN parsing, make and unmake.
- Legal move generation validated by perft against published node counts.
- Iterative deepening, alpha-beta with principal variation search, aspiration windows.
- Transposition table shared between threads with lock-free atomic entries.
- Quiescence search, MVV-LVA capture ordering, killer and history heuristics.
- Null-move pruning, late-move reductions, check extensions.
- Lazy SMP: independent searcher threads sharing the table, main thread reports.
- Hand-crafted evaluation (material and piece-square tables) used only to bootstrap datagen.
- NNUE: perspective network, 768 inputs per side, one hidden layer with SCReLU,
  incrementally updated accumulators, int16 quantised weights from bullet.
- Draw detection: repetition, fifty-move rule, insufficient material.
- Search limits go through a clock abstraction so a browser host can supply time.

## TUI

The screen shows a visual chessboard, the move list, and a command line. Moves are typed
in standard algebraic notation, for example `e4`, `Nf3`, `exd5`, `O-O`, `e8=Q`, and
ambiguous or illegal input is rejected with a message rather than guessed at. Commands
start with a slash: `/exit` quits. Further commands planned under the same convention
are `/new`, `/undo`, `/flip`, and `/pgn`. The engine replies on its own after each
legal move and its reply is shown in algebraic notation.

## Training pipeline

1. Datagen plays self-play games across all cores using the current best engine,
   starting from randomised opening plies, at a fixed node budget per move.
2. Positions are written with search score and game result in bullet's text
   format, then shuffled and interleaved with `bullet-utils`.
3. The trainer runs on Metal with an architecture close to bullet's `simple` example.
4. The quantised network is embedded in the engine and the arena confirms the gain
   over the previous network before it is accepted.
5. Repeat from step 1 with the stronger engine if more strength is wanted.

## Milestones

Each milestone is one pull request unless it grows too large to review.

1. Board, move generation, FEN, perft tests.
2. Search with hand-crafted evaluation and the UCI binary. Playable in any GUI.
3. Minimal TUI: visual board, algebraic notation move entry, `/exit`, engine replies. Polish comes later.
4. Arena with Stockfish anchors. First Elo measurement. Every later change is measured.
5. Lazy SMP multithreading with a shared transposition table.
6. Datagen, bullet trainer, NNUE inference. Arena confirms the gain over the hand-crafted evaluation.
7. TUI polish: eval and principal variation display, undo, flip, PGN export, optional node-limited difficulty levels.
8. Stretch: web build. The engine compiled to wasm with a browser board, single-threaded, host-provided clock.

## Learning

The project is educational. See the working style section of `CLAUDE.md`. Each milestone
adds a note under `docs/notes/` explaining the concept it introduces.

## Non-goals

- Opening books, endgame tablebases, pondering, advanced time management.
- Strength limiting or human-like play.
- Online play.

## Risks

- bullet's Metal backend is newer than CUDA. Mitigation: pin a revision, keep the
  network small, keep the data in text format so it can be trained elsewhere if needed.
- Stockfish is not installed on the development machine yet. Milestone 4 needs `brew install stockfish`.
- Stockfish's `UCI_Elo` scale is approximate. Mitigation: the done criterion leaves a
  wide margin above 1000, and the arena reports error bars.
- The no-comments rule is hardest on engine code full of bit tricks and tables.
  Mitigation: named constants, small functions, and tests that document behaviour.
