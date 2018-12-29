use regex::Regex;
use std::fmt;
use std::collections::BTreeSet;

use common::*;

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
   OpCode::Gtrr, OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr,
];

struct Instruction {
   op_code: OpCode,
   input: Vec<usize>,
}

fn main() {
   fn parse_input(contents: &String) -> Vec<Instruction> {
      let program_re: Regex = Regex::new(r"([a-z#]+) (\d+)(?: (\d+) (\d+))?").unwrap();

      contents.lines()
         .map(|row| {
            let captures = program_re.captures(row).unwrap();
            let mut iter = captures.iter().skip(1);

            let mut op_code = OpCode::Addr;
            let raw_op_code = iter.next().unwrap().unwrap().as_str();

            let input: Vec<usize> = iter
               .filter_map(|capture| {
                  capture.and_then(|m| { m.as_str().parse::<usize>().ok() })
               })
               .collect();

            for code in OP_CODES.iter() {
               if code.to_string() == raw_op_code {
                  op_code = code.clone();
                  break;
               }
            }

            Instruction {
               op_code,
               input,
            }
         })
         .collect()
   }


   run_input("21", "input.txt", &|contents| {
      let program = parse_input(contents);

      println!("Result A: {}", a(&program));
      println!("Result B: {}", b());
   });
}

fn a(program: &Vec<Instruction>) -> usize {
   run_program(vec![11474091, 0, 0, 0, 0, 0], program)
}

fn b() -> usize {
   // transpiled the input into Rust for speed

   let mut one: usize;
   let mut three: usize = 0;
   let mut four: usize;

   /*
      Strategy:

      Basically we're searching for a cycle in the output, once we find one. The result right before that
      should be our max
   */

   let mut seen = BTreeSet::new();
   let mut last = 0;
   loop {
      four = three | 0x10000;
      three = 2176960;

      'mid: loop {
         one = four & 0xFF;
         three += one;
         three = three & 0xFFFFFF;
         three = three * 65899;
         three = three & 0xFFFFFF;

         if four < 256 {
            break 'mid;
         }

         four = four / 256
      }

      if !seen.insert(three) {
         // return the value right before our cycle
         return last;
      }

      last = three;
   }
}

fn run_program(initial_registers: Vec<usize>, program: &Vec<Instruction>) -> usize {
   let ip_override = program.first().unwrap().input[0];
   let instructions = &program[1..];

   let mut registers = initial_registers;
   while registers[ip_override] < instructions.len() {
      let ip = registers[ip_override];

      if ip == 29 {
         println!("{}", registers[3]);
      }

      let instruction = &instructions[ip];
      registers = instruction.op_code.run(
         &instruction.input,
         &registers,
      );

      //println!("{:02}: {} {:?} at {:?}", ip, instruction.op_code, instruction.input, registers);

      registers[ip_override] += 1;
   }

   registers[ip_override] -= 1;

   return registers[0];
}

impl OpCode {
   fn run(&self, input: &Vec<usize>, registers: &Vec<usize>) -> Vec<usize> {
      let mut result = registers.clone();

      let value = |offset: usize| {
         input[offset]
      };


      let register = |offset: usize| {
         registers[value(offset)]
      };

      result[value(C)] = match self {
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

impl fmt::Display for OpCode {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", format!("{:?}", self).to_lowercase())
   }
}