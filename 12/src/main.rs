use regex::Regex;

use common::*;

const ONE_MASK: usize = 0b1;

struct Transition {
   state: Vec<bool>,
   plant_grows: bool,
}

fn main() {
   let initial_state_re: Regex = Regex::new(r"initial state: ([#.]+)").unwrap();
   let transition_re: Regex = Regex::new(r"([#.]{5}) => ([#.])").unwrap();

   run_day("12", &|contents, is_sample| {
      let mut lines = contents.lines();


      let initial_state: Vec<bool> = lines.next()
         .and_then(|line| initial_state_re.captures(line)).unwrap()[1]
         .chars()
         .map(|c| c == '#')
         .collect();

      let transitions: Vec<Transition> = lines.skip(1)
         .filter_map(|row| {
            let parsed_row = transition_re.captures(row).unwrap();

            let state = parsed_row[1].chars()
               .map(|c| c == '#')
               .collect();

            let plant_grows = parsed_row[2].chars().next().unwrap() == '#';

            // filter out no-grow states
            if plant_grows {
               Some(Transition {
                  state,
                  plant_grows,
               })
            } else {
               None
            }
         })
         .collect();


      a(&initial_state, &transitions, is_sample);
      if !is_sample {
         b(&initial_state, &transitions);
      }
   });
}

fn a(initial_state: &Vec<bool>, transitions: &Vec<Transition>, is_sample: bool) {
   let result = run_generations(20, initial_state, transitions).0;
   if is_sample {
      assert_eq!(result, 325);
   }

   println!("Result A: {}", result);
}

fn b(initial_state: &Vec<bool>, transitions: &Vec<Transition>) {
   // assume we're going to have a continuously moving plant colony to the right
   let (result, first_plant, num_plants) = run_generations(1000, initial_state, transitions);
   let movement = run_generations(1001, initial_state, transitions).1 - first_plant;

   // sanity check
   {
      let move_distance = (1010 - 1000) * movement;
      let offset_result = result + (move_distance * num_plants);
      let (check, _, _) = run_generations(1010, initial_state, transitions);

      assert_eq!(offset_result, check);
   }

   let offset_result = result + (((50000000000 - 1000) * movement) * num_plants);

   println!("Result B: {}", offset_result);
}

fn run_generations(generations: u64, initial_state: &Vec<bool>, transitions: &Vec<Transition>) -> (isize, isize, isize) {
   let mut plants = [
      initial_state.to_vec(),
      vec![false; initial_state.len()]
   ];

   let mut last_plant_index = 0;
   let mut plant_index = 1;
   let mut zero: isize = 0;

   for _generation in 1..=generations {
      for offset in 0..=2 {
         if plants[last_plant_index][offset] {
            for _ in 0..4 - offset {
               plants[last_plant_index].insert(0, false);
               zero += 1;
            }
            break;
         }
      }

      for offset in 1..=3 {
         let len = plants[last_plant_index].len();
         if plants[last_plant_index][len - offset] {
            plants[last_plant_index].resize(len + (5 - offset), false);
            break;
         }
      }

      // prep our plants
      let len = plants[last_plant_index].len();
      plants[plant_index].resize(len, false);

      for i in 2..len - 2 {
         plants[plant_index][i] = transition(&plants[last_plant_index][i - 2..=i + 2], transitions);
      }

      // swap our indices
      plant_index = last_plant_index;
      last_plant_index = !last_plant_index & ONE_MASK;
   }

   let mut sum = 0;
   let mut lowest_plant = isize::max_value();
   let mut num_plants = 0;
   for i in 0..plants[last_plant_index].len() {
      if plants[last_plant_index][i] {
         let real_index = (i as isize) - zero;
         if real_index < lowest_plant {
            lowest_plant = real_index;
         }

         sum += real_index;
         num_plants += 1;
      }
   }

   return (sum, lowest_plant, num_plants);
}

fn transition(plant_state: &[bool], transitions: &Vec<Transition>) -> bool {
   for transition in transitions.iter() {
      if &transition.state[..] == plant_state {
         return transition.plant_grows;
      }
   }

   return false;
}

//fn plants_to_string(plants: &Vec<bool>) -> String {
//   plants.iter()
//      .map(|plant| {
//         if *plant {
//            '#'
//         } else {
//            '.'
//         }
//      })
//      .collect()
//}