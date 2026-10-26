pub struct Block([u8; 1]); impl Block { pub fn only(&self) -> u8 { self.0[0] } }
