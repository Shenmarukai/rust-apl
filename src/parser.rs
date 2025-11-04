use tokenizer;
use tokenizer::Token;
use nodes::{Node, Parseable};

pub struct Parser {
    tokenizer: Box<tokenizer::Tokenizer>,
    current_token: Option<Box<tokenizer::Token>>
}

impl Parser {

    pub fn new(input_string: String) -> Parser {
        Parser {
            tokenizer: Box::new(tokenizer::Tokenizer::new(input_string)),
            current_token: None
        }
    }

    pub fn parse_next_statement(&mut self) -> Result<Box<Node>, String> {

        match self.read_next_token() {
            Ok(()) => {
                match self.current_token.clone().map(|t| *t) {
                    Some(tokenizer::Token::EndOfFile) => {
                        Err("End of File".to_string())
                    },
                    Some(_) => {
                        self.parse_dyadic()
                    },
                    None => {
                        Err("Everything is wrong".to_string())
                    }
                }
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn read_next_token(&mut self) -> Result<(), String> {
        match self.tokenizer.read_next_token() {
            Ok(token) => {
                self.current_token = Some(token);
                Ok(())
            },
            Err(msg) => {
                self.current_token = None;
                Err(msg)
            }
        }
    }

    fn end_of_source(&self) -> bool {
        match self.current_token.clone().map(|t| *t) {
            None => true,
            Some(tokenizer::Token::EndOfFile) => true,
            _ => false
        }
    }

    fn token_is_number(&self) -> bool {
        match self.current_token.clone().map(|t| *t) {
            Some(tokenizer::Token::Number(_)) => true,
            _ => false
        }
    }

    pub fn create_dyadic_result<F>(&mut self, left: Box<Node>, kind: F) -> Result<Box<Node>, String> where F: FnOnce(Box<Token>, Box<Node>, Box<Node>) -> Node, {
        let stash = self.stash();
        match self.parse_dyadic() {
            Ok(node) => {
                let item = Box::new(kind(stash, left, node));
                self.read_next_token();
                Ok(item)
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn parse_dyadic(&mut self) -> Result<Box<Node>, String> {
        if self.end_of_source() {
            Err("Unexpected end of source".to_string())
        } else {
            //Parse monadic on the left (otherwise it's an endless loop).
            match self.parse_monadic() {
                Ok(left) => {
                    if self.end_of_source() {
                        Ok(left)
                    } else {
                        //FIXME: We should really avoid copying here
                        let token = self.current_token.clone();

                        match token.map(|t| *t) {
                            Some(tokenizer::Token::Primitive(ref token_data)) => {
                                token_data.dyadic(self, left)
                            },
                            _ => {
                                Ok(left)
                            }
                        }
                    }
                },
                Err(msg) => Err(msg)
            }
        }
    }

    fn stash(&mut self) -> Box<tokenizer::Token> {
        let stash = self.current_token.take().unwrap();
        self.read_next_token();
        stash
    }

    pub fn create_monadic_result<F>(&mut self, kind: F) -> Result<Box<Node>, String> where F: FnOnce(Box<Token>, Box<Node>) -> Node, {
        let stash = self.stash();
        match self.parse_dyadic() {
            Ok(node) => {
                let item = Box::new(kind(stash, node));
                self.read_next_token();
                Ok(item)
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn parse_monadic(&mut self) -> Result<Box<Node>, String> {
        if self.end_of_source() {
            Err("Unexpected end of source".to_string())
        } else {

            //FIXME: We should really avoid copying here
            let token = self.current_token.clone();
            match token.map(|t| *t) {
                Some(tokenizer::Token::Primitive(ref token_data)) => {
                    token_data.monadic(self)
                },
                _ => self.parse_base_expression()
            }
        }
    }

    pub fn parse_base_expression(&mut self) -> Result<Box<Node>, String> {
        //This will either be an Array, a Number, or a Niladic primitive (or a bracketed thingy)
        if self.end_of_source() {
            Err("Unexpected end of source".to_string())
        } else {
            //FIXME: Better error handling
            //FIXME: We should really avoid copying here
            let token = self.current_token.clone();

            match token.map(|t| *t) {
                Some(tokenizer::Token::Number(_)) => self.parse_array(),
                Some(tokenizer::Token::Variable(_)) => self.parse_variable(),
                Some(tokenizer::Token::Primitive(ref token_data)) => {
                    match token_data.string.as_str() {
                        "⍬" => self.parse_zilde(),
                        "(" => Err("Not yet implemented".to_string()),
                        _ => Err("Unexpected primitive".to_string())
                    }
                },
                _ => Err("Unexpected token".to_string())
            }
        }
    }

    fn parse_array(&mut self) -> Result<Box<Node>, String> {
        let mut tokens: Vec<Box<Token>> = vec![];
        while self.token_is_number() {
            tokens.push(self.current_token.take().unwrap());
            self.read_next_token();
        }
        Ok(Box::new(Node::Array(tokens)))
    }

    fn parse_variable(&mut self) -> Result<Box<Node>, String> {
        let result = Box::new(Node::Variable(self.current_token.take().unwrap()));
        self.read_next_token();
        Ok(result)
    }

    fn parse_zilde(&mut self) -> Result<Box<Node>, String> {
        let result = Box::new(Node::Zilde(self.current_token.take().unwrap()));
        self.read_next_token();
        Ok(result)
    }
}
