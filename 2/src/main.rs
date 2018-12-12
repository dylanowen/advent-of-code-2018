use lib::*;
use std::collections::BTreeSet;

fn main() {
   run_input("2", "input.txt", &|contents| {
      let ids = contents.split_whitespace()
         .collect();

      a(&ids);
      b(&ids);
   });
}

fn a(ids: &Vec<&str>) {
   let mut two = 0;
   let mut three = 0;
   for id in ids {
      let (id_two, id_three) = id_letter_count(id);

      if id_two > 0 {
         two += 1
      }
      if id_three > 0 {
         three += 1
      }
   }

   let result = two * three;

   println!("Result A: {}", result);
}

fn b(ids: &Vec<&str>) {
   for id in ids {
      let found = ids.iter()
         .map(|other| id_diff(id, other))
         .filter(|result| result.0 == 1)
         .map(|result| result.1)
         .next();

      match found {
         Some(found) => {
            println!("Result B: {}", found);

            break;
         }
         None => (),
      }
   }

   //println!("Result B: {}", freq);
}

fn id_letter_count(id: &str) -> (i32, i32) {
   let mut seen = BTreeSet::new();

   let mut two = 0;
   let mut three = 0;
   for (i, c) in id.chars().enumerate() {
      if !seen.contains(&c) {
         let count = id[i..].chars()
            .filter(|inner_c| *inner_c == c)
            .count();

         match count {
            2 => two += 1,
            3 => three += 1,
            _ => (),
         }

         seen.insert(c);
      }
   }

   return (two, three);
}

fn id_diff(left: &str, right: &str) -> (i32, String) {
   let mut diff = 0;
   let mut same = String::new();
   for (i, c) in left.chars().enumerate() {
      if c != right.chars().nth(i).unwrap() {
         diff += 1;
      } else {
         same.push(c);
      }
   }

   return (diff, same);
}