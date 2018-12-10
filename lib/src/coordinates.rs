use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;

#[derive(Debug)]
pub struct Grid<T> {
   width: usize,
   height: usize,
   x_offset: isize,
   y_offset: isize,

   grid: Vec<Vec<T>>,
}

pub trait OffsetLociX {
   fn width(&self) -> usize;

   fn x_min(&self) -> isize;

   // exclusive max
   fn x_max(&self) -> isize {
      self.x_min() + (self.width() as isize)
   }

   fn raw_x(&self, x: isize) -> usize {
      (x - self.x_min()) as usize
   }

   fn real_x(&self, raw_x: usize) -> isize {
      (raw_x as isize) + self.x_min()
   }

   fn x_range(&self) -> Range<isize> {
      self.x_min()..self.x_max()
   }
}

pub trait OffsetLociY {
   fn height(&self) -> usize;

   fn y_min(&self) -> isize;

   // exclusive max
   fn y_max(&self) -> isize {
      self.y_min() + (self.height() as isize)
   }

   fn raw_y(&self, y: isize) -> usize {
      (y - self.y_min()) as usize
   }

   fn real_y(&self, raw_y: usize) -> isize {
      (raw_y as isize) + self.y_min()
   }

   fn y_range(&self) -> Range<isize> {
      self.y_min()..self.y_max()
   }
}

impl<T> Grid<T> {
   pub fn get_loci(&self, loci: &Loci) -> &T {
      self.get(loci.x, loci.y)
   }

   pub fn get(&self, x: isize, y: isize) -> &T {
      &self.grid[self.raw_y(y)][self.raw_x(x)]
   }

   pub fn set_loci(&mut self, loci: &Loci, value: T) {
      self.set(loci.x, loci.y, value);
   }

   pub fn set(&mut self, x: isize, y: isize, value: T) {
      let raw_x = self.raw_x(x);
      let raw_y = self.raw_y(y);

      self.grid[raw_y][raw_x] = value
   }
}

impl<T: Clone> Grid<T> {
   pub fn new(default: T, width: usize, height: usize) -> Grid<T> {
      Grid::new_offset(default, width, height, 0, 0)
   }

   pub fn new_loci_offset(default: T, dim: &Loci, offset: &Loci) -> Grid<T> {
      Grid::new_offset(default, dim.x as usize, dim.y as usize, offset.x, offset.y)
   }

   pub fn new_offset(default: T, width: usize, height: usize, x_offset: isize, y_offset: isize) -> Grid<T> {
      Grid {
         width,
         height,
         x_offset,
         y_offset,
         grid: vec![vec![default; width]; height],
      }
   }

   pub fn locis(&self) -> GridLocis {
      GridLocis::new(self)
   }

   pub fn iter(&self) -> GridIterator<T> {
      self.into_iter()
   }

   pub fn enumerate(&self) -> GridEnumerator<T> {
      GridEnumerator {
         iter: self.into_iter()
      }
   }
}

impl<T> OffsetLociX for Grid<T> {
   fn width(&self) -> usize {
      self.width
   }

   fn x_min(&self) -> isize {
      self.x_offset
   }
}

impl<T> OffsetLociY for Grid<T> {
   fn height(&self) -> usize {
      self.height
   }

   fn y_min(&self) -> isize {
      self.y_offset
   }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
   type Item = &'a T;
   type IntoIter = GridIterator<'a, T>;

   fn into_iter(self) -> Self::IntoIter {
      GridIterator {
         grid: &self,
         locis: GridLocis::new(self),
      }
   }
}

pub struct GridIterator<'a, T> {
   grid: &'a Grid<T>,
   locis: GridLocis,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
   type Item = &'a T;

   fn next(&mut self) -> Option<Self::Item> {
      self.locis.next()
         .map(|loci| {
            self.grid.get_loci(&loci)
         })
   }
}

pub struct GridEnumerator<'a, T> {
   iter: GridIterator<'a, T>
}

impl<'a, T> Iterator for GridEnumerator<'a, T> {
   type Item = (Loci, &'a T);

   fn next(&mut self) -> Option<Self::Item> {
      let loci = self.iter.locis.loci();

      self.iter.next().map(|result| (loci.unwrap(), result))
   }
}

#[derive(Debug)]
pub struct GridLocis {
   x: isize,
   y: isize,
   width: usize,
   height: usize,
   x_offset: isize,
   y_offset: isize,
}

impl GridLocis {
   fn new<T>(grid: &Grid<T>) -> GridLocis {
      return GridLocis {
         x: grid.x_offset,
         y: grid.y_offset,
         width: grid.width,
         height: grid.height,
         x_offset: grid.x_offset,
         y_offset: grid.y_offset,
      };
   }

   fn loci(&self) -> Option<Loci> {
      //println!("current: {},{} to {}, {}", self.x, self.y, self.x_max(), self.y_max());
      if self.x < self.x_max() && self.y < self.y_max() {
         Some(Loci::new(self.x, self.y))
      } else {
         None
      }
   }
}

impl OffsetLociX for GridLocis {
   fn width(&self) -> usize {
      self.width
   }

   fn x_min(&self) -> isize {
      self.x_offset
   }
}

impl OffsetLociY for GridLocis {
   fn height(&self) -> usize {
      self.height
   }

   fn y_min(&self) -> isize {
      self.y_offset
   }
}

impl Iterator for GridLocis {
   type Item = Loci;

   fn next(&mut self) -> Option<Self::Item> {
      self.loci()
         .map(|loci| {
            self.x += 1;
            if self.x >= self.x_max() {
               self.y += 1;
               self.x = self.x_min();
            }

            loci
         })
   }
}

#[derive(Debug)]
pub struct Loci {
   x: isize,
   y: isize,
}

impl Loci {
   pub fn max_value() -> Loci {
      Loci::new(isize::max_value(), isize::max_value())
   }

   pub fn new(x: isize, y: isize) -> Loci {
      Loci {
         x,
         y,
      }
   }

   #[inline]
   pub fn x(&self) -> isize {
      self.x
   }

   #[inline]
   pub fn y(&self) -> isize {
      self.y
   }

   #[inline]
   pub fn with_x(&self, x: isize) -> Loci {
      Loci::new(x, self.y)
   }

   #[inline]
   pub fn with_y(&self, y: isize) -> Loci {
      Loci::new(self.x, y)
   }

   pub fn distance(&self, other: &Loci) -> usize {
      ((self.x() - other.x()).abs() +
         (self.y() - other.y()).abs()) as usize
   }

   #[inline]
   pub fn add(&self, x: isize, y: isize) -> Loci {
      Loci::new(self.x + x, self.y + y)
   }

   #[inline]
   pub fn add_loci(&self, other: &Loci) -> Loci {
      self.add(other.x, other.y)
   }

   #[inline]
   pub fn add_x(&self, inc: isize) -> Loci {
      self.add(inc, 0)
   }

   #[inline]
   pub fn add_y(&self, inc: isize) -> Loci {
      self.add(0, inc)
   }

   #[inline]
   pub fn sub(&self, x: isize, y: isize) -> Loci {
      Loci::new(self.x - x, self.y - y)
   }

   #[inline]
   pub fn sub_loci(&self, other: &Loci) -> Loci {
      self.sub(other.x, other.y)
   }

   #[inline]
   pub fn sub_x(&self, inc: isize) -> Loci {
      self.sub(inc, 0)
   }

   #[inline]
   pub fn sub_y(&self, inc: isize) -> Loci {
      self.sub(0, inc)
   }
}

impl Clone for Loci {
   fn clone(&self) -> Self {
      Loci {
         x: self.x,
         y: self.y,
      }
   }
}

impl PartialEq for Loci {
   fn eq(&self, other: &Loci) -> bool {
      self.x == other.x && self.y == other.y
   }
}