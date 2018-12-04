use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use chrono::prelude::*;
use std::collections::HashMap;

extern crate regex;
extern crate chrono;

struct Row(DateTime<Utc>, String);

// struct Event<'a> {
//    id: &'a str,
//    x: usize,
//    y: usize,
//    width: usize,
//    height: usize,
// }

fn main() {
   let re = Regex::new(r"\[(.*)\] (.+)").unwrap();

   let mut f = File::open("input.txt").expect("file not found");

   let mut contents = String::new();
   f.read_to_string(&mut contents)
      .expect("something went wrong reading the file");

   let mut rows: Vec<Row> = contents.lines()
      .map(|row| {
         let parsed_row = re.captures(row).unwrap();

         let date_time = Utc.datetime_from_str(&parsed_row[1], "%Y-%m-%d %H:%M").unwrap();

         return Row(date_time, parsed_row[2].to_string());
      })
      .collect();

   rows.sort_by_key(|row| row.0);

   // for row in &rows {
   //    println!("{} {}", row.0, row.1);
   // }

   a(&rows);
   //b(&claims, &cloth);
}

fn a(rows: &Vec<Row>) {
   let re = Regex::new(r"#(\d+)").unwrap();

   let mut guard_events = HashMap::new();

   let mut current_guard = 0;
   let mut guard_sleep_count = [0; 60];
   let mut awake = true;
   for row in rows {
      let minute = row.0.minute() as usize;

      match row.1.as_ref() {
         "wakes up" => {
            if !awake {
               awake = true;
               for i in minute..60 {
                  guard_sleep_count[i] -= 1;
               }
            }
         }
         "falls asleep" => {
            if awake {
               awake = false;
               for i in minute..60 {
                  guard_sleep_count[i] += 1;
               }
            }
         }
         shift_row => {
            // write out our last guard
            if current_guard != 0 {
               guard_events.insert(current_guard, guard_sleep_count);

               // print!("{:4}: ", current_guard);
               // for count in guard_sleep_count.iter() {
               //    print!("{:2}", count);
               // }
               // println!("");
            }

            let shift_change = re.captures(shift_row).unwrap();

            current_guard = shift_change[1].parse::<usize>().unwrap();
            guard_sleep_count = match guard_events.get(&current_guard) {
               Some(last_sleep) => *last_sleep,
               _ => [0; 60],
            };
            awake = true;
         }
      }
   }

   // write out our last guard
   // guard_events.insert(current_guard, guard_sleep_count);
   // print!("{:4}: ", current_guard);
   // for count in guard_sleep_count.iter() {
   //    print!("{:2}", count);
   // }
   // println!("");

   println!("Finished Counting");

   let mut max_sleep = 0;
   let mut max_guard = 0;
   for (id, &guard_event) in guard_events.iter() {
      let mut sleep = 0;

      for count in guard_event.iter() {
         sleep += *count;
      }

      // debug
      // print!("{:4}: {:2} :", id, sleep);
      // for count in guard_event.iter() {
      //    print!("{:2}", count);
      // }
      // println!("");

      if max_sleep < sleep {
         println!("New Max: {}", sleep);
         max_sleep = sleep;
         max_guard = *id;
      }
   }

   let mut max_minute = 0;
   let mut max_minute_count = 0;
   for (minute, count) in guard_events.get(&max_guard).unwrap().iter().enumerate() {
      if max_minute_count < *count {
         max_minute_count = *count;
         max_minute = minute;
      }
   }

   println!("Result A: {} * {} = {}", max_guard, max_minute, max_guard * max_minute);

   let mut max_sleep_count = 0;
   for (id, &guard_event) in guard_events.iter() {
      for (minute, count) in guard_event.iter().enumerate() {
         if max_sleep_count < *count {
            max_sleep_count = *count;
            max_guard = *id;
            max_minute = minute;
         }
      }
   }

   println!("Result B: {} * {} = {}", max_guard, max_minute, max_guard * max_minute);
}
/*
fn b(claims: &Vec<Claim>, cloth: &Vec<Vec<i32>>) {
   for claim in claims {
      let mut overlap = 0;
      for y in claim.y..(claim.y + claim.height) {
         for x in claim.x..(claim.x + claim.width) {
            if cloth[x][y] > 1 {
               overlap += 1;
            }
         }
      }

      if overlap == 0 {
         println!("Result B: {}", claim.id);
         break;
      }
   }
}

fn fill_cloth(claim: &Claim, cloth: &mut Vec<Vec<i32>>) {
   for y in claim.y..(claim.y + claim.height) {
      for x in claim.x..(claim.x + claim.width) {
         cloth[x][y] += 1;
      }
   }
}
*/