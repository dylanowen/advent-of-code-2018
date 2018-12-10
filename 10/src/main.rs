extern crate lib;
extern crate regex;

use regex::Regex;

use lib::*;

fn main() {
   run_day("9", &|contents, _is_sample| {
//      let re: Regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
//
//      let parsed = re.captures(contents).unwrap();
//
//      let players = parsed[1].parse::<usize>().unwrap();
//      let max_marble = parsed[2].parse::<usize>().unwrap();

      a();
      b();
   });
}

fn a() {
   println!("Result A: {}", 0);
}

fn b() {
   println!("Result B: {}", 0);
}