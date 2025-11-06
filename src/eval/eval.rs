use std::str;
use num::complex::{
	Complex,
	Complex64,
};
use crate::{
	tokenizer::Token,
	parser::Parser,
	nodes::{
		Node,
		EvalNode,
	},
};

pub trait Printable {
	fn to_string(&self) -> String;
	fn to_typed_string(&self) -> String;
}

#[derive(PartialEq, Clone)]
pub enum Value {
	AplFloat(f64),
	AplInteger(isize),
	AplComplex(Complex64),
	AplArray(usize, Vec<usize>, Vec<Box<Value>>)
}

impl Printable for Value {
	fn to_string(&self) -> String {
		match *self {
			Value::AplFloat(f) => {
				format!("{}", f)
			},
			Value::AplInteger(i) => {
				format!("{}", i)
			},
			Value::AplArray(depth, ref _dimensions, ref contents) => {
				if depth != 1 {
					panic!("Multidimensional arrays aren't yet supported");
				}
				let segments: Vec<String> = contents.iter().map(|item| item.to_string()).collect();

				segments.join(" ")
			},
			Value::AplComplex(j) => {
				format!("{}J{}", j.re, j.im)
			}
		}
	}

	fn to_typed_string(&self) -> String {
		match *self {
			Value::AplFloat(_) => {
				format!("FLOAT({})", self.to_string())
			},
			Value::AplInteger(_) => {
				format!("INTEGER({})", self.to_string())
			},
			Value::AplArray(_, _, _) => {
				format!("ARRAY({})", self.to_string())
			},
			Value::AplComplex(_) => {
				format!("COMPLEX({})", self.to_string())
			}
		}
	}
}

pub fn eval_node(node: &Node) -> Result<Box<Value>,String> {
	match node {
		Node::Array(nodes) => Ok(eval_array(nodes)),
		_ => node.eval()
	}
}

fn eval_array(tokens: &[Box<Token>]) -> Box<Value> {
	if tokens.len() == 1 {
		match &tokens[0].as_ref() {
			&Token::Number(token_data) => {
				eval_number(&token_data.string)
			},
			_ => {
				panic!("Unsupported type in array")
			}
		}
	} else {
		let mut array_contents: Vec<Box<Value>> = vec![];
		for token in tokens.iter() {
			match token.as_ref() {
				Token::Number(token_data) => {
					array_contents.push(eval_number(&token_data.string))
				},
				_ => {
					panic!("Unsupported type in array")
				}
			}
		}
		Box::new(Value::AplArray(1, vec![array_contents.len()], array_contents))
	}
}

fn eval_number(token_string: &str) -> Box<Value> {
	match token_string.find('J') {
		//FIXME: This needs to handle exponents
		Some(pos) => {
			eval_complex(&token_string[..pos], &token_string[..pos + 1])
		},
		None => {
			match token_string.find('.') {
				Some(_) => {
					eval_float(token_string)
				},
				None => {
					eval_int(token_string)
				}
			}
		}
	}
}

fn get_string_and_sign(token_string: &str) -> (&str, bool){
	if let Some(rest) = token_string.strip_prefix('¯') {
		(rest, true)
	} else {
		(token_string, false)
	}
}

fn eval_complex(left: &str, right: &str) -> Box<Value> {
	let (left_match_string, left_is_negative) = get_string_and_sign(left);
	let (right_match_string, right_is_negative) = get_string_and_sign(right);

	match (left_match_string.parse::<f64>().ok(), right_match_string.parse::<f64>().ok()) {
		(Some(left_float), Some(right_float)) => {
			let left_final = if left_is_negative {
				-left_float
			} else {
				left_float
			};
			let right_final = if right_is_negative {
				-right_float
			} else {
				right_float
			};
			Box::new(Value::AplComplex(Complex::new(left_final, right_final)))
		},
		_ => {
			panic!("Bad complex {} {}", left, right)
		}
	}
}

fn eval_float(token_string: &str) -> Box<Value> {
	let (match_string, is_negative) = get_string_and_sign(token_string);

	match match_string.parse::<f64>().ok() {
		Some(fl) => {
			if is_negative {
				Box::new(Value::AplFloat(-fl))
			} else {
				Box::new(Value::AplFloat(fl))
			}
		},
		None => {
			panic!("Bad float {}", token_string)
		}
	}
}

fn eval_int(token_string: &str) -> Box<Value> {
	let (match_string, is_negative) = get_string_and_sign(token_string);

	match match_string.parse::<isize>().ok() {
		Some(i) => {
			if is_negative {
				Box::new(Value::AplInteger(-i))
			} else {
				Box::new(Value::AplInteger(i))
			}
		},
		None => {
			panic!("Bad int {}", token_string)
		}
	}
}

pub fn eval_dyadic<F>(func: F, left: &Node, right: &Node) -> Result<Box<Value>, String> where F: Fn(&Value, &Value) -> Result<Box<Value>, String> {
	match eval_node(left) {
		Ok(left) => {
			match eval_node(right) {
				Ok(right) => {
					func(&left, &right)
				},
				Err(msg) => {
					Err(msg)
				}
			}
		},
		Err(msg) => {
			Err(msg)
		}
	}
}

pub fn eval_monadic<F>(func: F, left: &Node) -> Result<Box<Value>, String> where F: Fn(&Value) -> Result<Box<Value>, String> {
	eval_node(left).and_then(|result| {
		func(&result)
	})
}

pub struct Evaluator {
	parser: Box<Parser>
}

impl Evaluator {

	pub fn new(input_string: String) -> Evaluator {
		Evaluator {
			parser: Box::new(Parser::new(input_string))
		}
	}

	pub fn eval(&mut self) -> Result<Box<Value>, String> {
		let tree = self.parser.parse_next_statement(); //TODO: Should loop?
		match tree {
			Ok(node) => {
				eval_node(&node)
			},
			Err(msg) => {
				Err(msg)
			}
		}
	}
}
