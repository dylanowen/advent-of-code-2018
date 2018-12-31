use std::slice;

use wasm_bindgen::prelude::*;
//use web_sys::console;

use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;
use common::canvas::render_grid;

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

#[wasm_bindgen]
pub fn lumberyard_width(lumberyard: *mut Grid<Acre>) -> usize {
   let lumberyard = unsafe { &mut *lumberyard };

   lumberyard.width()
}

#[wasm_bindgen]
pub fn lumberyard_height(lumberyard: *mut Grid<Acre>) -> usize {
   let lumberyard = unsafe { &mut *lumberyard };

   lumberyard.height()
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
      let lumberyard = &mut *lumberyard;
      let byte_size = lumberyard.width() * pixel_size * lumberyard.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (lumberyard, img_data)
   };

   render_grid(pixel_size, img_data, &lumberyard, &|cell| {
      Some(match cell {
         Acre::Open => 0xbeaf70FF,
         Acre::Tree => 0x298210FF,
         Acre::Lumberyard =>0x6b4b0bFF,
      })
   });
}