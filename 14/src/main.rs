use lib::*;

fn main() {
   assert_eq!("5158916779", a(9));
   assert_eq!("0124515891", a(5));
   assert_eq!("9251071085", a(18));
   assert_eq!("5941429882", a(2018));

   assert_eq!("9", b(51589));
   assert_eq!("18", b(92510));
   assert_eq!("2018", b(59414));

   run_input("14", "input.txt", &|contents| {
      let input = contents.parse::<usize>().unwrap();

      println!("Result A: {}", a(input));
      println!("Result B: {}", b(input));
   });
}

fn a(num_recipes: usize) -> String {
   let mut elfs = vec![
      0, 1
   ];
   let mut score_board: Vec<usize> = vec![3, 7];

   while score_board.len() < num_recipes + 10 {
      cook_recipes(&mut elfs, &mut score_board);
   }

   return slice_to_string(&score_board[num_recipes..num_recipes + 10]);
}

fn b(raw_search_num: usize) -> String {
   let mut elfs = vec![
      0, 1
   ];
   let mut score_board: Vec<usize> = vec![3, 7];
   let search_nums: String = slice_to_string(&split_num(raw_search_num));

   let found_index;
   'outer: loop {
      cook_recipes(&mut elfs, &mut score_board);

      // continue if our score_board isn't large enough
      if score_board.len() > search_nums.len() {
         let start = score_board.len() - search_nums.len() - 1;

         // search on our num + 1 for when 2 recipes are created at once
         let search_range = slice_to_string(
            &score_board[start..]
         );

         let found = search_range.find(&search_nums);

         match found {
            Some(i) => {
               found_index = start + i;
               break;
            }
            _ => {}
         }
      }
   }

   //print_score_board(&elfs, &score_board[score_board.len() - 100..].to_vec());

   return found_index.to_string();
}

fn cook_recipes(elfs: &mut Vec<usize>, score_board: &mut Vec<usize>) {
   // get recipes
   let recipes: Vec<(usize, usize)> = elfs.iter()
      .map(|elf| (*elf, score_board[*elf]))
      .collect();

   // combine recipes
   let combined = recipes.iter()
      .fold(0, |sum, recipe| sum + recipe.1);

   let mut split = split_num(combined);

   score_board.append(&mut split);

   // step forward
   for i in 0..elfs.len() {
      elfs[i] = (elfs[i] + recipes[i].1 + 1) % score_board.len();
   }
}

fn split_num(number: usize) -> Vec<usize> {
   // our loop can't handle 0 so just return the answer
   if number == 0 {
      return vec![0];
   }

   let mut result = Vec::new();
   let mut split_number = number;
   let mut last_power = 1;
   let mut power = 10;
   while split_number > 0 {
      let found = split_number % power;

      result.push(found / last_power);

      last_power = power;
      power *= 10;
      split_number -= found;
   }
   result.reverse();

   result
}

fn slice_to_string(nums: &[usize]) -> String {
   let strs: Vec<String> = nums.iter()
      .map(|score| score.to_string())
      .collect();

   strs.concat()
}

//fn print_score_board(elfs: &Vec<usize>, score_board: &Vec<usize>) {
//   for (i, score) in score_board.iter().enumerate() {
//      if elfs[0] == i {
//         print!("({})", score);
//      } else if elfs.iter().skip(1).find(|elf| **elf == i).is_some() {
//         print!("[{}]", score);
//      } else {
//         print!("{}", score);
//      }
//   }
//
//   println!();
//}