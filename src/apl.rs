use std::{
    env,
    fs::File,
    io::{
		self,
		BufRead,
		BufReader,
		stdin,
	},
};
use rust_apl::eval::eval::{
	Evaluator,
	Printable,
};

fn main() -> io::Result<()> {
	println!("Rust-APL version 0.0.1");
	let args: Vec<String> = env::args().collect();

	// Decide where to read from: file if an argument is provided, otherwise stdin.
	if let Some(path) = args.get(1) {
		let file = File::open(path)?;
		run(BufReader::new(file))
	} else {
		let stdin = stdin();
		run(stdin.lock())
	}
}

fn run<R: BufRead>(mut reader: R) -> io::Result<()> {
	let mut line = String::new();

	loop {
		line.clear();
		match reader.read_line(&mut line) {
			Ok(0) => break, // EOF
			Ok(_) => {
				let mut eval = Evaluator::new(line.clone());
				match eval.eval() {
					Ok(result) => println!("{}", result.to_string()),
					Err(msg) => eprintln!("Error: {}", msg),
				}
			}
			Err(err) => {
				eprintln!("Error: {}", err);
				break;
			}
		}
	}

	Ok(())
}
