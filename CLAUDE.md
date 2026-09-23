# chess

Rust chess engine trained with `bullet`, 1000 Elo as a floor, with a terminal UI to play against it.

## Layout

- `crates/board`: `no_std` board representation, move generation, Zobrist, perft.
- `crates/fen`: `no_std` FEN notation as a trait; `board` implements it for its types.
- `crates/eval`: `no_std` evaluation: the `Evaluator` trait, `Score`, and `PieceSquareTables`;
  the trained network becomes a second implementor.
- `crates/search`: `no_std` search: `Search<E: Evaluator>` deepens one ply per `deepen` edge
  with alpha-beta negamax; depth zero is quiescence, captures only with stand pat; `Depth`
  newtype.
- `crates/engine`: `std` orchestration: threads, time management, table allocation.
- `crates/tui`: terminal UI binary for playing against the engine.
- Crates are `no_std` unless the feature they exist for needs `std`. Planned: `uci` (`no_std`
  message types), `chess` binary, `web` (Dioxus, wasm), `arena`, `datagen`, `trainer`. Crates
  are added when their milestone starts.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask lint`, which parses every file once
  and runs `const-shape`, `distinct-signatures`, `fn-shape`, `manual-iteration`,
  `named-lifetimes`, `no-comments`, `no-free-fns`, `private-fns`, `state-graph`, `test-budget`,
  and `type-shape`, `cargo xtask wasm`, `cargo xtask magics`). Lints return a `Report` of
  `Violation`s at a `Site`; every xtask error is a `Failure` variant.

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
  module directory: `leaper/mod.rs` holds the type, `leaper/knight.rs`, `leaper/king.rs`, and so on
  hold one piece of logic each, private to the module. Always `dir/mod.rs`, never `dir.rs` beside
  `dir/`. Never name a type or module after a
  keyword; `ChessMove` in `chess_move.rs`, not `Move` behind `r#move`.
- Derive enum plumbing with `strum` (`VariantArray`, `EnumCount`, `FromRepr`, `EnumIter`,
  `EnumString`, `Display`) instead of hand-written variant arrays, counts, or letter tables.
- Index tables by enum with `enum_map::EnumMap`, never by an integer method on the enum. An `as`
  cast is allowed only inside a `const fn`, `const` item, or `const` block, or in one of the
  three bit boundary files `square.rs`, `bitboard.rs`, and `slider/magic.rs`; elsewhere use
  `From`, `TryFrom`, or an `EnumMap`. Loops in const context walk a slice with
  `while let [head, rest @ ..]` or recurse; no counter loops. `cargo xtask const-shape` enforces
  both.
- A family of behaviours is a trait with zero-sized implementors, not an enum matched on at
  runtime: `Rook: Slider`, `Knight: Leaper`. Per-implementor data is an associated const.
- Game logic is a state transition graph, and `cargo xtask state-graph` enforces its shape. A
  vertex is a type with `impl State for T {}`. An edge is a `pub` method that takes `self` by
  value and returns a vertex, or a trait whose implementors do so with the vertex as a parameter;
  it lives on its source vertex, there is at most one edge per (source, target) pair, and at most
  4 edges leave a vertex. A `&self` method returns a vertex only as a plain field projection,
  nothing takes `&mut self` on a vertex, and no tuple return holds a vertex. A `match` that names
  variants of a data-carrying enum outside that enum's file may only fill a table of literals,
  paths, and tuples; anything else dispatches through a trait. Every struct field is private,
  including `pub(crate)` and `pub(super)`, so values are built by constructors alone.
- Small PRs: one concept each. Split anything that needs more than one idea to review.
- Zero comments in Rust code. This includes `//`, `/* */`, and doc comments. `cargo xtask no-comments` enforces it in CI. Use clear names and small functions instead.
- No free functions. Every `fn` is a method or associated function of a struct, enum, or trait;
  the only exceptions are `main` and `#[test]` functions. `cargo xtask no-free-fns` enforces it.
- Lifetimes are named with a word (`'board`, `'scan`, `'ast`), never a single letter.
  `cargo xtask named-lifetimes` enforces it; `'_` and `'static` are exempt.
- At most 4 private functions per type, counted across all its inherent impl blocks; trait impl
  methods do not count. More than that means a second type is hiding inside the first.
  `cargo xtask private-fns` enforces it.
- Function bodies stay flat, outside test modules. No local bound to a boolean: a one-use
  condition is inlined, a mode is a type. No tuple literal bound to a local or seeding a fold:
  two values that travel together are a struct. At most 4 parameters after the receiver. Control
  flow nests at most two deep, counting `if`, `match` arms, loops, and closure bodies, with
  `else if` chains flat. `cargo xtask fn-shape` enforces it.
- Iterators are driven by combinators. No `let mut` bound to an iterator, and no `for` loop whose
  body only pushes, extends, or inserts into a collection, even behind an `if`: use `format`,
  `collect`, `fold`, or `extend`. `cargo xtask manual-iteration` enforces it.
- Types carry their meaning. No `bool` struct field: a stored flag is a stored mode, so it is an
  enum or a type parameter. No tuple type in a function signature or struct field outside a trait
  impl: values that travel together are a struct. No `Result<_, String>`: errors are an enum with
  `thiserror`. `cargo xtask type-shape` enforces all three.
- No two non-`pub` functions on one type share a signature (generics, receiver, parameter types,
  return type). Two such helpers mean a value is missing its type: `alpha` and `beta` both returning
  `Score` became `Bound<Lower>` and `Bound<Upper>`. `cargo xtask distinct-signatures` enforces it.
- Invariants live as high as possible: a type that cannot represent the invalid state, else a
  `const _: () = assert!(..)` at compile time, else a test. A test must fail for a reason no type,
  const assertion, or other test catches. `cargo xtask test-budget` enforces the budget: at most
  3 tests per file, 3 assertion sites, 20 lines, and 4 literals per test, no integer literal
  above 64 in test code, on average at most 1 test per file, and test code at most 20% of all
  lines. Prefer one exhaustive or oracle test over examples; randomness comes from `proptest`;
  deep checks are `#[ignore]` and run outside the PR gate.
- No documentation in the repository: no `docs/`, no notes, no design documents. The README
  stays a few lines. Anything the owner should read goes in the chat.
- CI must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` with the pedantic
  group enabled, `cargo test`, `cargo xtask wasm`, and `cargo xtask lint` as one CI job covering
  comments, free functions, the test budget, and the state graph. Run `cargo xtask ci` locally
  before opening a PR.
- Never silence a lint with a blanket `allow`. Use `#[expect(clippy::name, reason = "...")]` on the
  smallest item that needs it. The reason is an attribute, not a comment, and `expect` fails if the
  lint stops firing. Prefer fixing the code, for example `usize::from` or `u8::try_from` over `as`.
- Everything is Rust. No Python, shell, or other languages for tooling; add tasks to `xtask` instead.

## Decisions

- Elo is measured only against Stockfish anchors (`UCI_LimitStrength`, `UCI_Elo 1320`) by our own
  arena at 10s+0.1s. Done means scoring above 50% with error bars that exclude 50%.
- Interop notations are their own `no_std` crates exposing traits that `board` implements: `fen`
  for positions, `uci` for protocol messages. Square, piece, and move text forms stay on their
  types. I/O belongs to binaries.
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
