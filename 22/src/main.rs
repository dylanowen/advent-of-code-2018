use std::fmt;
use std::collections::HashMap;
use regex::Regex;

use priority_queue::PriorityQueue;

use lib::*;
use lib::coordinates::Grid;
use lib::coordinates::Loci;
use lib::coordinates::OffsetLociX;
use lib::coordinates::OffsetLociY;


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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
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

//   run_tests("22", "test_input_{}.txt",
//             vec![3], &|contents| {
//         let (depth, target) = parse_input(contents);
//
//         let cave = build_cave(depth, &target);
//
//         cave.print();
//         println!();
//
//         b(&target, &cave)
//      });

   run_day("22", &|contents, is_sample| {
      let (depth, target) = parse_input(contents);

      let cave = build_cave(depth, &target);

      //cave.print();

      let a_result = a(&target, &cave);
      println!("Result A: {}", a_result);



      if is_sample {
         assert_eq!(114, a_result);
         //assert_eq!(45, b_result);
      }
      else {
         let b_result = b(&target, &cave);
         println!("Result B: {}", b_result);
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

// A* Search
fn find_shortest_path(target: &Loci, cave: &Grid<Region>) -> Option<isize> {
   let start = Loci::new(0, 0);
   let goal = Loci::new(target.x(), target.y());

   let heuristic_cost_estimate = |from: &Loci, tool: &Tool| -> isize {
      let distance = from.sub_loci(&goal);

      let mut tool_tax = 0;
      if *tool == Tool::ClimbingGear {
         tool_tax += 6;
      }

      distance.x().abs() + distance.y().abs() + tool_tax
   };

   let mut closed_set: Vec<(Loci, Tool)> = Vec::new();
   let mut open_set: PriorityQueue<(Loci, Tool), isize> = PriorityQueue::new();
   open_set.push((start, Tool::Torch), 0);

   let mut came_from: HashMap<(Loci, Tool), (Loci, Tool)> = HashMap::new();
//   let mut g_score: HashMap<(Loci, Tool), isize> = HashMap::new();
//   g_score.insert((start, Tool::Torch), 0);

   let mut heuristic_score: HashMap<(Loci, Tool), isize> = HashMap::new();
   heuristic_score.insert((start, Tool::Torch), heuristic_cost_estimate(&start, &Tool::Torch));

   while !open_set.is_empty() {
      let ((current, current_tool), current_score) = open_set.pop().unwrap();
//         open_set.iter()
//            .fold((goal.clone(), Tool::Torch, isize::max_value()), |best, node| {
//               let score = *heuristic_score.get(node).unwrap();
//               if score < best.2 {
//                  (node.0.clone(), node.1, score)
//               } else {
//                  best
//               }
//            })
//      };


//      if loop_count % 1000 == 0 {
//         for y in cave.y_range() {
//            for x in cave.x_range() {
//               let loci = Loci::new(x, y);
//
//               if *target == loci {
//                  print!("T ");
//               } else {
//                  let found = came_from.iter()
//                     .find_map(|l| {
//                        if (l.0).0 == loci {
//                           Some((l.1).0)
//                        } else {
//                           None
//                        }
//                     });
//
//                  match found {
//                     Some(from) => {
//                        if from.x() < x {
//                           print!("< ")
//                        } else if from.x() > x {
//                           print!("> ")
//                        } else if from.y() < y {
//                           print!("^ ")
//                        } else {
//                           print!("v ")
//                        }
//                     }
//                     None => {
//                        print!("{} ", cave.get(x, y).region_type._simple_string());
//                     }
//                  }
//               }
//            }
//            println!();
//         }
//         println!();
//      }


      if current == goal {
//         let mut path = Vec::new();
//         let mut back_track = goal;
//         while back_track != start {
//            path.push(back_track);
//            back_track = came_from.get(&back_track).unwrap().clone();
//         }
//         path.reverse();
//
//         for y in cave.y_range() {
//            for x in cave.x_range() {
//               if path.contains(&Loci::new(x, y)) {
//                  print!("\u{001B}[33m*\u{001B}[0m ");
//               } else if target.x() == x && target.y() == y {
//                  print!("\u{001B}[32mT\u{001B}[0m ");
//               } else {
//                  print!("{} ", cave.get(x, y));
//               }
////               if target.x() == x && target.y() == y {
////                  print!("T ");
////               } else if path.contains(&Loci::new(x, y)) {
////                  print!("* ");
////               } else {
////                  print!("{} ", cave.get(x, y).region_type.simple_string());
////               }
//            }
//            println!();
//         }

         if !RegionType::Rocky.is_tool_valid(&current_tool) {
            // add seven for changing our tool to move
            return Some(current_score + 7);
         } else {
            // add one for a successful move
            return Some(current_score + 1);
         }
      }

      closed_set.push((current, current_tool));


      let neighbors: Vec<Loci> = current.neighbors().iter()
         .filter(|neighbor| {
            neighbor.x() >= 0 && neighbor.y() >= 0
         })
         // TODO do I need to clone these?
         .map(|neighbor| neighbor.clone())
         .collect();

      for neighbor in neighbors {
         // get the region for this neighbor
         let neighbor_region = cave.get_loci(&neighbor);

         for i in 0..TOOLS.len() {
            let tool = TOOLS[i].clone();
            // check if this tool is allowed for this region
           if !neighbor_region.is_tool_valid(&tool) {
              continue
           }

            // check if this region has already been checked
            if closed_set.contains(&(neighbor, tool)) {
               continue;
            }

            let move_cost;
            if tool == current_tool {
               move_cost = 1;
            } else {
               move_cost = 7;
            }

            //let tentative_g_score = *g_score.get(&(current, current_tool)).unwrap() + move_cost;
            let tentative_g_score = current_score + move_cost;

            // check if we already know about this neighbor / tool
            match open_set.get_priority(&(neighbor, tool)) {
               Some(existing_score) => {
                  if tentative_g_score >= *existing_score {
                     continue;
                  }
               }
               None => {}
            }

            // best path for now so record it
            open_set.push((neighbor, tool),  tentative_g_score);
            came_from.insert((neighbor, tool), (current.clone(), current_tool.clone()));
            //g_score.insert((neighbor.clone(), tool.clone()), tentative_g_score);
            heuristic_score.insert((neighbor, tool), tentative_g_score + heuristic_cost_estimate(&neighbor, &tool));
         }
      }
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