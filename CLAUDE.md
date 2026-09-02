# chess

Rust chess engine trained with `bullet`, targeting ~1000 Elo, with a terminal UI to play against it.

## Layout

- `crates/engine`: board, move generation, search, evaluation (library).
- `crates/tui`: terminal UI binary for playing against the engine.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask no-comments`).

## Rules

- All changes land through pull requests. `main` is protected; never push to it directly.
- Zero comments in Rust code. This includes `//`, `/* */`, and doc comments. `cargo xtask no-comments` enforces it in CI. Use clear names and small functions instead.
- CI must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo xtask no-comments`. Run `cargo xtask ci` locally before opening a PR.
- Everything is Rust. No Python, shell, or other languages for tooling; add tasks to `xtask` instead.
