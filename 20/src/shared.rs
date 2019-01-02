use std::fmt;
use std::slice::Iter;
use core::iter::Peekable;
use std::collections::BTreeSet;
use std::collections::btree_set;
use std::mem;

use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapFeature {
   Room,
   Door,
   Wall,
}

impl fmt::Display for MapFeature {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         MapFeature::Room | MapFeature::Door => write!(f, " "),
         MapFeature::Wall => write!(f, "#"),
      }
   }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
   North,
   East,
   South,
   West,
}

impl fmt::Display for Direction {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", format!("{:?}", self).chars().next().unwrap())
   }
}

pub type MapMove = [(Loci, MapFeature); 2];

// TODO remove this after https://github.com/rust-lang/rust/pull/55687 makes stable
type PathIter<'a> = Box<PathIterator<Item=MapMove> + 'a>;

pub enum Path {
   Segments(Vec<Path>),
   Branch(Vec<Path>),
   Static(Vec<Direction>),
}

impl Path {
   pub fn path_iterator(&self, loci: &Loci) -> PathIter {
      match self {
         Path::Segments(segments) => {
            // kick start this iteration
            let mut start_loci = BTreeSet::new();
            start_loci.insert(loci.clone());

            let mut segments_iter = segments.iter();

            match segments_iter.next() {
               Some(first_segment) => {
                  Box::new(PathSegmentsIterator {
                     segment_iter: segments_iter,
                     loci_iter: start_loci.into_iter(),
                     path_iter: Box::new(EmptyPathIterator),
                     last_segment: Box::new(first_segment),
                     last_locis: BTreeSet::new(),
                  })
               }
               None => {
                  // nothing to iterate on so return an empty iterator
                  Box::new(EmptyPathIterator)
               }
            }
         }
         Path::Branch(branches) => {
            Box::new(BranchIterator {
               branch_iter: branches.iter(),
               path_iter: Box::new(EmptyPathIterator),
               start_loci: loci.clone(),
               last_locis: BTreeSet::new(),
            })
         }
         Path::Static(path) => {
            Box::new(StaticPathIterator {
               path_iter: path.iter(),
               last_loci: loci.clone(),
            })
         }
      }
   }

   // calculate the max distance (up/down, left/right)
   fn max_distance(&self) -> (usize, usize) {
      match self {
         Path::Segments(segments) => {
            segments.iter().fold((0, 0), |(ud, lr), p| {
               let (p_ud, p_lr) = p.max_distance();

               (ud + p_ud, lr + p_lr)
            })
         }
         Path::Branch(branches) => {
            branches.iter().fold((0, 0), |(ud, lr), p| {
               let (p_ud, p_lr) = p.max_distance();

               (ud.max(p_ud), lr.max(p_lr))
            })
         }
         Path::Static(path) => {
            let (up_down, left_right) = path.iter().fold((0 as isize, 0 as isize), |(ud, lr), direction| {
               match direction {
                  Direction::North => (ud + 1, lr),
                  Direction::East => (ud, lr + 1),
                  Direction::South => (ud - 1, lr),
                  Direction::West => (ud, lr - 1),
               }
            });

            (up_down.abs() as usize, left_right.abs() as usize)
         }
      }
   }
}

impl fmt::Display for Path {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Path::Segments(segments) => {
            for segment in segments.iter() {
               write!(f, "{}", *segment)?;
            }

            Ok(())
         }
         Path::Branch(branches) => {
            write!(f, "(")?;
            let mut first = true;
            for branch in branches.iter() {
               if !first {
                  write!(f, "|")?;
               } else {
                  first = false;
               }
               write!(f, "{}", *branch)?;
            }
            write!(f, ")")
         }
         Path::Static(path) => {
            for direction in path.iter() {
               write!(f, "{}", direction)?;
            }

            Ok(())
         }
      }
   }
}

pub trait PathIterator: Iterator<Item=MapMove> {
   fn last_locis(&self) -> BTreeSet<Loci>;
}

struct PathSegmentsIterator<'a> {
   segment_iter: Iter<'a, Path>,
   loci_iter: btree_set::IntoIter<Loci>,
   path_iter: PathIter<'a>,

   last_segment: Box<&'a Path>,
   last_locis: BTreeSet<Loci>,
}

impl<'a> Iterator for PathSegmentsIterator<'a> {
   type Item = MapMove;

   fn next(&mut self) -> Option<Self::Item> {
      match self.path_iter.next() {
         option_result @ Some(_) => {
            option_result
         }
         None => {
            // we're done with this route so grab the last locis
            self.last_locis.append(&mut self.path_iter.last_locis());

            self.next_loci_iter()
         }
      }
   }
}

impl<'a> PathIterator for PathSegmentsIterator<'a> {
   fn last_locis(&self) -> BTreeSet<Loci> {
      self.last_locis.clone()
   }
}

impl<'a> PathSegmentsIterator<'a> {
   fn next_loci_iter(&mut self) -> Option<MapMove> {
      match self.loci_iter.next() {
         Some(next_loci) => {
            // start our iteration over again
            self.path_iter = self.last_segment.path_iterator(&next_loci);

            self.next()
         }
         None => {
            match self.segment_iter.next() {
               Some(next_segment) => {
                  self.last_segment = Box::new(next_segment);

                  // swap out our loci iterators for a new empty set and get the next round of iterators
                  self.loci_iter = mem::replace(&mut self.last_locis, BTreeSet::new()).into_iter();

                  // start our iteration over again
                  self.next_loci_iter()
               }
               None => {
                  // we're done
                  None
               }
            }
         }
      }
   }
}

struct BranchIterator<'a> {
   branch_iter: Iter<'a, Path>,
   path_iter: PathIter<'a>,

   start_loci: Loci,
   last_locis: BTreeSet<Loci>,
}

impl<'a> Iterator for BranchIterator<'a> {
   type Item = MapMove;

   fn next(&mut self) -> Option<Self::Item> {
      match self.path_iter.next() {
         result @ Some(_) => result,
         None => {
            // we're done with this route so grab the last locis
            self.last_locis.append(&mut self.path_iter.last_locis());

            match self.branch_iter.next() {
               Some(next_branch) => {
                  // start our iteration over again
                  self.path_iter = next_branch.path_iterator(&self.start_loci);

                  self.next()
               }
               None => {
                  // We're done
                  None
               }
            }
         }
      }
   }
}

impl<'a> PathIterator for BranchIterator<'a> {
   fn last_locis(&self) -> BTreeSet<Loci> {
      self.last_locis.clone()
   }
}

struct StaticPathIterator<'a> {
   path_iter: Iter<'a, Direction>,

   last_loci: Loci,
}

impl<'a> Iterator for StaticPathIterator<'a> {
   //where I: Iterator<Item=&'a Direction> {
   type Item = MapMove;

   fn next(&mut self) -> Option<Self::Item> {
      self.path_iter.next()
         .map(|direction| {
            let inc = match direction {
               Direction::North => Loci::new(0, -1),
               Direction::East => Loci::new(1, 0),
               Direction::South => Loci::new(0, 1),
               Direction::West => Loci::new(-1, 0),
            };

            let door_loci = self.last_loci.add_loci(&inc);
            let room_loci = door_loci.add_loci(&inc);

            self.last_loci = room_loci.clone();

            [
               (door_loci, MapFeature::Door),
               (room_loci, MapFeature::Room),
            ]
         })
   }
}

impl<'a> PathIterator for StaticPathIterator<'a> {
   fn last_locis(&self) -> BTreeSet<Loci> {
      let mut result = BTreeSet::new();
      result.insert(self.last_loci);

      result
   }
}

struct EmptyPathIterator;

impl Iterator for EmptyPathIterator {
   type Item = MapMove;

   fn next(&mut self) -> Option<Self::Item> { None }
}

impl PathIterator for EmptyPathIterator {
   fn last_locis(&self) -> BTreeSet<Loci> {
      BTreeSet::new()
   }
}


pub fn parse_input(contents: &String) -> Path {
   let mut bytes = contents.chars().skip(1).peekable();

   let path = parse_path(&mut bytes);

   path
}

pub fn build_map(path: &Path) -> (Loci, Grid<MapFeature>) {
   let (max_up_down, max_left_right) = path.max_distance();
   // multiply by 2 to give us room for our doors between rooms
   let center = Loci::new((max_left_right as isize + 2) * 2, (max_up_down as isize + 2) * 2);
   let width = (center.x() as usize) * 2;
   let height = (center.y() as usize) * 2;

   let mut map = Grid::new(MapFeature::Wall, width, height);

   // always start in a room
   map.set_loci(&center, MapFeature::Room);

   for map_move in path.path_iterator(&center) {
      map.set_loci(&map_move[0].0, map_move[0].1);
      map.set_loci(&map_move[1].0, map_move[1].1);
   }

   (center, prune_map(map))
}

fn prune_map(old_map: Grid<MapFeature>) -> Grid<MapFeature> {
   let mut min = Loci::max_value();
   let mut max = Loci::min_value();

   for (loci, feature) in old_map.enumerate() {
      if *feature != MapFeature::Wall {
         min = min.min_x(loci.x()).min_y(loci.y());
         max = max.max_x(loci.x()).max_y(loci.y());
      }
   }

   // add some outer walls back in
   min = min.sub(1, 1);
   max = max.add(2, 2);

   let mut map = Grid::new_loci_offset(MapFeature::Wall, &max.sub_loci(&min), &min);

   for y in map.y_range() {
      for x in map.x_range() {
         map.set(x, y, *old_map.get(x, y))
      }
   }

   map
}

pub fn ab(start: &Loci, map: &Grid<MapFeature>) -> (usize, usize) {
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

fn parse_path<I>(input_iter: &mut Peekable<I>) -> Path
   where I: Iterator<Item=char> {
   let mut segments: Vec<Path> = vec![];
   loop {
      match input_iter.peek().unwrap() {
         '(' => segments.push(parse_branch(input_iter)),
         'N' | 'E' | 'S' | 'W' => {
            segments.push(parse_static(input_iter))
         }
         _ => break,
      }
   }

   if segments.len() == 0 {
      // if we have an empty path represent it as static
      Path::Static(vec![])
   }
   else if segments.len() == 1 {
      // if we only have one, don't bother with wrapping it
      segments.swap_remove(0)
   } else {
      Path::Segments(segments)
   }
}

fn parse_branch<I>(input_iter: &mut Peekable<I>) -> Path
   where I: Iterator<Item=char> {
   let mut branches: Vec<Path> = vec![];

   // consume the first (
   input_iter.next().unwrap();

   // consume the first path
   branches.push(parse_path(input_iter));

   loop {
      match input_iter.peek().unwrap() {
         ')' => break,
         '|' => {
            // consume a break, then the next path
            input_iter.next();
            branches.push(parse_path(input_iter));
         }
         _ => {
            // this shouldn't happen
            break;
         }
      };
   }

   // consume the last )
   input_iter.next();

   // if we only have one, don't bother with wrapping it
   if branches.len() == 1 {
      branches.swap_remove(0)
   } else {
      Path::Branch(branches)
   }
}

fn parse_static<I>(input_iter: &mut Peekable<I>) -> Path
   where I: Iterator<Item=char> {
   let mut path = vec![];
   loop {
      match input_iter.peek() {
         Some('N') => path.push(Direction::North),
         Some('E') => path.push(Direction::East),
         Some('S') => path.push(Direction::South),
         Some('W') => path.push(Direction::West),
         _ => break,
      }

      // actually move forward
      input_iter.next();
   }

   Path::Static(path)
}