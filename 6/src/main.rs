extern crate lib;
extern crate regex;

use regex::Regex;

use lib::*;
use lib::grid;

#[derive(Debug)]
struct Cord(usize, usize);

impl PartialEq for Cord {
    fn eq(&self, other: &Cord) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn main() {
    grid::new_offset(0, 1, 1, 0, 0);

    run_day("6", &|contents, is_sample| {
        let re: Regex = Regex::new(r"(\d+), (\d+)").unwrap();

        let mut min = Cord(usize::max_value(), usize::max_value());
        let mut max = Cord(0, 0);
        let locations: Vec<Cord> = contents.lines()
            .map(|row| {
                let parsed_row = re.captures(row).unwrap();

                let x = parsed_row[1].parse::<usize>().unwrap();
                let y = parsed_row[2].parse::<usize>().unwrap();

                if x < min.0 {
                    min.0 = x;
                }
                if x > max.0 {
                    max.0 = x;
                }

                if y < min.1 {
                    min.1 = x;
                }
                if y > max.1 {
                    max.1 = y;
                }

                return Cord(x, y);
            })
            .collect();

        // give us some breathing room
        min.0 -= 1;
        min.1 -= 1;
        max.0 += 1;
        max.1 += 1;

        // get the region depending on if we're running the sample or not
        let region_range;
        if is_sample {
            region_range = 32;
        } else {
            region_range = 10000;
        }

        a(&locations, &min, &max);
        b(&locations, &min, &max, region_range);
    });
}

fn b(locations: &Vec<Cord>, min: &Cord, max: &Cord, region_range: usize) {
    let x_offset = min.0;
    let y_offset = min.1;

    let width = max.0 - x_offset;
    let height = max.1 - y_offset;

    let mut grid: Vec<Vec<usize>> = vec![vec![0; height]; width];

    for x_raw in 0..grid.len() {
        let x = x_raw + x_offset;
        'y: for y_raw in 0..grid[x_raw].len() {
            let y = y_raw + y_offset;
            let current_cord = Cord(x, y);

            let mut total_distance = 0;
            for location in locations {
                total_distance += manhattan_distance(location, &current_cord);
                if total_distance > region_range {
                    // we hit our limit so break out
                    continue 'y;
                }
            }

            // if we get here we know we're in a region
            grid[x_raw][y_raw] = 1;
        }
    }

//    for y in 0..height {
//        for x in 0..width {
//            match grid[x][y] {
//                true => print!("#"),
//                _ => print!("."),
//            }
//        }
//        println!()
//    }

    // count up the size of our region
    let region_size = grid.iter()
        .fold(0, |sum, column| -> usize {
            sum + column.iter()
                .fold(0, |inner_sum, cell| inner_sum + cell)
        });

    println!("Result B: {}", region_size);
}

fn a(locations: &Vec<Cord>, min: &Cord, max: &Cord) {
    let x_offset = min.0;
    let y_offset = min.1;

    let width = max.0 - x_offset;
    let height = max.1 - y_offset;

    let mut manhattan_grid: Vec<Vec<Option<&Cord>>> = vec![vec![None; height]; width];

    for x_raw in 0..manhattan_grid.len() {
        let x = x_raw + x_offset;
        for y_raw in 0..manhattan_grid[x_raw].len() {
            let y = y_raw + y_offset;
            let current_cord = Cord(x, y);

            manhattan_grid[x_raw][y_raw] = find_closest_location(&current_cord, locations);

            //println!("found({:?})~>[{}][{}] {:?}", current_cord, x_raw, y_raw, grid[x_raw][y_raw])
        }
    }

    // trim infinity locations
    let mut finite_locations = locations.iter()
        .map(|location| -> &Cord { &location })
        .collect();
    for x in 0..width {
        prune_infinite(manhattan_grid[x][0], &mut finite_locations);
        prune_infinite(manhattan_grid[x][height - 1], &mut finite_locations);
    }
    for y in 1..(height - 1) {
        prune_infinite(manhattan_grid[0][y], &mut finite_locations);
        prune_infinite(manhattan_grid[width - 1][y], &mut finite_locations);
    }

    // debug
//    for y in 0..height {
//        for x in 0..width {
//            match grid[x][y] {
//                Some(_) => print!("#"),
//                _ => print!("."),
//            }
//        }
//        println!()
//    }

//    for finite_location in finite_locations.iter() {
//        println!("{:?}", finite_location);
//    }

    let mut max_area = 0;
    for location in finite_locations {
        let mut area = 0;
        for column in manhattan_grid.iter() {
            for maybe_cord in column.iter() {
                match maybe_cord {
                    Some(cord) => if *cord == location {
                        area += 1;
                    }
                    _ => {}
                }
            }
        }

        if area > max_area {
            max_area = area;
        }
    }

    println!("Result A: {}", max_area);
}

fn prune_infinite(infinite_location: Option<&Cord>, finite_locations: &mut Vec<&Cord>) {
    match infinite_location {
        Some(infinite) => {
            finite_locations.iter()
                .position(|cord| *cord == infinite)
                .map(|position| finite_locations.remove(position));
        }
        _ => {}
    }
}

fn find_closest_location<'a>(current_cord: &Cord, locations: &'a Vec<Cord>) -> Option<&'a Cord> {
    let mut min_distance = usize::max_value();
    let mut closest = None;
    let mut dup = false;
    for location in locations {
        let distance = manhattan_distance(current_cord, location);
        //println!("{}", distance);

        if distance < min_distance {
            closest = Some(location);
            min_distance = distance;
            dup = false;
        } else if distance == min_distance {
            dup = true;
        }
    }

    // we found a duplicate so don't return anything
    if dup {
        return None;
    }

    return closest;
}

fn manhattan_distance(left: &Cord, right: &Cord) -> usize {
    return ((left.0 as isize - right.0 as isize).abs() +
        (left.1 as isize - right.1 as isize).abs()) as usize;
}