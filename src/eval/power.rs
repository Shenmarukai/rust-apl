use num::complex::{Complex, Complex64};

use crate::nodes;
use crate::eval::eval::{Value, eval_dyadic};
use crate::eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn power_float(f: f64, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            if f == 0.0 && val < 0.0 {
                Err("Cannot take 0 to a negative power".to_string()) //FIXME: Make this a constant
            } else {
                Ok(Box::new(Value::AplFloat(f.powf(val))))
            }
        },
        &Value::AplInteger(val) => {
            if f == 0.0 && val < 0 {
                Err("Cannot take 0 to a negative power".to_string())
            } else {
                Ok(Box::new(Value::AplFloat(f.powf(val as f64))))
            }
        },
        &Value::AplComplex(c) => {
            let fpow = f.powf(c.re);
            let im_times_lnf = c.im * f.ln();
            let real =  fpow * im_times_lnf.cos();
            let imaginary = fpow * im_times_lnf.sin();
            Ok(Box::new(Value::AplComplex(Complex::new(real, imaginary))))
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(power_float, f, other)
        }
    }
}

fn power_integer(i: isize, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            if i == 0 && val < 0.0 {
                Err("Cannot take 0 to a negative power".to_string())
            } else {
                Ok(Box::new(Value::AplFloat((i as f64).powf(val))))
            }
        },
        &Value::AplInteger(val) => {
            if i == 0 && val < 0 {
                Err("Cannot take 0 to a negative power".to_string())
            } else {
                Ok(Box::new(Value::AplInteger((i as f64).powf(val as f64) as isize)))
            }
        },
        &Value::AplComplex(_c) => {
            power_float(i as f64, other)
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(power_integer, i, other)
        }
    }
}

fn power_complex(c: &Complex64, other:&Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(val) => {
            power_complex(c, &Value::AplComplex(Complex::new(val, 0.0)))
        },
        &Value::AplInteger(val) => {
            power_complex(c, &Value::AplComplex(Complex::new(val as f64, 0.0)))
        },
        &Value::AplComplex(_c) => {
            Err("power is not supported on complex numbers".to_string())
        },
        &Value::AplArray(_, _, _) => {
            simple_dyadic_array(power_complex, c, other)
        }
    }
}

fn power_array(array: &Value, other: &Value) -> Result<Box<Value>, String> {
    match other {
        &Value::AplFloat(_) |  &Value::AplInteger(_) | &Value::AplComplex(_) => {
            inverse_simple_dyadic_array(power, array, other)
        },
        &Value::AplArray(_, _, _) => {
            dual_dyadic_array(power, array, other)
        }
    }
}

pub fn power(first: &Value, other: &Value) -> Result<Box<Value>, String> {
    match first{
        &Value::AplFloat(f) => {
            power_float(f, other)
        },
        &Value::AplInteger(i) => {
            power_integer(i, other)
        }
        &Value::AplComplex(ref c) => {
            power_complex(c, other)
        },
        &Value::AplArray(ref _depth, ref _dimensions, ref _values) => {
            power_array(first, other)
        }
    }
}

pub fn eval_power(left: &nodes::Node, right: &nodes::Node) -> Result<Box<Value>, String> {
    eval_dyadic(power, left, right)
}

