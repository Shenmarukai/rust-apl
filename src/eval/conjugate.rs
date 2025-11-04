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

pub fn conjugate(first: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(_) | &Value::AplInteger(_) => {
            Ok(Box::new(first.clone()))
        },
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(c.conj())))
        },
        Value::AplArray(_depth, _dimensions, _values) => {
            simple_monadic_array(conjugate, first)
        }
    }
}

pub fn eval_conjugate(left: &Node) -> Result<Box<Value>, String> {
    eval_monadic(conjugate, left)
}
