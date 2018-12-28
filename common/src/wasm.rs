use std::mem;
use std::os::raw::c_void;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
   console_error_panic_hook::set_once();
}

// In order to work with the memory we expose (de)allocation methods
#[wasm_bindgen]
pub fn alloc_vec(size: usize) -> *mut c_void {
   let mut img_data = Vec::with_capacity(size);

   let ptr = img_data.as_mut_ptr();
   mem::forget(img_data);
   return ptr as *mut c_void;
}

#[wasm_bindgen]
pub fn dealloc_vec(ptr: *mut c_void, cap: usize) {
   unsafe {
      let _buf = Vec::from_raw_parts(ptr, 0, cap);
   }
}