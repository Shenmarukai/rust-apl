use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{Value, eval_monadic};
use eval::divide::divide;
use eval::magnitude::magnitude;

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
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(sign, first)
        }
    }
}

pub fn eval_sign(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(sign, left)
}
