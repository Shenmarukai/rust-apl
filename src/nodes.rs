use crate::{
    tokenizer::{
        Token,
        TokenData,
    },
    parser::Parser,
    eval::{
        eval::Value,
        add::eval_addition,
        subtract::eval_subtraction,
        multiply::eval_multiplication,
        divide::eval_division,
        maximum::eval_maximum,
        minimum::eval_minimum,
        exponential::eval_exponential,
        conjugate::eval_conjugate,
        negate::eval_negate,
        reciprocal::eval_reciprocal,
        sign::eval_sign,
        magnitude::eval_magnitude,
        ceiling::eval_ceiling,
        floor::eval_floor,
        power::eval_power,
    },
};

pub trait EvalNode {
    fn eval(&self) -> Result<Box<Value>, String>;
}

pub trait Parseable {
    fn monadic(&self, parser: &mut Parser) -> Result<Box<Node>, String>;
    fn dyadic(&self, parser: &mut Parser, left: Box<Node>) -> Result<Box<Node>, String>;
}

impl Parseable for TokenData {
    fn monadic(&self, parser: &mut Parser) -> Result<Box<Node>, String> {
        match self.string.as_str() {
            "+" => parser.create_monadic_result(Node::Conjugate),
            "-" | "−" => parser.create_monadic_result(Node::Negate),
            "×" => parser.create_monadic_result(Node::Sign),
            "÷" => parser.create_monadic_result(Node::Reciprocal),
            "|" | "∣" => parser.create_monadic_result(Node::Magnitude),
            "⌈" => parser.create_monadic_result(Node::Ceiling),
            "⌊" => parser.create_monadic_result(Node::Floor),
            "⋆" | "*" => parser.create_monadic_result(Node::Exponential),
            _ => parser.parse_base_expression()
        }
    }

    fn dyadic(&self, parser: &mut Parser, left: Box<Node>) -> Result<Box<Node>, String> {
        match self.string.as_str() {
            "+" => parser.create_dyadic_result(left, Node::Addition),
            "-" | "−" => parser.create_dyadic_result(left, Node::Subtraction),
            "×" => parser.create_dyadic_result(left, Node::Multiplication),
            "÷" => parser.create_dyadic_result(left, Node::Division),
            "⌈" => parser.create_dyadic_result(left, Node::Maximum),
            "⌊" => parser.create_dyadic_result(left, Node::Minimum),
            "⋆" | "*" => parser.create_dyadic_result(left, Node::Power),
            _ => Err("Unknown operator".to_string())
        }
    }
}

#[derive(Debug)]
pub enum Node {
    //Dyadic
    Addition(Box<Token>, Box<Node>, Box<Node>),
    Subtraction(Box<Token>, Box<Node>, Box<Node>),
    Multiplication(Box<Token>, Box<Node>, Box<Node>),
    Division(Box<Token>, Box<Node>, Box<Node>),
    Maximum(Box<Token>, Box<Node>, Box<Node>),
    Minimum(Box<Token>, Box<Node>, Box<Node>),
    Power(Box<Token>, Box<Node>, Box<Node>),

    //Monadic
    Conjugate(Box<Token>, Box<Node>),
    Negate(Box<Token>, Box<Node>),
    Reciprocal(Box<Token>, Box<Node>),
    Sign(Box<Token>, Box<Node>),
    Magnitude(Box<Token>, Box<Node>),
    Ceiling(Box<Token>, Box<Node>),
    Floor(Box<Token>, Box<Node>),
    Exponential(Box<Token>, Box<Node>),

    //Niladic
    Variable(Box<Token>),
    Array(Vec<Box<Token>>),
    Zilde(Box<Token>),
}

impl EvalNode for Node {
    fn eval(&self) -> Result<Box<Value>, String> {
        match self {
            Node::Addition(_, left, right) => eval_addition(left, right),
            Node::Subtraction(_, left, right) => eval_subtraction(left, right),
            Node::Multiplication(_, left, right) => eval_multiplication(left, right),
            Node::Division(_, left, right) => eval_division(left, right),
            Node::Maximum(_, left, right) => eval_maximum(left, right),
            Node::Minimum(_, left, right) => eval_minimum(left, right),
            Node::Power(_, left, right) => eval_power(left, right),

            Node::Conjugate(_, left) => eval_conjugate(left),
            Node::Negate(_, left) => eval_negate(left),
            Node::Reciprocal(_, left) => eval_reciprocal(left),
            Node::Sign(_, left) => eval_sign(left),
            Node::Magnitude(_, left) => eval_magnitude(left),
            Node::Ceiling(_, left) => eval_ceiling(left),
            Node::Floor(_, left) => eval_floor(left),
            Node::Exponential(_, left) => eval_exponential(left),

            _ => Err("Not yet implemented".to_string())
        }
    }
}

pub fn node_to_string(node: &Node) -> String {
    format!("{:?}", node)
}
