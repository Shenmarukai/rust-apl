use num::complex::Complex;
use crate::nodes;

use crate::eval::array_helpers::{simple_monadic_array};
use crate::eval::eval::{Value, eval_monadic};

pub fn ceiling(first: &Value) -> Result<Box<Value>, String> {
    match first {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplInteger(val.ceil() as isize)))
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(val)))
        },
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(Complex::new(c.re.ceil() as f64, c.im.ceil() as f64))))
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(ceiling, first)
        }
    }
}

pub fn eval_ceiling(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(ceiling, left)
}
