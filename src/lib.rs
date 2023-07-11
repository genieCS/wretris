mod utils;
mod block;
mod color_grid;
mod backend;
mod board;
mod tetris;

use crate::tetris::Tetris;
use cursive::{
    self,
    view::{Nameable, Selector},
};
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement,
};
use std::sync::Mutex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Cursive {
    backend: Mutex<cursive::Cursive>,
}

#[wasm_bindgen]
impl Cursive {
    #[wasm_bindgen(js_name = "retris")]
    pub fn retris(canvas: HtmlCanvasElement) -> Cursive {
        alert("Hello, wasm-retris!");
        let mut siv: cursive::Cursive = cursive::Cursive::new();
        let tetris = Tetris::new().with_name("retris");
        siv.add_layer(tetris);
        siv.focus(&Selector::Name("retris")).unwrap();
        siv.set_fps(1000);
        let siv: Mutex<cursive::Cursive> = std::sync::Mutex::new(siv);
        siv.lock().unwrap().run_with(|| backend::backend(canvas));
        Cursive { backend: siv }
    }
}

