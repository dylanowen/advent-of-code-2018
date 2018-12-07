extern crate chrono;
extern crate lib;
extern crate regex;

use std::collections::HashMap;

use chrono::prelude::*;
use regex::Regex;

use lib::*;

struct Row(DateTime<Utc>, String);

fn main() {
   run_day("4", &|contents, _is_sample| {
      let re = Regex::new(r"\[(.*)\] (.+)").unwrap();

      let mut rows: Vec<Row> = contents.lines()
         .map(|row| {
            let parsed_row = re.captures(row).unwrap();

            let date_time = Utc.datetime_from_str(&parsed_row[1], "%Y-%m-%d %H:%M").unwrap();

            return Row(date_time, parsed_row[2].to_string());
         })
         .collect();

      rows.sort_by_key(|row| row.0);

      // debug
      // for row in &rows {
      //    println!("{} {}", row.0, row.1);
      // }

      let guard_schedules = calculate_guard_schedules(&rows);

      a(&guard_schedules);
      b(&guard_schedules);
   });
}

fn calculate_guard_schedules(rows: &Vec<Row>) -> HashMap<usize, [usize; 60]> {
   let re = Regex::new(r"#(\d+)").unwrap();

   let mut guard_schedules = HashMap::new();

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
               guard_schedules.insert(current_guard, guard_sleep_count);

               // debug
               // print!("{:4}: ", current_guard);
               // for count in guard_sleep_count.iter() {
               //    print!("{:2}", count);
               // }
               // println!("");
            }

            let shift_change = re.captures(shift_row).unwrap();

            current_guard = shift_change[1].parse::<usize>().unwrap();
            guard_sleep_count = match guard_schedules.get(&current_guard) {
               Some(last_sleep) => *last_sleep,
               _ => [0; 60],
            };
            awake = true;
         }
      }
   }

   // write out our last guard
   guard_schedules.insert(current_guard, guard_sleep_count);

   // debug
   // print!("{:4}: ", current_guard);
   // for count in guard_sleep_count.iter() {
   //    print!("{:2}", count);
   // }
   // println!("");
   // println!("Finished Counting");

   return guard_schedules;
}

fn a(guard_schedules: &HashMap<usize, [usize; 60]>) {
   let mut max_sleep = 0;
   let mut max_guard = 0;
   for (id, &guard_schedule) in guard_schedules.iter() {
      let mut sleep = 0;

      for count in guard_schedule.iter() {
         sleep += *count;
      }

      // debug
      // print!("{:4}: {:2} :", id, sleep);
      // for count in guard_schedule.iter() {
      //    print!("{:2}", count);
      // }
      // println!("");

      if max_sleep < sleep {
         //println!("New Max: {}", sleep);
         max_sleep = sleep;
         max_guard = *id;
      }
   }

   let mut max_minute = 0;
   let mut max_minute_count = 0;
   for (minute, count) in guard_schedules.get(&max_guard).unwrap().iter().enumerate() {
      if max_minute_count < *count {
         max_minute_count = *count;
         max_minute = minute;
      }
   }

   println!("Result A: {} * {} = {}", max_guard, max_minute, max_guard * max_minute);
}

fn b(guard_schedules: &HashMap<usize, [usize; 60]>) {
   let mut max_sleep_count = 0;
   let mut max_guard = 0;
   let mut max_minute = 0;
   for (id, &guard_schedule) in guard_schedules.iter() {
      for (minute, count) in guard_schedule.iter().enumerate() {
         if max_sleep_count < *count {
            max_sleep_count = *count;
            max_guard = *id;
            max_minute = minute;
         }
      }
   }

   println!("Result B: {} * {} = {}", max_guard, max_minute, max_guard * max_minute);
}