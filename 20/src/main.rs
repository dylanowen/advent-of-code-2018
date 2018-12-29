use std::collections::BTreeSet;

use common::*;
use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

use crate::shared::*;

mod shared;

fn main() {
   run_tests("20", "test_input_{}.txt",
             vec![3, 10, 18, 23, 31],
             &|contents| {
                let (center, map) = build_map(parse_input(contents));

                ab(&center, &map).0
             },
   );

   run_day_real("20", &|contents, _is_sample| {
      let (map, center) = build_map(parse_input(contents));

      let (a_result, b_result) = ab(&map, &center);
      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);
   });
}

fn ab(start: &Loci, map: &Grid<MapFeature>) -> (usize, usize) {
   let mut distance_grid = Grid::new_offset(
      usize::max_value(),
      map.width(),
      map.height(),
      map.x_min(),
      map.y_min(),
   );
   distance_grid.set_loci(start, 0);

   let mut locations = BTreeSet::new();
   locations.insert(start.clone());

   let mut max = 0;
   let mut count = 0;
   while !locations.is_empty() {
      let mut next_locations: BTreeSet<Loci> = BTreeSet::new();

      for location in locations.iter() {
         let distance = distance_grid.get_loci(location);

         let neighbors: Vec<Loci> = location.neighbors().iter()
            .filter_map(|neighbor| {
               if *map.get_loci(neighbor) == MapFeature::Door {
                  let mut double = neighbor.sub_loci(location);
                  double = double.add_loci(&double);

                  // move into the room
                  Some(location.add_loci(&double))
               } else {
                  None
               }
            })
            .collect();

         let neighbor_distance = distance + 1;
         for neighbor in neighbors {
            let last_distance = *distance_grid.get_loci(&neighbor);

            if neighbor_distance < last_distance {
               max = max.max(neighbor_distance);
               if neighbor_distance >= 1000 {
                  count += 1;
               }

               distance_grid.set_loci(&neighbor, neighbor_distance);

               next_locations.insert(neighbor);
            }
         }
      }

      locations = next_locations;
   }

   (max, count)
}

//
//fn a(initial_area: &Grid<Acre>) -> usize {
//   run_lumber(10, initial_area)
//}
//
//fn b(initial_area: &Grid<Acre>) -> usize {
//   run_lumber(1000000000, initial_area)
//}
//
//fn run_lumber(minutes: usize, initial_area: &Grid<Acre>) -> usize {
//   let areas = [
//      RefCell::new(initial_area.clone()),
//      RefCell::new(initial_area.clone()),
//   ];
//
//   let mut area_index = 0;
//   let mut last_area_index = 1;
//
//   //let mut cycle_finder: HashMap<u64, Grid<Acre>> = HashMap::new();
//   let mut cycle_finder = BTreeSet::new();
//
//   for minute in 0..minutes {
//      area_index = last_area_index;
//      last_area_index = !last_area_index & ONE_MASK;
//
//      run_minute(&areas[last_area_index].borrow(), &mut areas[area_index].borrow_mut());
//
//      // find cycles in our game
//      let hash = grid_hash(&areas[area_index].borrow());
//      if !cycle_finder.insert(hash) {
//         // found a cycle!
//         // subtract 1 from minutes, because we finished this minute but didn't loop far enough to count it
//         return run_cycle_lumber(minutes - minute - 1, &areas[area_index].borrow());
//      }
//   }
//
//   return get_area_score(&areas[area_index].borrow());
//}
//
//fn run_cycle_lumber(minutes: usize, initial_area: &Grid<Acre>) -> usize {
//   let initial_hash = grid_hash(initial_area);
//   let width = initial_area.width();
//   let height = initial_area.height();
//   let mut areas = vec![
//      RefCell::new(initial_area.clone())
//   ];
//
//   let mut area_index = 0;
//   let mut last_area_index;
//
//   let mut minute = 0;
//   while minute < minutes {
//      last_area_index = area_index;
//      area_index = areas.len();
//
//      let area = RefCell::new(Grid::new(Acre::Open, width, height));
//
//      run_minute(&areas[last_area_index].borrow(), &mut area.borrow_mut());
//
//      minute += 1;
//
//      let hash = grid_hash(&area.borrow());
//      if initial_hash == hash {
//         // we looped in our cycle so set our index to the start and break out
//         area_index = 0;
//
//         break;
//      }
//      else {
//         // no loop so add this area
//         areas.push(area);
//      }
//   }
//
//   //found our cycle loop
//   while minute < minutes {
//      area_index += 1;
//      if area_index >= areas.len() {
//         area_index = 0;
//      }
//
//      minute += 1;
//   }
//
//   return get_area_score(&areas[area_index].borrow());
//}
//
//fn run_minute(last_area: &Grid<Acre>, area: &mut Grid<Acre>) {
//   for y in last_area.y_range() {
//      for x in last_area.x_range() {
//         let (_, tree_count, lumberyard_count) = count_adjacent(x, y, &last_area);
//
//         let next_acre: Acre = match last_area.get(x, y) {
//            Acre::Open =>
//               if tree_count >= 3 {
//                  Acre::Tree
//               } else {
//                  Acre::Open
//               }
//            Acre::Tree =>
//               if lumberyard_count >= 3 {
//                  Acre::Lumberyard
//               } else {
//                  Acre::Tree
//               }
//            Acre::Lumberyard =>
//               if lumberyard_count >= 1 && tree_count >= 1 {
//                  Acre::Lumberyard
//               } else {
//                  Acre::Open
//               }
//         };
//
//         area.set(x, y, next_acre);
//      }
//   }
//}
//
//fn grid_hash<T: Hash>(grid: &Grid<T>) -> u64 {
//   let mut hasher = DefaultHasher::new();
//   grid.hash(&mut hasher);
//   hasher.finish()
//}
//
//fn get_area_score(area: &Grid<Acre>) -> usize {
//   let mut tree_count = 0;
//   let mut lumber_count = 0;
//   for y in area.y_range() {
//      for x in area.x_range() {
//         match area.get(x, y) {
//            Acre::Open => {}
//            Acre::Tree => tree_count += 1,
//            Acre::Lumberyard => lumber_count += 1,
//         };
//      }
//   }
//
//   return tree_count * lumber_count;
//}
//
//fn count_adjacent(x: isize, y: isize, area: &Grid<Acre>) -> (usize, usize, usize) {
//   let mut open_count = 0;
//   let mut tree_count = 0;
//   let mut lumberyard_count = 0;
//
//   let min_x = area.x_min().max(x - 1);
//   let max_x = (area.x_max() - 1).min(x + 1);
//   let min_y = area.y_min().max(y - 1);
//   let max_y = (area.y_max() - 1).min(y + 1);
//
//   for yi in min_y..=max_y {
//      for xi in min_x..=max_x {
//         if x != xi || y != yi {
//            match area.get(xi, yi) {
//               Acre::Open => open_count += 1,
//               Acre::Tree => tree_count += 1,
//               Acre::Lumberyard => lumberyard_count += 1,
//            }
//         }
//      }
//   }
//
//   (open_count, tree_count, lumberyard_count)
//}
//
//impl fmt::Display for Acre {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//      match *self {
//         Acre::Tree => write!(f, "\u{001B}[32m|\u{001B}[0m"),
//         Acre::Lumberyard => write!(f, "\u{001B}[33m#\u{001B}[0m"),
//         Acre::Open => write!(f, "\u{001B}[30m.\u{001B}[0m"),
//      }
//   }
//}