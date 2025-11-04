use tokenizer::Token;
use tokenizer::TokenData;
use eval::eval::Value;
use parser::Parser;

use eval::add::eval_addition;
use eval::subtract::eval_subtraction;
use eval::multiply::eval_multiplication;
use eval::divide::eval_division;
use eval::maximum::eval_maximum;
use eval::minimum::eval_minimum;
use eval::exponential::eval_exponential;

use eval::conjugate::eval_conjugate;
use eval::negate::eval_negate;
use eval::reciprocal::eval_reciprocal;
use eval::sign::eval_sign;
use eval::magnitude::eval_magnitude;
use eval::ceiling::eval_ceiling;
use eval::floor::eval_floor;
use eval::power::eval_power;

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
            &Node::Addition(_, ref left, ref right) => eval_addition(left, right),
            &Node::Subtraction(_, ref left, ref right) => eval_subtraction(left, right),
            &Node::Multiplication(_, ref left, ref right) => eval_multiplication(left, right),
            &Node::Division(_, ref left, ref right) => eval_division(left, right),
            &Node::Maximum(_, ref left, ref right) => eval_maximum(left, right),
            &Node::Minimum(_, ref left, ref right) => eval_minimum(left, right),
            &Node::Power(_, ref left, ref right) => eval_power(left, right),

            &Node::Conjugate(_, ref left) => eval_conjugate(left),
            &Node::Negate(_, ref left) => eval_negate(left),
            &Node::Reciprocal(_, ref left) => eval_reciprocal(left),
            &Node::Sign(_, ref left) => eval_sign(left),
            &Node::Magnitude(_, ref left) => eval_magnitude(left),
            &Node::Ceiling(_, ref left) => eval_ceiling(left),
            &Node::Floor(_, ref left) => eval_floor(left),
            &Node::Exponential(_, ref left) => eval_exponential(left),

            _ => Err("Not yet implemented".to_string())
        }
    }
}

pub fn node_to_string(node: &Node) -> String {
    format!("{:?}", node)
}
