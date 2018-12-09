extern crate lib;
extern crate regex;

use regex::Regex;

use lib::*;

fn main() {
   run_day("9", &|contents, is_sample| {
      let re: Regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

      let parsed = re.captures(contents).unwrap();

      let players = parsed[1].parse::<usize>().unwrap();
      let max_marble = parsed[2].parse::<usize>().unwrap();

      a(players, max_marble);

      if !is_sample {
         b(players, max_marble);
      }
   });
}


fn a(players: usize, max_marble: usize) {
   let high_score = play_game(players, max_marble);
   println!("Result A: {}", high_score);
}

fn b(players: usize, max_marble: usize) {
   let mut last_high_score = 0;
   let mut last_max_marble = 0;
   for marble in 0..=max_marble {
      let high_score = play_game(players, marble);

      if high_score != last_high_score {
         let diff_score = high_score - last_high_score;
         let diff_marble = marble - last_max_marble;

         println!("{} {}", diff_marble, diff_score);

         last_high_score = high_score;
         last_max_marble = marble;
      }
   }

   //println!("{}", play_game(players, max_marble * 100));
}

fn play_game(players: usize, max_marble: usize) -> usize {
   let mut scores = vec![0; players + 1];
   // play the first 2 plays for simplicity
   let mut current_player = 2;
   let mut current_index = 1;
   let mut circle = vec![0, 1];
   for marble in 2..=max_marble {
//      if (marble % (max_marble / 100)) == 0 {
//         println!("progress: {}", marble);
//      }

      if (marble % 23) == 0 {
         // special case
         scores[current_player] += marble;

         if current_index < 7 {
            current_index = circle.len() - (7 - current_index);
         } else {
            current_index -= 7;
         }

         let removed_marble = circle.remove(current_index);

         scores[current_player] += removed_marble;
      } else {
         // normal play
         current_index += 2;
         if current_index > circle.len() {
            // loop around and insert at the beginning
            current_index = 1;
         }

         circle.insert(current_index, marble);
      }

      current_player += 1;
      if current_player > players {
         current_player = 1;
      }
   }

//   let mut high_scores: Vec<usize> = scores.iter()
//      .filter_map(|v| {
//         if *v > 0 {
//            Some(*v)
//         }
//         else {
//            None
//         }
//      })
//      .collect();
//
//   high_scores.sort();
//
//   println!("{:?}", high_scores);

   return scores.iter()
      .fold(0, |max, score| {
         if *score > max {
            *score
         } else {
            max
         }
      });
}