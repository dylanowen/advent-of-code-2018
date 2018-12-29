use regex::Regex;
use std::fmt;
use std::cell::RefCell;

use common::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Coordinate(isize, isize, isize, isize);

fn main() {
   fn parse_input(contents: &String) -> Vec<Coordinate> {
      let re: Regex = Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)").unwrap();

      let cords: Vec<Coordinate> = contents.lines()
         .map(|row| {
            let captures = re.captures(row).unwrap();
            let iter = captures.iter().skip(1);

            let input: Vec<isize> = iter
               .filter_map(|capture| {
                  capture.and_then(|m| { m.as_str().parse::<isize>().ok() })
               })
               .collect();

            Coordinate(input[0], input[1], input[2], input[3])
         })
         .collect();

      cords
   }

   run_tests("25", "test_input_{}.txt",
             vec![2, 4, 3, 8],
             &|contents| {
                let coordinates = parse_input(contents);

                a(&coordinates).len()
             },
   );

   run_input("25", "input.txt", &|contents| {
      let coordinates = parse_input(contents);

      let a_result = a(&coordinates);
      println!("Result A: {}", a_result.len());


      //println!("Result B: {}", b_result);
   });
}

fn a(coordinates: &Vec<Coordinate>) -> Vec<Vec<Coordinate>> {
   let mut constellations: Vec<RefCell<Vec<Coordinate>>> = Vec::new();

   for coordinate in coordinates {
      let mut found = vec![];

      for i in 0..constellations.len() {
         for other_coord in constellations[i].borrow().iter() {
            if distance(coordinate, other_coord) <= 3 {
               found.push(i);

               // one match is good enough (and we don't want to double count in found)
               break;
            }
         }
      }

      if found.len() > 0 {
         {
            let main = *found.first().unwrap();
            let mut closest_constellation = constellations[main].borrow_mut();
            closest_constellation.push(coordinate.clone());

            // merge our constellations
            for i in 1..found.len() {
               closest_constellation.extend(constellations[found[i]].borrow().iter());
            }
         }

         // drop the merged constellations
         for i in (1..found.len()).rev() {
            constellations.remove(found[i]);
         }
      } else {
         let new_constellation = RefCell::new(vec![coordinate.clone()]);

         constellations.push(new_constellation);
      }
   }

   constellations.iter()
      .map(|cell| cell.borrow().clone())
      .collect()
}

fn distance(left: &Coordinate, right: &Coordinate) -> isize {
   (left.0 - right.0).abs() + (left.1 - right.1).abs() + (left.2 - right.2).abs() + (left.3 - right.3).abs()
}

impl fmt::Display for Coordinate {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
   }
}