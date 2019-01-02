use common::*;

use crate::shared::*;

mod shared;

fn main() {
   run_tests("20", "test_input_{}.txt",
             vec![3, 10, 18, 23, 31, 6],
             &|contents| {
                let (center, map) = build_map(&parse_input(contents));

                ab(&center, &map).0
             },
   );

   run_day_real("20", &|contents, _is_sample| {
      let (map, center) = build_map(&parse_input(contents));

      let (a_result, b_result) = ab(&map, &center);
      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);
   });
}