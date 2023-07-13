use cursive_buffered_backend::BufferedBackend;
use cursive::backends;

pub fn backend() -> Box<BufferedBackend> {
    let wasm_backend = backends::wasm::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(wasm_backend);
    Box::new(buffered_backend)
}
