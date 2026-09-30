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
- `crates/uci`: `no_std` protocol types: `Command` parsed by `TryFrom<&str>` (its `Position` holds a
  `Board` and typed `Moves`, its `EngineOption` typed variants), `Response`
  printed by `Display`, and the `Receiver` trait through which a command is delivered, so no
  other crate matches on `Command`.
- `crates/engine`: `std` orchestration. `Engine<S: Sink>` owns the board, the transposition
  table, the stop flag, and its output sink, and implements `Receiver`; `Thinker` runs
  iterative deepening against a `Deadline` built from the `go` limits and a `TimeBudget`.
- `crates/chess`: the UCI binary: a stdin thread that also raises the stop flag on `stop` and
  `quit`, a stdout sink, and a fold of commands over the engine.
- `crates/arena`: `std` match runner. `Opponent` is the trait a game talks to: `UciProcess`
  drives a child process over pipes with a reader thread and timeouts, `InProcessEngine` drives
  `engine::Engine` in process so tests need no binary.
- `crates/tui`: terminal UI binary for playing against the engine.
- Crates are `no_std` unless the feature they exist for needs `std`. Planned: `web` (Dioxus,
  wasm), `datagen`, `trainer`. Crates are added when their milestone starts.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask lint`, which parses every file once
  and runs `const-shape`, `distinct-signatures`, `fn-shape`, `literal-names`,
  `manual-iteration`, `module-nesting`, `named-lifetimes`, `no-comments`, `no-forwarders`,
  `no-free-fns`, `no-numbers-in-binaries`, `no-parameter-bags`, `primitive-boundary`,
  `private-fns`, `public-surface`, `state-graph`, `test-budget`, and `type-shape`, `cargo xtask wasm`,
  `cargo xtask mutants`,
  `cargo xtask magics`). Lints return a `Report` of
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
- Modules nest by use. A file whose only users outside its parent all sit inside one sibling
  module belongs inside that sibling, as `bitboard/square_iter.rs` or `search/negamax/window/`.
  A `use` path never reaches past a module directory's `mod.rs` into its children from outside;
  import what the directory re-exports. A module directory re-exports at most 8 names and a
  crate root is exempt. `cargo xtask module-nesting` enforces all three.
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
- A crate root re-exports exactly what another crate names. An export with no user outside its
  crate is removed, and added back the day a user appears. A crate no other crate depends on
  yet has no surface to check. `cargo xtask public-surface` enforces it.
- A struct with no logic of its own (only constructors, field accessors, and `..self` updaters,
  no trait impls) that exactly one other file names is a parameter bag carrying values between
  that file's methods. Give it the logic that consumes it. `cargo xtask no-parameter-bags`
  enforces it.
- Primitives only at the boundary. Numbers and text are wire forms; a value crosses into one at a
  type's door and never waits there. A primitive number type (`u8`..`u128`, `i8`..`i128`, `f32`,
  `f64`) appears in a field, variant, const, or signature only in
  the file of a newtype over it (`Depth(u8)`, `NodeCount(u64)`, `Score(i32)`) or in a bit-boundary
  file (`square.rs`, `direction.rs`, `slider/magic.rs`, `slider/magics.rs`,
  `slider/attack_table.rs`, `zobrist_keys/split_mix.rs`, `xtask/src/task/magics/`). Locals and
  turbofish (`token.parse::<u64>()`) are the crossing itself and are exempt. A `usize` is a
  container size or index: in `crates/` it may be declared only under a name that says so
  (`length`, `capacity`, `size`, `bytes`, `index`); a `usize` named after what it counts is a
  count and gets a newtype (`GameCount`, `RoundCount`, `PlyCount`). Text is inspected
  (`split_*`, `strip_*`, `trim`, `chars`) or produced (`format!`, `to_string`, `String::from`)
  only in a file that hand-writes a `FromStr`, `TryFrom`, or `Display` impl, or as the direct
  argument of an I/O write; calling `.parse()` anywhere is using a door, not building one. The
  text clause covers `crates/`; xtask's values are text. `cargo xtask primitive-boundary`
  enforces both.
- A binary is wiring. Every file reached from a `main.rs` under `crates/` is free of numeric
  literals outside tests: a number a binary needs is a setting, read from its config file, whose defaults are
  data in an embedded `.default.toml` rather than code. `cargo xtask no-numbers-in-binaries`
  enforces it.
- Small PRs: one concept each. Split anything that needs more than one idea to review.
- Zero comments in Rust code. This includes `//`, `/* */`, and doc comments. `cargo xtask no-comments` enforces it in CI. Use clear names and small functions instead.
- No free functions. Every `fn` is a method or associated function of a struct, enum, or trait;
  the only exceptions are `main` and `#[test]` functions. `cargo xtask no-free-fns` enforces it.
- Lifetimes are named with a word (`'board`, `'scan`, `'ast`), never a single letter.
  `cargo xtask named-lifetimes` enforces it; `'_` and `'static` are exempt.
- Function names say what the function does. No name under three letters and none from the
  vague list (`of`, `with`, `get`, `set`, `make`, `check`, `build`, `handle`, `process`,
  `helper`, `data`, `value`, and the like); `new` is the one conventional exception, and trait
  impl methods take their name from the trait. `cargo xtask literal-names` enforces it.
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
- A method does its own work. A body that is one call handing the method's own inputs to a type
  declared in this workspace is a forwarder: expose the part instead, or call the target
  directly. Newtypes may still name a foreign type's method (`Bitboard::count` over
  `u64::count_ones`). `cargo xtask no-forwarders` enforces it.
- No two non-`pub` functions on one type share a signature (generics, receiver, parameter types,
  return type). Two such helpers mean a value is missing its type: `alpha` and `beta` both returning
  `Score` became `Bound<Lower>` and `Bound<Upper>`. `cargo xtask distinct-signatures` enforces it.
- Invariants live as high as possible: a type that cannot represent the invalid state, else a
  `const _: () = assert!(..)` at compile time, else a test. A test must fail for a reason no type,
  const assertion, or other test catches. `cargo xtask test-budget` enforces the budget: at most
  3 tests per file, 3 assertion sites, 20 lines, and 4 literals per test, no integer literal
  above 64 in test code. Prefer one exhaustive or oracle test over examples; randomness comes
  from `proptest`; deep checks are `#[ignore]` and run outside the PR gate.
- Tests are judged by the mutants they kill. `cargo xtask mutants` runs `cargo-mutants` on the
  lines a PR changes against `origin/main`, with the workspace's tests, and every mutant in
  the diff must be caught or unviable. Tables of literals (`slider/magics.rs`, the
  piece-square `placement_table.rs`) and `xtask` are excluded in `.cargo/mutants.toml`. Full
  crate runs are slow and memory-heavy, and two at once once crashed the owner's laptop, so
  `cargo xtask mutants` is the only way to run them: it takes an exclusive lock, refuses if a
  `cargo-mutants` process already exists or less memory is free than its compiler tasks need
  (2 GiB each plus 2), and caps the run at 2 jobs and half the cores of compiler tasks. The session hook denies a raw `cargo mutants`.
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
