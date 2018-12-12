use regex::Regex;

use lib::*;
use lib::coordinates::Grid;
use lib::coordinates::Loci;
use lib::coordinates::OffsetLociX;

struct Point {
   loci: Loci,
   velocity: Loci,
}

impl Clone for Point {
   fn clone(&self) -> Self {
      Point {
         loci: self.loci.clone(),
         velocity: self.velocity.clone(),
      }
   }
}

impl PartialEq for Point {
   fn eq(&self, other: &Point) -> bool {
      self.loci == other.loci && self.velocity == other.velocity
   }
}

fn main() {
   run_day("10", &|contents, is_sample| {
      let re: Regex = Regex::new(r"position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>").unwrap();

      let points: Vec<Point> = contents.lines()
         .map(|row| {
            let parsed_row = re.captures(row).unwrap();

            let x = parsed_row[1].parse::<isize>().unwrap();
            let y = parsed_row[2].parse::<isize>().unwrap();

            let v_x = parsed_row[3].parse::<isize>().unwrap();
            let v_y = parsed_row[4].parse::<isize>().unwrap();

            return Point {
               loci: Loci::new(x, y),
               velocity: Loci::new(v_x, v_y),
            };
         })
         .collect();

      ab(&points, is_sample);
   });
}

fn ab(original_points: &Vec<Point>, is_sample: bool) {
   let mut points = original_points.to_vec();

   let grid;
   let render_second;
   if is_sample {
      render_second = 3;

      grid = Grid::new_loci_offset(
         0,
         &Loci::new(30, 15),
         &Loci::new(-10, -5),
      );
   } else {
      render_second = 10645;

      grid = Grid::new_loci_offset(
         0,
         &Loci::new(70, 20),
         &Loci::new(180, 135),
      );
   }

   for second in 0..=render_second {
      // check to see if any point is on the grid
      //let visible_points = points.iter()
      //   .find(|point| {
      //      point.loci.x() >= grid.x_min() && point.loci.x() < grid.x_max() &&
      //         point.loci.y() >= grid.y_min() && point.loci.y() < grid.y_max()
      //   })
      //   .is_some();

      // cheat and just render for our known time
      let visible_points = second == render_second;

      if visible_points {
         println!("Seconds: {}", second);

         // print
         for loci in grid.locis() {
            if points.iter().find(|point| point.loci == loci).is_some() {
               print!("#")
            } else {
               print!(".")
            }

            if loci.x() == (grid.x_max() - 1) {
               println!();
            }
         }

         println!();
      }

      // move each point
      points.iter_mut()
         .for_each(|point| {
            point.loci = point.loci.add_loci(&point.velocity)
         });
   }
}