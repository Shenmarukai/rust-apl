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

pub fn ceiling(first: &Value) -> Result<Box<Value>, String> {
    match first {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplInteger(val.ceil() as isize)))
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(val)))
        },
        &Value::AplComplex(c) => {
            Ok(Box::new(Value::AplComplex(Complex::new(c.re.ceil(), c.im.ceil()))))
        },
        Value::AplArray(_depth, _dimensions, _values) => {
            simple_monadic_array(ceiling, first)
        }
    }
}

pub fn eval_ceiling(left: &Node) -> Result<Box<Value>, String> {
    eval_monadic(ceiling, left)
}
