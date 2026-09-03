# chess

Rust chess engine trained with `bullet`, 1000 Elo as a floor, with a terminal UI to play against it.

## Layout

- `crates/board`: `no_std` board representation, move generation, FEN, Zobrist, perft.
- `crates/engine`: `std` orchestration: threads, time management, table allocation.
- `crates/tui`: terminal UI binary for playing against the engine.
- Crates are `no_std` unless the feature they exist for needs `std`. Planned: `eval`, `search`
  (`no_std`), `uci` (`no_std` message types), `chess` binary, `web` (Dioxus, wasm), `arena`,
  `datagen`, `trainer`. Crates are added when their milestone starts.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask no-comments`, `cargo xtask wasm`).

## Rules

- All changes land through pull requests. `main` is protected; never push to it directly.
- Never merge or approve a PR. Open it, wait for CI, report the link, and stop. The owner reads,
  comments, requests edits, and merges on GitHub. A PreToolUse hook in `.claude/settings.json`
  denies merge, approve, branch protection, repo settings, and push-to-main commands. It matches
  on command text, so keep those strings out of shell commands and use the Write tool for files
  that mention them.
- Delete a branch as soon as its PR is merged or closed. GitHub deletes the remote branch on
  merge and the Cleanup workflow deletes it when a PR is closed unmerged. After either, sync
  `main` and delete the local branch. Never leave stale branches.
- When a PR lands, merge main into every other open PR branch, run `cargo xtask ci`, and push, so
  each open PR is always tested against current main. Never rebase or force push.
- One struct, enum, or trait per file, named after it in snake case, across the whole repo.
  Its impls and its tests live in the same file. A type whose logic has several parts becomes a
  module: `leaper.rs` holds the type, `leaper/knight.rs`, `leaper/king.rs`, and so on hold one
  piece of logic each, private to the module. Never name a type or module after a
  keyword; `ChessMove` in `chess_move.rs`, not `Move` behind `r#move`.
- Derive enum plumbing with `strum` (`VariantArray`, `EnumCount`, `FromRepr`, `EnumIter`,
  `EnumString`, `Display`) instead of hand-written variant arrays, counts, or letter tables.
- Index tables by enum with `enum_map::EnumMap`, never by an integer method on the enum. The
  only `as usize` casts on enums live inside `const fn` table construction.
- Small PRs: one concept each. Split anything that needs more than one idea to review.
- Zero comments in Rust code. This includes `//`, `/* */`, and doc comments. `cargo xtask no-comments` enforces it in CI. Use clear names and small functions instead.
- No documentation in the repository: no `docs/`, no notes, no design documents. The README
  stays a few lines. Anything the owner should read goes in the chat.
- CI must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` with the pedantic
  group enabled, `cargo test`, `cargo xtask wasm`, and `cargo xtask no-comments`. Run `cargo xtask ci`
  locally before opening a PR.
- Never silence a lint with a blanket `allow`. Use `#[expect(clippy::name, reason = "...")]` on the
  smallest item that needs it. The reason is an attribute, not a comment, and `expect` fails if the
  lint stops firing. Prefer fixing the code, for example `usize::from` or `u8::try_from` over `as`.
- Everything is Rust. No Python, shell, or other languages for tooling; add tasks to `xtask` instead.

## Decisions

- Elo is measured only against Stockfish anchors (`UCI_LimitStrength`, `UCI_Elo 1320`) by our own
  arena at 10s+0.1s. Done means scoring above 50% with error bars that exclude 50%.
- Notations (FEN, long algebraic, SAN) belong to `board`; protocol messages to `uci`; I/O to binaries.
- TUI: visual board, standard algebraic notation for moves, slash commands starting with `/exit`.
- Sliding attack tables use checked-in magic numbers. Const evaluation of the rook table was
  measured at 39 seconds per compile and rejected in favour of runtime memoisation.
- The trained network is embedded with `include_bytes!`. `bullet_lib` is pinned and uses `metal`.
- Non-goals: opening books, tablebases, pondering, strength limiting, online play.

## Working style

The owner is learning how chess engines and neural network training work. Treat every PR as a
lesson as well as a deliverable, delivered in the chat:

- When a PR is opened, explain in the chat the concept it introduces, why the engine needs it,
  what to read in the diff, and one experiment to run. PR descriptions stay short: what changed,
  which files, how it was tested.
- Tests double as documentation: name them after the behaviour they demonstrate.
- Before starting a milestone, give a short primer and check how deep to go.
- The owner runs the training pipeline themselves: datagen, training, embedding the
  network, arena measurement. Suggest experiments and measure their effect in the arena.
