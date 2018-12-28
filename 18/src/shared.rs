use common::coordinates::Grid;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Acre {
   Open,
   Tree,
   Lumberyard,
}

pub fn next_lumberyard(last_area: &Grid<Acre>, area: &mut Grid<Acre>) {
   for y in last_area.y_range() {
      for x in last_area.x_range() {
         let (_, tree_count, lumberyard_count) = count_adjacent(x, y, &last_area);

         let next_acre: Acre = match last_area.get(x, y) {
            Acre::Open =>
               if tree_count >= 3 {
                  Acre::Tree
               } else {
                  Acre::Open
               }
            Acre::Tree =>
               if lumberyard_count >= 3 {
                  Acre::Lumberyard
               } else {
                  Acre::Tree
               }
            Acre::Lumberyard =>
               if lumberyard_count >= 1 && tree_count >= 1 {
                  Acre::Lumberyard
               } else {
                  Acre::Open
               }
         };

         area.set(x, y, next_acre);
      }
   }
}

fn count_adjacent(x: isize, y: isize, area: &Grid<Acre>) -> (usize, usize, usize) {
   let mut open_count = 0;
   let mut tree_count = 0;
   let mut lumberyard_count = 0;

   let min_x = area.x_min().max(x - 1);
   let max_x = (area.x_max() - 1).min(x + 1);
   let min_y = area.y_min().max(y - 1);
   let max_y = (area.y_max() - 1).min(y + 1);

   for yi in min_y..=max_y {
      for xi in min_x..=max_x {
         if x != xi || y != yi {
            match area.get(xi, yi) {
               Acre::Open => open_count += 1,
               Acre::Tree => tree_count += 1,
               Acre::Lumberyard => lumberyard_count += 1,
            }
         }
      }
   }

   (open_count, tree_count, lumberyard_count)
}

pub fn parse_input(contents: &String) -> Grid<Acre> {
   let lines: Vec<Vec<Acre>> = contents.lines()
      .map(|row| {
         row.chars()
            .map(|c| match c {
               '|' => Acre::Tree,
               '#' => Acre::Lumberyard,
               '.' => Acre::Open,
               u => panic!("Unexpected char: {}", u)
            })
            .collect()
      })
      .collect();

   let mut area = Grid::new(Acre::Open, lines[0].len(), lines.len());

   for y in area.y_range() {
      for x in area.x_range() {
         area.set(x, y, lines[y as usize][x as usize])
      }
   }

   area
}