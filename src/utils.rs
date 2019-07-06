use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn as_f32_array(v: &[f32]) -> Result<js_sys::Float32Array, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let location = v.as_ptr() as u32 / 4;

    Ok(js_sys::Float32Array::new(&memory_buffer).subarray(location, location + v.len() as u32))
}
