# chess

Rust chess engine trained with `bullet`, targeting ~1000 Elo, with a terminal UI to play against it.

## Layout

- `crates/board`: `no_std` board representation, move generation, FEN, Zobrist, perft.
- `crates/engine`: `std` orchestration: threads, time management, table allocation.
- `crates/tui`: terminal UI binary for playing against the engine.
- Crates are `no_std` unless the feature they exist for needs `std`. See `docs/SCOPE.md`.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask no-comments`).

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
- Zero comments in Rust code. This includes `//`, `/* */`, and doc comments. `cargo xtask no-comments` enforces it in CI. Use clear names and small functions instead.
- CI must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo xtask no-comments`. Run `cargo xtask ci` locally before opening a PR.
- Everything is Rust. No Python, shell, or other languages for tooling; add tasks to `xtask` instead.

## Scope

See `docs/SCOPE.md` for goals, crate layout, milestones, and decisions. Update it when a decision changes.

## Working style

The owner is learning how chess engines and neural network training work. Treat every
milestone as a lesson as well as a deliverable:

- Each PR description explains the concept being introduced, why the engine needs it,
  what to read in the diff, and one experiment the owner can run.
- Explanations live in `docs/notes/`, one note per milestone, never in code comments.
- Tests double as documentation: name them after the behaviour they demonstrate.
- Before starting a milestone, give a short primer and check how deep to go.
- The owner runs the training pipeline themselves: datagen, training, embedding the
  network, arena measurement. Suggest experiments and measure their effect in the arena.
