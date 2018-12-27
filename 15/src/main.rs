use std::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

use common::*;
use common::coordinates::Grid;
use common::coordinates::Loci;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Unit {
   species: Species,
   x: isize,
   y: isize,
   health: isize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Species {
   Elf,
   Goblin,
}

const GOBLIN_ATTACK_POWER: isize = 3;

fn main() {
   fn parse_input(contents: &String) -> (Vec<Unit>, Grid<bool>) {
      let input: Vec<Vec<char>> = contents.lines()
         .map(|line| line.chars().collect())
         .collect();

      let width = input[0].len();
      let height = input.len();

      let mut map = Grid::new(false, width, height);
      let mut units = vec![];
      for (y, row) in input.iter().enumerate() {
         for (x, c) in row.iter().enumerate() {
            match *c {
               '#' => map.set(x as isize, y as isize, true),
               'E' => units.push(Unit {
                  species: Species::Elf,
                  x: x as isize,
                  y: y as isize,
                  health: 200,
               }),
               'G' => units.push(Unit {
                  species: Species::Goblin,
                  x: x as isize,
                  y: y as isize,
                  health: 200,
               }),
               '.' => {}
               u => panic!("Unexpected character: {}", u),
            }
         }
      }

      (units, map)
   }

   run_tests("15", "test_input_{}.txt",
             vec![36334, 39514, 27755, 28944, 18740, 13400, 13987, 10234],
             &|contents| {
                let (units, map) = parse_input(contents);

                a(&units, &map)
             },
   );

   run_day("15", &|contents, is_sample| {
      let (units, map) = parse_input(contents);

      let a_result = a(&units, &map);
      let b_result = b(&units, &map);

      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);

      if is_sample {
         assert_eq!(27730, a_result);
         assert_eq!(4988, b_result);
      }
   });
}

fn a(input_units: &Vec<Unit>, map: &Grid<bool>) -> isize {
   main_game(input_units, map, 3, &|_| false)
}

fn b(input_units: &Vec<Unit>, map: &Grid<bool>) -> isize {
   for attack_power in 4..200 {
      let game_result = main_game(input_units, map, attack_power, &|unit| {
         unit.species == Species::Elf
      });

      if game_result > 0 {
         return game_result;
      }
   }

   panic!("no answer");
}

fn main_game<R>(input_units: &Vec<Unit>,
                map: &Grid<bool>,
                elf_attack_power: isize,
                quit_on_death: &R) -> isize where
   R: Fn(&Unit) -> bool {
   let mut units: Vec<RefCell<Unit>> = input_units.iter()
      .map(|u| RefCell::new(u.clone()))
      .collect();

   let mut rounds = 0;
   'outer: loop {
      //print_map(&units, map);

      // sort our units
      units.sort();

      for i in 0..units.len() {
         let mut unit = units[i].borrow_mut();

         // if our unit is "dead" skip it
         if !unit.is_alive() {
            continue;
         }

         let alive_units = units.iter().enumerate()
            .filter_map(|(index, unit)| {
               if i != index && unit.borrow().is_alive() {
                  Some(unit)
               } else {
                  None
               }
            })
            .collect();
         let alive_enemies = find_species(unit.species.enemy(), &alive_units);

         // no enemies to fight so we're done
         if alive_enemies.is_empty() {
            break 'outer;
         }

         let mut enemies_to_attack = unit.in_range_enemies(&alive_enemies);

         if enemies_to_attack.is_empty() {
            // not next to an enemy, so lets move

            match unit.find_closest_enemy_path(&alive_enemies, &alive_units, &map) {
               Some(path) => {
                  let next_location = path[0];

                  //println!("{},{} -> {},{}", unit.x, unit.y, next_location.x(), next_location.y());

                  unit.x = next_location.x();
                  unit.y = next_location.y();

                  // see if we found any enemies after moving
                  enemies_to_attack = unit.in_range_enemies(&alive_enemies);
               }
               None => {
                  //println!("{},{} -> X", unit.x, unit.y);
               }
            }
         }

         // find a target to attack
         let maybe_target = &mut enemies_to_attack.iter()
            .fold(None, |result: Option<&RefCell<Unit>>, enemy| {
               match result {
                  Some(other) => Some(if enemy.borrow().health < other.borrow().health { *enemy } else { other }),
                  None => Some(*enemy)
               }
            });

         match maybe_target {
            Some(target) => {
               // we have an enemy to attack
               target.borrow_mut().health -= match unit.species {
                  Species::Goblin => GOBLIN_ATTACK_POWER,
                  Species::Elf => elf_attack_power,
               };

               // on death, see if we should quit
               if target.borrow().health <= 0 {
                  if quit_on_death(&target.borrow()) {
                     return 0;
                  }
               }
            }
            None => {}
         }
      }

      rounds += 1;
   }

   let remaining_hit_points = units.iter()
      .filter(|u| u.borrow().is_alive())
      .fold(0, |sum, u| sum + u.borrow().health);

   return rounds * remaining_hit_points;
}

fn contains_unit(x: isize, y: isize, units: &Vec<&RefCell<Unit>>) -> bool {
   units.iter()
      .filter(|u| u.borrow().is_alive())
      .find(|u| u.borrow().x == x && u.borrow().y == y)
      .is_some()
}

fn find_species<'a>(species: Species, alive_units: &Vec<&'a RefCell<Unit>>) -> Vec<&'a RefCell<Unit>> {
   alive_units.iter()
      .filter(|u| u.borrow().species == species)
      .map(|c| *c)
      .collect()
}

fn is_space_open(x: isize,
                 y: isize,
                 alive_units: &Vec<&RefCell<Unit>>,
                 map: &Grid<bool>) -> bool {
   !*map.get(x, y) && !contains_unit(x, y, alive_units)
}

impl PartialOrd for Unit {
   fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Unit {
   fn is_alive(&self) -> bool {
      self.health > 0
   }

   fn in_range_enemies<'a>(&self, alive_enemies: &Vec<&'a RefCell<Unit>>) -> Vec<&'a RefCell<Unit>> {
      alive_enemies.iter()
         .filter_map(|enemy| {
            let x_dist = (enemy.borrow().x - self.x).abs();
            let y_dist = (enemy.borrow().y - self.y).abs();

            if x_dist + y_dist == 1 {
               Some(*enemy)
            } else {
               None
            }
         })
         .collect()
   }

   fn find_closest_enemy_path(&self,
                              alive_enemies: &Vec<&RefCell<Unit>>,
                              alive_units: &Vec<&RefCell<Unit>>,
                              map: &Grid<bool>) -> Option<Vec<Loci>> {
      alive_enemies.iter()
         .filter_map(|enemy_cell| {
            let enemy = enemy_cell.borrow();

            self.find_shortest_path(enemy.x, enemy.y, alive_units, map)
         })
         .fold(None, |maybe_shortest_path, path| {
            match maybe_shortest_path {
               Some(shortest_path) => Some(
                  if path.len() < shortest_path.len() {
                     path
                  } else if path.len() == shortest_path.len() {
                     let last = path.len() - 1;
                     // choose the path with the lowest coordinate start
                     if path[last] < shortest_path[last] {
                        path
                     } else {
                        shortest_path
                     }
                  } else {
                     shortest_path
                  }
               ),
               None => Some(path)
            }
         })
   }

   // A* Search
   fn find_shortest_path(&self, x: isize, y: isize, alive_units: &Vec<&RefCell<Unit>>, map: &Grid<bool>) -> Option<Vec<Loci>> {
      let start = Loci::new(self.x, self.y);
      let goal = Loci::new(x, y);

      let heuristic_cost_estimate = |from: &Loci| -> isize {
         let distance = from.sub_loci(&goal);

         distance.x().abs() + distance.y().abs()
      };

      let mut closed_set: Vec<Loci> = Vec::new();
      let mut open_set: Vec<Loci> = vec![start];

      let mut came_from: HashMap<Loci, Loci> = HashMap::new();
      let mut g_score: HashMap<Loci, isize> = HashMap::new();
      g_score.insert(start, 0);

      let mut f_score: HashMap<Loci, isize> = HashMap::new();
      f_score.insert(start, heuristic_cost_estimate(&start));

      while !open_set.is_empty() {
         let current = {
            *open_set.iter()
               .fold((&goal, isize::max_value()), |best, node| {
                  let score = *f_score.get(node).unwrap();
                  if score < best.1 {
                     (node, score)
                  } else {
                     best
                  }
               }).0
         };

         if current == goal {
            let mut path = Vec::new();
            let mut back_track = goal;
            while back_track != start {
               path.push(back_track);
               back_track = came_from.get(&back_track).unwrap().clone();
            }
            path.reverse();

            return Some(path);
         }

         open_set = open_set.into_iter()
            .filter(|node| *node != current)
            .collect();
         closed_set.push(current);

         let tentative_g_score = *g_score.get(&current).unwrap() + 1;

         let neighbors: Vec<Loci> = current.neighbors().iter()
            .filter(|neighbor| {
               (**neighbor == goal || is_space_open(neighbor.x(), neighbor.y(), alive_units, map)) &&
                  !closed_set.contains(neighbor)
            })
            .map(|neighbor| neighbor.clone())
            .collect();

         for neighbor in neighbors {
            // check if we already know about this neighbor
            if !open_set.contains(&neighbor) {
               open_set.push(neighbor);
            } else {
               let old_g_score = *g_score.get(&neighbor).unwrap();
               if tentative_g_score > old_g_score {
                  // if our tentative g_score is worse, return
                  continue;
               } else if tentative_g_score == old_g_score {
                  // if our tentative g_score is equal, check the coordinate order
                  let came_from = came_from.get(&neighbor).unwrap();

                  // choose the inverse of reading order since we're looking backwards
                  if *came_from < current {
                     continue;
                  }
               }
            }

            // best path for now so record it
            came_from.insert(neighbor.clone(), current.clone());
            g_score.insert(neighbor.clone(), tentative_g_score);
            f_score.insert(neighbor.clone(), tentative_g_score + heuristic_cost_estimate(&neighbor));
         }
      }

      // no path could be found
      return None;
   }
}

impl Ord for Unit {
   fn cmp(&self, other: &Unit) -> Ordering {
      self.y.cmp(&other.y)
         .then(self.x.cmp(&other.x))
   }
}

impl fmt::Display for Species {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         Species::Elf => write!(f, "E"),
         Species::Goblin => write!(f, "G"),
      }
   }
}

impl Species {
   fn enemy(&self) -> Species {
      match self {
         Species::Elf => Species::Goblin,
         Species::Goblin => Species::Elf,
      }
   }
}

//fn pause() {
//   let mut _input = String::new();
//   io::stdin().read_line(&mut _input).ok().expect("Expected enter");
//}
//
//fn print_map(units: &Vec<RefCell<Unit>>, map: &Grid<bool>) {
//   for y in map.y_range() {
//      for x in map.x_range() {
//         match find_unit(x, y, units) {
//            Some(u) => print!("{}", u.borrow().species),
//            None => match map.get(x, y) {
//               true => print!("#"),
//               false => print!("."),
//            }
//         }
//      }
//      println!();
//   }
//}
//
//fn find_unit(x: isize, y: isize, units: &Vec<RefCell<Unit>>) -> Option<&RefCell<Unit>> {
//   units.iter()
//      .filter(|u| u.borrow().is_alive())
//      .find(|u| u.borrow().x == x && u.borrow().y == y)
//}