# 01: Bitboards and attack tables

Milestone 1, PR 1. This note explains the ideas behind the first real code in the `board`
crate. Read it before the diff; the diff then reads as an implementation of these ideas.

## The problem

Everything a chess engine does, it does millions of times per second: "which squares does this
rook attack?", "is the king in check?", "where can this knight go?". A naive board is an
8x8 array of pieces and answering those questions means looping over squares. The engine
would spend all its time in those loops.

## The idea: one bit per square

A chessboard has 64 squares and a `u64` has 64 bits. If bit `i` means "square `i` is in the
set", a `u64` is a set of squares, and the CPU's bitwise instructions become set operations
that finish in a single cycle:

| Set operation           | Bit operation   | In this crate                 |
|-------------------------|-----------------|-------------------------------|
| union                   | `a \| b`        | `a.union(b)` or `a \| b`      |
| intersection            | `a & b`         | `a.intersection(b)` or `a & b`|
| difference              | `a & !b`        | `a.difference(b)`             |
| complement              | `!a`            | `a.complement()` or `!a`      |
| membership              | `a & (1 << i)`  | `a.contains(square)`          |
| size                    | popcount        | `a.count()`                   |
| smallest element        | trailing zeros  | `a.first()`                   |
| move every square north | `a << 8`        | `a.shift(Direction::North)`   |

That `u64` is a **bitboard**. The `Bitboard` type is a newtype around `u64` so the compiler
stops us from mixing it up with hashes, move encodings, or other plain integers.

A position will later be stored as a handful of bitboards: one per piece kind per colour
(white pawns, white knights, ..., black king) plus the union of each colour. "All pieces" is
the union of two bitboards. "Empty squares" is the complement. These come in PR 2.

### Square numbering

Squares are numbered `a1 = 0, b1 = 1, ..., h1 = 7, a2 = 8, ..., h8 = 63`: rank by rank,
starting from white's left. So `square = rank * 8 + file`, `file = square % 8`,
`rank = square / 8`. Moving north is `+8`, east is `+1`. This is the layout every shift in
`Bitboard::shift` relies on.

The `Square`, `File`, and `Rank` types are enums rather than integers, so a function that
takes a `Square` cannot be handed a move or a piece count by accident. `Square::ALL` lists
every square in index order, which is how a `usize` gets back to a `Square` without an
unchecked cast.

### Shifts must not wrap

Shifting a bitboard left by one moves every square east, except that h-file squares would
wrap onto the a-file of the next rank. `Bitboard::shift` masks the h-file off before shifting
east and the a-file off before shifting west. The test
`every_shift_agrees_with_stepping_each_square` checks all 64 squares in all 8 directions
against `Square::offset`, which does the arithmetic on file and rank explicitly.

### Iterating a bitboard

`for square in bitboard` visits squares from `a1` towards `h8`. Each step finds the lowest
set bit with `trailing_zeros` and clears it with `bits & (bits - 1)`. This is how move
generation will later loop over the knights, then over each knight's targets.

## Attack tables

Knights, kings, and pawns move a fixed pattern from any square. Their attacks depend only on
the square, so we compute them once into a 64-entry table and look them up. The tables in
`attacks/leapers.rs` are built at compile time by `const fn` loops; the compiler evaluates
them and bakes the results into the binary.

Sliding pieces (rook, bishop, queen) are harder: their attacks depend on which other squares
are occupied, because pieces block them. A rook on d4 with a blocker on d6 attacks d5 and d6
but not d7 or d8. There are 2^64 possible occupancies, so a plain table is out.

### Relevant occupancy

Only squares along the piece's lines matter, and the last square in each direction never
matters either, because the rook attacks it whether or not it is occupied. The squares that
do matter are the **relevant occupancy** mask, `Slider::relevant_occupancy`. A rook has at
most 12 relevant squares (on a corner), a bishop at most 9 (in the centre). That is 2^12 =
4096 possible occupancies per square at most, a table we can afford: 102,400 rook entries
and 5,248 bishop entries in total.

### Magic bitboards

We still need to turn a masked occupancy (a 64-bit value with up to 12 scattered bits) into a
table index from 0 to 4095. **Magic bitboards** do this with a multiplication:

```
index = (occupancy & mask) * magic >> (64 - bits)
```

Multiplying by a well-chosen 64-bit constant, the "magic number", scatters the relevant bits
so that the top `bits` bits of the product are distinct for every occupancy that produces a
different attack set. Two occupancies may share an index only if they yield the same attacks,
which happens often (the pieces behind the first blocker do not matter) and is harmless.

There is no formula for magic numbers. `cargo xtask magics` finds them by trial: pick a
random sparse 64-bit number, run every occupancy of the mask through the formula, and check
that no two occupancies with different attack sets collide. Most candidates fail; one usually
works within a few thousand tries. The finder is seeded, so rerunning it produces the same
file. The numbers are checked into `crates/board/src/attacks/magics.rs`.

### Enumerating occupancies

The finder and the tests need every subset of a mask. The trick in `Bitboard::subset_after`
is `(subset - mask) & mask`: subtracting the mask borrows through the mask's bits like a
binary counter that skips the bits outside the mask. Starting from the empty set it visits
every subset exactly once and returns to empty after the last one. The test
`subsets_enumerate_every_combination_of_a_mask_once` shows the property.

### Filling the table: a decision

The scope originally said the sliding tables would be built at compile time from the magics,
like the leaper tables. I tried it. The rook table is 102,400 entries, each computed by
walking rays, and the compiler's constant evaluator is an interpreter running roughly a
million operations per second. Building the table took 39 seconds on every compile of the
crate, and the compiler emitted a warning about long-running constant evaluation that it
repeats as it goes.

The tables are instead **memoised at runtime**: they start as zeroed `AtomicU64` arrays in the
binary's zero-initialised data (so they cost nothing to load, which also matters for the web
build), and each lookup that finds a zero computes the attacks by walking rays and stores
them. A sliding piece always attacks at least one square, so zero can never be a real entry
(`sliding_attacks_are_never_empty_so_zero_marks_an_unfilled_slot` pins this down). The
atomics make the racing writes from several search threads well defined; they all write the
same value.

The cost is one predictable branch per lookup and slow first calls. The alternative was a
build script generating source for the tables, which is a common engine approach but needs
the ray-walking code shared between the build script and the crate. If the memoisation ever
shows up in a profile, that is the fallback.

### The magic numbers are verified, not trusted

`magic_lookups_match_ray_walking_for_every_relevant_occupancy` runs all 107,648 occupancies
through the tables and compares with straightforward ray walking. If someone edits a magic
number by hand, this test fails.

## Moves

A `Move` packs into 16 bits: 6 for the origin square, 6 for the destination, 2 for the kind
(normal, promotion, en passant, castling), 2 for the promotion piece. Small moves matter
because the search stores millions of them in move lists and hash tables. `Promotion` is its
own four-variant enum so an "e7e8 promoting to king" move cannot be represented.

## What to read in the diff

1. `crates/board/src/square.rs`: the coordinate types and `Square::translate`.
2. `crates/board/src/bitboard.rs`: the set operations, `shift`, the iterators.
3. `crates/board/src/attacks/leapers.rs`: compile-time tables in twenty lines.
4. `crates/board/src/attacks/sliding.rs`: `relevant_occupancy`, `Magic::index`, `lookup`.
5. `xtask/src/magics.rs`: the finder.
6. The tests in each file, named after the property they show.

## An experiment

Print a few bitboards to see the mapping. Add this to any test, or run it in a scratch
binary:

```rust
use board::attacks;
use board::{Bitboard, Square};

let blockers = Bitboard::from_square(Square::D6).with(Square::F4);
println!("{}", attacks::rook(Square::D4, blockers));
println!("{}", attacks::bishop(Square::D4, blockers));
println!("{}", attacks::queen(Square::D4, blockers));
```

Then change the seed in `xtask/src/magics.rs`, run `cargo xtask magics`, and watch the whole
magics file change while `cargo test --package board` still passes. Any collision-free set of
numbers works; the ones checked in are not special.
