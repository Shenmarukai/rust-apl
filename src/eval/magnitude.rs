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

pub fn magnitude(first: &Value) -> Result<Box<Value>, String> {
	match first {
		&Value::AplFloat(val) => {
			Ok(Box::new(Value::AplFloat(val.abs())))
		},
		&Value::AplInteger(val) => {
			Ok(Box::new(Value::AplInteger(val.abs())))
		},
		&Value::AplComplex(c) => {
			let ii = c.re * c.re;
			let jj = c.im * c.im;
			let iijj = ii + jj;
			Ok(Box::new(Value::AplFloat(iijj.sqrt())))
		},
		Value::AplArray(_depth, _dimensions, _values) => {
			simple_monadic_array(magnitude, first)
		}
	}
}

pub fn eval_magnitude(left: &Node) -> Result<Box<Value>, String> {
	eval_monadic(magnitude, left)
}
