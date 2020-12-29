use std::cell::Cell;
use regex::{Regex};

pub mod shell {
    pub fn test() {
        println!("in the named module!")
    }
}

pub fn bare_func() {
    println!("in bare func!")
}

#[allow(dead_code)]
pub struct Lexer {
    pub curr_index: Cell<usize>,
    pub token_list: Vec<Token>,
    pub formula: String,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum TokenType {
    MultipleLineComment,
    SingleLineComment,
    Identifier,
    Number,
    Comma,  // ,
    Colon,  // :
    Semicolon, // ;
    LeftCurve, // (
    RightCurve, // )
    LeftSquare, // [
    RightSquare, // ]
    LeftBraces, // {
    RightBraces, // }
    STRING,
    PLUS, // +
    SUBSTRACT, // - 
    MULTIP, // *
    DIVIDER, // /
    ASSIGN, // =
    GT, // >
    LT, // <
    GE, // >=
    LE, // <=
    EQ, // ==
    AND, // &&
    OR,  // ||
    EOL,
}

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

//use crate::TokenType;

impl Lexer {

    pub fn analyze(&mut self) {
        //let s = "/* coment */hello#abc def\nabc \"123\" \n -90.123f123,;:+-=*/abcd &&& ||";
        let r = Regex::new(r#"(?s)/\*[^/]*\*/|#[^\n]*|[_a-zA-Z][\-_a-zA-Z0-9]*|".*"|\-?[0-9]+\.?[0-9]*[lf]*|[,:;\+\-\*/=\(\)\{\}\[\]]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|"#).unwrap();
        let regex_multi_comment = Regex::new(r#"(?s)/\*[^/]*\*/"#).unwrap();
        let regex_single_comment = Regex::new(r#"#[^\n]*"#).unwrap();
        let regex_identifier = Regex::new(r#"^[_a-zA-Z][\-_a-zA-Z0-9]*"#).unwrap();
        let regex_string = Regex::new(r#"".*""#).unwrap();
        let regex_number = Regex::new(r#"\-?[0-9]+\.?[0-9]*[lf]*"#).unwrap();

        for (_i, c) in r.captures_iter(&self.formula).enumerate() {
            for j in 0..c.len() {
                if regex_multi_comment.is_match(&c[j]) {
                    let token = Token{literal: String::from(&c[j]), token_type: TokenType::MultipleLineComment};
                    self.token_list.push(token);
                    break;
                }

                if regex_single_comment.is_match(&c[j]) {
                    let token = Token{literal: String::from(&c[j]), token_type: TokenType::SingleLineComment};
                    self.token_list.push(token);
                    break;
                }

                if regex_identifier.is_match(&c[j]) {
                    let token = Token{literal: String::from(&c[j]), token_type: TokenType::Identifier};
                    self.token_list.push(token);
                    break;
                }

                if regex_string.is_match(&c[j]) {
                    let token = Token{literal: String::from(&c[j]), token_type: TokenType::STRING};
                    self.token_list.push(token);
                    break;
                }

                if regex_number.is_match(&c[j]) {
                    let token = Token{literal: String::from(&c[j]), token_type: TokenType::Number};
                    self.token_list.push(token);
                    break;
                }
                
                match &c[j] {
                    "+" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::PLUS};
                        self.token_list.push(token);
                    },

                    "-" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::SUBSTRACT};
                        self.token_list.push(token);
                    },

                    "*" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::MULTIP};
                        self.token_list.push(token);
                    },

                    "/" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::DIVIDER};
                        self.token_list.push(token);
                    },

                    "=" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::ASSIGN};
                        self.token_list.push(token);
                    },
                    
                    "(" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::LeftCurve};
                        self.token_list.push(token);
                    },

                    ")" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::RightCurve};
                        self.token_list.push(token);
                    },

                    "[" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::LeftSquare};
                        self.token_list.push(token);
                    },

                    "]" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::RightSquare};
                        self.token_list.push(token);
                    },

                    "{" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::LeftBraces};
                        self.token_list.push(token);
                    },

                    "}" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::RightBraces};
                        self.token_list.push(token);
                    },

                    ">" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::GT};
                        self.token_list.push(token);
                    },

                    "<" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::LT};
                        self.token_list.push(token);
                    },

                    ">=" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::GE};
                        self.token_list.push(token);
                    },

                    "<=" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::LE};
                        self.token_list.push(token);
                    },

                    "==" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::EQ};
                        self.token_list.push(token);
                    },

                    "&&" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::AND};
                        self.token_list.push(token);
                    },

                    "||" => {
                        let token = Token{literal: String::from(&c[j]), token_type: TokenType::OR};
                        self.token_list.push(token);
                    },
                    //_ => println!("group {},{} : {}", _i, j, &c[j]),
                    _ => {},
                }
            }
        }
    }

    /*
    pick() {
        if (this.curr_index == this.token_list.length - 1) {
            return "EOL"
        }

        var token = this.token_list[this.curr_index]
        this.curr_index = this.curr_index + 1
        return token
    }
    */

    pub fn pick(&self) -> Option<&Token> {
        
        if self.curr_index.get() == self.token_list.len() - 1 {
            return None
        }

        //println!("##[a] curr_index: {}", self.curr_index.get());

        let new_val = self.curr_index.get() + 1;

        self.curr_index.set(new_val);

        
        return Some(&self.token_list[self.curr_index.get() - 1]);
    }

    /*
    
    lookup(n = 0) {
        return this.token_list[this.curr_index + n]
    }

    lookups(n = 0) {
        return this.token_list[this.curr_index + n - 1]
    }
    
    */

    pub fn lookup(&self, n: usize) -> Option<&Token> {
        //println!("##shit!!!!");
        //println!("##[b] curr_index: {}", self.curr_index.get() + n);

        return Some(&self.token_list[self.curr_index.get() + n]);
    }

    pub fn lookups(&self, n: usize) -> Option<&Token> {
        return Some(&self.token_list[self.curr_index.get() + n - 1]);
    }
}