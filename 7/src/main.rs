extern crate lib;
extern crate regex;

use regex::Regex;

use lib::*;
use lib::coordinates::Grid;
use lib::coordinates::Loci;

fn main() {
   run_day("6", &|contents, is_sample| {
      //let re: Regex = Regex::new(r"(\d+), (\d+)").unwrap();


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