use regex::Regex;
use std::option::Option::{Some, None};
use std::cmp::Ordering;

use priority_queue::PriorityQueue;

use common::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct NanoBot {
   location: Coordinate,
   radius: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Coordinate(i64, i64, i64);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct CoordRange(i64, i64);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Region {
   x: CoordRange,
   y: CoordRange,
   z: CoordRange,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RegionScore {
   potential_bots: usize,
   zero_distance: i64,
}

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

   run_tests("23", "test_input_{}.txt",
             vec![36],
             &|contents| {
                let nanobots = parse_input(contents);

                b(&nanobots)
             },
   );

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

   let region = Region {
      x: outer_x_range,
      y: outer_y_range,
      z: outer_z_range,
   };

   find_closest(nanobots, &region).unwrap().distance(&ZERO_COORDINATE)
}

fn find_closest(nanobots: &Vec<NanoBot>, entire_region: &Region) -> Option<Coordinate> {
   let mut candidates: PriorityQueue<Region, RegionScore> = PriorityQueue::new();
   candidates.push(entire_region.clone(), RegionScore {
      potential_bots: entire_region.potential_bots(nanobots),
      zero_distance: entire_region.zero_distance(),
   });

   while !candidates.is_empty() {
      //let mut i = 0;
      //for (region, score) in candidates.clone().into_sorted_iter() {
      //   if i > 20 {
      //      break;
      //   }
      //   println!("{:?} {:?} ", region._area(), score);
      //   i += 1;
      //}
      //println!();

      let (region, _) = candidates.pop().unwrap();

      if region.x.distance() == 1 && region.y.distance() == 1 && region.z.distance() == 1 {
         return Some(region.lower_bound());
      }

      for split_region in region.split() {
         candidates.push(split_region, RegionScore {
            potential_bots: split_region.potential_bots(nanobots),
            zero_distance: split_region.zero_distance(),
         });
      }
   }

   None
}

impl NanoBot {
   fn distance(&self, other: &NanoBot) -> i64 {
      self.location.distance(&other.location)
   }
}

impl Region {
   fn split(&self) -> Vec<Region> {
      let mut split_regions = vec![];
      for x in self.x.split().iter() {
         if x.distance() == 0 {
            continue;
         }
         for y in self.y.split().iter() {
            if y.distance() == 0 {
               continue;
            }
            for z in self.z.split().iter() {
               if z.distance() == 0 {
                  continue;
               }

               split_regions.push(Region {
                  x: x.clone(),
                  y: y.clone(),
                  z: z.clone(),
               })
            }
         }
      }

      split_regions
   }

   fn potential_bots(&self, nanobots: &Vec<NanoBot>) -> usize {
      let center = Coordinate(self.x.mid(), self.y.mid(), self.z.mid());

      // if this divides evenly our center is offset, so give us some more room
      let tolerance = (self.x.distance() / 2) + if self.x.distance() % 2 == 0 { 1 } else { 0 } +
         (self.y.distance() / 2) + if self.y.distance() % 2 == 0 { 1 } else { 0 } +
         (self.z.distance() / 2) + if self.z.distance() % 2 == 0 { 1 } else { 0 };

      let mut in_range = 0;
      for bot in nanobots.iter() {
         if center.distance(&bot.location) <= bot.radius + tolerance {
            in_range += 1;
         }
      }

      in_range
   }

   fn zero_distance(&self) -> i64 {
      let mut lowest = i64::max_value();
      for x in &[self.x.0, self.x.1] {
         for y in &[self.y.0, self.y.1] {
            for z in &[self.z.0, self.z.1] {
               let distance = ZERO_COORDINATE.distance(&Coordinate(*x, *y, *z));

               if distance < lowest {
                  lowest = distance;
               }
            }
         }
      }

      lowest
   }

   fn _area(&self) -> i128 {
      self.x.distance() as i128 * self.y.distance() as i128 * self.z.distance() as i128
   }

   fn lower_bound(&self) -> Coordinate {
      Coordinate(self.x.0, self.y.0, self.z.0)
   }
}

impl Ord for RegionScore {
   fn cmp(&self, other: &RegionScore) -> Ordering {
      self.potential_bots.cmp(&other.potential_bots)
         .then_with(|| {
            self.zero_distance.cmp(&other.zero_distance)
         })
   }
}

impl PartialOrd for RegionScore {
   fn partial_cmp(&self, other: &RegionScore) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl CoordRange {
   fn split(&self) -> [CoordRange; 2] {
      [
         CoordRange(self.0, self.mid()),
         CoordRange(self.mid(), self.1),
      ]
   }

   fn mid(&self) -> i64 {
      (self.1 - self.0) / 2 + self.0
   }

   fn distance(&self) -> i64 {
      self.1 - self.0
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