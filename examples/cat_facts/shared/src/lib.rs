pub use app::*;
use crux_core::Core;
pub use effects::{Effect, Outcome};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod effects;
// TODO hide this plumbing

uniffi_macros::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Core<Effect, Outcome, CatFacts> = Core::new();
}

#[wasm_bindgen]
pub fn message(data: &[u8]) -> Vec<u8> {
    CORE.message(data)
}

#[wasm_bindgen]
pub fn response(data: &[u8]) -> Vec<u8> {
    CORE.response(data)
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    CORE.view()
}
