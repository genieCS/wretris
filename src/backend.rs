use cursive::backend::Backend;
use cursive::backends::wasm;

pub fn backend() -> Box<dyn Backend> {
    let wasm_backend: Box<dyn Backend> = wasm::Backend::init().unwrap();
    wasm_backend
}
