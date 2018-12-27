use std::mem;
use std::slice;
use std::os::raw::c_void;

use wasm_bindgen::prelude::*;
//use web_sys::console;

use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

use crate::shared::*;

mod shared;

#[wasm_bindgen]
pub fn init() {
   console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn new_ground(contents: String) -> *mut Grid<Ground> {
   Box::into_raw(Box::new(parse_input(&contents)))
}

#[wasm_bindgen]
pub fn delete_ground(ground: *mut Grid<Ground>) {
   unsafe {
      Box::from_raw(ground);
   }
}

#[wasm_bindgen]
pub fn ground_width(ground: *mut Grid<Ground>) -> usize {
   unsafe {
      let ground = &mut *ground;

      ground.width()
   }
}

#[wasm_bindgen]
pub fn ground_height(ground: *mut Grid<Ground>) -> usize {
   unsafe {
      let ground = &mut *ground;

      ground.height()
   }
}

#[wasm_bindgen]
pub fn tick_ground(ground: *mut Grid<Ground>) -> bool {
   unsafe {
      let ground = &mut *ground;

      tick(ground)
   }
}

#[wasm_bindgen]
pub fn get_water_count(ground: *mut Grid<Ground>) -> usize {
   let ground = unsafe { &mut *ground };

   count_water(ground).0
}

#[wasm_bindgen]
pub fn get_water_locked(ground: *mut Grid<Ground>) -> usize {
   let ground = unsafe { &mut *ground };

   count_water(ground).1
}

#[wasm_bindgen]
pub fn render_ground(pixel_size: usize, img_data_pointer: *mut u32, ground: *mut Grid<Ground>, full_render: bool) {
   let (ground, img_data) = unsafe {
      let ground = &mut *ground;
      let byte_size = ground.width() * pixel_size * ground.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (ground, img_data)
   };

   let row_width = ground.width() * pixel_size;

   for ground_y in ground.y_range() {
      let y = ground.raw_y(ground_y) as usize;
      for ground_x in ground.x_range() {
         let x = ground.raw_x(ground_x) as usize;

         let start_x = x * pixel_size;
         let end_x = start_x + pixel_size;
         let start_y = y * pixel_size;
         let end_y = start_y + pixel_size;

         let mut water = false;
         let color: u32 = match ground.get(ground_x, ground_y) {
            Ground::Clay => 0x6d5800FF,
            Ground::Sand => 0xbeaf70FF,
            _ => {
               water = true;
               0x5dade2FF
            }
         };

         if full_render || water {
            for img_y in start_y..end_y {
               for img_x in start_x..end_x {
                  let pixel = (img_y * row_width) + img_x;

                  // flip our endianness to match the actual u8 byte array
                  img_data[pixel] = color.to_be();
               }
            }
         }
      }
   }
}

// In order to work with the memory we expose (de)allocation methods
#[wasm_bindgen]
pub fn alloc_img_data(size: usize) -> *mut c_void {
   let mut img_data = Vec::with_capacity(size);

   let ptr = img_data.as_mut_ptr();
   mem::forget(img_data);
   return ptr as *mut c_void;
}

#[wasm_bindgen]
pub fn dealloc(ptr: *mut c_void, cap: usize) {
   unsafe {
      let _buf = Vec::from_raw_parts(ptr, 0, cap);
   }
}