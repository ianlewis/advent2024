// Copyright 2024 Ian Lewis
//
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

// https://adventofcode.com/2024/day/3
// --- Day 3: Mull It Over ---
//
// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock!
// You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the
// North Pole Toboggan Rental Shop. The Historians head out to take a look.
//
// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
//
// The computer appears to be trying to run a program, but its memory (your puzzle input) is
// corrupted. All of the instructions have been jumbled up!
//
// It seems like the goal of the program is just to multiply some numbers. It does that with
// instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46)
// multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.
//
// However, because the program's memory has been corrupted, there are also many invalid characters
// that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*,
// mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.
//
// For example, consider the following section of corrupted memory:
//
// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
//
// Only the four highlighted sections are real mul instructions. Adding up the result of each
// instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
//
// Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of
// the results of the multiplications?

// Program day3 reads the program on stdin and prints the sum of all mul() operations.

// use std::io::{self, BufRead, BufReader, ErrorKind, Read};
use std::error;
use std::io;
use std::io::{BufRead, Read};
use std::process;

pub struct Lexer<R: io::Read> {
    reader: io::BufReader<R>,
    col: usize,
}

impl<R: io::Read> Lexer<R> {
    pub fn new(r: R) -> Self {
        Lexer {
            reader: io::BufReader::new(r),
            col: 1,
        }
    }

    fn peek(&mut self, n: usize) -> Result<String, Box<dyn error::Error>> {
        // TODO: Do not call fill_buf every call to peek.
        //       The buffer returned from fill_buf should be fully consumed before calling fill_buf
        //       again because fill_buf will result in reads from the underlying reader.
        let buf = std::str::from_utf8(self.reader.fill_buf()?)?;
        if buf.chars().count() < n {
            return Ok(buf.to_string());
            // return Ok(String::new());
        }
        Ok(buf[..n].to_string())
    }

    fn peek_equal(&mut self, s: &String) -> Result<bool, Box<dyn error::Error>> {
        let buf = self.peek(s.chars().count())?;
        Ok(buf == *s)
    }

    fn next_char(&mut self) -> Result<Option<char>, Box<dyn error::Error>> {
        let mut buf = [0; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                self.col += 1;
                Ok(Some(c))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None), // Handle EOF
            Err(e) => Err(e.into()),
        }
    }

    fn read_until(&mut self, tok: String) -> Result<bool, Box<dyn error::Error>> {
        while !self.peek_equal(&tok)? {
            let c = self.next_char()?;
            if c.is_none() {
                return Ok(false);
            }
        }

        self.reader.consume(tok.len());
        Ok(true)
    }

    fn read_tok(&mut self, tok: String) -> Result<bool, Box<dyn error::Error>> {
        if self.peek_equal(&tok)? {
            self.reader.consume(tok.len());
            return Ok(true);
        }
        Ok(false)
    }

    fn read_num(&mut self) -> Result<i64, Box<dyn error::Error>> {
        let buf = self.peek(3)?;
        let mut digits = 0;
        for c in buf.chars() {
            if c.is_numeric() {
                digits += 1;
            } else {
                break;
            }
        }
        self.reader.consume(digits);
        Ok(buf[..digits].parse::<i64>()?)
    }
}

fn run(r: impl io::BufRead) -> Result<i64, Box<dyn error::Error>> {
    let mut total = 0;
    let mut lex = Lexer::new(r);
    while lex.read_until("mul".to_string())? {
        if !lex.read_tok("(".to_string())? {
            continue;
        }
        let left = lex.read_num()?;
        if !lex.read_tok(",".to_string())? {
            continue;
        }
        let right = lex.read_num()?;
        if !lex.read_tok(")".to_string())? {
            continue;
        }
        total += left * right;
    }

    Ok(total)
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let result = match run(stdin.lock()) {
        Ok(d) => d,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", result);

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input =
            Bytes::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = run(input.reader())?;
        assert_eq!(result, 161);
        Ok(())
    }

    #[test]
    fn test_lexer_read_tok() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(",");
        let mut lex = Lexer::new(input.reader());
        let found = lex.read_tok(",".to_string()).unwrap();

        assert_eq!(found, true);
        Ok(())
    }

    #[test]
    fn test_lexer_read_num() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("123");
        let mut lex = Lexer::new(input.reader());
        let n = lex.read_num().unwrap();

        assert_eq!(n, 123i64);
        Ok(())
    }
}
