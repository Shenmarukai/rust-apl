use num::complex::{
	Complex,
	Complex64,
};
use crate::{
	nodes::Node,
	eval::{
		eval::{
			Value,
			eval_dyadic,
		},
		array_helpers::{
			simple_dyadic_array,
			dual_dyadic_array,
			inverse_simple_dyadic_array,
		},
	},
};

fn multiply_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
	match *other {
		Value::AplFloat(val) => {
			Ok(Box::new(Value::AplFloat(f * val)))
		},
		Value::AplInteger(val) => {
			multiply_float(f, &Value::AplFloat(val as f64))
		},
		Value::AplComplex(_val) => {
			multiply_complex(&Complex::new(f, 0.0), other)
		},
		Value::AplArray(_, _, _) => {
			simple_dyadic_array(multiply_float, f, other)
		}
	}
}

fn multiply_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
	match *other {
		Value::AplFloat(_val) => {
			multiply_float(i as f64, other)
		},
		Value::AplInteger(val) => {
			Ok(Box::new(Value::AplInteger(i * val)))
		},
		Value::AplComplex(_val) => {
			multiply_complex(&Complex::new(i as f64, 0.0), other)
		},
		Value::AplArray(_, _, _) => {
			simple_dyadic_array(multiply_integer, i, other)
		}
	}
}

fn multiply_complex(c: &Complex64, other: &Value) -> Result<Box<Value>, String> {
	match *other {
		Value::AplFloat(f) => {
			multiply_complex(c, &Value::AplComplex(Complex::new(f, 0.0)))
		},
		Value::AplInteger(i) => {
			multiply_complex(c, &Value::AplComplex(Complex::new(i as f64, 0.0)))
		},
		Value::AplComplex(other_c) => {
			Ok(Box::new(Value::AplComplex(c * other_c)))
		},
		Value::AplArray(_, _, _) => {
			simple_dyadic_array(multiply_complex, c, other)
		}
	}
}

fn multiply_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
	match other {
		&Value::AplFloat(_) |  &Value::AplInteger(_) | &Value::AplComplex(_) => {
			inverse_simple_dyadic_array(multiply, array, other)
		},
		&Value::AplArray(_, _, _) => {
			dual_dyadic_array(multiply, array, other)
		}
	}
}

pub fn multiply(first: &Value, other: &Value) -> Result<Box<Value>, String> {
	match first{
		&Value::AplFloat(f) => {
			multiply_float(f, other)
		},
		&Value::AplInteger(i) => {
			multiply_integer(i, other)
		}
		Value::AplComplex(c) => {
			multiply_complex(c, other)
		},
		Value::AplArray(_depth, _dimensions, _values) => {
			multiply_array(first, other)
		}
	}
}

pub fn eval_multiplication(left: &Node, right: &Node) -> Result<Box<Value>, String> {
	eval_dyadic(multiply, left, right)
}
