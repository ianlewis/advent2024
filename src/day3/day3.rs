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

use std::io::{self, BufRead, BufReader, ErrorKind, Read};
use std::process::ExitCode;
use std::error;

pub struct Lexer<R: Read> {
    reader: BufReader<R>,
    col: usize,
}

#derive[(Debug)]
enum LexerError {
    // An unexpected EOF encountered by the Lexer.
    UnexpectedEof,

    // Wrap std:io errors.
    IOErr(ErrorKind),
}

impl<R: Read> Lexer<R> {
    pub fn new(r: R) -> Self {
        Lexer {
            reader: BufReader::new(r),
            col: 1,
        }
    }

    fn peek(&mut self, n: usize) -> Result<String> {
        let buf = std::str::from_utf8(self.reader.fill_buf().map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?
            .to_string();
        if buf.chars().count() < n {
            return Err(LexerError::UnexpectedEof);
        }
        Ok(buf[..n].to_string())
    }

    fn peek_equal(&mut self, s: &String) -> Result<bool, String> {
        let buf = std::str::from_utf8(self.reader.fill_buf().map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?
            .to_string();
        let s_len = s.chars().count();
        if buf.chars().count() < s_len {
            return Ok(false);
        }
        Ok(buf[..s_len].to_string() == *s)
    }

    fn next_char(&mut self) -> Result<Option<char>, String> {
        let mut buf = [0; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                self.col += 1;
                Ok(Some(c))
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None), // Handle EOF
            Err(e) => Err(e.to_string()),
        }
    }

    fn read_until(&mut self, tok: String) -> Result<bool, String> {
        while !self.peek_equal(&tok)? {
            let c = self.next_char()?;
            if c.is_none() {
                return Ok(false);
            }
        }

        self.reader.consume(tok.len());
        Ok(true)
    }

    fn read_tok(&mut self, tok: String) -> Result<bool, String> {
        if self.peek_equal(&tok)? {
            self.reader.consume(tok.len());
            return Ok(true);
        }
        Ok(false)
    }

    fn read_num(&mut self) -> Result<Option<i64>, String> {
        let mut buf = String::new();
        loop {
            if let Some(c) = self.next_char()? {
                if c.is_numeric() {
                    buf.push(c);
                } else {
                    break;
                }
            }
        }
        if buf.chars().count() == 0 {
            return Ok(None);
        }
        let n = buf.parse::<i64>().map_err(|err| err.to_string())?;
        Ok(Some(n))
    }

    // fn pop_buf(&mut self, n: usize) {
    //     let c = self.buffer.chars();
    //     let c_cnt = c.count();
    //     if n > c_cnt {
    //         return;
    //     }
    //     let n_rm = c_cnt - n;
    //     self.buffer = self.buffer[n_rm..].to_string()
    // }
}

fn run(r: impl BufRead) -> Result<i64, String> {
    let mut lex = Lexer::new(r);
    while lex.read_until("mul".to_string())? {
        if !lex.read_tok("(".to_string())? {
            continue;
        }
        if let Some(n) = lex.read_num()? {
            println!("{}", n);
        } else {
            continue;
        }
        if !lex.read_tok(",".to_string())? {
            println!("no comma");
            continue;
        }
        if let Some(n) = lex.read_num()? {
            println!("{}", n);
        } else {
            continue;
        }
        if !lex.read_tok(")".to_string())? {
            println!("no )");
            continue;
        }
        // TODO: multiply numbers.
    }

    Ok(0)
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    let result = match run(stdin.lock()) {
        Ok(d) => d,
        Err(e) => {
            println!("error running: {e:?}");
            return ExitCode::from(1);
        }
    };

    println!("{}", result);

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), String> {
        let input =
            Bytes::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = run(input.reader())?;
        assert_eq!(result, 161);
        Ok(())
    }

    #[test]
    fn test_lexer_read_tok() -> Result<(), String> {
        let input = Bytes::from(",");
        let mut lex = Lexer::new(input.reader());
        let found = lex.read_tok(",".to_string()).unwrap();

        assert_eq!(found, true);
        Ok(())
    }
}
