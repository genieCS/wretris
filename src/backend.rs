use cursive::backend::Backend;
use cursive::backends::wasm;
use web_sys::HtmlCanvasElement;

pub fn backend() -> Box<dyn Backend> {
    let wasm_backend: Box<dyn Backend> = wasm::Backend::init().unwrap();
    wasm_backend
}

pub fn backend_with_canvas(canvas: HtmlCanvasElement) -> Box<dyn Backend> {
    let wasm_backend: Box<dyn Backend> = wasm::Backend::new(canvas).unwrap();
    wasm_backend
}
