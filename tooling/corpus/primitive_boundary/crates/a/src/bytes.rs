pub struct Bytes<'bytes>(&'bytes [u8]); impl Bytes<'_> { pub fn first(&self) -> u8 { self.0[0] } }
