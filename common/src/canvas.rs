use crate::coordinates::Grid;
use crate::coordinates::OffsetLociX;
use crate::coordinates::OffsetLociY;

pub fn render_grid<C, T>(pixel_size: usize, img_data: &mut [u32], grid: &Grid<T>, colorizer: &C) where
   C: Fn(&T) -> Option<u32> {

   for grid_y in grid.y_range() {
      for grid_x in grid.x_range() {
         match colorizer(grid.get(grid_x, grid_y)) {
            Some(color) => {
               set_grid_square(grid_x, grid_y, color, pixel_size, img_data, grid);
            }
            None => {}
         }
      }
   }
}

pub fn get_img_square_range(x: usize, y: usize, pixel_size: usize) -> (usize, usize, usize, usize) {
   let start_x = x * pixel_size;
   let end_x = start_x + pixel_size;
   let start_y = y * pixel_size;
   let end_y = start_y + pixel_size;

   (start_x, end_x, start_y, end_y)
}

pub fn set_grid_square<T>(grid_x: isize, grid_y: isize, color: u32, pixel_size: usize, img_data: &mut [u32], grid: &Grid<T>) {
   let img_width = grid.width() * pixel_size;
   let x = grid.raw_x(grid_x) as usize;
   let y = grid.raw_y(grid_y) as usize;
   let (start_x, end_x, start_y, end_y) = get_img_square_range(x, y, pixel_size);

   for img_y in start_y..end_y {
      for img_x in start_x..end_x {
         set_img_pixel(img_x, img_y, color, img_width, img_data);
      }
   }
}

pub fn set_img_pixel(img_x: usize, img_y: usize, color: u32, img_width: usize, img_data: &mut [u32]) {
   let pixel = (img_y * img_width) + img_x;

   // flip our endianness to match the actual u8 byte array
   img_data[pixel] = color.to_be();
}