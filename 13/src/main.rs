use lib::*;

use std::fmt;
use lib::coordinates::Grid;
use lib::coordinates::Loci;
use lib::coordinates::OffsetLociX;
use lib::coordinates::OffsetLociY;

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Turn {
   Left,
   Straight,
   Right,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Train {
   dir: char,
   last_turn: Turn,
   moved: bool,
}

fn main() {
   fn parse_input(contents: &String) -> (Grid<Option<Train>>, Grid<char>) {
      let input: Vec<Vec<char>> = contents.lines()
         .map(|line| line.chars().collect())
         .collect();

      let width = input.iter().fold(0, |max, row| {
         if row.len() > max {
            row.len()
         } else {
            max
         }
      });
      let height = input.len();

      let mut tracks = Grid::new(' ', width, height);
      let mut trains = Grid::new(None, width, height);

      for y in 0..input.len() {
         for x in 0..input[y].len() {
            let mut track = None;
            let mut train = None;
            match input[y][x] {
               ' ' => {} // do nothing for a space,
               trk @ '-' | trk @ '|' | trk @ '\\' | trk @ '/' | trk @ '+' => {
                  track = Some(trk)
               }
               trn @ '>' | trn @ '<' | trn @ '^' | trn @ 'v' => {
                  track = Some(under_track(x, y, &input));
                  train = Some(trn);
               }
               u => println!("Unexpected char: {}", u)
            }

            track.map(|t| tracks.set(x as isize, y as isize, t));
            train.map(|t| {
               trains.set(x as isize, y as isize, Some(Train {
                  dir: t,
                  last_turn: Turn::Right,
                  moved: false,
               }))
            });
         }
      }

      (trains, tracks)
   }

   run_day("13", &|contents, _is_sample| {
      let (trains, tracks) = parse_input(contents);

      a(&trains, &tracks);
   });

   run_input("13", "sample_input_2.txt", &|contents| {
      let (trains, tracks) = parse_input(contents);

      b(&trains, &tracks);
   });

   run_input("13", "input.txt", &|contents| {
      let (trains, tracks) = parse_input(contents);

      b(&trains, &tracks);
   });
}

fn a(initial_trains: &Grid<Option<Train>>, tracks: &Grid<char>) {
   let mut moved_state = true;
   let mut trains = initial_trains.clone();

   let mut collision = Loci::new(0, 0);
   'outer: loop {
      // print
      //for y in trains.y_range() {
      //   for x in trains.x_range() {
      //      match trains.get(x, y) {
      //         Some(t) => print!("{}", t),
      //         None => print!("{}", tracks.get(x, y))
      //      }
      //   }
      //   println!();
      //}

      for y in trains.y_range() {
         for x in trains.x_range() {
            let successful = trains.get(x, y)
               .filter(|train| train.moved != moved_state)
               .map(|train| {
                  // remove the last train
                  trains.set(x, y, None);

                  let (next_x, next_y, mut next_train) = next_train(&train, x, y, tracks);

                  // check for collision
                  if trains.get(next_x, next_y).is_some() {
                     collision = Loci::new(next_x, next_y);

                     false
                  } else {
                     next_train.moved = !next_train.moved;

                     trains.set(next_x, next_y, Some(next_train));

                     true
                  }
               })
               .unwrap_or(true);

            if !successful {
               break 'outer;
            }
         }
      }

      moved_state = !moved_state;
   }

   println!("Result A: {},{}", collision.x(), collision.y());
}

fn b(initial_trains: &Grid<Option<Train>>, tracks: &Grid<char>) {
   let mut moved_state = true;
   let mut trains = initial_trains.clone();

   let mut last_train = Loci::new(0, 0);
   loop {
      // print
      //for y in trains[last_trains_index].y_range() {
      //   for x in trains[last_trains_index].x_range() {
      //      match trains[last_trains_index].get(x, y) {
      //         Some(t) => print!("{}", t),
      //         None => print!("{}", tracks.get(x, y))
      //      }
      //   }
      //   println!();
      //}

      let mut train_count = 0;
      for y in trains.y_range() {
         for x in trains.x_range() {
            trains.get(x, y)
               .filter(|train| train.moved != moved_state)
               .map(|train| {
                  // remove the last train
                  trains.set(x, y, None);

                  let (next_x, next_y, mut next_train) = next_train(&train, x, y, tracks);

                  // check for collision
                  match trains.get(next_x, next_y) {
                     Some(collided_train) => {
                        //println!("Crash at: {} {}", next_x, next_y);

                        // subtract for our collided train if we already counted it
                        if collided_train.moved == moved_state {
                           train_count -= 1;
                        }

                        // clear the train
                        trains.set(next_x, next_y, None);
                     }
                     None => {
                        train_count += 1;
                        next_train.moved = !next_train.moved;

                        // set our train
                        trains.set(next_x, next_y, Some(next_train));
                        // save this train's location
                        last_train = Loci::new(next_x, next_y);
                     }
                  }
               });
         }
      }

      moved_state = !moved_state;

      if train_count == 1 {
         break;
      }
   }

   println!("Result B: {},{}", last_train.x(), last_train.y());
}

fn next_train(train: &Train, x: isize, y: isize, tracks: &Grid<char>) -> (isize, isize, Train) {
   let mut next_x = x;
   let mut next_y = y;
   let next_train = match train.dir {
      '^' => {
         next_y = y - 1;

         match tracks.get(x, next_y) {
            '\\' => train.turn('<'),
            '/' => train.turn('>'),
            '+' => train.intersection(),
            _ => train.clone(),
         }
      }
      '>' => {
         next_x = x + 1;

         match tracks.get(next_x, y) {
            '\\' => train.turn('v'),
            '/' => train.turn('^'),
            '+' => train.intersection(),
            _ => train.clone(),
         }
      }
      'v' => {
         next_y = y + 1;

         match tracks.get(x, next_y) {
            '\\' => train.turn('>'),
            '/' => train.turn('<'),
            '+' => train.intersection(),
            _ => train.clone(),
         }
      }
      '<' => {
         next_x = x - 1;

         match tracks.get(next_x, y) {
            '\\' => train.turn('^'),
            '/' => train.turn('v'),
            '+' => train.intersection(),
            _ => train.clone(),
         }
      }
      u => panic!("Unexpected train: {}", u)
   };

   (next_x, next_y, next_train)
}

// Get the track under a train
fn under_track(x: usize, y: usize, input: &Vec<Vec<char>>) -> char {
   fn check_connection(expected: char, input: char) -> bool {
      input == expected || input == '\\' || input == '/' || input == '+'
   }

   let up = y > 0 && check_connection('|', input[y - 1][x]);
   let down = y < (input.len() - 1) && check_connection('|', input[y + 1][x]);
   let left = x > 0 && check_connection('-', input[y][x - 1]);
   let right = x < (input[y].len() - 1) && check_connection('-', input[y][x + 1]);

   if up && down && left && right {
      '+'
   } else if (down && right && !up && !left) || (up && left && !down && !right) {
      '/'
   } else if (down && left && !up && !right) || (up && right && !down && !left) {
      '\\'
   } else if up && down {
      '|'
   } else if right && left {
      '-'
   } else {
      panic!("Unexpected case");
   }
}

impl Train {
   fn intersection(&self) -> Train {
      let next_turn = self.next_turn();
      let next_dir = match next_turn {
         Turn::Left => match self.dir {
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            '>' => '^',
            u => panic!("Unexpected train: {}", u)
         },
         Turn::Right => match self.dir {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            u => panic!("Unexpected train: {}", u)
         },
         Turn::Straight => self.dir,
      };

      Train {
         dir: next_dir,
         last_turn: next_turn,
         moved: self.moved,
      }
   }

   fn turn(&self, dir: char) -> Train {
      Train {
         dir,
         last_turn: self.last_turn,
         moved: self.moved,
      }
   }

   fn next_turn(&self) -> Turn {
      match self.last_turn {
         Turn::Left => Turn::Straight,
         Turn::Straight => Turn::Right,
         Turn::Right => Turn::Left,
      }
   }
}

impl fmt::Display for Train {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.dir)
   }
}