use common::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum OpCode {
   Addr,
   Addi,
   Mulr,
   Muli,
   Banr,
   Bani,
   Borr,
   Bori,
   Setr,
   Seti,
   Gtir,
   Gtri,
   Gtrr,
   Eqir,
   Eqri,
   Eqrr,
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

const OP_CODES: [OpCode; 16] = [
   OpCode::Addr, OpCode::Addi, OpCode::Mulr, OpCode::Muli,
   OpCode::Banr, OpCode::Bani, OpCode::Borr, OpCode::Bori,
   OpCode::Setr, OpCode::Seti, OpCode::Gtir, OpCode::Gtri,
   OpCode::Gtrr, OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr
];

struct ManualStep {
   before: Vec<usize>,
   instruction: Vec<usize>,
   after: Vec<usize>,
}

fn main() {
   let before_after_re: Regex = Regex::new(r"Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]\s+(\d+) (\d+) (\d+) (\d+)\s+After:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
   let program_re: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

   run_input("16", "input.txt", &|contents| {
      let mut last_match_i = 0;
      let manual: Vec<ManualStep> = before_after_re.captures_iter(contents)
         .map(|captures| {
            let mut inner_iter = captures.iter();
            last_match_i = inner_iter.next().unwrap().unwrap().end();

            let results: Vec<usize> = inner_iter
               .map(|capture| {
                  capture.and_then(|m| m.as_str().parse::<usize>().ok()).unwrap()
               })
               .collect();

            ManualStep {
               before: results[0..4].to_vec(),
               instruction: results[4..8].to_vec(),
               after: results[8..12].to_vec(),
            }
         })
         .collect();

      let program_section: String = contents.chars().skip(last_match_i).collect();
      let program: Vec<Vec<usize>> = program_section.trim().lines()
         .map(|row| {
            let parsed_row: Vec<usize> = program_re.captures(row).unwrap().iter().skip(1)
               .map(|capture| {
                  capture.and_then(|m| m.as_str().parse::<usize>().ok()).unwrap()
               })
               .collect();

            parsed_row
         })
         .collect();

      a(&manual);
      b(&manual, &program);
   });
}

fn a(manual: &Vec<ManualStep>) {
   let mut three_or_more_samples = 0;

   for step in manual {
      let mut valid_count = 0;
      for op_code in OP_CODES.iter() {
         let result = op_code.run(&step.instruction[1..], &step.before);

         if result == step.after {
            valid_count += 1;
         }
      }

      if valid_count >= 3 {
         three_or_more_samples += 1;
      }
   }

   println!("Result A: {}", three_or_more_samples);
}

fn b(manual: &Vec<ManualStep>, program: &Vec<Vec<usize>>) {
   let op_mapping = derive_op_mapping(manual);

   let mut registers = vec![0, 0, 0, 0];
   for instruction in program.iter() {
      let op_code = op_mapping.get(&instruction[0]).unwrap();

      registers = op_code.run(&instruction[1..], &registers);
   }

   println!("Result B: {} at {:?}", registers[0], registers);
}

fn derive_op_mapping(manual: &Vec<ManualStep>) -> HashMap<usize, OpCode> {
   let mut potential_ops: Vec<OpCode> = OP_CODES.to_vec();
   let mut op_mapping: HashMap<usize, OpCode> = HashMap::new();

   while potential_ops.len() > 0 {
      for step in manual {
         let op_id = step.instruction[0];
         let mut valid_count = 0;

         // we don't know the answer for this step so see what is possible
         if !op_mapping.contains_key(&op_id) {
            let mut valid_op = OpCode::Addr;
            for op_code in potential_ops.iter() {
               let result = op_code.run(&step.instruction[1..], &step.before);

               if result == step.after {
                  valid_count += 1;
                  valid_op = op_code.clone();
               }
            }

            // only one valid OpCode so mark it
            if valid_count == 1 {
               potential_ops.retain(|op| *op != valid_op);
               op_mapping.insert(op_id, valid_op);
            }
         }
      }
   }

   op_mapping
}

impl OpCode {
   fn run(&self, input: &[usize], registers: &Vec<usize>) -> Vec<usize> {
      let mut result = registers.clone();

      let value = |offset: usize| {
         input[offset]
      };


      let register = |offset: usize| {
         registers[value(offset)]
      };

      result[input[C]] = match self {
         OpCode::Addr => {
            register(A) + register(B)
         }
         OpCode::Addi => {
            register(A) + value(B)
         }
         OpCode::Mulr => {
            register(A) * register(B)
         }
         OpCode::Muli => {
            register(A) * value(B)
         }
         OpCode::Banr => {
            register(A) & register(B)
         }
         OpCode::Bani => {
            register(A) & value(B)
         }
         OpCode::Borr => {
            register(A) | register(B)
         }
         OpCode::Bori => {
            register(A) | value(B)
         }
         OpCode::Setr => {
            register(A)
         }
         OpCode::Seti => {
            value(A)
         }
         OpCode::Gtir => {
            if value(A) > register(B) { 1 } else { 0 }
         }
         OpCode::Gtri => {
            if register(A) > value(B) { 1 } else { 0 }
         }
         OpCode::Gtrr => {
            if register(A) > register(B) { 1 } else { 0 }
         }
         OpCode::Eqir => {
            if value(A) == register(B) { 1 } else { 0 }
         }
         OpCode::Eqri => {
            if register(A) == value(B) { 1 } else { 0 }
         }
         OpCode::Eqrr => {
            if register(A) == register(B) { 1 } else { 0 }
         }
      };

      result
   }
}