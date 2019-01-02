use std::slice;
use std::collections::BTreeSet;
use std::mem;

use wasm_bindgen::prelude::*;

use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;
use common::canvas::{render_grid, set_grid_square};

pub use common::wasm::*;
use web_sys::console;

use crate::shared::*;

mod shared;

const MAX_BRIGHTNESS: u32 = 0xDD;
const MAX_COLOR_DISTANCE: u32 = 4000;

#[wasm_bindgen]
pub fn render_next_path(path_iter: *mut Box<PathIterator<Item=MapMove>>,
                        step_size: usize,
                        pixel_size: usize,
                        img_data_pointer: *mut u32,
                        map_pointer: *mut (Loci, Grid<MapFeature>)) -> bool {
   let (path_iter, map, img_data) = unsafe {
      let path_iter = &mut *path_iter;
      let (_, map) = &mut *map_pointer;
      let byte_size = map.width() * pixel_size * map.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (path_iter, map, img_data)
   };

   // render everything else
   render_map(pixel_size, img_data_pointer, map_pointer, false);

   for _ in 0..step_size {
      match path_iter.next() {
         Some(map_moves) => {
            for map_move in &map_moves {
               map.set_loci(&map_move.0, map_move.1);

               // print what we just drew
               set_grid_square(
                  map_move.0.x(),
                  map_move.0.y(),
                  0x42a7f4FF,
                  pixel_size,
                  img_data,
                  map,
               )
            }
         }
         None => {
            return false;
         }
      }
   }

   return true;
}

#[wasm_bindgen]
pub fn render_distance(locations_pointer: *mut (BTreeSet<Loci>, Grid<usize>),
                       step_size: usize,
                       pixel_size: usize,
                       img_data_pointer: *mut u32,
                       map_pointer: *mut (Loci, Grid<MapFeature>)) -> bool {
   let (locations, distance_grid, img_data, map) = unsafe {
      let (locations, distance_grid) = &mut *locations_pointer;
      let (_, map) = &mut *map_pointer;

      let byte_size = map.width() * pixel_size * map.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (locations, distance_grid, img_data, map)
   };

   for _ in 0..step_size {
      if locations.is_empty() {
         return false;
      }

      let mut next_locations: BTreeSet<Loci> = BTreeSet::new();

      for location in locations.iter() {
         let distance = distance_grid.get_loci(location);

         let neighbor_distance = distance + 1;
         for door in location.neighbors().iter() {
            if *map.get_loci(door) == MapFeature::Door {
               let direction = door.sub_loci(location);
               let room = door.add_loci(&direction);

               let last_distance = *distance_grid.get_loci(&room);

               if neighbor_distance < last_distance {
                  distance_grid.set_loci(&room, neighbor_distance);

                  next_locations.insert(room);

                  let distance_fraction = (neighbor_distance as u32) * MAX_BRIGHTNESS / MAX_COLOR_DISTANCE;
                  let gray_scale = MAX_BRIGHTNESS - (0xFF & distance_fraction);
                  let red = gray_scale << 24;
                  let green = gray_scale << 16;
                  let blue = gray_scale << 8;

                  let color = 0x000000FF | red | green | blue;
                  console::log_1(&format!("{:x} {:x} {:x} {:x} {:x}", distance_fraction, gray_scale, red, green, color).into());
                  for loci in [*door, room].iter() {
                     set_grid_square(
                        loci.x(),
                        loci.y(),
                        color,
                        pixel_size,
                        img_data,
                        map,
                     );
                  }
               }
            }
         }
      }

      mem::replace(locations, next_locations);
   }

   return true;
}

#[wasm_bindgen]
pub fn render_map(pixel_size: usize, img_data_pointer: *mut u32, map: *mut (Loci, Grid<MapFeature>), full_render: bool) {
   let (map, img_data) = unsafe {
      let (_, map) = &mut *map;
      let byte_size = map.width() * pixel_size * map.height() * pixel_size;

      let img_data = slice::from_raw_parts_mut(img_data_pointer, byte_size);

      (map, img_data)
   };

   render_grid(pixel_size, img_data, &map, &|cell| {
      if full_render || *cell != MapFeature::Wall {
         Some(match cell {
            MapFeature::Room | MapFeature::Door => 0x997f6cFF,
            MapFeature::Wall => 0x303030FF,
         })
      } else {
         None
      }
   });
}

#[wasm_bindgen]
pub fn calculate_max_distance(map: *mut (Loci, Grid<MapFeature>)) -> usize {
   let (start, map) = unsafe { &mut *map };

   ab(start, map).0
}

#[wasm_bindgen]
pub fn calculate_total_far_distances(map: *mut (Loci, Grid<MapFeature>)) -> usize {
   let (start, map) = unsafe { &mut *map };

   ab(start, map).1
}

#[wasm_bindgen]
pub fn new_locations(map: *mut (Loci, Grid<MapFeature>)) -> *mut (BTreeSet<Loci>, Grid<usize>) {
   let (start, map) = unsafe { &mut *map };

   let mut distance_grid = Grid::new_offset(
      usize::max_value(),
      map.width(),
      map.height(),
      map.x_min(),
      map.y_min(),
   );
   distance_grid.set_loci(start, 0);

   let mut locations = BTreeSet::new();
   locations.insert(start.clone());

   Box::into_raw(Box::new((locations, distance_grid)))
}

#[wasm_bindgen]
pub fn delete_locations(locations_pointer: *mut (BTreeSet<Loci>, Grid<usize>)) {
   unsafe {
      Box::from_raw(locations_pointer);
   }
}

#[wasm_bindgen]
pub fn new_path(contents: String) -> *mut Path {
   Box::into_raw(Box::new(parse_input(&contents)))
}

#[wasm_bindgen]
pub fn delete_path(path: *mut Path) {
   unsafe {
      Box::from_raw(path);
   }
}

#[wasm_bindgen]
pub fn new_map(path: *mut Path) -> *mut (Loci, Grid<MapFeature>) {
   let path = unsafe { &mut *path };
   // lets figure out our map size, then duplicate it to render the generation
   let (center, initial_map) = build_map(path);

   let mut map = Grid::new_offset(
      MapFeature::Wall,
      initial_map.width(),
      initial_map.height(),
      initial_map.x_min(),
      initial_map.y_min(),
   );

   // always start in a room
   map.set_loci(&center, MapFeature::Room);

   Box::into_raw(Box::new((center, map)))
}

#[wasm_bindgen]
pub fn delete_map(map: *mut (Loci, Grid<MapFeature>)) {
   unsafe {
      Box::from_raw(map);
   }
}

#[wasm_bindgen]
pub fn map_width(map: *mut (Loci, Grid<MapFeature>)) -> usize {
   let (_, map) = unsafe { &mut *map };

   map.width()
}

#[wasm_bindgen]
pub fn map_height(map: *mut (Loci, Grid<MapFeature>)) -> usize {
   let (_, map) = unsafe { &mut *map };

   map.height()
}

#[wasm_bindgen]
pub fn new_path_iter(path: *mut Path, map: *mut (Loci, Grid<MapFeature>)) -> *mut Box<PathIterator<Item=MapMove>> {
   let path = unsafe { &mut *path };
   let (center, _) = unsafe { &mut *map };

   let path_iter = path.path_iterator(center);

   Box::into_raw(Box::new(path_iter))
}

#[wasm_bindgen]
pub fn delete_path_iter(path_iter: *mut Box<Iterator<Item=MapMove>>) {
   unsafe {
      Box::from_raw(path_iter);
   }
}