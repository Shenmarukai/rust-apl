use num::complex::{
    Complex,
    Complex64
};
use crate::{
    nodes::Node,
    eval::{
        eval::{
            Value,
            eval_dyadic,
        },
        array_helpers::{
            simple_dyadic_array,
            dual_dyadic_array,
            inverse_simple_dyadic_array,
        },
    },
};

fn subtract_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
    match *other {
        Value::AplFloat(val) => {
            Ok(Box::new(Value::AplFloat(f - val)))
        },
        Value::AplInteger(val) => {
            subtract_float(f, &Value::AplFloat(val as f64))
        },
        Value::AplComplex(_val) => {
            subtract_complex(&Complex::new(f, 0.0), other)
        },
        Value::AplArray(_, _, _) => {
            simple_dyadic_array(subtract_float, f, other)
        }
    }
}

fn subtract_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
    match *other {
        Value::AplFloat(_val) => {
            subtract_float(i as f64, other)
        },
        Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(i - val)))
        },
        Value::AplComplex(_val) => {
            subtract_complex(&Complex::new(i as f64, 0.0), other)
        },
        Value::AplArray(_, _, _) => {
            simple_dyadic_array(subtract_integer, i, other)
        }
    }
}

fn subtract_complex(c: &Complex64, other: &Value) -> Result<Box<Value>, String> {
    match *other {
        Value::AplFloat(f) => {
            subtract_complex(c, &Value::AplComplex(Complex::new(f, 0.0)))
        },
        Value::AplInteger(i) => {
            subtract_complex(c, &Value::AplComplex(Complex::new(i as f64, 0.0)))
        },
        Value::AplComplex(other_c) => {
            Ok(Box::new(Value::AplComplex(c - other_c)))
        },
        Value::AplArray(_, _, _) => {
            simple_dyadic_array(subtract_complex, c, other)
        }
    }
}

fn subtract_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_) |  &Value::AplInteger(_) | &Value::AplComplex(_) => {
            inverse_simple_dyadic_array(subtract, array, other)
        },
        &Value::AplArray(_, _, _) => {
            dual_dyadic_array(subtract, array, other)
        }
    }
}

pub fn subtract(first: &Value, other: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            subtract_float(f, other)
        },
        &Value::AplInteger(i) => {
            subtract_integer(i, other)
        }
        Value::AplComplex(c) => {
            subtract_complex(c, other)
        },
        Value::AplArray(_depth, _dimensions, _values) => {
            subtract_array(first, other)
        }
    }
}

pub fn eval_subtraction(left: &Node, right: &Node) -> Result<Box<Value>, String> {
    eval_dyadic(subtract, left, right)
}
