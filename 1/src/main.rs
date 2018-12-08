extern crate lib;

use lib::*;
use std::collections::BTreeSet;

fn main() {
   run_input("2", "input.txt", &|contents| {
      let movements = contents.split_whitespace()
         .map(|x| x.parse::<i32>().unwrap())
         .collect();

      a(&movements);
      b(&movements);
   });
}

fn a(movements: &Vec<i32>) {
   let mut freq = 0;
   for movement in movements {
      freq = freq + movement;
   }

   println!("Result A: {}", freq);
}

fn b(movements: &Vec<i32>) {
   let mut seen = BTreeSet::new();

   let mut freq = 0;
   let mut itr = movements.iter();
   loop {
      match itr.next() {
         Some(movement) => {
            freq = freq + movement;

            if seen.contains(&freq) {
               break;
            }

            seen.insert(freq);
         }
         None => {
            itr = movements.iter();
         }
      }
   }

   println!("Result B: {}", freq);
}