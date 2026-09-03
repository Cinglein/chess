# 01: Squares and pieces

Milestone 1, PR 1. The `board` crate starts with the vocabulary every later type is built
from: where a square is, and what a piece is.

## Why start here

An engine asks "which squares does this rook attack?" and "is the king in check?" millions
of times per second. Every fast answer to those questions rests on a fixed numbering of the
64 squares, because the next PR turns "a set of squares" into a single 64-bit integer. This
PR fixes the numbering and gives it types.

## Square numbering

Squares are numbered `a1 = 0, b1 = 1, ..., h1 = 7, a2 = 8, ..., h8 = 63`: rank by rank,
starting from white's left. So:

```
square = rank * 8 + file
file   = square % 8
rank   = square / 8
```

Moving north is `+8`, east is `+1`, north-east is `+9`. Every bit shift in the next PR
relies on exactly this layout, so it is worth committing to memory.

## Enums, not integers

`File`, `Rank`, and `Square` are enums with 8, 8, and 64 variants. A function that takes a
`Square` cannot be handed a piece count or a move by mistake, and a `match` on a `Square` is
checked for completeness by the compiler.

Each enum has an `ALL` array in index order and an `index` method. Together they replace
integer casts: `Square::ALL[i]` gets from a `usize` back to a `Square` with a bounds check,
and `square.index()` goes the other way. `from_index` wraps the bounds check in an `Option`
for callers with untrusted input.

Everything is `const fn`, so later PRs can build lookup tables at compile time with plain
loops.

## Stepping between squares

`Square::translate(file_delta, rank_delta)` is the one place coordinate arithmetic happens.
It returns `None` when the step leaves the board, using `checked_add_signed` so there is no
signed-to-unsigned cast to get wrong. `Square::offset(Direction)` is the eight-way special
case that ray walking will use for sliding pieces.

## Pieces

`Color` has two variants and an `opposite`. `PieceKind` has six. `Piece` pairs them. The
letters (`P N B R Q K` for white, lowercase for black) are the ones FEN uses, so the FEN
parser in a later PR reads straight off `Piece::from_letter`.

## What to read in the diff

1. `crates/board/src/square.rs`: the three enums, `Square::translate`, `Display` and
   `FromStr` for algebraic notation like `e4`.
2. `crates/board/src/piece.rs`: `Color`, `PieceKind`, `Piece`, and the FEN letters.
3. The tests at the bottom of each file. Their names state the property they demonstrate.

## An experiment

In a scratch test or binary:

```rust
use board::{Direction, Square};

let square: Square = "e4".parse().unwrap();
println!("{} is index {}", square, square.index());
println!("{:?}", square.offset(Direction::NorthEast));
println!("{:?}", Square::H8.offset(Direction::NorthEast));
```

Then try `"e9".parse::<Square>()` and read the error. Change the order of variants in
`Square` and watch `every_square_roundtrips_through_its_index_and_coordinates` fail: the
numbering is load-bearing.
