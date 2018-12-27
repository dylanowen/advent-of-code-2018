use regex::Regex;
use std::option::Option::{Some, None};
use std::ops::Range;

use common::*;

//#[derive(Copy, Clone, PartialEq, Eq, Debug)]
//enum Attack {
//   Fire,
//   Slashing,
//   Bludgeoning,
//   Cold,
//   Radiation,
//}
//
//#[derive(Copy, Clone, PartialEq, Eq, Debug)]
//enum Army {
//   Immune,
//   Infection,
//}
//
//const ATTACKS: [Attack; 5] = [
//   Attack::Fire,
//   Attack::Slashing,
//   Attack::Bludgeoning,
//   Attack::Cold,
//   Attack::Radiation,
//];
//
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct NanoBot {
   location: Coordinate,
   radius: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Coordinate(i64, i64, i64);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct CoordRange(i64, i64);

const ZERO_COORDINATE: Coordinate = Coordinate(0, 0, 0);

fn main() {
   fn parse_input(contents: &String) -> Vec<NanoBot> {
      let re: Regex = Regex::new(r"pos=<([-\d]+),([-\d]+),([-\d]+)>, r=([-\d]+)").unwrap();

      contents.lines()
         .map(|row| {
            let captures = re.captures(row).unwrap();

            let mut iter = captures.iter().skip(1)
               .map(|capture| {
                  capture.and_then(|m| { m.as_str().parse::<i64>().ok() }).unwrap()
               });

            NanoBot {
               location: Coordinate(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()),
               radius: iter.next().unwrap(),
            }
         })
         .collect()
   }

//   run_tests("23", "test_input_{}.txt",
//             vec![36],
//             &|contents| {
//                let nanobots = parse_input(contents);
//
//                b2(&nanobots)
//             },
//   );

   run_day("23", &|contents, is_sample| {
      let nanobots = parse_input(contents);

      let a_result = a(&nanobots);
      println!("Result A: {}", a_result);

      if is_sample {
         assert_eq!(7, a_result);
      } else {
         let b_result = b(&nanobots);
         println!("Result B: {}", b_result);
      }
   });
}

fn a(nanobots: &Vec<NanoBot>) -> isize {
   let mut max_r = i64::min_value();
   let mut max: NanoBot = NanoBot { location: Coordinate(0, 0, 0), radius: i64::min_value() };

   for bot in nanobots.iter() {
      if bot.radius > max_r {
         max_r = bot.radius;
         max = bot.clone();
      }
   }

   let mut in_range = 0;
   for bot in nanobots.iter() {
      if max.distance(bot) <= max.radius {
         in_range += 1;
      }
   }

   in_range
}

fn b2(nanobots: &Vec<NanoBot>) -> i64 {
   // narrow down our search area
   let mut groups: Vec<Vec<NanoBot>> = vec![];

   for bot in nanobots.iter() {
      let mut any_overlap = false;

      for i in 0..groups.len() {
         let mut group_overlap = false;
         for other in groups[i].iter() {
            if bot.distance(&other) <= bot.radius + other.radius {
               group_overlap = true;
               break;
            }
         }
         groups[i].push(bot.clone());

         any_overlap = any_overlap | group_overlap;
      }

      if !any_overlap {
         groups.push(vec![bot.clone()])
      }
   }

   for group in groups.iter() {
      println!("len: {}", group.len());
      for bot in group.iter() {
         println!("\t{:?}", bot);
      }
      println!();
   }

   println!("{:?}", groups);
   0
}

fn b(nanobots: &Vec<NanoBot>) -> i64 {
   let mut min: Coordinate = Coordinate::max_value();
   let mut max: Coordinate = Coordinate::min_value();

   for bot in nanobots.iter() {
      min.0 = min.0.min(bot.location.0);
      min.1 = min.1.min(bot.location.1);
      min.2 = min.2.min(bot.location.2);

      max.0 = max.0.max(bot.location.0);
      max.1 = max.1.max(bot.location.1);
      max.2 = max.2.max(bot.location.2);
   }

   let (outer_x_range, outer_y_range, outer_z_range) = min.ranges(&max);

   find_closest(nanobots, &outer_x_range, &outer_y_range, &outer_z_range, 0, 0).unwrap().0
      .distance(&ZERO_COORDINATE)
}

fn find_closest(nanobots: &Vec<NanoBot>,
                outer_x_range: &CoordRange,
                outer_y_range: &CoordRange,
                outer_z_range: &CoordRange,
                initial_max: usize,
                debug_offset: usize) -> Option<(Coordinate, usize)> {
   let debug_prefix = "  ".repeat(debug_offset);

   let max_search_area = outer_x_range.distance() as u128 * outer_y_range.distance() as u128 * outer_z_range.distance() as u128;
   if max_search_area < 1_000 {
      return simple_find_closest(nanobots, outer_x_range, outer_y_range, outer_z_range, initial_max, debug_offset);
   }

   let mut max_in_range = initial_max;
   let mut max_ranges: Vec<(CoordRange, CoordRange, CoordRange)> = vec![];

   for x_range in outer_x_range.split().iter() {
      // don't search if the range doesn't make sense
      if !x_range.is_logical() {
         continue;
      }
      let x_mid = x_range.mid();

      for y_range in outer_y_range.split().iter() {
         // don't search if the range doesn't make sense
         if !y_range.is_logical() {
            continue;
         }
         let y_mid = y_range.mid();

         for z_range in outer_z_range.split().iter() {
            // don't search if the range doesn't make sense
            if !z_range.is_logical() {
               continue;
            }

            // where we are searching:
            let center = Coordinate(x_mid, y_mid, z_range.mid());

            let tolerance = ((x_range.distance() / 2) + 1)
               + ((y_range.distance() / 2) + 1)
               + ((z_range.distance() / 2) + 1);

            let mut in_range = 0;
            for bot in nanobots.iter() {
               //println!("{:?} {}", bot, center.distance(&bot.location));

               if center.distance(&bot.location) <= bot.radius + tolerance {
                  in_range += 1;
               }
            }

            if in_range > max_in_range {
               max_in_range = in_range;
               max_ranges = vec![(x_range.clone(), y_range.clone(), z_range.clone())];

               println!("{}new max: {} {:?}", debug_prefix, max_in_range, (x_range.clone(), y_range.clone(), z_range.clone()));
            } else if in_range == max_in_range {
               max_ranges.push((x_range.clone(), y_range.clone(), z_range.clone()));

               //println!("{}other max: {:?}", debug_prefix, (x_range.clone(), y_range.clone(), z_range.clone()));
            }

            println!("in_range: {} {:?} {}", in_range, center, tolerance);
         }
      }
   }

   return None;

   let mut single_max = initial_max;
   let mut single_points: Vec<(Coordinate, usize)> = vec![];

   if max_ranges.len() > 0 {
      //println!("{}found: {:?}", debug_prefix, max_ranges);
   }

   // search the other ranges
   for range in max_ranges {
      //println!("{}Stepping into {:?} {:?} {:?} {} {} {}", debug_prefix, range.0, range.1, range.2, (range.0).distance(), (range.1).distance(), (range.2).distance());

      match find_closest(nanobots, &range.0, &range.1, &range.2, single_max, debug_offset + 1) {
         Some(closest) => {
            if closest.1 > single_max {
               single_max = closest.1;
               single_points = vec![closest];

               println!("{}new_max: {} @ {:?}", debug_prefix, single_max, closest);
            } else if closest.1 == single_max {
               single_points.push(closest);
            }
         }
         None => {}
      }
   }
   //println!("{}done", debug_prefix);

   single_points.sort_unstable_by(|left, right| {
      right.1.cmp(&left.1)
         .then_with(|| {
            left.0.distance(&ZERO_COORDINATE)
               .cmp(&right.0.distance(&ZERO_COORDINATE))
         })
   });

   //println!("{:?}", single_points);

   single_points.first().map(|p| p.clone())
}

fn simple_find_closest(nanobots: &Vec<NanoBot>,
                       x_range: &CoordRange,
                       y_range: &CoordRange,
                       z_range: &CoordRange,
                       initial_max: usize,
                       debug_offset: usize) -> Option<(Coordinate, usize)> {
   let debug_prefix = "  ".repeat(debug_offset);
   //println!("{} simple find over: {}", debug_prefix, x_range.distance() * y_range.distance() * z_range.distance());

   let mut max_in_range = initial_max;
   let mut max_coord = Coordinate(0, 0, 0);
   let mut found = false;

   for x in x_range.range() {
      for y in y_range.range() {
         for z in z_range.range() {
            let current = Coordinate(x, y, z);
            let mut in_range = 0;
            for bot in nanobots.iter() {
               if bot.location.distance(&current) <= bot.radius {
                  in_range += 1;
               }
            }

            if in_range > max_in_range || (
               in_range == max_in_range &&
                  ZERO_COORDINATE.distance(&current) < ZERO_COORDINATE.distance(&max_coord)
            ) {
               max_in_range = in_range;
               max_coord = current.clone();
               found = true;
            }
         }
      }
   }

   if found {
      Some((max_coord, max_in_range))
   } else {
      None
   }
}

impl NanoBot {
   fn distance(&self, other: &NanoBot) -> i64 {
      self.location.distance(&other.location)
   }
}

impl CoordRange {
   fn is_logical(&self) -> bool {
      self.1 >= self.0
   }

   fn split(&self) -> [CoordRange; 2] {
//      println!("{:?}", [
//         Range(self.0, self.mid()),
//         Range(self.mid(), self.1),
//      ]);

//      let left = CoordRange(self.0, self.mid());
//      let right = CoordRange(self.mid(), self.1);
//      [
//         CoordRange(left.0, left.mid()),
//         CoordRange(left.mid(), left.1),
//         CoordRange(right.0, right.mid()),
//         CoordRange(right.mid(), right.1),
//      ]

      [
         CoordRange(self.0, self.mid()),
         CoordRange(self.mid(), self.1),
      ]
   }

   fn mid(&self) -> i64 {
      //println!("mid: {} {} {}", self.0, self.1, (self.1 - self.0) / 2 + self.0);

      (self.1 - self.0) / 2 + self.0
   }

   fn distance(&self) -> i64 {
      self.1 - self.0
   }

   fn range(&self) -> Range<i64> {
      self.0..self.1
   }
}

impl Coordinate {
   fn min_value() -> Coordinate {
      Coordinate(i64::min_value(), i64::min_value(), i64::min_value())
   }

   fn max_value() -> Coordinate {
      Coordinate(i64::max_value(), i64::max_value(), i64::max_value())
   }

   fn distance(&self, other: &Coordinate) -> i64 {
      (self.0 - other.0).abs() +
         (self.1 - other.1).abs() +
         (self.2 - other.2).abs()
   }

   fn ranges(&self, other: &Coordinate) -> (CoordRange, CoordRange, CoordRange) {
      (
         CoordRange(self.0, other.0),
         CoordRange(self.1, other.1),
         CoordRange(self.2, other.2),
      )
   }
}