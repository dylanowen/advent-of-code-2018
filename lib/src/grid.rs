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
    pub fn get(&self, cord: &Cord) -> &T {
        &self[cord.x][cord.y]
    }

    pub fn set(&mut self, cord: &Cord, value: T) {
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

    pub fn new_cord_offset(default: T, dim: &Cord, offset: &Cord) -> Grid<T> {
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

    pub fn cords(&self) -> GridCords {
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
            cords: GridCords::new(self),
        }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    cords: GridCords,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cords.next()
            .map(|cord| {
                self.grid.get(&cord)
            })
    }
}

pub struct GridEnumerator<'a, T> {
    iter: GridIterator<'a, T>
}

impl<'a, T> Iterator for GridEnumerator<'a, T> {
    type Item = (Cord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cord = self.iter.cords.cord();

        self.iter.next().map(|result| (cord.unwrap(), result))
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

    fn cord(&self) -> Option<Cord> {
        if self.x < self.width && self.y < self.height {
            Some(Cord::new(self.x, self.y))
        } else {
            None
        }
    }
}

impl Iterator for GridCords {
    type Item = Cord;

    fn next(&mut self) -> Option<Self::Item> {
        self.cord()
            .map(|cord| {
                self.y += 1;
                if self.y > self.height {
                    self.x += 1;
                    self.y = self.y_offset;
                }

                cord
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
pub struct Cord {
    x: isize,
    y: isize,
}

impl Cord {
    pub fn max_value() -> Cord {
        Cord::new(isize::max_value(), isize::max_value())
    }

    pub fn new(x: isize, y: isize) -> Cord {
        Cord {
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
    pub fn with_x(&self, x: isize) -> Cord {
        Cord::new(x, self.y)
    }

    #[inline]
    pub fn with_y(&self, y: isize) -> Cord {
        Cord::new(self.x, y)
    }

    pub fn distance(&self, other: &Cord) -> usize {
        ((self.x() - other.x()).abs() +
            (self.y() - other.y()).abs()) as usize
    }

    #[inline]
    pub fn add(&self, x: isize, y: isize) -> Cord {
        Cord::new(self.x + x, self.y + y)
    }

    #[inline]
    pub fn add_cord(&self, other: &Cord) -> Cord {
        self.add(other.x, other.y)
    }

    #[inline]
    pub fn add_x(&self, inc: isize) -> Cord {
        self.add(inc, 0)
    }

    #[inline]
    pub fn add_y(&self, inc: isize) -> Cord {
        self.add(0, inc)
    }

    #[inline]
    pub fn sub(&self, x: isize, y: isize) -> Cord {
        Cord::new(self.x - x, self.y - y)
    }

    #[inline]
    pub fn sub_cord(&self, other: &Cord) -> Cord {
        self.sub(other.x, other.y)
    }

    #[inline]
    pub fn sub_x(&self, inc: isize) -> Cord {
        self.sub(inc, 0)
    }

    #[inline]
    pub fn sub_y(&self, inc: isize) -> Cord {
        self.sub(0, inc)
    }
}

impl Clone for Cord {
    fn clone(&self) -> Self {
        Cord {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Cord {
    fn eq(&self, other: &Cord) -> bool {
        self.x == other.x && self.y == other.y
    }
}