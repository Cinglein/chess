use magics::HexLiteral;
#[test]
fn hex_literals_are_grouped_in_fours_for_readability() {
    assert_eq!(
        HexLiteral::from(u64::MAX).to_string(),
        "0xffff_ffff_ffff_ffff"
    );
    assert_eq!(HexLiteral::from(1).to_string(), "0x0000_0000_0000_0001");
}
