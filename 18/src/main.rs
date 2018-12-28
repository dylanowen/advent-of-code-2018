use std::fmt;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use common::*;
use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

use crate::shared::*;

mod shared;

const ONE_MASK: usize = 0b1;

fn main() {
   run_day("18", &|contents, is_sample| {
      let area = parse_input(contents);

      let a_result = a(&area);
      println!("Result A: {}", a_result);

      if is_sample {
         assert_eq!(1147, a_result);
      }
      else {
         let b_result = b(&area);
         println!("Result B: {}", b_result);
      }
   });
}

fn a(initial_area: &Grid<Acre>) -> usize {
   run_lumber(10, initial_area)
}

fn b(initial_area: &Grid<Acre>) -> usize {
   run_lumber(1000000000, initial_area)
}

fn run_lumber(minutes: usize, initial_area: &Grid<Acre>) -> usize {
   let areas = [
      RefCell::new(initial_area.clone()),
      RefCell::new(initial_area.clone()),
   ];

   let mut area_index = 0;
   let mut last_area_index = 1;

   //let mut cycle_finder: HashMap<u64, Grid<Acre>> = HashMap::new();
   let mut cycle_finder = BTreeSet::new();

   for minute in 0..minutes {
      area_index = last_area_index;
      last_area_index = !last_area_index & ONE_MASK;

      next_lumberyard(&areas[last_area_index].borrow(), &mut areas[area_index].borrow_mut());

      // find cycles in our game
      let hash = grid_hash(&areas[area_index].borrow());
      if !cycle_finder.insert(hash) {
         // found a cycle!
         // subtract 1 from minutes, because we finished this minute but didn't loop far enough to count it
         return run_cycle_lumber(minutes - minute - 1, &areas[area_index].borrow());
      }
   }

   return get_area_score(&areas[area_index].borrow());
}

fn run_cycle_lumber(minutes: usize, initial_area: &Grid<Acre>) -> usize {
   let initial_hash = grid_hash(initial_area);
   let width = initial_area.width();
   let height = initial_area.height();
   let mut areas = vec![
      RefCell::new(initial_area.clone())
   ];

   let mut area_index = 0;
   let mut last_area_index;

   let mut minute = 0;
   while minute < minutes {
      last_area_index = area_index;
      area_index = areas.len();

      let area = RefCell::new(Grid::new(Acre::Open, width, height));

      next_lumberyard(&areas[last_area_index].borrow(), &mut area.borrow_mut());

      minute += 1;

      let hash = grid_hash(&area.borrow());
      if initial_hash == hash {
         // we looped in our cycle so set our index to the start and break out
         area_index = 0;

         break;
      }
      else {
         // no loop so add this area
         areas.push(area);
      }
   }

   //found our cycle loop
   while minute < minutes {
      area_index += 1;
      if area_index >= areas.len() {
         area_index = 0;
      }

      minute += 1;
   }

   return get_area_score(&areas[area_index].borrow());
}

fn grid_hash<T: Hash>(grid: &Grid<T>) -> u64 {
   let mut hasher = DefaultHasher::new();
   grid.hash(&mut hasher);
   hasher.finish()
}

fn get_area_score(area: &Grid<Acre>) -> usize {
   let mut tree_count = 0;
   let mut lumber_count = 0;
   for y in area.y_range() {
      for x in area.x_range() {
         match area.get(x, y) {
            Acre::Open => {}
            Acre::Tree => tree_count += 1,
            Acre::Lumberyard => lumber_count += 1,
         };
      }
   }

   return tree_count * lumber_count;
}

impl fmt::Display for Acre {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         Acre::Tree => write!(f, "\u{001B}[32m|\u{001B}[0m"),
         Acre::Lumberyard => write!(f, "\u{001B}[33m#\u{001B}[0m"),
         Acre::Open => write!(f, "\u{001B}[30m.\u{001B}[0m"),
      }
   }
}