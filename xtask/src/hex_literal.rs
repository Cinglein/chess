use std::fmt;

pub struct HexLiteral(pub u64);

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

#[cfg(test)]
mod tests {
    use super::HexLiteral;

    #[test]
    fn hex_literals_are_grouped_in_fours_for_readability() {
        assert_eq!(HexLiteral(u64::MAX).to_string(), "0xffff_ffff_ffff_ffff");
        assert_eq!(HexLiteral(1).to_string(), "0x0000_0000_0000_0001");
    }
}
