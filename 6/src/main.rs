use regex::Regex;

use common::*;
use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

fn main() {
   run_day("6", &|contents, is_sample| {
      let re: Regex = Regex::new(r"(\d+), (\d+)").unwrap();

      let mut min = Loci::max_value();
      let mut max = Loci::new(0, 0);
      let locations: Vec<Loci> = contents.lines()
         .map(|row| {
            let parsed_row = re.captures(row).unwrap();

            let x = parsed_row[1].parse::<isize>().unwrap();
            let y = parsed_row[2].parse::<isize>().unwrap();

            if x < min.x() {
               min = min.with_x(x);
            }
            if x > max.x() {
               max = max.with_x(x);
            }

            if y < min.y() {
               min = min.with_y(y);
            }
            if y > max.y() {
               max = max.with_y(y);
            }

            return Loci::new(x, y);
         })
         .collect();

      // give us some breathing room
      min = min.sub(1, 1);
      max = max.add(1, 1);

      // get the region depending on if we're running the sample or not
      let region_range;
      if is_sample {
         region_range = 32;
      } else {
         region_range = 10000;
      }

      a(&locations, &min, &max);
      b(&locations, &min, &max, region_range);
   });
}

fn a(locations: &Vec<Loci>, min: &Loci, max: &Loci) {
   let mut manhattan_grid: coordinates::Grid<Option<&Loci>> = Grid::new_loci_offset(
      None,
      &max.sub_loci(min),
      min,
   );

   // for each coordinate in the graph find the closest "location"
   for loci in manhattan_grid.locis() {
      manhattan_grid.set_loci(&loci, find_closest_location(&loci, locations))
   }

   // trim infinity locations
   let mut finite_locations = locations.iter()
      .map(|location| -> &Loci { &location })
      .collect();

   for x in manhattan_grid.x_range() {
      prune_infinite(*manhattan_grid.get(x, manhattan_grid.y_min()), &mut finite_locations);
      prune_infinite(*manhattan_grid.get(x, manhattan_grid.y_max() - 1), &mut finite_locations);
   }
   for y in manhattan_grid.y_range() {
      prune_infinite(*manhattan_grid.get(manhattan_grid.x_min(), y), &mut finite_locations);
      prune_infinite(*manhattan_grid.get(manhattan_grid.x_max() - 1, y), &mut finite_locations);
   }

   // debug
   //for loci in manhattan_grid.locis() {
   //   match manhattan_grid.get(&loci) {
   //      Some(_) => print!("#"),
   //      _ => print!("."),
   //   }
   //
   //   if loci.y() == manhattan_grid.y_min() {
   //      println!();
   //   }
   //}

   //for finite_location in finite_locations.iter() {
   //    println!("{:?}", finite_location);
   //}

   let mut max_area = 0;
   for location in finite_locations {
      let mut area = 0;

      for maybe_loci in manhattan_grid.iter() {
         match maybe_loci {
            Some(loci) => if *loci == location {
               area += 1;
            }
            _ => {}
         }
      }

      if area > max_area {
         max_area = area;
      }
   }

   println!("Result A: {}", max_area);
}

fn prune_infinite(infinite_location: Option<&Loci>, finite_locations: &mut Vec<&Loci>) {
   match infinite_location {
      Some(infinite) => {
         finite_locations.iter()
            .position(|loci| *loci == infinite)
            .map(|position| finite_locations.remove(position));
      }
      _ => {}
   }
}


fn b(locations: &Vec<Loci>, min: &Loci, max: &Loci, region_range: usize) {
   let mut region_grid: coordinates::Grid<usize> = Grid::new_loci_offset(
      0,
      &max.sub_loci(min),
      min,
   );

   'main: for loci in region_grid.locis() {
      let mut total_distance = 0;
      for location in locations {
         total_distance += location.distance(&loci);
         if total_distance > region_range {
            // we hit our limit so break out
            continue 'main;
         }
      }

      // if we get here we know we're in a region
      region_grid.set_loci(&loci, 1);
   }

   //for y in 0..height {
   //    for x in 0..width {
   //        match grid[x][y] {
   //            true => print!("#"),
   //            _ => print!("."),
   //        }
   //    }
   //    println!()
   //}

   // count up the size of our region
   let region_size = region_grid.iter()
      .fold(0, |sum, cell| -> usize {
         sum + cell
      });

   println!("Result B: {}", region_size);
}

fn find_closest_location<'a>(current_loci: &Loci, locations: &'a Vec<Loci>) -> Option<&'a Loci> {
   let mut min_distance = usize::max_value();
   let mut closest = None;
   let mut dup = false;
   for location in locations {
      let distance = current_loci.distance(location);
      //println!("{}", distance);

      if distance < min_distance {
         closest = Some(location);
         min_distance = distance;
         dup = false;
      } else if distance == min_distance {
         dup = true;
      }
   }

   // we found a duplicate so don't return anything
   if dup {
      return None;
   }

   return closest;
}