// Copyright 2025 Ian Lewis
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections;
use std::error;
use std::io::{self, BufRead};
use std::process;

// Program is a list of the opcodes and their operands.
struct Program {
    instructions: Vec<usize>,
}

// Interpreter is a program interpreter.
#[derive(Clone)]
struct Interpreter {
    // pc is the instruction pointer.
    pc: usize,

    // a is the value of the a register
    a: i64,
    // a is the value of the a register
    b: i64,
    // a is the value of the a register
    c: i64,
}

impl Interpreter {
    // exec executes the given program and returns all outputs of the program.
    fn exec(&mut self, p: &Program) -> Result<Vec<usize>, Box<dyn error::Error>> {
        let mut out = Vec::new();

        while self.pc < p.instructions.len() {
            match p.instructions[self.pc] {
                // adv
                0 => self.a /= 2_i64.pow(self._read_combo(p)?.try_into()?),
                // bxl
                1 => self.b ^= self._read_literal(p)?,
                // bst
                2 => self.b = self._read_combo(p)? % 8,
                // jnz
                3 => {
                    if self.a != 0 {
                        self.pc = self._read_literal(p)?.try_into()?;
                        continue;
                    }
                }
                // bxc
                4 => self.b ^= self.c,
                // out
                5 => {
                    let op: usize = self._read_combo(p)?.try_into()?;
                    out.push(op % 8);
                }
                // bdv
                6 => self.b = self.a / 2_i64.pow(self._read_combo(p)?.try_into()?),
                // cdv
                7 => self.c = self.a / 2_i64.pow(self._read_combo(p)?.try_into()?),
                unknown_opcode => {
                    return Err(format!("Invalid Opcode: {}", unknown_opcode).into());
                }
            }

            self.pc += 2;
        }

        Ok(out)
    }

    fn _read_literal(&self, p: &Program) -> Result<i64, Box<dyn error::Error>> {
        Ok(i64::try_from(p.instructions[self.pc + 1])?)
    }

    fn _read_combo(&self, p: &Program) -> Result<i64, Box<dyn error::Error>> {
        let op = self._read_literal(p)?;
        match op {
            0..=3 => Ok(op),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            invalid_operand => Err(format!("Invalid operand: {}", invalid_operand).into()),
        }
    }
}

fn read_input(r: impl BufRead) -> Result<(Interpreter, Program), Box<dyn error::Error>> {
    let mut p = Program {
        instructions: vec![],
    };
    let mut i = Interpreter {
        pc: 0,
        a: 0,
        b: 0,
        c: 0,
    };
    let lines: Vec<String> = r.lines().collect::<Result<Vec<_>, _>>()?;

    let reg_a: Vec<_> = lines[0].split(":").collect();
    i.a = reg_a[1].trim().parse::<i64>()?;
    let reg_b: Vec<_> = lines[1].split(":").collect();
    i.b = reg_b[1].trim().parse::<i64>()?;
    let reg_c: Vec<_> = lines[2].split(":").collect();
    i.c = reg_c[1].trim().parse::<i64>()?;

    let p_line: Vec<_> = lines[4].split(":").collect();
    for num_str in p_line[1].trim().split(",") {
        p.instructions.push(num_str.parse::<usize>()?);
    }

    Ok((i, p))
}

fn find_a_reg(m: Interpreter, p: &Program) -> Result<Option<i64>, Box<dyn error::Error>> {
    //	find_a_reg relies on the structure of the specific input program given
    // which has the structure:
    //
    // WHILE A != 0 {
    //     B = ...
    //     A /= 8
    //     print(B)
    // }
    //
    // Since A is managed in octal values one instruction is output per octal
    // bit in A. We loop through through the possible values of A and process the
    // smaller values first. At each bit we check if it matches the output of the
    // program starting from the end. If we find a match we go to the next bit.
    //
    // Some bits are not 1:1 with output instructions so we need to keep a
    //stack so we can go back and check values if the current one doesn't result in a
    // match.

    let mut stack: collections::VecDeque<(i64, i64, Vec<usize>)> = collections::VecDeque::new();

    for i in (0..8).rev() {
        stack.push_back((i, 0, vec![]));
    }

    while !stack.is_empty() {
        let (n, a, instr) = stack.pop_back().unwrap();

        let mut mc = m.clone();
        let a_mul = a * 8 + n;
        mc.a = a_mul;
        let out = mc.exec(p)?;
        if out[0] == p.instructions[p.instructions.len() - instr.len() - 1] {
            // Debugging.
            // println!(
            //     "{:o} {} {}",
            //     a_mul,
            //     out.iter()
            //         .map(|o| o.to_string())
            //         .collect::<Vec<_>>()
            //         .join(","),
            //     p.instructions
            //         .iter()
            //         .map(|o| o.to_string())
            //         .collect::<Vec<_>>()
            //         .join(",")
            // );
            if p.instructions.len() == out.len() {
                return Ok(Some(a_mul));
            }
            for next_n in (0..8).rev() {
                stack.push_back((next_n, a_mul, out.clone()));
            }
        }
    }

    Ok(None)
}

fn run(r: impl BufRead) -> Result<(String, i64), Box<dyn error::Error>> {
    let (mut i, p) = read_input(r)?;
    let i2 = i.clone();

    let out_str = i
        .exec(&p)?
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Ok((out_str, find_a_reg(i2, &p)?.unwrap_or(-1)))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock()) {
        Ok(n) => n,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", n);
    println!("{}", n2);

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(n2, 58);
        Ok(())
    }

    #[test]
    fn test_copy() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, "5,7,3,0");
        assert_eq!(n2, 117440);
        Ok(())
    }
}
