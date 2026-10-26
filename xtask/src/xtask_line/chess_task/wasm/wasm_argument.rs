use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum WasmArgument {
    #[strum(serialize = "--target")]
    Target,
    #[strum(serialize = "wasm32-unknown-unknown")]
    Wasm32,
}
