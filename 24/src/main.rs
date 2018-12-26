use regex::Regex;
use std::fmt;
use std::option::Option::{Some, None};
use std::cmp::Ordering;
use std::cell::RefCell;

use lib::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Attack {
   Fire,
   Slashing,
   Bludgeoning,
   Cold,
   Radiation,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Army {
   Immune,
   Infection,
}

const ATTACKS: [Attack; 5] = [
   Attack::Fire,
   Attack::Slashing,
   Attack::Bludgeoning,
   Attack::Cold,
   Attack::Radiation,
];

#[derive(Clone, PartialEq, Eq, Debug)]
struct Group {
   army: Army,
   group_id: usize,
   units: isize,
   hit_points: isize,
   weaknesses: Vec<Attack>,
   immunities: Vec<Attack>,
   attack_damage: isize,
   attack_type: Attack,
   initiative: isize,
   target: Option<usize>,
}

fn main() {
   fn parse_input(contents: &String) -> Vec<Group> {
      let re: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (?:\((.+)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
      let attributes_re: Regex = Regex::new(r"(\w+) to (\w+)(?:, (\w+))*").unwrap();

      let first_newline = contents.find('\n').unwrap();

      let dropped_contents: String = contents.chars()
         .skip(first_newline)
         .collect();

      let raw_armies: Vec<&str> = dropped_contents
         .split("Infection:")
         .map(|s| s.trim())
         .filter(|s| !s.is_empty())
         .collect();

      let mut army = Army::Infection;
      let groups: Vec<Group> = raw_armies.iter()
         .flat_map(|raw_army| {
            let mut group_id = 0;
            army = if army == Army::Infection { Army::Immune } else { Army::Infection };

            raw_army.lines()
               .map(|row| {
                  group_id += 1;

                  let captures = re.captures(row).unwrap();

                  let unit_count = captures[1].parse::<isize>().unwrap();
                  let hit_points = captures[2].parse::<isize>().unwrap();
                  let attack_damage = captures[4].parse::<isize>().unwrap();
                  let attack_type = Attack::find(&captures[5]).unwrap();
                  let initiative = captures[6].parse::<isize>().unwrap();

                  let mut weaknesses = vec![];
                  let mut immunities = vec![];

                  match captures.get(3) {
                     Some(raw_attributes) => {
                        let split_attributes = raw_attributes.as_str().split(';')
                           .map(|s| s.trim());

                        for split_attribute in split_attributes {
                           let ar_captures = attributes_re.captures(split_attribute.trim()).unwrap();

                           let ar_attacks = ar_captures.iter().skip(2)
                              .filter_map(|maybe_m| {
                                 maybe_m.and_then(|m| Attack::find(m.as_str()))
                              })
                              .collect();

                           if ar_captures[1] == *"weak" {
                              weaknesses = ar_attacks;
                           } else {
                              immunities = ar_attacks;
                           }
                        }
                     }
                     _ => {}
                  }

                  Group {
                     army,
                     group_id,
                     units: unit_count,
                     hit_points,
                     weaknesses,
                     immunities,
                     attack_damage,
                     attack_type,
                     initiative,
                     target: None,
                  }
               })
               .collect::<Vec<Group>>()
         })
         .collect();

      groups
   }

   run_day("24", &|contents, is_sample| {
      let groups = parse_input(contents);

      let a_result = a(&groups);
      println!("Result A: {}", a_result);
      let b_result = b(&groups);
      println!("Result B: {}", b_result);

      if is_sample {
         assert_eq!(5216, a_result);
         assert_eq!(51, b_result);
      }
   });
}

fn a(initial_groups: &Vec<Group>) -> isize {
   run(initial_groups, 0).1
}

fn b(initial_groups: &Vec<Group>) -> isize {
   let mut boost = 1;

   loop {
      let (winners, units) = run(initial_groups, boost);

      if winners == Army::Immune {
         return units;
      }

      boost += 1;
   }
}

fn run(initial_groups: &Vec<Group>, boost: isize) -> (Army, isize) {
   let mut groups: Vec<RefCell<Group>> = initial_groups.iter()
      .map(|g| {
         let mut group = g.clone();

         if group.army == Army::Immune {
            group.attack_damage += boost;
         }

         RefCell::new(group)
      })
      .collect();

   let mut found_immune = true;
   let mut found_infection = true;
   while found_immune && found_infection {
      //println!();
      //print_armies(&groups);

      // targeting
      groups.sort_unstable_by(|left, right| {
         left.borrow().target_order(&right.borrow())
      });

      let mut taken: Vec<usize> = vec![];
      for i in 0..groups.len() {
         match groups[i].borrow_mut().set_target(&groups, i, &taken) {
            Some(found) => taken.push(found),
            None => {}
         }
      }

      //print_groups(&groups);

      // attacking
      groups.sort_unstable_by(|left, right| {
         left.borrow().attack_order(&right.borrow())
      });

      let mut killed_units = 0;
      for i in 0..groups.len() {
         let group = groups[i].borrow();

         match group.find_target_index(&groups) {
            Some(target_index) => {
               let mut target = groups[target_index].borrow_mut();

               let damage = group.damage_to(&target);
               let lost_units = target.units_lost(damage);

               killed_units += lost_units;

               //println!("{} attacks {} for {}", group, target, lost_units);

               target.units -= lost_units;
            }
            None => {}
         }
      }

      // check for a stalemate
      if killed_units == 0 {
         return (Army::Infection, 0);
      }

      // remove dead groups
      groups = groups.iter()
         .filter(|g| g.borrow().units > 0)
         .map(|g| g.clone())
         .collect();

      // check for exit
      found_immune = false;
      found_infection = false;
      for group in groups.iter() {
         match group.borrow().army {
            Army::Infection => found_infection = true,
            Army::Immune => found_immune = true,
         }
      }
   }

   let remaining_units = groups.iter().fold(0, |sum, g| sum + g.borrow().units);

   if found_immune {
      (Army::Immune, remaining_units)
   } else {
      (Army::Infection, remaining_units)
   }
}

//fn print_armies(initial_groups: &Vec<RefCell<Group>>) {
//   let mut groups: Vec<Group> = initial_groups.iter()
//      .map(|cell| cell.borrow().clone())
//      .collect();
//   groups.sort_by(Group::id_order);
//
//   println!("Immune System:");
//   for group in groups.iter() {
//      if group.army == Army::Immune {
//         println!("{} contains {} units", group, group.units)
//      }
//   }
//
//   println!("Infection:");
//   for group in groups.iter() {
//      if group.army == Army::Infection {
//         println!("{} contains {} units", group, group.units)
//      }
//   }
//}
//
//fn print_groups(groups: &Vec<RefCell<Group>>) {
//   for cell in groups {
//      let group = cell.borrow();
//      println!("{:<9}[{}] units {:4} {:4} effective_power: {:6}, {:?}", group.army.to_string(), group.group_id, group.units, group.attack_damage, group.effective_power(), group);
//   }
//}

impl Group {
   fn effective_power(&self) -> isize {
      self.units * self.attack_damage
   }

   fn damage_to(&self, other: &Group) -> isize {
      if other.immunities.contains(&self.attack_type) {
         0
      } else if other.weaknesses.contains(&self.attack_type) {
         self.effective_power() * 2
      } else {
         self.effective_power()
      }
   }

   fn units_lost(&self, damage: isize) -> isize {
      (damage / self.hit_points).min(self.units)
   }

   fn find_target_index(&self, groups: &Vec<RefCell<Group>>) -> Option<usize> {
      self.target.map(|target| {
         for i in 0..groups.len() {
            let group = &groups[i].borrow();

            // return our target if we found it
            if self.army != group.army && group.group_id == target {
               return i;
            }
         }

         panic!("There should always be a match");
      })
   }

   fn set_target(&mut self, groups: &Vec<RefCell<Group>>, self_i: usize, taken: &Vec<usize>) -> Option<usize> {
      let mut found_i = None;
      let mut max_damage = isize::min_value();

      for i in 0..groups.len() {
         // don't borrow our mutable borrow
         if i != self_i {
            let group = &groups[i].borrow();

            // don't attack ourselves or something that isn't available
            if self.army != group.army && !taken.contains(&i) {
               let damage = self.damage_to(&group);
               if damage <= 0 {
                  // we can't deal any damage so don't do anything
                  continue;
               } else if damage > max_damage {
                  max_damage = damage;
                  found_i = Some(i);
               } else if damage == max_damage {
                  let last_group = &groups[found_i.unwrap()].borrow();

                  match group.effective_initiative_order(last_group) {
                     Ordering::Greater => {
                        // only do something if this new group is greater
                        found_i = Some(i)
                     }
                     _ => {}
                  }
               }
            }
         }
      }

      self.target = found_i.map(|i| groups[i].borrow().group_id);

      found_i
   }

   fn target_order(&self, other: &Group) -> Ordering {
      self.effective_initiative_order(other).reverse()
   }

   fn effective_initiative_order(&self, other: &Group) -> Ordering {
//      println!("{} {} {} {} {:?} {:?} {:?}", self.effective_power(), other.effective_power(), self.initiative, other.initiative,
//               self.effective_power().cmp(&other.effective_power()),
//               self.initiative.cmp(&other.initiative),
//               self.effective_power().cmp(&other.effective_power())
//                  .then(self.initiative.cmp(&other.initiative))
//                  );

      self.effective_power().cmp(&other.effective_power())
         .then(self.initiative.cmp(&other.initiative))
      //.reverse()
   }

   fn attack_order(&self, other: &Group) -> Ordering {
      self.initiative.cmp(&other.initiative).reverse()
   }

   //fn id_order(&self, other: &Group) -> Ordering {
   //   self.group_id.cmp(&other.group_id)
   //}
}

impl fmt::Display for Group {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}[{}]", self.army, self.group_id)
   }
}

impl Attack {
   fn find(raw_attack: &str) -> Option<Attack> {
      for attack in ATTACKS.iter() {
         if attack.to_string() == raw_attack {
            return Some(attack.clone());
         }
      }

      return None;
   }
}

impl fmt::Display for Attack {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", format!("{:?}", self).to_lowercase())
   }
}

impl fmt::Display for Army {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", format!("{:?}", self))
   }
}