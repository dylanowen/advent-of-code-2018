extern crate lib;
extern crate regex;

use regex::Regex;

use lib::*;
use lib::grid::Cord;
use lib::grid::Grid;

fn main() {
    run_day("6", &|contents, is_sample| {
        let re: Regex = Regex::new(r"(\d+), (\d+)").unwrap();

        let mut min = Cord::max_value();
        let mut max = Cord::new(0, 0);
        let locations: Vec<Cord> = contents.lines()
            .map(|row| {
                let parsed_row = re.captures(row).unwrap();

                let x = parsed_row[1].parse::<isize>().unwrap();
                let y = parsed_row[2].parse::<isize>().unwrap();

                if x < min.x() {
                    min = min.with_x(x);
                }
                if x > max.x() {
                    max = max.with_x(x);
                }

                if y < min.y() {
                    min = min.with_y(y);
                }
                if y > max.y() {
                    max = max.with_y(y);
                }

                return Cord::new(x, y);
            })
            .collect();

        // give us some breathing room
        min = min.sub(1, 1);
        max = max.add(1, 1);

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
    let mut region_grid: grid::Grid<usize> = Grid::new_cord_offset(
        0,
        &max.sub_cord(min),
        min,
    );

    'main: for (cord, _) in region_grid.enumerate() {
        let mut total_distance = 0;
        for location in locations {
            total_distance += location.distance(&cord);
            if total_distance > region_range {
                // we hit our limit so break out
                continue 'main;
            }
        }

        // if we get here we know we're in a region
        region_grid.set(&cord, 1);
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


    let region_size = region_grid.iter()
        .fold(0, |sum, cell| -> usize {
            sum + cell
        });

    println!("Result B: {}", region_size);
}

fn a(locations: &Vec<Cord>, min: &Cord, max: &Cord) {
    let mut manhattan_grid: grid::Grid<Option<&Cord>> = Grid::new_cord_offset(
        None,
        &max.sub_cord(min),
        min,
    );

    for cord in manhattan_grid.cords() {
        manhattan_grid.set(&cord, find_closest_location(&cord, locations))
    }

    // trim infinity locations
    let mut finite_locations = locations.iter()
        .map(|location| -> &Cord { &location })
        .collect();
    for x in 0..manhattan_grid.width {
        prune_infinite(manhattan_grid[x][0isize], &mut finite_locations);
        prune_infinite(manhattan_grid[x][manhattan_grid.height - 1], &mut finite_locations);
    }
    for y in 1..(manhattan_grid.height - 1) {
        prune_infinite(manhattan_grid[0isize][y], &mut finite_locations);
        prune_infinite(manhattan_grid[manhattan_grid.width - 1][y], &mut finite_locations);
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

        for maybe_cord in manhattan_grid.iter() {
            match maybe_cord {
                Some(cord) => if *cord == location {
                    area += 1;
                }
                _ => {}
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
        let distance = current_cord.distance(location);
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