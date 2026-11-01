use std::fmt;

pub struct HexLiteral(u64);

impl From<u64> for HexLiteral {
    fn from(value: u64) -> HexLiteral {
        HexLiteral(value)
    }
}

impl fmt::Display for HexLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = format!("{:016x}", self.0);
        let groups: Vec<&str> = digits
            .as_bytes()
            .chunks(4)
            .map(|chunk| std::str::from_utf8(chunk).expect("hex digits are ascii"))
            .collect();
        write!(formatter, "0x{}", groups.join("_"))
    }
}
