use crate::nodes;

use crate::eval::eval::{Value, eval_monadic};
use crate::eval::divide::divide_integer;

pub fn reciprocal(first: &Value) -> Result<Box<Value>, String> {
    divide_integer(1, first)
}

pub fn eval_reciprocal(left: &nodes::Node) -> Result<Box<Value>, String> {
    eval_monadic(reciprocal, left)
}
