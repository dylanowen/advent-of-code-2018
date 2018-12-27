use std::fmt;
use std::collections::BTreeSet;
use regex::Regex;
use std::cmp::Reverse;
use std::cmp::Ordering;

use priority_queue::PriorityQueue;

use common::*;
use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;


#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Region {
   region_type: RegionType,
   geologic_index: isize,
   erosion_level: isize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum RegionType {
   Rocky,
   Wet,
   Narrow,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum Tool {
   Torch,
   ClimbingGear,
   Neither,
}

const TOOLS: [Tool; 3] = [
   Tool::Torch,
   Tool::ClimbingGear,
   Tool::Neither,
];

fn main() {
   fn parse_input(contents: &String) -> (isize, Loci) {
      let depth_re: Regex = Regex::new(r"depth: (\d+)").unwrap();
      let target_re: Regex = Regex::new(r"target: (\d+),(\d+)").unwrap();

      let mut lines = contents.lines();

      let depth = lines.next()
         .map(|row| {
            depth_re.captures(row).unwrap()[1].parse().unwrap()
         })
         .unwrap();

      let target: Loci = lines.next()
         .map(|row| {
            let captures = target_re.captures(row).unwrap();

            Loci::new(captures[1].parse().unwrap(), captures[2].parse().unwrap())
         })
         .unwrap();

      (depth, target)
   }

   run_tests("22", "test_input_{}.txt",
             vec![18], &|contents| {
         let (depth, target) = parse_input(contents);

         let cave = build_cave(depth, &target);

         cave.print();
         println!();

         b(&target, &cave)
      });

   run_day("22", &|contents, is_sample| {
      let (depth, target) = parse_input(contents);

      let cave = build_cave(depth, &target);

      //cave.print();

      let a_result = a(&target, &cave);
      println!("Result A: {}", a_result);
      let b_result = b(&target, &cave);
      println!("Result B: {}", b_result);

      if is_sample {
         assert_eq!(114, a_result);
         assert_eq!(45, b_result);
      }
   });
}

fn a(target: &Loci, cave: &Grid<Region>) -> usize {
   let mut risk = 0;

   for y in 0..=target.y() {
      for x in 0..=target.x() {
         risk += cave.get(x, y).risk_level();
      }
   }

   risk
}

fn b(target: &Loci, cave: &Grid<Region>) -> isize {
   find_shortest_path(target, cave).unwrap()
}

#[derive(Copy, Clone, Debug, Hash)]
struct PathScore {
   location: Loci,
   tool: Tool,
   best_path_minutes: isize,
}

impl PartialEq for PathScore {
   fn eq(&self, other: &PathScore) -> bool {
      // don't check our minutes for equality
      self.location == other.location && self.tool == other.tool
   }
}

impl Eq for PathScore {}

impl Ord for PathScore {
   fn cmp(&self, other: &PathScore) -> Ordering {
      self.location.cmp(&other.location)
         .then(self.tool.cmp(&other.tool))
   }
}

impl PartialOrd for PathScore {
   fn partial_cmp(&self, other: &PathScore) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl fmt::Display for PathScore {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}:({},{})[{}]", self.best_path_minutes, self.location.x(), self.location.y(), self.tool)
   }
}

// A* Search
fn find_shortest_path(target: &Loci, cave: &Grid<Region>) -> Option<isize> {
   let mut debug_grid = Grid::new((0, Tool::Neither), cave.width(), cave.height());

   let start = PathScore {
      location: Loci::new(0, 0),
      tool: Tool::Torch,
      best_path_minutes: 0,
   };
   let goal = target.clone();

   let heuristic_cost_estimate = |from: &PathScore| -> isize {
      let distance = from.location.sub_loci(&goal);

      let mut tool_tax = 0;
      if from.tool == Tool::ClimbingGear {
         tool_tax += 7;
      }

      distance.x().abs() + distance.y().abs() + tool_tax
   };

   let mut closed_set: BTreeSet<PathScore> = BTreeSet::new();
   let mut open_set: PriorityQueue<PathScore, Reverse<isize>> = PriorityQueue::new();
   open_set.push(start, Reverse(heuristic_cost_estimate(&start)));

//   let mut real_score: HashMap<(Loci, Tool), isize> = HashMap::new();
//   real_score.insert((start, Tool::Torch), 0);

   while !open_set.is_empty() {
      for y in debug_grid.y_range() {
         for x in debug_grid.x_range() {
            print!("{:2}{} ", debug_grid.get(x, y).0, debug_grid.get(x, y).1);
         }
         println!();
      }
      for (p_score, h) in open_set.clone().into_sorted_iter() {
         println!("{} {} ", h.0, p_score);
      }
//      println!();
//      for p_score in closed_set.iter() {
//         println!("{}", p_score);
//      }
      println!();

      let (current, _) = open_set.pop().unwrap();

      // debug
      let last_debug = debug_grid.get_loci(&current.location);
      if last_debug.0 <= 0 || last_debug.0 > current.best_path_minutes {
         debug_grid.set_loci(&current.location, (current.best_path_minutes, current.tool));
      }

      if current.location == goal {
         for y in debug_grid.y_range() {
            for x in debug_grid.x_range() {
               if x == target.x() && y == target.y() {
                  print!("{:3}* ", debug_grid.get(x, y).0);
               } else {
                  print!("{:3}{} ", debug_grid.get(x, y).0, debug_grid.get(x, y).1);
               }
            }
            println!();
         }

         return Some(current.best_path_minutes);
      }

      closed_set.insert(current);

      // get our possible moves
      let mut possible_moves: Vec<PathScore> = current.location.valid_neighbors(cave).iter()
         .filter_map(|neighbor| {
            // get the region for this neighbor
            let neighbor_region = cave.get_loci(&neighbor);

            if neighbor_region.is_tool_valid(&current.tool) {
               Some(PathScore {
                  location: *neighbor,
                  tool: current.tool,
                  best_path_minutes: current.best_path_minutes + 1,
               })
            } else {
               None
            }
         })
         .collect();

      // get our possible tool changes
      let tool_changes = TOOLS.iter()
         .filter_map(|tool| {
            if *tool != current.tool {
               Some(PathScore {
                  location: current.location,
                  tool: tool.clone(),
                  best_path_minutes: current.best_path_minutes + 7,
               })
            } else {
               None
            }
         });

      possible_moves.extend(tool_changes);

      for score in possible_moves {
         // check if this region has already been checked
         if closed_set.contains(&score) {
            continue;
         }

         // check if we already know about this neighbor / tool
         match open_set.iter().find(|(p_score, _)| **p_score == score) {
            Some((old_value, _)) => {
               // if our tentative real_score is worse, return
               if score.best_path_minutes >= old_value.best_path_minutes {
                  continue;
               }
            }
            None => {}
         }

         // best path for now so record it
         open_set.push(score, Reverse(score.best_path_minutes + heuristic_cost_estimate(&score)));
      }


      //println!("{:?}", current.location.valid_neighbors(cave));
//      for neighbor in current.location.valid_neighbors(cave) {
//
//
//         for i in 0..TOOLS.len() {
//            let tool = TOOLS[i].clone();
//            // check if this tool is allowed for this region
//            if !neighbor_region.is_tool_valid(&tool) {
//               continue;
//            }
//
//            let mut neighbor_score = PathScore {
//               location: neighbor,
//               tool,
//               best_path_minutes: isize::max_value(),
//            };
//
//            // check if this region has already been checked
//            if closed_set.contains(&neighbor_score) {
//               continue;
//            }
//
//            let move_cost;
//            if tool == current.tool {
//               move_cost = 1;
//            } else {
//               move_cost = 8;
//            }
//
//            let tentative_real_score = current.best_path_minutes + move_cost;
//
//            // check if we already know about this neighbor / tool
//            match open_set.iter().find(|(p_score, _)| **p_score == neighbor_score) {
//               Some((old_value, _)) => {
//                  // if our tentative real_score is worse, return
//                  if tentative_real_score >= old_value.best_path_minutes {
//                     continue;
//                  }
//               }
//               None => {}
//            }
//
//            // best path for now so record it
//            neighbor_score.best_path_minutes = tentative_real_score;
//            //println!("{} for {}", neighbor_score, tentative_real_score + heuristic_cost_estimate(&neighbor, &tool));
//            open_set.push(neighbor_score, Reverse(tentative_real_score + heuristic_cost_estimate(&neighbor, &tool)));
//
//            //real_score.insert((neighbor, tool), tentative_real_score);
//         }
//      }
   }

   // no path could be found
   return None;
}

fn build_cave(depth: isize, target: &Loci) -> Grid<Region> {
   let dimensions = (target.x() as usize * 2).max(target.y() as usize * 2).max(10);
   let mut cave = Grid::new(
      Region { region_type: RegionType::Rocky, geologic_index: 0, erosion_level: 0 },
      dimensions.min(150),
      dimensions.min(900),
   );

   for y in cave.y_range() {
      for x in cave.x_range() {
         let geologic_index: isize;
         if (x == 0 && y == 0) || (x == target.x() && y == target.y()) {
            //entrance or target
            geologic_index = 0;
         } else if y == 0 {
            geologic_index = x * 16807;
         } else if x == 0 {
            geologic_index = y * 48271;
         } else {
            geologic_index = cave.get(x - 1, y).erosion_level * cave.get(x, y - 1).erosion_level;
         }

         let erosion_level = (geologic_index + depth) % 20183;

         let region_type = match erosion_level % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            _ => RegionType::Narrow,
         };

         cave.set(x, y, Region {
            region_type,
            geologic_index,
            erosion_level,
         })
      }
   }

   cave
}

impl Region {
   fn risk_level(&self) -> usize {
      match self.region_type {
         RegionType::Rocky => 0,
         RegionType::Wet => 1,
         RegionType::Narrow => 2,
      }
   }

   fn is_tool_valid(&self, tool: &Tool) -> bool {
      self.region_type.is_tool_valid(tool)
   }
}

impl RegionType {
   fn _simple_string(&self) -> &str {
      match *self {
         RegionType::Rocky => ".",
         RegionType::Wet => "=",
         RegionType::Narrow => "|",
      }
   }

   fn is_tool_valid(&self, tool: &Tool) -> bool {
      match *self {
         RegionType::Rocky => {
            if *tool == Tool::Neither {
               // You cannot use neither (you'll likely slip and fall).
               return false;
            }
         }
         RegionType::Wet => {
            if *tool == Tool::Torch {
               // if it gets wet, you won't have a light source
               return false;
            }
         }
         RegionType::Narrow => {
            if *tool == Tool::ClimbingGear {
               // You cannot use the climbing gear (it's too bulky to fit)
               return false;
            }
         }
      }

      return true;
   }
}

impl fmt::Display for Region {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.region_type.fmt(f)
   }
}

impl fmt::Display for RegionType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         RegionType::Rocky => write!(f, "\u{001B}[30m.\u{001B}[0m"),
         RegionType::Wet => write!(f, "\u{001B}[34m=\u{001B}[0m"),
         RegionType::Narrow => write!(f, "\u{001B}[31m|\u{001B}[0m"),
      }
   }
}

impl fmt::Display for Tool {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         Tool::Torch => write!(f, "T"),
         Tool::ClimbingGear => write!(f, "C"),
         Tool::Neither => write!(f, " "),
      }
   }
}