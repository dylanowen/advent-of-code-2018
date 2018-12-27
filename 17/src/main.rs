use common::*;
use common::coordinates::Grid;

use crate::shared::*;

mod shared;

fn main() {
   run_tests("17", "test_input_{}.txt",
             vec![(45, 17)],
             &|contents| {
                let ground = parse_input(contents);

                ab(&ground)
             },
   );

   run_day("17", &|contents, is_sample| {
      let ground = parse_input(contents);

      let (a_result, b_result) = ab(&ground);
      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);
      if is_sample {
         assert_eq!(57, a_result);
         assert_eq!(29, b_result);
      }
   });
}

fn ab(initial_ground: &Grid<Ground>) -> (usize, usize) {
   let mut ground = initial_ground.clone();

   while tick(&mut ground) {
//      println!("Step");
//      print_subset(&ground);
   }

   count_water(&ground)
}

// only print the ground that has water in it
//fn print_subset(ground: &Grid<Ground>) {
//   for y in ground.y_range() {
//      // found water
//      let mut found_water= false;
//      for x in ground.x_range() {
//         let dirt = ground.get(x, y);
//         found_water = found_water || *dirt == Ground::WaterFalling || *dirt == Ground::WaterLocked;
//
//         print!("{} ", dirt);
//      }
//      println!();
//
//      if !found_water {
//         return;
//      }
//   }
//}