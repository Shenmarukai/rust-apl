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

pub fn negate(first: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            Ok(Box::new(Value::AplFloat(-f)))
        },
        &Value::AplInteger(i) => {
            Ok(Box::new(Value::AplInteger(-i)))
        }
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(-c)))
        },
        Value::AplArray(_depth, _dimensions, _values) => {
            simple_monadic_array(negate, first)
        }
    }
}

pub fn eval_negate(left: &Node) -> Result<Box<Value>, String> {
    eval_monadic(negate, left)
}
