mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn new_float32_vec() -> *mut Vec<f32> {
    Box::into_raw(Box::new(Vec::new()))
}

#[wasm_bindgen]
pub fn free_float32_vec(v: *mut Vec<f32>) {
    let _ = unsafe { Box::from_raw(v) };
}

#[wasm_bindgen]
pub fn float32_vec_data_address(v: *mut Vec<f32>) -> *mut f32 {
    let v = unsafe { &mut *v };
    v.as_mut_ptr()
}

#[wasm_bindgen]
pub fn float32_vec_len(v: *const Vec<f32>) -> usize {
    let v = unsafe { &*v };
    v.len()
}

#[wasm_bindgen]
pub fn float32_vec_write_test_data(v: *mut Vec<f32>) {
    let v = unsafe { &mut *v };
    let test_data: [f32; 12] = [
        -1.0, -1.0,  1.0, // v0
        1.0, -1.0,  1.0,  // v1
        1.0,  1.0,  1.0,  // v2
       -1.0,  1.0,  1.0,  // v3
    ];
    for x in test_data {
        v.push(x)
    }
}
