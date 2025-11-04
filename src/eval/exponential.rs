use std::f64::consts::E;
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

pub fn exponential(first: &Value) -> Result<Box<Value>, String> {
	match first {
		&Value::AplFloat(val) => {
			Ok(Box::new(Value::AplFloat(val.exp())))
		},
		&Value::AplInteger(val) => {
			Ok(Box::new(Value::AplFloat((val as f64).exp())))
		},
		&Value::AplComplex(c) => {
			let powed = E.powf(c.re);
			let left = c.im.cos();
			let right = c.im.sin();
			let complex = Complex::new(left, right);
			let result = Complex::new(powed, 0.0) * complex;
			Ok(Box::new(Value::AplComplex(result)))
		},
		Value::AplArray(_depth, _dimensions, _values) => {
			simple_monadic_array(exponential, first)
		}
	}
}

pub fn eval_exponential(left: &Node) -> Result<Box<Value>, String> {
	eval_monadic(exponential, left)
}
