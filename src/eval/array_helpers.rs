use std::result;
use crate::eval::eval::Value;

pub fn simple_dyadic_array<T: Clone, F>(func: F, param: T, other: &Value) -> result::Result<Box<Value>, String> where F: Fn(T, &Value) -> result::Result<Box<Value>, String> {
    match other {
        Value::AplArray(depth, dimensions, values) => {
            let mut result_values: Vec<Box<Value>> = vec![];
            let mut error_state = "".to_string();
            let mut errored = false;

            for value in values.iter() {
                if !errored {
                    match func(param.clone(), value) {
                        result::Result::Ok(val) => {
                            result_values.push(val);
                        },
                        result::Result::Err(err) => {
                            errored = true;
                            error_state = err;
                        }
                    }
                }
            };

            if errored {
                result::Result::Err(error_state)
            } else {
                result::Result::Ok(Box::new(Value::AplArray(*depth, dimensions.clone(), result_values)))
            }
        },
        _ => {
            panic!("This should never be reached")
        }
    }
}

pub fn inverse_simple_dyadic_array<T: Clone, F>(func: F, param: &Value, other: T) -> result::Result<Box<Value>, String> where F: Fn(&Value, T) -> result::Result<Box<Value>, String> {
    match param {
        Value::AplArray(depth, dimensions, values) => {
            let mut result_values: Vec<Box<Value>> = vec![];
            let mut error_state = "".to_string();
            let mut errored = false;
            for value in values.iter() {
                if !errored {
                    match func(value, other.clone()) {
                        result::Result::Ok(val) => {
                            result_values.push(val);
                        },
                        result::Result::Err(err) => {
                            errored = true;
                            error_state = err;
                        }
                    }
                }
            }

            if errored {
                result::Result::Err(error_state)
            } else {
                result::Result::Ok(Box::new(Value::AplArray(*depth, dimensions.clone(), result_values)))
            }
        },
        _ => {
            panic!("This should never be reached")
        }
    }
}

pub fn dual_dyadic_array<F>(func: F, param: &Value, other: &Value) -> result::Result<Box<Value>, String> where F: Fn(&Value, &Value) -> result::Result<Box<Value>, String> {
    match param {
        Value::AplArray(left_depth, left_dimensions, left_values) => {
            match other {
                Value::AplArray(right_depth, right_dimensions, right_values) => {
                    //Different depths are considered a rank error
                    //Different shapes are considered a length error
                    if left_depth != right_depth {
                        return result::Result::Err("Rank error".to_string())
                    } else if left_dimensions != right_dimensions {
                        return result::Result::Err("Length error".to_string())
                    }

                    let mut result_values: Vec<Box<Value>> = vec![];
                    let mut error_state = "".to_string();
                    let mut errored = false;

                    for index in 0..left_values.len() {
                        match func(&left_values[index], &right_values[index]) {
                            result::Result::Ok(val) => {
                                result_values.push(val);
                            },
                            result::Result::Err(err) => {
                                error_state = err;
                                errored = true;
                                break;
                            }
                        }
                    };

                    if errored {
                        result::Result::Err(error_state)
                    } else {
                        result::Result::Ok(Box::new(Value::AplArray(*left_depth, left_dimensions.clone(), result_values)))
                    }
                },
                _ => {
                    panic!("This should never be reached")
                }
            }
        },
        _ => {
            panic!("This should never be reached")
        }
    }
}

pub fn simple_monadic_array<F>(func: F, param: &Value) -> result::Result<Box<Value>, String> where F: Fn(&Value) -> result::Result<Box<Value>, String> {
    match param {
        Value::AplArray(depth, dimensions, values) => {
            let mut result_values: Vec<Box<Value>> = vec![];
            let mut error_state = "".to_string();
            let mut errored = false;

            for value in values.iter() {
                if !errored {
                    match func(value) {
                        result::Result::Ok(val) => {
                            result_values.push(val);
                        },
                        result::Result::Err(err) => {
                            errored = true;
                            error_state = err;
                        }
                    }
                }
            }

            if errored {
                result::Result::Err(error_state)
            } else {
                result::Result::Ok(Box::new(Value::AplArray(*depth, dimensions.clone(), result_values)))
            }
        },
        _ => {
            panic!("This should never be reached")
        }
    }
}
