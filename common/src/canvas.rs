use crate::coordinates::Grid;
use crate::coordinates::OffsetLociX;
use crate::coordinates::OffsetLociY;

pub fn render_grid<C, T>(pixel_size: usize, img_data: &mut [u32], grid: &Grid<T>, colorizer: &C) where
   C: Fn(&T) -> Option<u32> {
   let row_width = grid.width() * pixel_size;

   for grid_y in grid.y_range() {
      let y = grid.raw_y(grid_y) as usize;
      for grid_x in grid.x_range() {
         let x = grid.raw_x(grid_x) as usize;

         let start_x = x * pixel_size;
         let end_x = start_x + pixel_size;
         let start_y = y * pixel_size;
         let end_y = start_y + pixel_size;

         match colorizer(grid.get(grid_x, grid_y)) {
            Some(color) => {
               for img_y in start_y..end_y {
                  for img_x in start_x..end_x {
                     let pixel = (img_y * row_width) + img_x;

                     // flip our endianness to match the actual u8 byte array
                     img_data[pixel] = color.to_be();
                  }
               }
            }
            None => {}
         }
      }
   }
}