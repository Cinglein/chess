# chess

A Rust chess engine with a neural network evaluation trained using [bullet](https://github.com/jw1912/bullet), and a terminal UI to play against it.

## Development

```
cargo xtask ci
cargo xtask wasm
```

Runs formatting, clippy, tests, the lints, and the wasm build exactly as CI does.
