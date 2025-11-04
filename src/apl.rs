use eval::eval::Evaluator;
use eval::eval::Printable;
use std::io::{stdin, BufRead};

fn main() {
   println!("Rust-APL version 0.0.1");
   let mut reader = stdin().lock();
   loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let mut eval = Evaluator::new(line);
                match eval.eval() {
                    Ok(result) => {
                        println!("{}", result.to_string());
                    },
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            },
            Err(msg) => {
                println!("Error: {}", msg);
            }
        }
   }
}
