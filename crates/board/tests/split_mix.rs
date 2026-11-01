use board::SplitMix64;

const REFERENCE_STREAM: [&str; 3] = ["e220a8397b1dcdaf", "6e789e6aa1b965f4", "6c45d188009454f"];

#[test]
fn seed_zero_reproduces_the_published_splitmix64_stream() {
    let stream = REFERENCE_STREAM
        .iter()
        .scan(SplitMix64::new(0), |generator, _| {
            *generator = generator.next();
            Some(format!("{:x}", generator.output()))
        });
    assert!(stream.eq(REFERENCE_STREAM.iter().map(ToString::to_string)));
}
