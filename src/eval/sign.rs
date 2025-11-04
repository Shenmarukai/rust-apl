use crate::{
	nodes::Node,
	eval::{
		eval::{
			Value,
			eval_monadic,
		},
		array_helpers::simple_monadic_array,
		divide::divide,
		magnitude::magnitude,
	},
};

pub fn sign(first: &Value) -> Result<Box<Value>, String> {
	match first {
		&Value::AplFloat(val) => {
			Ok(if val < 0.0 {
				Box::new(Value::AplInteger(-1))
			} else if val > 0.0 {
				Box::new(Value::AplInteger(1))
			} else {
				Box::new(Value::AplInteger(0))
			})
		},
		&Value::AplInteger(val) => {
			Ok(if val < 0 {
				Box::new(Value::AplInteger(-1))
			} else if val > 0 {
				Box::new(Value::AplInteger(1))
			} else {
				Box::new(Value::AplInteger(0))
			})
		},
		&Value::AplComplex(_c) => {
			magnitude(first).and_then(|magnituded| {
				divide(first, &magnituded)
			})
		},
		Value::AplArray(_depth, _dimensions, _values) => {
			simple_monadic_array(sign, first)
		}
	}
}

pub fn eval_sign(left: &Node) -> Result<Box<Value>, String> {
	eval_monadic(sign, left)
}
