use cursive_buffered_backend::BufferedBackend;
use web_sys::{
    HtmlCanvasElement,
};

pub fn backend(canvas: HtmlCanvasElement) -> Box<BufferedBackend> {
    let wasm_backend = wasm_backend::backend::Backend::init(canvas);
    let buffered_backend = BufferedBackend::new(Box::new(wasm_backend));
    Box::new(buffered_backend)
}