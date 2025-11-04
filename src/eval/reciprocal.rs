use crate::{
    nodes::Node,
    eval::{
        eval::{
            Value,
            eval_monadic,
        },
        divide::divide_integer,
    },
};

pub fn reciprocal(first: &Value) -> Result<Box<Value>, String> {
    divide_integer(1, first)
}

pub fn eval_reciprocal(left: &Node) -> Result<Box<Value>, String> {
    eval_monadic(reciprocal, left)
}
