use crate::nodes;
use crate::eval::eval::{Value, eval_dyadic};
use crate::eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn minimum_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplFloat(if f < val { f } else { val })))
        },
        &Value::AplInteger(val) => {
            Ok(if f < val as f64 { Box::new(Value::AplFloat(f)) } else { Box::new(Value::AplInteger(val)) })
        },
        &Value::AplComplex(_c) => {
            Err("minimum is not supported on complex numbers".to_string())
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(minimum_float, f, other)
        }
    }
}

fn minimum_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            Ok(if (i as f64) < val { Box::new(Value::AplInteger(i)) } else { Box::new(Value::AplFloat(val)) })
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(if i < val { i } else { val })))
        },
        &Value::AplComplex(_c) => {
            Err("minimum is not supported on complex numbers".to_string())
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(minimum_integer, i, other)
        }
    }
}

fn minimum_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_) |  &Value::AplInteger(_) | &Value::AplComplex(_) => {
            inverse_simple_dyadic_array(minimum, array, other)
        },
        &Value::AplArray(_, _, _) => {
            dual_dyadic_array(minimum, array, other)
        }
    }
}

pub fn minimum(first: &Value, other: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            minimum_float(f, other)
        },
        &Value::AplInteger(i) => {
            minimum_integer(i, other)
        }
        &Value::AplComplex(_c) => {
            Err("minimum is not supported on complex numbers".to_string())
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            minimum_array(first, other)
        }
    }
}

pub fn eval_minimum(left: &nodes::Node, right: &nodes::Node) -> Result<Box<Value>, String> {
    eval_dyadic(minimum, left, right)
}
