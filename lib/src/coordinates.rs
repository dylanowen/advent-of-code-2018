use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    x_offset: isize,
    y_offset: isize,

    grid: Vec<GridColumn<T>>,
}

impl<T> Grid<T> {
    pub fn get(&self, cord: &Loci) -> &T {
        &self[cord.x][cord.y]
    }

    pub fn set(&mut self, cord: &Loci, value: T) {
        self[cord.x][cord.y] = value
    }

    fn raw_x(&self, x: isize) -> usize {
        (x - self.x_offset) as usize
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(default: T, width: usize, height: usize) -> Grid<T> {
        Grid::new_offset(default, width, height, 0, 0)
    }

    pub fn new_cord_offset(default: T, dim: &Loci, offset: &Loci) -> Grid<T> {
        Grid::new_offset(default, dim.x as usize, dim.y as usize, offset.x, offset.y)
    }

    pub fn new_offset(default: T, width: usize, height: usize, x_offset: isize, y_offset: isize) -> Grid<T> {
        let column = GridColumn {
            y_offset,
            column: vec![default; height],
        };

        Grid {
            width,
            height,
            x_offset,
            y_offset,
            grid: vec![column; width],
        }
    }

    pub fn locis(&self) -> GridCords {
        GridCords::new(self)
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

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = GridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: &self,
            locis: GridCords::new(self),
        }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    locis: GridCords,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.locis.next()
            .map(|loci| {
                self.grid.get(&loci)
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

pub struct GridCords {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    y_offset: isize,
}

impl GridCords {
    fn new<T>(grid: &Grid<T>) -> GridCords {
        GridCords {
            x: grid.x_offset,
            y: grid.y_offset,
            width: grid.width as isize,
            height: grid.height as isize,
            y_offset: grid.y_offset,
        }
    }

    fn loci(&self) -> Option<Loci> {
        if self.x < self.width && self.y < self.height {
            Some(Loci::new(self.x, self.y))
        } else {
            None
        }
    }
}

impl Iterator for GridCords {
    type Item = Loci;

    fn next(&mut self) -> Option<Self::Item> {
        self.loci()
            .map(|loci| {
                self.y += 1;
                if self.y > self.height {
                    self.x += 1;
                    self.y = self.y_offset;
                }

                loci
            })
    }
}


impl<T> Index<isize> for Grid<T> {
    type Output = GridColumn<T>;

    fn index(&self, index: isize) -> &Self::Output {
        &self.grid[self.raw_x(index)]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = GridColumn<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index as isize]
    }
}

impl<T> IndexMut<isize> for Grid<T> {
    fn index_mut(&mut self, index: isize) -> &mut GridColumn<T> {
        let raw_x = self.raw_x(index);

        &mut self.grid[raw_x]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut GridColumn<T> {
        &mut self[index as isize]
    }
}

#[derive(Debug)]
pub struct GridColumn<T> {
    y_offset: isize,
    column: Vec<T>,
}

impl<T> GridColumn<T> {
    fn raw_y(&self, y: isize) -> usize {
        (y - self.y_offset) as usize
    }
}

impl<T> Index<isize> for GridColumn<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        &self.column[self.raw_y(index)]
    }
}

impl<T> Index<usize> for GridColumn<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index as isize]
    }
}

impl<T> IndexMut<isize> for GridColumn<T> {
    fn index_mut(&mut self, index: isize) -> &mut T {
        let raw_y = self.raw_y(index);

        &mut self.column[raw_y]
    }
}

impl<T> IndexMut<usize> for GridColumn<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self[index as isize]
    }
}

impl<T: Clone> Clone for GridColumn<T> {
    fn clone(&self) -> Self {
        GridColumn {
            y_offset: self.y_offset,
            column: self.column.to_vec(),
        }
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