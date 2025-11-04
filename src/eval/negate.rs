use nodes;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{Value, eval_monadic};

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
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(negate, first)
        }
    }
}

pub fn eval_negate(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(negate, left)
}
