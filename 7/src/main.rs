#[macro_use]
extern crate unic_char_range;

use std::collections::HashMap;
use std::iter::FromIterator;

use regex::Regex;

use lib::*;

#[derive(PartialEq)]
enum StepState {
   Waiting,
   Running,
   Done,
}

fn main() {
   run_day("7", &|contents, is_sample| {
      let re: Regex = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

      let dependencies: Vec<(char, char)> = contents.lines()
         .map(|row| {
            let parsed_row = re.captures(row).unwrap();

            let from = parsed_row[1].parse::<char>().unwrap();
            let to = parsed_row[2].parse::<char>().unwrap();

            return (from, to);
         })
         .collect();

      a(&dependencies);
      b(&dependencies, is_sample);
   });
}

fn a(dependencies: &Vec<(char, char)>) {
   let mut depends_on: HashMap<char, Vec<char>> = HashMap::new();

   for dependency in dependencies {
      let (from, to) = dependency;

      // insert the chars into the table
      if !depends_on.contains_key(&from) {
         depends_on.insert(*from, vec![]);
      }
      if !depends_on.contains_key(&to) {
         depends_on.insert(*to, vec![]);
      }

      let on = depends_on.get_mut(to).unwrap();

      on.push(*from);
   }

   for depends in depends_on.values_mut() {
      depends.sort();
   }

   //for depends in depends_on.iter() {
   //   for on in depends.1 {
   //      print!("{} ", on);
   //   }
   //
   //   println!("-> {}", depends.0)
   //}

   let mut run_tree: HashMap<char, bool> = HashMap::new();
   for c in chars!('A'..='Z') {
      run_tree.insert(c, false);
   }

   let mut run_order_raw: Vec<char> = vec![];
   'outer: loop {
      for c in chars!('A'..='Z') {
         match depends_on.get(&c) {
            Some(depends) => {
               // don't run something we already ran
               if !run_tree.get(&c).unwrap_or(&false) {
                  // check to see if it's dependencies have run
                  let ready = depends.iter()
                     .fold(true, |result, has_run| {
                        result && *run_tree.get(has_run).unwrap()
                     });

                  if ready {
                     run_order_raw.push(c);
                     run_tree.insert(c, true);

                     continue 'outer;
                  }
               }
            }
            _ => {}
         }
      }

      break;
   }

   let run_order = String::from_iter(run_order_raw.iter());

   println!("Result A: {}", run_order);
}

fn b(dependencies: &Vec<(char, char)>, is_sample: bool) {
   let mut depends_on: HashMap<char, Vec<char>> = HashMap::new();

   for dependency in dependencies {
      let (from, to) = dependency;

      // insert the chars into the table
      if !depends_on.contains_key(&from) {
         depends_on.insert(*from, vec![]);
      }
      if !depends_on.contains_key(&to) {
         depends_on.insert(*to, vec![]);
      }

      let on = depends_on.get_mut(to).unwrap();

      on.push(*from);
   }

   for depends in depends_on.values_mut() {
      depends.sort();
   }

   //for depends in depends_on.iter() {
   //   for on in depends.1 {
   //      print!("{} ", on);
   //   }
   //
   //   println!("-> {}", depends.0)
   //}

   let workers;
   let worker_time;
   if is_sample {
      workers = 2;
      worker_time = 0;
   } else {
      workers = 5;
      worker_time = 60;
   }

   let mut run_tree: HashMap<char, (StepState, usize)> = HashMap::new();
   for c in chars!('A'..='Z') {
      run_tree.insert(c, (StepState::Waiting, (c as usize) - ('@' as usize) + worker_time));
   }

   let mut time = 0;
   let mut worker_tasks: Vec<char> = vec![];
   loop {
      // for every free worker look for work
      'workers_loop: for _ in 0..(workers - worker_tasks.len()) {
         for c in chars!('A'..='Z') {
            match depends_on.get(&c) {
               Some(depends) => {
                  // don't run something we already ran or is running
                  if run_tree.get(&c).unwrap().0 == StepState::Waiting {
                     // check to see if it's dependencies have run
                     let ready = depends.iter()
                        .fold(true, |result, has_run| {
                           result && run_tree.get(has_run).unwrap().0 == StepState::Done
                        });

                     if ready {
                        // assign a worker and mark this task in progress
                        let mut task = run_tree.get_mut(&c).unwrap();

                        worker_tasks.push(c);
                        task.0 = StepState::Running;

                        continue 'workers_loop;
                     }
                  }
               }
               _ => {}
            }
         }
      }

      if worker_tasks.len() <= 0 {
         // no more workers so we're done;
         break;
      }

      //println!("time: {}", time);

      // move each worker forward
      worker_tasks = worker_tasks.iter()
         .filter_map(|working_on| {
            let task = run_tree.get_mut(working_on).unwrap();

            task.1 -= 1;

            //println!("\t{} status {}", working_on, task.1);

            if task.1 > 0 {
               Some(*working_on)
            } else {
               // mark the worker's task as done
               task.0 = StepState::Done;

               None
            }
         })
         .collect();

      time += 1;
   }

   println!("Result B: {}", time);
}