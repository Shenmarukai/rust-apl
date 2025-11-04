use num::complex::{Complex, Complex64};

use nodes;
use eval::eval::{Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array};

fn add_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplFloat(f + val)))
        },
        &Value::AplInteger(val) => {
            add_float(f, &Value::AplFloat(val as f64))
        },
        &Value::AplComplex(_val) => {
            add_complex(&Complex::new(f, 0.0), other)
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(add_float, f, other)
        }
    }
}

fn add_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_val) => {
            add_float(i as f64, other)
        },
        &Value::AplInteger(val) => {
            Ok(Box::new(Value::AplInteger(i + val)))
        },
        &Value::AplComplex(_val) => {
            add_complex(&Complex::new(i as f64, 0.0), other)
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(add_integer, i, other)
        }
    }
}

fn add_complex(c: &Complex64, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(f) => {
            add_complex(c, &Value::AplComplex(Complex::new(f, 0.0)))
        },
        &Value::AplInteger(i) => {
            add_complex(c, &Value::AplComplex(Complex::new(i as f64, 0.0)))
        },
        &Value::AplComplex(other_c) => {
            Ok(Box::new(Value::AplComplex(c + other_c)))
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(add_complex, c, other)
        }
    }
}

fn add_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            simple_dyadic_array(add_float, val, array)
        },
        &Value::AplInteger(val) => {
            simple_dyadic_array(add_integer, val, array)
        },
        &Value::AplComplex(val) => {
            simple_dyadic_array(add_complex, &val, array)
        },
        &Value::AplArray(_, _, _) => {
            dual_dyadic_array(add, array, other)
        }
    }
}

pub fn add(first: &Value, other: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            add_float(f, other)
        },
        &Value::AplInteger(i) => {
            add_integer(i, other)
        }
        &Value::AplComplex(ref c) => {
            add_complex(c, other)
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            add_array(first, other)
        }
    }
}

pub fn eval_addition(left: &nodes::Node, right: &nodes::Node) -> Result<Box<Value>, String> {
    eval_dyadic(add, left, right)
}
