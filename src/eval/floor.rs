use num::complex::Complex;
use crate::nodes;

use crate::eval::array_helpers::{simple_monadic_array};
use crate::eval::eval::{Value, eval_monadic};

pub fn floor(first: &Value) -> Result<Box<Value>, String> {
    match first {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplInteger(val.floor() as isize)))
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(val)))
        },
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(Complex::new(c.re.floor() as f64, c.im.floor() as f64))))
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(floor, first)
        }
    }
}

pub fn eval_floor(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(floor, left)
}
