use regex::Regex;
use std::fmt;

use lib::*;
use lib::coordinates::Grid;
use lib::coordinates::Loci;
use lib::coordinates::OffsetLociX;
use lib::coordinates::OffsetLociY;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Ground {
   Clay,
   Sand,
   WaterFalling,
   WaterLocked,
}

fn main() {
   fn parse_input(contents: &String) -> Grid<Ground> {
      let re: Regex = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)").unwrap();

      let mut min = Loci::max_value();
      let mut max = Loci::new(0, 0);
      let clay_lines: Vec<(char, isize, isize, isize)> = contents.lines()
         .map(|row| {
            let parsed_row = re.captures(row).unwrap();

            let singular = parsed_row[1].chars().next().unwrap();
            let singular_start = parsed_row[2].parse::<isize>().unwrap();

            let multiple_from = parsed_row[4].parse::<isize>().unwrap();
            let multiple_to = parsed_row[5].parse::<isize>().unwrap();

            match singular {
               'x' => {
                  min = min.min_x(singular_start);
                  max = max.max_x(singular_start);

                  min = min.min_y(multiple_from);
                  max = max.max_y(multiple_to);
               }
               'y' => {
                  min = min.min_y(singular_start);
                  max = max.max_y(singular_start);

                  min = min.min_x(multiple_from);
                  max = max.max_x(multiple_to);
               }
               u => panic!("Unexpected char: {}", u)
            }

            (singular, singular_start, multiple_from, multiple_to)
         })
         .collect();

      // make max inclusive
      max = max.add(1, 1);
      // make sure we have enough space for falling water on the left and right
      min = min.sub_x(1);
      max = max.add_x(1);

      let mut ground = Grid::new_loci_offset(
         Ground::Sand,
         &max.sub_loci(&min),
         &min,
      );

      for (singular, singular_start, multiple_from, multiple_to) in clay_lines {
         let x_min;
         let x_max;
         let y_min;
         let y_max;
         match singular {
            'x' => {
               x_min = singular_start;
               x_max = singular_start;
               y_min = multiple_from;
               y_max = multiple_to;
            }
            'y' => {
               x_min = multiple_from;
               x_max = multiple_to;
               y_min = singular_start;
               y_max = singular_start;
            }
            u => panic!("Unexpected char: {}", u)
         }

         for x in x_min..=x_max {
            for y in y_min..=y_max {
               ground.set(x, y, Ground::Clay);
            }
         }
      }

      // add the water
      ground.set(500, ground.y_min(), Ground::WaterFalling);

      ground
   }

   run_tests("17", "test_input_{}.txt",
             vec![(45, 17)],
             &|contents| {
                let ground = parse_input(contents);

                ab(&ground)
             },
   );

   run_day("17", &|contents, is_sample| {
      let ground = parse_input(contents);

      let (a_result, b_result) = ab(&ground);
      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);
      if is_sample {
         assert_eq!(57, a_result);
         assert_eq!(29, b_result);
      }
   });
}

fn ab(initial_ground: &Grid<Ground>) -> (isize, isize) {
   let mut ground = initial_ground.clone();

   let mut acted = true;
   while acted {
      acted = false;

//      println!("Step");
//      print_subset(&ground);

      for x in ground.x_range() {
         for y in ground.y_range() {
            let dirt = ground.get(x, y);

            match dirt {
               Ground::WaterFalling => {
                  let down_y = y + 1;
                  if down_y < ground.y_max() {
                     match ground.get(x, down_y) {
                        Ground::Clay | Ground::WaterLocked => {
                           // check if we have anything to fill in
                           if (x - 1 < ground.x_min() || *ground.get(x - 1, y) != Ground::WaterFalling) ||
                              (x + 1 >= ground.x_max() || *ground.get(x + 1, y) != Ground::WaterFalling) {
                              acted = fill_ledge(x, y, &mut ground) || acted;
                           }

                           //println!("{} {}", x, y);
                        }
                        Ground::Sand => {
                           ground.set(x, down_y, Ground::WaterFalling);
                           acted = true;
                        }
                        _ => {}
                     }
                  }
               }
               _ => {}
            }
         }
      }
   }

   let mut water_count = 0;
   let mut water_locked = 0;

   for dirt in ground.iter() {
      match *dirt {
         Ground::WaterFalling => water_count += 1,
         Ground::WaterLocked => {
            water_count += 1;
            water_locked += 1;
         }
         _ => {}
      }
   }

   return (water_count, water_locked);
}

fn fill_ledge(x_start: isize, y: isize, ground: &mut Grid<Ground>) -> bool {
   let mut acted = false;

   // fill left
   let mut min_found = x_start;
   let mut max_found = x_start;

   let mut left_found_wall = false;
   let mut right_found_wall = false;

   for x in (ground.x_min()..x_start).rev() {
      let dirt = *ground.get(x, y);
      match dirt {
         Ground::Sand | Ground::WaterFalling => {
            // can only overwite sand or water falling
            ground.set(x, y, Ground::WaterFalling);
            acted = acted || dirt == Ground::Sand;

            // check if we have something to stand on
            match ground.get(x, y + 1) {
               Ground::Clay | Ground::WaterLocked => {
                  min_found = x;
               }
               _ => {
                  break;
               }
            }
         }
         Ground::Clay => {
            // found an edge
            left_found_wall = true;
            break;
         }
         _ => {
            break;
         }
      }
   }

   for x in x_start + 1..ground.x_max() {
      let dirt = *ground.get(x, y);
      match ground.get(x, y) {
         Ground::Sand | Ground::WaterFalling => {
            // can only overwrite sand
            ground.set(x, y, Ground::WaterFalling);
            acted = acted || dirt == Ground::Sand;

            // check if we have something to stand on
            match ground.get(x, y + 1) {
               Ground::Clay | Ground::WaterLocked => {
                  max_found = x;
               }
               _ => {
                  break;
               }
            }
         }
         Ground::Clay => {
            // found an edge
            right_found_wall = true;
            break;
         }
         _ => {
            break;
         }
      }
   }

   // convert to locked water
   if left_found_wall && right_found_wall {
      for x in min_found..=max_found {
         ground.set(x, y, Ground::WaterLocked);
         acted = true;
      }
   }

   return acted;
}

impl fmt::Display for Ground {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         Ground::Clay => write!(f, "#"),
         Ground::Sand => write!(f, "."),
         Ground::WaterFalling => write!(f, "|"),
         Ground::WaterLocked => write!(f, "~"),
      }
   }
}

// only print the ground that has water in it
//fn print_subset(ground: &Grid<Ground>) {
//   for y in ground.y_range() {
//      // found water
//      let mut found_water= false;
//      for x in ground.x_range() {
//         let dirt = ground.get(x, y);
//         found_water = found_water || *dirt == Ground::WaterFalling || *dirt == Ground::WaterLocked;
//
//         print!("{} ", dirt);
//      }
//      println!();
//
//      if !found_water {
//         return;
//      }
//   }
//}