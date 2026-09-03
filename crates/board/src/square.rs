use core::fmt;
use core::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const ALL: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Option<File> {
        if index < Self::ALL.len() {
            Some(Self::ALL[index])
        } else {
            None
        }
    }

    #[must_use]
    pub const fn from_char(letter: char) -> Option<File> {
        match letter {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }

    #[must_use]
    pub const fn char(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub const ALL: [Rank; 8] = [
        Rank::One,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Option<Rank> {
        if index < Self::ALL.len() {
            Some(Self::ALL[index])
        } else {
            None
        }
    }

    #[must_use]
    pub const fn from_char(digit: char) -> Option<Rank> {
        match digit {
            '1' => Some(Rank::One),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            _ => None,
        }
    }

    #[must_use]
    pub const fn char(self) -> char {
        match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub const ORTHOGONAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIAGONAL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    #[must_use]
    pub const fn file_delta(self) -> isize {
        match self {
            Direction::North | Direction::South => 0,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::SouthWest | Direction::West | Direction::NorthWest => -1,
        }
    }

    #[must_use]
    pub const fn rank_delta(self) -> isize {
        match self {
            Direction::East | Direction::West => 0,
            Direction::North | Direction::NorthEast | Direction::NorthWest => 1,
            Direction::South | Direction::SouthEast | Direction::SouthWest => -1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub const COUNT: usize = 64;

    pub const ALL: [Square; Self::COUNT] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];

    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Square {
        Self::ALL[rank.index() * 8 + file.index()]
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Option<Square> {
        if index < Self::COUNT {
            Some(Self::ALL[index])
        } else {
            None
        }
    }

    #[must_use]
    pub const fn file(self) -> File {
        File::ALL[self.index() % 8]
    }

    #[must_use]
    pub const fn rank(self) -> Rank {
        Rank::ALL[self.index() / 8]
    }

    #[must_use]
    pub const fn translate(self, file_delta: isize, rank_delta: isize) -> Option<Square> {
        let Some(file) = self.file().index().checked_add_signed(file_delta) else {
            return None;
        };
        let Some(rank) = self.rank().index().checked_add_signed(rank_delta) else {
            return None;
        };
        match (File::from_index(file), Rank::from_index(rank)) {
            (Some(file), Some(rank)) => Some(Self::new(file, rank)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn offset(self, direction: Direction) -> Option<Square> {
        self.translate(direction.file_delta(), direction.rank_delta())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseSquareError;

impl fmt::Display for ParseSquareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected a file letter a-h followed by a rank digit 1-8")
    }
}

impl core::error::Error for ParseSquareError {}

impl fmt::Display for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.char())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.char())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.file(), self.rank())
    }
}

impl FromStr for Square {
    type Err = ParseSquareError;

    fn from_str(text: &str) -> Result<Square, ParseSquareError> {
        let mut chars = text.chars();
        let file = chars.next().and_then(File::from_char);
        let rank = chars.next().and_then(Rank::from_char);
        match (file, rank, chars.next()) {
            (Some(file), Some(rank), None) => Ok(Square::new(file, rank)),
            _ => Err(ParseSquareError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, File, Rank, Square};

    #[test]
    fn squares_are_numbered_rank_by_rank_from_a1() {
        assert_eq!(Square::A1.index(), 0);
        assert_eq!(Square::H1.index(), 7);
        assert_eq!(Square::A2.index(), 8);
        assert_eq!(Square::H8.index(), 63);
        assert_eq!(Square::new(File::E, Rank::Four), Square::E4);
        assert_eq!(Square::E4.file(), File::E);
        assert_eq!(Square::E4.rank(), Rank::Four);
    }

    #[test]
    fn every_square_roundtrips_through_its_index_and_coordinates() {
        for (index, square) in Square::ALL.iter().enumerate() {
            assert_eq!(square.index(), index);
            assert_eq!(Square::from_index(index), Some(*square));
            assert_eq!(Square::new(square.file(), square.rank()), *square);
        }
        assert_eq!(Square::from_index(64), None);
    }

    #[test]
    fn squares_display_and_parse_in_algebraic_notation() {
        assert_eq!(Square::E4.to_string(), "e4");
        assert_eq!("e4".parse(), Ok(Square::E4));
        assert_eq!("h8".parse(), Ok(Square::H8));
        assert!("e9".parse::<Square>().is_err());
        assert!("i1".parse::<Square>().is_err());
        assert!("e".parse::<Square>().is_err());
        assert!("e44".parse::<Square>().is_err());
    }

    #[test]
    fn stepping_off_the_board_yields_none() {
        assert_eq!(Square::E4.offset(Direction::North), Some(Square::E5));
        assert_eq!(Square::E4.offset(Direction::SouthWest), Some(Square::D3));
        assert_eq!(Square::A1.offset(Direction::West), None);
        assert_eq!(Square::A1.offset(Direction::South), None);
        assert_eq!(Square::H8.offset(Direction::NorthEast), None);
        assert_eq!(Square::H4.translate(1, 2), None);
        assert_eq!(Square::B1.translate(1, 2), Some(Square::C3));
    }
}
