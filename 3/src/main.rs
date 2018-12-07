extern crate lib;

use lib::*;

struct Claim<'a> {
   id: &'a str,
   x: usize,
   y: usize,
   width: usize,
   height: usize,
}

fn main() {
   run_input("3", "input.txt", &|contents, _is_sample| {
      let mut max_width: usize = 0;
      let mut max_height: usize = 0;

      let claims: Vec<Claim> = contents.lines()
         .map(|row| {
            let mut split_row = row.split_whitespace();

            let id: &str = &split_row.next().unwrap()[1..];
            split_row.next();
            let x_y: Vec<&str> = split_row.next().unwrap()
               .splitn(3, |c| c == ',' || c == ':')
               .collect();
            let w_h: Vec<&str> = split_row.next().unwrap()
               .splitn(3, 'x')
               .collect();

            let x = x_y[0].parse::<usize>().unwrap();
            let y = x_y[1].parse::<usize>().unwrap();
            let width = w_h[0].parse::<usize>().unwrap();
            let height = w_h[1].parse::<usize>().unwrap();

            if x + width > max_width {
               max_width = x + width;
            }
            if y + height > max_height {
               max_height = y + height;
            }

            return Claim {
               id,
               x,
               y,
               width,
               height,
            };
         })
         .collect();

      max_width += 1;
      max_height += 1;

      let cloth = a(&claims, max_width, max_height);
      b(&claims, &cloth);
   });
}

fn a(claims: &Vec<Claim>, max_width: usize, max_height: usize) -> Vec<Vec<i32>> {
   let mut cloth = vec![vec![0; max_height]; max_width];

   for claim in claims {
      fill_cloth(claim, &mut cloth);
   }

   let mut overlaps = 0;
   for column in &cloth {
      for cell in column {
         if *cell > 1 {
            overlaps += 1;
         }
      }
   }

   println!("Result A: {}", overlaps);

   return cloth;
}

fn b(claims: &Vec<Claim>, cloth: &Vec<Vec<i32>>) {
   for claim in claims {
      let mut overlap = 0;
      for y in claim.y..(claim.y + claim.height) {
         for x in claim.x..(claim.x + claim.width) {
            if cloth[x][y] > 1 {
               overlap += 1;
            }
         }
      }

      if overlap == 0 {
         println!("Result B: {}", claim.id);
         break;
      }
   }
}

fn fill_cloth(claim: &Claim, cloth: &mut Vec<Vec<i32>>) {
   for y in claim.y..(claim.y + claim.height) {
      for x in claim.x..(claim.x + claim.width) {
         cloth[x][y] += 1;
      }
   }
}