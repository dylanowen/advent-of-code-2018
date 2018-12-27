use common::run_day;

const ASCII_DIFF: i32 = ('a' as i32) - ('A' as i32);

fn main() {
   run_day("5", &|polymer, _is_sample| {
      a(polymer);
      b(polymer);
   });
}

fn a(polymer: &String) {
   println!("Result A: {}", compress_polymer(polymer));
}

fn b(polymer: &String) {
   let mut min = polymer.len() + 1;
   for bad_num in ('a' as u8)..('z' as u8) + 1 {
      let bad = bad_num as char;
      let bad_cap = (bad_num - ASCII_DIFF as u8) as char;

      let filtered_polymer = polymer.chars()
         .filter(|&c| c != bad && c != bad_cap)
         .collect();

      let result = compress_polymer(&filtered_polymer);

      if result < min {
         min = result;
      }
   }

   println!("Result B: {}", min);
}

fn compress_polymer(polymer: &String) -> usize {
   let base_char = 35;

   let mut new_polymer = String::with_capacity(polymer.len());

   let mut last = base_char;
   for c in polymer.chars() {
      let current = c as u8;

      // debug
      //println!("{:1} {:1}: {}", last as char, c, new_polymer);

      // check if these characters are "polar"
      if (current as i32 - last as i32).abs() == ASCII_DIFF {
         match new_polymer.pop() {
            Some(popped) => {
               last = popped as u8;
            }
            _ => {
               last = base_char;
            }
         }
      } else {
         // check for our base case
         if last > base_char {
            new_polymer.push(last as char);
         }

         last = current;
      }
   }

   return new_polymer.len();
}