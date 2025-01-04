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

// Program day3 reads the program on stdin and prints the sum of all mul() operations and the sum
// of all mul() operations when respecting do() and don't().

use std::error;
use std::io;
use std::io::{BufRead, Read};
use std::process;

pub struct Lexer<R: io::Read> {
    reader: io::BufReader<R>,
}

// TODO: Support utf-8 properly.
//       Current code uses String.len() which returns the number of bytes and not utf-8 characters.
//       Getting proper utf-8 substrings is also non-trivial in Rust it seems.

impl<R: io::Read> Lexer<R> {
    pub fn new(r: R) -> Self {
        Lexer {
            reader: io::BufReader::new(r),
        }
    }

    // peek reads n bytes from the current reader without advancing the reader's position.
    fn peek(&mut self, n: usize) -> Result<String, Box<dyn error::Error>> {
        // TODO: Do not call fill_buf every call to peek.
        //       The buffer returned from fill_buf should be fully consumed before calling fill_buf
        //       again because we convert it to a String on every call to peek.
        let buf = std::str::from_utf8(self.reader.fill_buf()?)?;
        if buf.len() < n {
            return Ok(buf.to_string());
        }
        Ok(buf[..n].to_string())
    }

    // read_until reads from the reader until it encounters one of the given tokens. If one is
    // found then it is returned. If the reader is fully read without encountering a token then
    // None is returned.
    fn read_until(&mut self, tokens: &[String]) -> Result<Option<String>, Box<dyn error::Error>> {
        // Get maximum length of given tokens.
        let length = tokens.iter().map(|tok| tok.len()).max().unwrap_or(0);
        loop {
            let buf = self.peek(length)?;
            if buf.is_empty() {
                // EOF
                return Ok(None);
            }

            let mut found_tok: Option<String> = None;
            for tok in tokens.iter() {
                if buf.len() >= tok.len() && buf[..tok.len()] == *tok {
                    found_tok = Some(tok.clone());
                    break;
                }
            }

            if let Some(tok) = &found_tok {
                self.reader.consume(tok.len());
                return Ok(found_tok);
            } else {
                self.reader.consume(1);
            }
        }
    }

    // read_tok reads an expected token from the reader. Returns whether the token was read or not.
    fn read_tok(&mut self, tok: String) -> Result<bool, Box<dyn error::Error>> {
        let buf = self.peek(tok.len())?;
        if buf == tok {
            self.reader.consume(buf.len());
            return Ok(true);
        }
        Ok(false)
    }

    // read_num reads an expected number (up to 3 digits) from the reader and returns it. If a
    // number was not present at the current location an error is returned.
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
        let n = buf[..digits].parse::<i64>()?;
        self.reader.consume(digits);
        Ok(n)
    }
}

fn run(r: impl io::BufRead) -> Result<i64, Box<dyn error::Error>> {
    let mut total = 0;
    let mut lex = Lexer::new(r);
    loop {
        let found_tok = lex.read_until(&["mul".to_string()])?;
        if found_tok.is_none() {
            break;
        }

        if !lex.read_tok("(".to_string())? {
            continue;
        }

        let left_result = lex.read_num();
        // TODO: check for parse error
        if left_result.is_err() {
            continue;
        }

        if !lex.read_tok(",".to_string())? {
            continue;
        }

        let right_result = lex.read_num();
        // TODO: check for parse error
        if right_result.is_err() {
            continue;
        }

        if !lex.read_tok(")".to_string())? {
            continue;
        }
        total += left_result.unwrap() * right_result.unwrap();
    }

    Ok(total)
}

fn run_do(r: impl io::BufRead) -> Result<i64, Box<dyn error::Error>> {
    let mut total = 0;
    let mut lex = Lexer::new(r);
    let mut enabled = true;
    loop {
        if !enabled {
            let found_tok = lex.read_until(&["do()".to_string()])?;
            if found_tok.is_none() {
                break;
            }
            enabled = true;
            continue;
        }

        let found_tok = lex.read_until(&["mul".to_string(), "don't()".to_string()])?;
        if found_tok.is_none() {
            break;
        }

        let tok = found_tok.unwrap();
        if tok == "don't()" {
            enabled = false;
            continue;
        }

        if !lex.read_tok("(".to_string())? {
            continue;
        }

        let left_result = lex.read_num();
        // TODO: check for parse error
        if left_result.is_err() {
            continue;
        }

        if !lex.read_tok(",".to_string())? {
            continue;
        }

        let right_result = lex.read_num();
        // TODO: check for parse error
        if right_result.is_err() {
            continue;
        }

        if !lex.read_tok(")".to_string())? {
            continue;
        }
        total += left_result.unwrap() * right_result.unwrap();
    }

    Ok(total)
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    if let Err(e) = stdin.lock().read_to_end(&mut buf) {
        println!("error running: {e:?}");
        return process::ExitCode::from(1);
    }

    let cur = io::Cursor::new(&buf);
    let result = match run(cur) {
        Ok(d) => d,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", result);

    let cur_do = io::Cursor::new(&buf);
    let result_do = match run_do(cur_do) {
        Ok(d) => d,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", result_do);

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
    fn test_run_do_nothing() -> Result<(), Box<dyn error::Error>> {
        let inputs: [&str; 4] = ["mul(4*", "mul(6,9!", "?(12,34)", "mul ( 2 , 4 )"];

        for input in inputs {
            let b = Bytes::from(input);
            let result = run(b.reader())?;
            assert_eq!(result, 0);
        }

        Ok(())
    }

    #[test]
    fn test_run_mul_paren_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_run_mul_paren_num_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_run_mul_paren_num_comma_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2,mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_run_mul_paren_num_comma_num_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2,12mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_run_do() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let result = run_do(input.reader())?;
        assert_eq!(result, 48);
        Ok(())
    }

    #[test]
    fn test_lexer_read_until() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("foobar");
        let mut lex = Lexer::new(input.reader());
        let found = lex.read_until(&["bar".to_string()]).unwrap();

        assert_eq!(found, Some("bar".to_string()));
        Ok(())
    }

    #[test]
    fn test_lexer_read_tok() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(",");
        let mut lex = Lexer::new(input.reader());
        let found = lex.read_tok(",".to_string()).unwrap();

        assert!(found);
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
