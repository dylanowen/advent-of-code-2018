use std::collections::HashMap;
use lib::*;
use lib::coordinates::Grid;
use lib::coordinates::Loci;
use lib::coordinates::OffsetLociX;
use lib::coordinates::OffsetLociY;

fn main() {
   assert_eq!(4, calc_power_level(&Loci::new(3, 5), 8));
   assert_eq!(-5, calc_power_level(&Loci::new(122, 79), 57));
   assert_eq!(0, calc_power_level(&Loci::new(217, 196), 39));
   assert_eq!(4, calc_power_level(&Loci::new(101, 153), 71));

   run_day("11", &|contents, _is_sample| {
      let serial_number = contents.parse::<isize>().unwrap();

      let mut grid = Grid::new_offset(0, 300, 300, 1, 1);

      for loci in grid.locis() {
         let power_level = calc_power_level(&loci, serial_number);

         grid.set_loci(&loci, power_level);
      }

      a(&grid);
      b(&grid);
   });
}

fn a(grid: &Grid<isize>) {
   let mut max_power = 0;
   let mut max_loci = Loci::new(0, 0);
   for loci in grid.locis() {
      // make sure not to extend outside of what we can check
      if loci.x() < (grid.x_max() - 3) && loci.y() < (grid.x_max() - 3) {
         let square_power = sum_power_simple(&loci, 3, grid);
         if square_power > max_power {
            max_power = square_power;
            max_loci = loci.clone();
         }

         //print!("{:4}", square_power);
         //
         //if loci.x() == (grid.x_max() - 4) {
         //   println!();
         //}
      }
   }

   println!("Result A: {},{}", max_loci.x(), max_loci.y());
}

fn b(grid: &Grid<isize>) {
   let mut max_power = 0;
   let mut max_loci = Loci::new(0, 0);
   let mut max_size= 0;

   let mut memoizer: Grid<HashMap<isize, isize>> = Grid::new_offset(HashMap::new(), 300, 300, 1, 1);

   for length in 1..=20 {
      for x in 1..=grid.x_max() - length {
         for y in 1..=grid.y_max() - length {
            let square_power = sum_power(x, y, length, grid, &mut memoizer);
            if square_power > max_power {
               max_power = square_power;
               max_loci = Loci::new(x, y);
               max_size = length;
            }
         }
      }
   }

   println!("Result B: {},{},{}", max_loci.x(), max_loci.y(), max_size);
}

fn calc_power_level(loci: &Loci, serial_number: isize) -> isize {
   let rack_id = loci.x() + 10;

   let mut power_level = rack_id * loci.y();
   power_level += serial_number;
   power_level *= rack_id;
   power_level = (power_level % 1000) / 100;
   power_level -= 5;

   return power_level;
}

fn sum_power_simple(loci: &Loci, length: isize, grid: &Grid<isize>) -> isize {
   // assume the loci has enough space in the grid
   let mut sum = 0;
   for x in loci.x()..loci.x() + length {
      for y in loci.y()..loci.y() + length {
         //println!("{:?} {:?} {:?}", loci, loci.add(x, y), grid.get_loci(&loci.add(x, y)));
         sum += grid.get(x, y);
      }
   }

   return sum;
}

fn sum_power(x: isize, y: isize, length: isize, grid: &Grid<isize>, memoizer: &mut Grid<HashMap<isize, isize>>) -> isize {
   // assume the loci has enough space in the grid
   let result;
   if length <= 3 {
      // drop out to our simple case
      result = sum_power_simple(&Loci::new(x, y), length, grid);
   } else {
      let mut sum = 0;
      for xi in x..x + length {
         sum += grid.get(xi, y);
      }
      for yi in y + 1..y + length {
         sum += grid.get(x, yi);
      }

      // the calling function makes sure this exists
      sum += memoizer.get(x + 1, y + 1).get(&(length - 1)).unwrap();

      result = sum;
   }

   memoizer.get_mut(x, y)
      .insert(length, result);

   result
}