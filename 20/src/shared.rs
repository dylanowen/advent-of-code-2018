use std::fmt;
use core::iter::Peekable;
use std::collections::BTreeSet;

use common::coordinates::Grid;
use common::coordinates::Loci;
use common::coordinates::OffsetLociX;
use common::coordinates::OffsetLociY;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
   North,
   East,
   South,
   West,
}

//pub const DIRECTIONS: [Direction; 4] = [
//   Direction::North,
//   Direction::East,
//   Direction::South,
//   Direction::West,
//];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapFeature {
   Room,
   Door,
   Wall,
   //Unknown,
}

pub trait Path: fmt::Display {
   fn build_map(&self, start: &Loci, map: &mut Grid<MapFeature>) -> BTreeSet<Loci>;

   // calculate the max distance (up/down, left/right)
   fn max_distance(&self) -> (usize, usize);
}

struct PathSegments {
   segments: Vec<Box<Path>>,
}

struct Branch {
   branches: Vec<Box<Path>>,
}

struct StaticPath {
   path: Vec<Direction>,
}

impl Path for PathSegments {
   fn build_map(&self, start: &Loci, map: &mut Grid<MapFeature>) -> BTreeSet<Loci> {
      let mut current_locis = BTreeSet::new();
      current_locis.insert(start.clone());

      for segment in self.segments.iter() {
         let mut next_locis = BTreeSet::new();

         for loci in current_locis.iter() {
            next_locis.append(&mut segment.build_map(loci, map));
         }

         current_locis = next_locis;
      }

      current_locis
   }

   fn max_distance(&self) -> (usize, usize) {
      self.segments.iter().fold((0, 0), |(ud, lr), p| {
         let (p_ud, p_lr) = p.max_distance();

         (ud + p_ud, lr + p_lr)
      })
   }
}

impl Path for Branch {
   fn build_map(&self, start: &Loci, map: &mut Grid<MapFeature>) -> BTreeSet<Loci> {
      let mut locis = BTreeSet::new();

      for branch in self.branches.iter() {
         locis.append(&mut branch.build_map(start, map));
      }

      locis
   }

   fn max_distance(&self) -> (usize, usize) {
      self.branches.iter().fold((0, 0), |(ud, lr), p| {
         let (p_ud, p_lr) = p.max_distance();

         (ud.max(p_ud), lr.max(p_lr))
      })
   }
}


impl Path for StaticPath {
   fn build_map(&self, start: &Loci, map: &mut Grid<MapFeature>) -> BTreeSet<Loci> {
      let mut location = start.clone();

      for direction in self.path.iter() {
         let inc = match direction {
            Direction::North => Loci::new(0, -1),
            Direction::East => Loci::new(1, 0),
            Direction::South => Loci::new(0, 1),
            Direction::West => Loci::new(-1, 0),
         };

         location = location.add_loci(&inc);
         map.set_loci(&location, MapFeature::Door);

         location = location.add_loci(&inc);
         map.set_loci(&location, MapFeature::Room);
      }

      let mut end = BTreeSet::new();
      end.insert(location);

      end
   }

   fn max_distance(&self) -> (usize, usize) {
      let (up_down, left_right) = self.path.iter().fold((0 as isize, 0 as isize), |(ud, lr), direction| {
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


pub fn parse_input(contents: &String) -> Box<Path> {
   let mut bytes = contents.chars().skip(1).peekable();

   let path = parse_path(&mut bytes);

   path
}

pub fn build_map(path: Box<Path>) -> (Loci, Grid<MapFeature>) {
   let (max_up_down, max_left_right) = path.max_distance();
   // multiply by 2 to give us room for our doors between rooms
   let center = Loci::new((max_left_right as isize + 2) * 2, (max_up_down as isize + 2) * 2);
   let width = (center.x() as usize) * 2;
   let height = (center.y() as usize) * 2;

   //let mut seen_grid = Grid::new(vec![], width, height);
   let mut map = Grid::new(MapFeature::Wall, width, height);

   // always start in a room
   map.set_loci(&center, MapFeature::Room);

   path.build_map(&center, &mut map);

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

fn parse_path<I>(input_iter: &mut Peekable<I>) -> Box<Path>
   where I: Iterator<Item=char> {
   let mut segments: Vec<Box<Path>> = vec![];
   loop {
      match input_iter.peek().unwrap() {
         '(' => segments.push(parse_branch(input_iter)),
         'N' | 'E' | 'S' | 'W' => {
            segments.push(parse_static(input_iter))
         }
         _ => break,
      }
   }

   // if we only have one, don't bother with wrapping it
   if segments.len() == 1 {
      segments.swap_remove(0)
   } else {
      Box::new(PathSegments {
         segments,
      })
   }
}

fn parse_branch<I>(input_iter: &mut Peekable<I>) -> Box<Path>
   where I: Iterator<Item=char> {
   let mut branches: Vec<Box<Path>> = vec![];

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
      Box::new(Branch {
         branches,
      })
   }
}

fn parse_static<I>(input_iter: &mut Peekable<I>) -> Box<Path>
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

   Box::new(StaticPath {
      path
   })
}

impl fmt::Display for PathSegments {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for segment in self.segments.iter() {
         write!(f, "{}", *segment)?;
      }

      Ok(())
   }
}

impl fmt::Display for Branch {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "(")?;
      let mut first = true;
      for branch in self.branches.iter() {
         if !first {
            write!(f, "|")?;
         } else {
            first = false;
         }
         write!(f, "{}", *branch)?;
      }
      write!(f, ")")
   }
}

impl fmt::Display for StaticPath {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for direction in self.path.iter() {
         write!(f, "{}", direction)?;
      }

      Ok(())
   }
}

impl fmt::Display for Direction {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", format!("{:?}", self).chars().next().unwrap())
   }
}

impl fmt::Display for MapFeature {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         MapFeature::Room | MapFeature::Door => write!(f, " "),
         //MapFeature::Door => write!(f, "|"),
         MapFeature::Wall => write!(f, "#"),
         //MapFeature::Unknown => write!(f, "?"),
      }
   }
}