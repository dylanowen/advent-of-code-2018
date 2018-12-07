use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use chrono::prelude::*;
use std::collections::HashMap;

extern crate regex;
extern crate chrono;

//struct Row(DateTime<Utc>, String);

fn main() {
   run("sample_input.txt");
   println!("");
   run("input.txt");
}

fn run(file_name: &str) {
   /*let rows: Vec<Row> =*/ parse_input(file_name);

   //let guard_schedules = calculate_guard_schedules(&rows);

   //a(&guard_schedules);
   //b(&guard_schedules);
}

fn parse_input(file_name: &str) /*-> Vec<Row>*/ {
   let re = Regex::new(r"\[(.*)\] (.+)").unwrap();

   let mut f = File::open(file_name).expect("file not found");

   let mut contents = String::new();
   f.read_to_string(&mut contents)
      .expect("something went wrong reading the file");

   let mut rows: Vec<usize> = contents.lines()
      .map(|row| {
         let parsed_row = re.captures(row).unwrap();

         //let date_time = Utc.datetime_from_str(&parsed_row[1], "%Y-%m-%d %H:%M").unwrap();

         //return Row(date_time, parsed_row[2].to_string());
         return 0;
      })
      .collect();

   //rows.sort_by_key(|row| row.0);

   //return rows;
}

/*
fn a() {

   println!("Result A: {}", );
}

fn b() {
   println!("Result B: {}", );
}
*/