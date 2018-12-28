use std::slice;

use wasm_bindgen::prelude::*;
//use web_sys::console;

use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;
use common::canvas::render_grid;

pub use common::wasm::*;

use crate::shared::*;

mod shared;

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
   let ground = unsafe { &mut *ground };

   ground.width()
}

#[wasm_bindgen]
pub fn ground_height(ground: *mut Grid<Ground>) -> usize {
   let ground = unsafe { &mut *ground };

   ground.height()
}

#[wasm_bindgen]
pub fn tick_ground(ground: *mut Grid<Ground>) -> bool {
   let ground = unsafe { &mut *ground };

   tick(ground)
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

   render_grid(pixel_size, img_data, &ground, &|cell| {
      let mut water = false;
      let color = match cell {
         Ground::Clay => 0x6d5800FF,
         Ground::Sand => 0xbeaf70FF,
         _ => {
            water = true;
            0x5dade2FF
         }
      };

      if full_render || water {
         Some(color)
      }
      else {
         None
      }
   });
}