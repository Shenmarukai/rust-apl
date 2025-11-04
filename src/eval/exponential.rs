use num::complex::Complex;
use crate::nodes;
use crate::eval::array_helpers::{simple_monadic_array};
use crate::eval::eval::{Value, eval_monadic};
use crate::math_constants::e;

pub fn exponential(first: &Value) -> Result<Box<Value>, String> {
    match first {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplFloat(val.exp())))
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplFloat((val as f64).exp())))
        },
        &Value::AplComplex(c) => {
            let powed = e.powf(c.re);
            let left = c.im.cos();
            let right = c.im.sin();
            let complex = Complex::new(left, right);
            let result = Complex::new(powed, 0.0) * complex;
            Ok(Box::new(Value::AplComplex(result)))
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(exponential, first)
        }
    }
}

pub fn eval_exponential(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(exponential, left)
}
