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

// Program day3 reads the program on stdin and prints the sum of all mul() operations.

use std::error;
use std::io;
use std::io::BufRead;
use std::process;

pub struct Lexer<R: io::Read> {
    reader: io::BufReader<R>,
}

impl<R: io::Read> Lexer<R> {
    pub fn new(r: R) -> Self {
        Lexer {
            reader: io::BufReader::new(r),
        }
    }

    fn peek(&mut self, n: usize) -> Result<String, Box<dyn error::Error>> {
        // TODO: Do not call fill_buf every call to peek.
        //       The buffer returned from fill_buf should be fully consumed before calling fill_buf
        //       again because fill_buf will result in reads from the underlying reader.
        let buf = std::str::from_utf8(self.reader.fill_buf()?)?;
        if buf.chars().count() < n {
            return Ok(buf.to_string());
        }
        Ok(buf[..n].to_string())
    }

    fn read_until(&mut self, tok: String) -> Result<bool, Box<dyn error::Error>> {
        loop {
            let buf = self.peek(tok.chars().count())?;
            if buf.len() == 0 {
                // EOF
                return Ok(false);
            }
            if buf == tok {
                self.reader.consume(buf.len());
                return Ok(true);
            } else {
                self.reader.consume(1);
            }
        }
    }

    fn read_tok(&mut self, tok: String) -> Result<bool, Box<dyn error::Error>> {
        let buf = self.peek(tok.chars().count())?;
        if buf == tok {
            self.reader.consume(buf.len());
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
        let n = buf[..digits].parse::<i64>()?;
        self.reader.consume(digits);
        Ok(n)
    }
}

fn run(r: impl io::BufRead) -> Result<i64, Box<dyn error::Error>> {
    let mut total = 0;
    let mut lex = Lexer::new(r);
    while lex.read_until("mul".to_string())? {
        if !lex.read_tok("(".to_string())? {
            continue;
        }

        let left_result = lex.read_num();
        // TODO: check for parse error
        if let Err(_) = left_result {
            continue;
        }

        if !lex.read_tok(",".to_string())? {
            continue;
        }

        let right_result = lex.read_num();
        // TODO: check for parse error
        if let Err(_) = right_result {
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
    fn test_mul_paren_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_mul_paren_num_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_mul_paren_num_comma_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2,mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_mul_paren_num_comma_num_mul() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("mul(2,12mul(2,4)");
        let result = run(input.reader())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_lexer_read_until() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("foobar");
        let mut lex = Lexer::new(input.reader());
        let found = lex.read_until("bar".to_string()).unwrap();

        assert_eq!(found, true);
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
