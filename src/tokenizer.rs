#[derive(Clone, Debug)]
pub struct TokenData {
	pub string: String,
	#[allow(dead_code)]
	row: usize,
	#[allow(dead_code)]
	col: usize
}

#[derive(Clone, Debug)]
pub enum Token {
	Number(TokenData),
	Newline(TokenData),
	String(TokenData),
	Primitive(TokenData),
	Variable(TokenData),
	EndOfFile
}

struct Backtrack {
	initial_next: usize,
	initial_char: Option<char>,
	initial_row: usize,
	initial_col: usize
}

struct CharReader {
	source: String,
	next: usize,
	current_char: Option<char>,
	row: usize,
	col: usize
}

impl CharReader {
	pub fn new(input_string: String) -> CharReader {
		CharReader {
			source: input_string,
			next: 0,
			current_char: None,
			row: 0,
			col: 0
		}
	}

	fn read_and_stash_char(&mut self) {
		if self.next < self.source.len() {
			let ch = self.source[self.next..].chars().next().unwrap();
			let next = self.next + ch.len_utf8();
			self.next = next;
			self.row += 1;
			self.current_char = Some(ch);
		} else {
			self.current_char = None;
		}
	}

	fn wind_past_whitespace(&mut self) {
		while let Some(' ') = self.current_char {
			self.read_and_stash_char();
		}
	}

	fn wind_past_comments(&mut self) {
		if let Some('⍝') = self.current_char {
			while let Some(ch) = self.current_char {
				if ch == '\n' || ch == '\r' {
					break;
				}
				self.read_and_stash_char();
			}
		}
	}

	fn create_backtrack(&self) -> Backtrack {
		Backtrack {
			initial_next: self.next,
			initial_char: self.current_char,
			initial_row: self.row,
			initial_col: self.col
		}
	}

	fn backtrack(&mut self, backtrack: &Backtrack) {
		self.next = backtrack.initial_next;
		self.current_char = backtrack.initial_char;
		self.row = backtrack.initial_row;
		self.col = backtrack.initial_col;
	}
}

pub struct Tokenizer {
	char_reader: Box<CharReader>,
}

impl Tokenizer {
	pub fn new(input_string: String) -> Tokenizer {
		let mut char_reader = CharReader::new(input_string);
		char_reader.read_and_stash_char();
		Tokenizer {
			char_reader: Box::new(char_reader)
		}
	}

	pub fn read_next_token(&mut self) -> Result<Box<Token>, String> {
		self.char_reader.wind_past_whitespace();
		self.char_reader.wind_past_comments();
		match self.char_reader.current_char {
			Some(first_char) => {
				//FIXME: Make Tokenizer a trait, turn this into two lines of code!
				if is_valid_newline_start(first_char) {
					return newline_tokenizer(&mut self.char_reader)
				}
				if is_dot(first_char) {
					return dot_tokenizer(&mut self.char_reader)
				}
				if is_valid_number_start(first_char) {
					return number_tokenizer(&mut self.char_reader)
				}
				if is_valid_string_start(first_char) {
					return string_tokenizer(&mut self.char_reader)
				}
				if is_valid_primitive_start(first_char) {
					return primitive_tokenizer(&mut self.char_reader)
				}
				if is_valid_variable_start(first_char) {
					return variable_tokenizer(&mut self.char_reader)
				}
				Err(format!("No valid token found starting with {}", first_char))
			},
			None => {
				Ok(Box::new(Token::EndOfFile))
			}
		}
	}
}

fn is_valid_number_start(char: char) -> bool {
	//Needs to be either upper dash, period, or 0-9
	char.is_ascii_digit() || char == '.' || char == '¯'
}

fn is_period(char_reader: &CharReader) -> bool {
	matches!(char_reader.current_char, Some('.'))
}

fn is_number(char_reader: &CharReader) -> bool {
	match char_reader.current_char {
		Some(maybe_number) => {
			maybe_number.is_ascii_digit()
		},
		_ => false
	}
}

fn is_complex(char_reader: &CharReader) -> bool {
	matches!(char_reader.current_char, Some('J'))
}

fn is_negative(char_reader: &CharReader) -> bool {
	matches!(char_reader.current_char, Some('¯'))
}

fn number_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	let mut period_encountered = false;
	let mut complex_encountered = false;
	let mut first_character = true;
	let mut allowed_negative = false;
	let mut token: Vec<char> = vec![];

	loop {
		if first_character {
			first_character = false;
			if is_period(char_reader) {
				period_encountered = true;
			}
			token.push(char_reader.current_char.unwrap());
		} else if is_negative(char_reader) {
			if allowed_negative {
				allowed_negative = false;
				token.push(char_reader.current_char.unwrap());
			} else {
				return Err("Invalid number".to_string());
			}
		} else if is_complex(char_reader) {
			if complex_encountered {
				return Err("Invalid number".to_string());
			} else {
				complex_encountered = true;
				period_encountered = false;
				allowed_negative = true;
				token.push(char_reader.current_char.unwrap());
			}
		} else if is_period(char_reader) {
			allowed_negative = false;
			if period_encountered {
				return Err("Invalid number".to_string());
			} else {
				period_encountered = true;
				token.push(char_reader.current_char.unwrap());
			}
		} else if is_number(char_reader) {
			allowed_negative = false;
			token.push(char_reader.current_char.unwrap());
		} else {
			if token[token.len() - 1] == '.' ||
				token[token.len() - 1] == 'J' ||
				token[token.len() - 1] == '¯' {
				return Err("Invalid number".to_string());
			}
			return Ok(Box::new(Token::Number(TokenData {
				string: token.into_iter().collect(),
				row: 0,
				col: 0
			})));
		}
		char_reader.read_and_stash_char();
	}
}

fn is_valid_newline_start(char: char) -> bool {
	char == '\n' || char == '\r'
}

fn newline_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	match char_reader.current_char {
		Some('\r') => {
			char_reader.read_and_stash_char();
			match char_reader.current_char {
				Some('\n') => {
					char_reader.read_and_stash_char();
					Ok(Box::new(Token::Newline(TokenData {
						string: "\r\n".to_string(),
						row: 0,
						col: 0
					})))
				},
				_ => {
					Ok(Box::new(Token::Newline(TokenData {
						string: "\r".to_string(),
						row: 0,
						col: 0
					})))
				}
			}
		},
		_ => {
			char_reader.read_and_stash_char();
			Ok(Box::new(Token::Newline(TokenData {
				string: "\n".to_string(),
				row: 0,
				col: 0
			})))
		}
	}
}

fn is_valid_string_start(char: char) -> bool {
	char == '\'' || char == '"'
}

fn string_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	let mut token: Vec<char> = vec![];
	let opening_character = char_reader.current_char.unwrap();
	char_reader.read_and_stash_char();

	loop {
		match char_reader.current_char {
			Some(char) if opening_character == char => {
				//Lookahead
				let backtrack = char_reader.create_backtrack();
				char_reader.read_and_stash_char();
				match char_reader.current_char {
					Some(char) if opening_character == char => {
						//It's a quote - continue
						token.push(char);
					},
					_ => {
						char_reader.backtrack(&backtrack);
						return Ok(Box::new(Token::String(TokenData {
							string: token.into_iter().collect(),
							row: 0,
							col: 0
						})));
					}
				}
			},
			Some(char) => {
				token.push(char);
			},
			None => {
				return Err("Unexpected end of file".to_string());
			}
		};
		char_reader.read_and_stash_char();
	}
}

fn is_valid_variable_start(char: char) -> bool {
	char == '∆' || char == '⍙' || ('A'..='z').contains(&char)
}

fn variable_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	let mut token: Vec<char> = vec![];

	while let Some(ch) = char_reader.current_char {
		if !is_valid_variable_start(ch) {
			break;
		}
		token.push(ch);
		char_reader.read_and_stash_char();
	}

	Ok(Box::new(Token::Variable(TokenData {
		string: token.into_iter().collect(),
		row: 0,
		col: 0
	})))
}

fn is_dot(char: char) -> bool {
	char == '.'
}

fn dot_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	let backtrack = char_reader.create_backtrack();
	char_reader.read_and_stash_char();
	match char_reader.current_char {
		Some(char) if char.is_ascii_digit() => {
			char_reader.backtrack(&backtrack);
			number_tokenizer(char_reader)
		},
		_ => {
			Ok(Box::new(Token::Primitive(TokenData {
				string: ".".to_string(),
				row: 0,
				col: 0
			})))
		}
	}
}

fn is_valid_primitive_start(char: char) -> bool {
	vec!['+','−','×','÷','⌈','⌊','∣','|','⍳','?','⋆','*','⍟','○','!','⌹','<','≤','=','≥','>','≠','≡','≢','∊','⍷','∪','∩','~','∨','∧','⍱','⍲','⍴',',','⍪','⌽','⊖','⍉','↑','↓','⊂','⊃','⌷','⍋','⍒','⊤','⊥','⍺','⍕','⍎','⊣','⊢','▯','⍞','/','\\','⍀','⌿','∘','¨','[',']','⍬','⋄','∇','⍫','(',')','←', '{', '}', '⍵', '-'].contains(&char)
}

fn primitive_tokenizer(char_reader: &mut CharReader) -> Result<Box<Token>, String> {
	let opening_character = char_reader.current_char.unwrap();
	if opening_character == '∘' {
		let backtrack = char_reader.create_backtrack();
		char_reader.read_and_stash_char();
		match char_reader.current_char {
			Some('.') => {
				char_reader.read_and_stash_char();
				Ok(Box::new(Token::Primitive(TokenData {
					string: "∘.".to_string(),
					row: 0,
					col: 0
				})))
			},
			_ => {
				char_reader.backtrack(&backtrack);
				Ok(Box::new(Token::Primitive(TokenData {
					string: "∘".to_string(),
					row: 0,
					col: 0
				})))
			}
		}
	} else {
		char_reader.read_and_stash_char();
		Ok(Box::new(Token::Primitive(TokenData {
			string: opening_character.to_string(),
			row: 0,
			col: 0
		})))
	}
}
