use num::complex::{Complex, Complex64};
use nodes;
use eval::eval::{Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn divide_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(0.0) => {
            Err("Domain error - division by zero".to_string())
        },
        &Value::AplFloat(val) => {
            Ok(Box::new(Value::AplFloat(f / val)))
        },
        &Value::AplInteger(val) => {
            divide_float(f, &Value::AplFloat(val as f64))
        },
        &Value::AplComplex(_val) => {
            divide_complex(&Complex::new(f, 0.0), other)
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(divide_float, f, other)
        }
    }
}

pub fn divide_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_val) => {
            divide_float(i as f64, other)
        },
        &Value::AplInteger(0) => {
            Err("Domain error - division by zero".to_string())
        },
        &Value::AplInteger(val) => {
            let remainder = i % val;
            if remainder != 0 {
                divide_float(i as f64, &Value::AplFloat(val as f64))
            } else {
                Ok(Box::new(Value::AplInteger(i / val)))
            }
        },
        &Value::AplComplex(_val) => {
            divide_complex(&Complex::new(i as f64, 0.0), other)
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(divide_integer, i, other)
        }
    }
}

fn divide_complex(c: &Complex64, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(f) => {
            divide_complex(c, &Value::AplComplex(Complex::new(f, 0.0)))
        },
        &Value::AplInteger(i) => {
            divide_complex(c, &Value::AplComplex(Complex::new(i as f64, 0.0)))
        },
        &Value::AplComplex(other_c) => {
            Ok(Box::new(Value::AplComplex(c / other_c))) //FIXME: Doesn't catch divide by zero
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(divide_complex, c, other)
        }
    }
}

fn divide_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_) |  &Value::AplInteger(_) | &Value::AplComplex(_) => {
            inverse_simple_dyadic_array(divide, array, other)
        },
        &Value::AplArray(_, _, _) => {
            dual_dyadic_array(divide, array, other)
        }
    }
}

pub fn divide(first: &Value, other: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            divide_float(f, other)
        },
        &Value::AplInteger(i) => {
            divide_integer(i, other)
        }
        &Value::AplComplex(ref c) => {
            divide_complex(c, other)
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            divide_array(first, other)
        }
    }
}

pub fn eval_division(left: &nodes::Node, right: &nodes::Node) -> Result<Box<Value>, String> {
    eval_dyadic(divide, left, right)
}
