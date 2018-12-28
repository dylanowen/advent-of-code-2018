use std::slice;

use wasm_bindgen::prelude::*;
//use web_sys::console;

use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

use crate::shared::*;

pub use common::wasm::*;

mod shared;

#[wasm_bindgen]
pub fn new_lumberyard(contents: String) -> *mut Grid<Acre> {
   Box::into_raw(Box::new(parse_input(&contents)))
}

#[wasm_bindgen]
pub fn delete_lumberyard(lumberyard: *mut Grid<Acre>) {
   unsafe {
      Box::from_raw(lumberyard);
   }
}

//
#[wasm_bindgen]
pub fn lumberyard_width(lumberyard: *mut Grid<Acre>) -> usize {
   let ground = unsafe { &mut *lumberyard };

   ground.width()
}

#[wasm_bindgen]
pub fn lumberyard_height(lumberyard: *mut Grid<Acre>) -> usize {
   let ground = unsafe { &mut *lumberyard };

   ground.height()
}


#[wasm_bindgen]
pub fn tick_lumberyard(lumberyard: *mut Grid<Acre>) {
   let mut lumberyard = unsafe { &mut *lumberyard };

   // copy our old values
   let last = lumberyard.clone();

   next_lumberyard(&last, &mut lumberyard);
}

#[wasm_bindgen]
pub fn render_lumberyard(pixel_size: usize, img_data_pointer: *mut u32, lumberyard: *mut Grid<Acre>) {
   let (lumberyard, img_data) = unsafe {
      let ground = &mut *lumberyard;
      let byte_size = ground.width() * pixel_size * ground.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (ground, img_data)
   };

   let row_width = lumberyard.width() * pixel_size;

   for ground_y in lumberyard.y_range() {
      let y = lumberyard.raw_y(ground_y) as usize;
      for ground_x in lumberyard.x_range() {
         let x = lumberyard.raw_x(ground_x) as usize;

         let start_x = x * pixel_size;
         let end_x = start_x + pixel_size;
         let start_y = y * pixel_size;
         let end_y = start_y + pixel_size;

         let color: u32 = match lumberyard.get(ground_x, ground_y) {
            Acre::Open => 0xbeaf70FF,
            Acre::Tree => 0x298210FF,
            Acre::Lumberyard =>0x6b4b0bFF,
         };

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