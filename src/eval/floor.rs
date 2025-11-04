use num::complex::Complex;
use crate::{
	nodes::Node,
	eval::{
		eval::{
			Value,
			eval_monadic,
		},
		array_helpers::simple_monadic_array,
	},
};

pub fn floor(first: &Value) -> Result<Box<Value>, String> {
	match first {
		&Value::AplFloat(val) => {
			Ok(Box::new(Value::AplInteger(val.floor() as isize)))
		},
		&Value::AplInteger(val) => {
			Ok(Box::new(Value::AplInteger(val)))
		},
		&Value::AplComplex(c) => {
			Ok(Box::new(Value::AplComplex(Complex::new(c.re.floor(), c.im.floor()))))
		},
		Value::AplArray(_depth, _dimensions, _values) => {
			simple_monadic_array(floor, first)
		}
	}
}

pub fn eval_floor(left: &Node) -> Result<Box<Value>, String> {
	eval_monadic(floor, left)
}
