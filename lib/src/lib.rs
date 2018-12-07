use std::fs::File;
use std::io::prelude::*;

pub mod grid;



pub fn run_day<R>(day: &str, runner: &R) where
    R: Fn(&String, bool) {

    run_day_sample(day, runner);

    let input = parse_input(day, "input.txt");

    println!("Running");
    runner(&input, false);
}

// for when we're not ready to run the full thing
pub fn run_day_sample<R>(day: &str, runner: &R) where
    R: Fn(&String, bool) {

    let sample_input = parse_input(day, "sample_input.txt");

    println!("Running Sample");
    runner(&sample_input, true);
}


pub fn parse_input(day: &str, file_name: &str) -> String {
    let mut f = load_input(day, file_name);

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

// handle my non-standard setup
pub fn load_input(day: &str, file_name: &str) -> File {
    return File::open(file_name)
        .or(File::open(format!("{}/{}", day, file_name)))
        .expect(format!("file not found: {}", file_name).as_ref());
}