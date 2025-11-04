use crate::nodes;

use crate::eval::array_helpers::{simple_monadic_array};
use crate::eval::eval::{Value, eval_monadic};

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
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(magnitude, first)
        }
    }
}

pub fn eval_magnitude(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(magnitude, left)
}
