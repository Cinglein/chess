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
- `tooling/task`: the library behind xtask: `SourceFile`, `Report`, `Violation`, `Site`, `Failure`,
  `Policy` from `tooling/policy.toml`, `Workspace`, `CommandLine`, and the shared scanners. Tooling
  crates live under `tooling/` (lints under `tooling/lints/`), one crate per lint or task, so the
  affected graph, the public surface, and the mutation run judge each on its own.
- `crates/settings`: `std` settings loading shared by the binaries: `EmbeddedDefaults` reads a toml
  file or writes the embedded defaults when it is missing, and `SettingsError` names what went wrong.
- `crates/datagen`: `std` training data binary. `SelfPlay` plays the in-process engine against
  itself at fixed nodes from `RandomOpenings` on worker threads and streams every `Finished`
  game over a channel; `Example` turns each `Label` into bullet's 32-byte `ChessBoard` record
  through the `bulletformat` crate; `datagen.toml` holds nodes per move, opening plies, the
  position target, and concurrency, and `--seed` defaults to the clock.
- `crates/trainer`: `std` training binary over bullet, pinned by git revision. `ShuffledData` writes a
  seeded permutation of the data file beside it; `Schedule` and `Machine` turn `trainer.toml` into
  bullet's `TrainingSchedule` and `LocalSettings`; `Network::trainer` builds the
  `(768 -> HIDDEN)x2 -> 1` graph from `eval::NetworkFormat`, which holds the sizes, quantisation
  factors, and eval scale that the engine's network evaluator shares. The `metal` feature is
  selected by target in the manifest, so CI compiles the crate against bullet's mock device.
- Crates are `no_std` unless the feature they exist for needs `std`. Planned: `web` (Dioxus,
  wasm). Crates are added when their milestone starts.
- `xtask`: repository tooling (`cargo xtask ci`, `cargo xtask lint`, which parses every file once
  and runs `const-shape`, `distinct-signatures`, `expression-flow`, `fn-shape`, `literal-names`,
  `manual-iteration`, `module-nesting`, `named-lifetimes`, `named-numbers`, `no-comments`,
  `no-forwarders`, `no-free-fns`, `no-numbers-in-binaries`, `no-parameter-bags`,
  `primitive-boundary`, `private-fns`, `public-surface`, `settings-shape`, `state-graph`,
  `test-budget`, and `type-shape`, `cargo xtask wasm`,
  `cargo xtask clippy`, `cargo xtask test`, `cargo xtask test-time`, `cargo xtask stacked`,
  `cargo xtask affected`, which prints the crates a change reaches as changed crates plus their
  reverse dependents, and `--affected` on `ci`, `clippy`, `wasm`, `test`, `test-time`, and
  `mutants-full`, which restricts them to that set, `cargo xtask mutants`,
  `cargo xtask mutants-full`, `cargo xtask mutant-costs <directory>`, which prints the
  `[mutant_millis]` table measured from the shard artifacts' `outcomes.json` under a directory,
  `cargo xtask magics`). Lints return a `Report` of
  `Violation`s at a `Site`; every xtask error is a `Failure` variant.

## Rules

- All changes land through pull requests. `main` is protected; never push to it directly.
- A chain of dependent PRs is a GitHub stack. Link the branches bottom to top with
  `gh stack link <bottom> ... <top>` (or `gh stack submit`), never `gh pr create --base <branch>`.
  The Stack workflow runs `cargo xtask stacked` and fails any PR whose base is not `main` and
  that belongs to no stack; the session hook denies the `--base` form.
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
  Fixed words are variants, never literals: xtask builds every cargo, git, gh, and test-binary
  invocation as a `CommandLine` of typed arguments (`CargoArgument`, `GitArgument`, `MutantsArgument`, and the
  rest) plus `Display` values such as a `CrateName`, and `Workspace::line` roots it.
- Index tables by enum with `enum_map::EnumMap`, never by an integer method on the enum. An `as`
  cast is allowed only inside a `const fn`, `const` item, or `const` block, or in a bit boundary
  file (the one list in `tooling/task/src/bit_boundary.rs`); elsewhere use `From`, `TryFrom`,
  or an `EnumMap`. Loops in const context walk a slice with
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
- A crate root re-exports exactly what another crate names. A crate's own binary target is
  another crate: what its `main.rs` tree names counts. An export with no user outside the library
  is removed, and added back the day a user appears. A library no other crate depends on yet has
  no surface to check. `cargo xtask public-surface` enforces it.
- A struct with no logic of its own (only constructors, field accessors, and `..self` updaters,
  no trait impls; a method that builds a value of another type is logic) that exactly one other
  file names is a parameter bag carrying values between that file's methods. Give it the logic that consumes it. `cargo xtask no-parameter-bags`
  enforces it.
- Primitives only at the boundary. Numbers and text are wire forms; a value crosses into one at a
  type's door and never waits there. A primitive number type (`u8`..`u128`, `i8`..`i128`, `f32`,
  `f64`) appears in a field, variant, const, or signature only in
  the file of a newtype over it (`Depth(u8)`, `NodeCount(u64)`, `Score(i32)`) or in a bit-boundary
  file (`BitBoundary` in `tooling/task`: `square/`, `bitboard/`, `direction.rs`, `slider/magic.rs`,
  `slider/magics.rs`, `slider/attack_table.rs`, `zobrist_keys/split_mix.rs`, `tooling/magics/src/`;
  const-shape, primitive-boundary and named-numbers share it). Locals and
  turbofish (`token.parse::<u64>()`) are the crossing itself and are exempt. A `usize` is a
  container size or index: in `crates/` it may be declared only under a name that says so
  (`length`, `capacity`, `size`, `bytes`, `index`); a `usize` named after what it counts is a
  count and gets a newtype (`GameCount`, `RoundCount`, `PlyCount`). Text is inspected
  (`split_*`, `strip_*`, `trim`, `chars`) or produced (`format!`, `to_string`, `String::from`)
  only in a file that hand-writes a `FromStr`, `TryFrom`, or `Display` impl, or as the direct
  argument of an I/O write; calling `.parse()` anywhere is using a door, not building one. The
  text clause covers `crates/`; xtask's values are text. A wire type is a primitive for this
  rule: `serde_json::Value`, `toml::Value`, `serde_yaml::Value`, `rust_decimal::Decimal`,
  `bigdecimal::BigDecimal`, `num_bigint::BigInt` and `BigUint`, and `Any` (the list is
  `wire_types` in `tooling/policy.toml`, matched by full path through the file's `use` tree)
  appear only in the file of a newtype over them. `cargo xtask primitive-boundary`
  enforces both.
- A binary is wiring. Every file reached from a `main.rs` under `crates/` is free of numeric
  literals outside tests: a number a binary needs is a setting, read from its config file, whose defaults are
  data in an embedded `.default.toml` rather than code. `cargo xtask no-numbers-in-binaries`
  enforces it. A settings struct lives in the binary, implements `settings::EmbeddedDefaults`
  with its default file, and every struct that derives `Deserialize` with named fields carries
  `#[serde(deny_unknown_fields)]`, so a misspelt key is an error; `cargo xtask settings-shape`
  enforces that for every struct reachable through the fields of an `EmbeddedDefaults`
  implementor, which is how it tells a file we write from an external schema such as `cargo
  metadata`. xtask is such a binary: its lint budgets are settings in `tooling/policy.toml`, read
  through `Policy`, and the xtask package is a library of tasks plus a `main.rs` that only parses
  the command line, so every lint applies to it.
- A number has a name. In `crates/`, outside tests, test support files, and the bit boundary
  files, a numeric literal other than 0 and 1 appears only inside a `const` item, so every
  tunable can be found by name and moved to a setting or a parameter when its value is measured.
  Literals inside macro arguments count. `cargo xtask named-numbers` enforces it.
- Memory budget: anything this session runs on the owner's laptop stays under 8 GiB, because
  the laptop carries other work. `.cargo/config.toml` caps every cargo build at 3 compiler jobs,
  the mutation task derives its caps from the same 8 GiB (`granted_gib` in `tooling/policy.toml`),
  and long runs go one at a time. More needs the owner's explicit permission, given per run.
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
- Control flow is an expression. Outside tests and `const fn`s, a body has no
  `return`, `break`, `continue`, `loop`, `for`, or `while`; every `if` has an `else`, and every
  `if` or `match` yields a value that is used. Early exit goes through `?` on a `Result` or an
  `Option`; a loop is an iterator driven by combinators or a `try_fold` over `ControlFlow`. No
  assignment to a local, through its fields or indices, plain or compound, and no `let mut` that
  is only reassigned: a mutable local is borrowed as `&mut` or has methods called on it, and a
  patched copy is an updater method on its type (`Placement::toggled`). Writes through `self` or
  through a `&mut` slot (`*slot = entry`) are the method side of mutation and stay.
  `cargo xtask expression-flow` enforces it across the whole repository, `syn` visitors
  included: a visitor decides through a pure method that returns an `Option` or a mode such
  as `Walk`, and the visit method is one expression that extends or matches on it.
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
- No two functions on one type share a signature (generics, receiver, parameter types, return
  type), public or not. Two such functions mean a value is missing its type or the part they
  differ by is missing its type parameter: `alpha` and `beta` both returning `Score` became
  `Bound<Lower>` and `Bound<Upper>`, and `remaining` and `increment` on `Clock` became
  `time::<Remaining>` and `time::<Increment>`. Prefer a zero-sized type parameter over a runtime
  enum, except where a `const fn` must dispatch, as `Bitboard::combined(other, SetOperation)`
  does. `cargo xtask distinct-signatures` enforces it.
- Invariants live as high as possible: a type that cannot represent the invalid state, else a
  `const _: () = assert!(..)` at compile time, else a test. A test must fail for a reason no type,
  const assertion, or other test catches. `cargo xtask test-budget` enforces the budget: at most
  3 tests per file, 3 assertion sites, 20 lines, and 4 literals per test, no integer literal
  above 64 in test code, at most 40 tests per 100 files, and test code at most 18% of all lines,
  where a file declared by a `#[cfg(test)] mod` counts whole. The lint reads inside `proptest!`
  closures, and every assertion sits in a `#[test]` fn: helpers return values and the test
  asserts them. `cargo xtask test-time` runs each test
  alone and enforces the time budget: at most 1 s per test and 4 s for the whole suite. Prefer
  one exhaustive or oracle test over examples; randomness comes from `proptest`; deep checks
  are `#[ignore]` and run outside the PR gate.
- Tests are judged by the mutants they kill. `cargo xtask mutants` runs `cargo-mutants` on the
  lines a PR changes against `origin/main`, tested by the changed crates and the dependents
  that have tests, in three parallel slices (`MUTANTS_SLICE=k/n`), and every mutant in
  the diff must be caught, unviable, or a timeout (a mutant that hangs a test is a mutant the
  tests noticed), so no PR adds debt. `cargo xtask mutants-full` mutates the
  whole workspace, or one `MUTANTS_SHARD`: `board/0/5` is one slice of a crate, `eval+fen` is a
  set of whole crates, and a shard runs only the tests of its crates and of the dependents that
  have tests, so a lint mutant never relinks xtask. The Mutants workflow costs each crate's mutants
  by the measured `mutant_millis` for that crate (unlisted crates take the default), slices a
  crate that costs more than `per_shard` (every crate also pays `per_crate` for its own discovery and
  baseline), packs the rest by cost, largest first, and runs each crate of a shard as its own
  `cargo-mutants` run so no mutant pays for another crate's tests; the shards run on every push
  to `main`, on the Monday schedule, and on demand, never on a pull request: a PR is gated by
  the in-diff run, the whole-workspace run costs about 4300 runner-seconds and cannot fit the
  PR budget on 20 runners, and a miss it finds on `main` gets its own PR (when a shard runs long, download
  that run's shard artifacts and refresh the table with `cargo xtask mutant-costs <directory>`). A missed mutant anywhere
  fails the shard. A mutant no test can
  observe, such as a destructor, is named one by one in `exclude_re`. Tables of literals (`slider/magics.rs`, the
  piece-square `placement_table.rs`) and `xtask` are excluded in `.cargo/mutants.toml`. Full
  crate runs are slow and memory-heavy, and two at once once crashed the owner's laptop, so
  `cargo xtask mutants` is the only way to run them: it takes an exclusive lock, refuses if a
  `cargo-mutants` process already exists or the granted budget is not free, and runs in place,
  one mutant at a time, with the compiler tasks the budget allows (3). The session hook denies a raw `cargo mutants`.
- No documentation in the repository: no `docs/`, no notes, no design documents. The README
  stays a few lines. Anything the owner should read goes in the chat.
- CI must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` with the pedantic
  group enabled, `cargo test`, `cargo xtask wasm`, and `cargo xtask lint` as one CI job covering
  comments, free functions, the test budget, and the state graph. Run `cargo xtask ci` locally
  before opening a PR. A PR's checks finish inside 5 minutes, ideally 1: `cargo xtask test`
  runs `--lib` because every test lives in a library, the in-diff mutants job is cut at 6 minutes
  and a shard at 8, and anything slower is a bug to fix, not a timeout to raise. CI runs only
  what a change reaches: every CI job runs its task with `--affected` on a pull request and does
  nothing when no crate it covers changed; the Mutants workflow's `affected` job publishes the
  plan from `cargo xtask affected`, one entry per job (`fmt`, `clippy`, `test`, `lint`,
  `mutants`, `full`) with that job's crates and, for `full`, the packed shards. A crate change reaches every job over the crate and its dependents. An xtask change
  reaches `fmt`, `lint`, and xtask's own `clippy` and `test` always, and `mutants` or `full`
  only when the changed file is in the `use` and `mod` closure of that job's entry files
  (`task/mutants/mod.rs`; `task/affected/mod.rs`, `main.rs`, `lib.rs`, `task/mod.rs`, and
  `Cargo.toml` reach every job); a tool rerun covers every crate. Pushes to `main` and the Monday
  schedule run everything, including the Mutants workflow, and `ci-gate` (on a PR) and
  `mutants-gate` (on `main`) always run and fail if any job they need failed, so `ci-gate` is
  the check to require.
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
