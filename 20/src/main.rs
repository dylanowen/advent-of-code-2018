use std::collections::BTreeSet;

use common::*;
use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

use crate::shared::*;

mod shared;

fn main() {
   run_tests("20", "test_input_{}.txt",
             vec![3, 10, 18, 23, 31],
             &|contents| {
                let (center, map) = build_map(parse_input(contents));

                ab(&center, &map).0
             },
   );

   run_day_real("20", &|contents, _is_sample| {
      let (map, center) = build_map(parse_input(contents));

      let (a_result, b_result) = ab(&map, &center);
      println!("Result A: {}", a_result);
      println!("Result B: {}", b_result);
   });
}

fn ab(start: &Loci, map: &Grid<MapFeature>) -> (usize, usize) {
   let mut distance_grid = Grid::new_offset(
      usize::max_value(),
      map.width(),
      map.height(),
      map.x_min(),
      map.y_min(),
   );
   distance_grid.set_loci(start, 0);

   let mut locations = BTreeSet::new();
   locations.insert(start.clone());

   let mut max = 0;
   let mut count = 0;
   while !locations.is_empty() {
      let mut next_locations: BTreeSet<Loci> = BTreeSet::new();

      for location in locations.iter() {
         let distance = distance_grid.get_loci(location);

         let neighbors: Vec<Loci> = location.neighbors().iter()
            .filter_map(|neighbor| {
               if *map.get_loci(neighbor) == MapFeature::Door {
                  let mut double = neighbor.sub_loci(location);
                  double = double.add_loci(&double);

                  // move into the room
                  Some(location.add_loci(&double))
               } else {
                  None
               }
            })
            .collect();

         let neighbor_distance = distance + 1;
         for neighbor in neighbors {
            let last_distance = *distance_grid.get_loci(&neighbor);

            if neighbor_distance < last_distance {
               max = max.max(neighbor_distance);
               if neighbor_distance >= 1000 {
                  count += 1;
               }

               distance_grid.set_loci(&neighbor, neighbor_distance);

               next_locations.insert(neighbor);
            }
         }
      }

      locations = next_locations;
   }

   (max, count)
}