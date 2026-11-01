# chess

A Rust chess engine with a neural network evaluation trained using [bullet](https://github.com/jw1912/bullet), and a terminal UI to play against it.

## Development

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
lint .
```

`lint` is the runner from [meta-dev](https://github.com/Studio-Terza/meta-dev), at the revision in `.github/workflows/ci.yml`. `cargo run -p magics` regenerates the magic tables.
