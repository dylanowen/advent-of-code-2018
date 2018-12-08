extern crate lib;

use std::collections::HashMap;
use std::iter::FromIterator;

use lib::*;

struct Node {
   children: Vec<Node>,
   metadata: Vec<usize>,
}

fn main() {
   run_day("9", &|contents, _is_sample| {


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