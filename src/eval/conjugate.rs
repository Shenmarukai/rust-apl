use crate::nodes;
use crate::eval::array_helpers::{simple_monadic_array};
use crate::eval::eval::{Value, eval_monadic};

pub fn conjugate(first: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(_) | &Value::AplInteger(_) => {
            Ok(Box::new(first.clone()))
        },
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(c.conj())))
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(conjugate, first)
        }
    }
}

pub fn eval_conjugate(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(conjugate, left)
}
