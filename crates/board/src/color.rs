use core::ops::Not;

use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr, VariantArray)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        self.opposite()
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn each_color_is_the_opposite_of_the_other() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
}
