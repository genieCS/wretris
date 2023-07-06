use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    EMPTY = 0,
    HINT = 1,
    WARNING = 2,
    I = 3,
    O = 4,
    T = 5,
    S = 6,
    Z = 7,
    J = 8,
    L = 9,
}
