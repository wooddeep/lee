use crate::parser::*;
use crate::lexer::*;
use crate::tree::*;
use std::ops::Deref;


pub struct Executor<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> Executor<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Self {
        Executor { parser }
    }

    pub fn parser(&'a mut self) -> &'a mut Parser<'a> {
        self.parser
    }

    pub fn eval_program(&mut self) {
        let etl = self.parser.parse_program();

        println!("##statment len: {}", etl.len());

        for et in etl.iter() {

            match et {
                Etree::Tree(tree) => {

                    match tree.token_type {
                        TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                            let value = self.eval_num_calc(&tree);
                            println!("## result = {:?}", value)
                        },
                        _ => {}
                    }
                },

                Etree::IfTree(itree) => {
                    println!("## eval if tree!");

                    if self.eval_num_calc(&itree.condition.as_ref().unwrap()) > Value::Float(0f32) { // tree::Etree::Tree(condition.unwrap()
                        println!("## in if branch!");
                    }
                },

                Etree::FuncTree(ftree) => {

                },

                _ => {}
            }
        }
    }

    pub fn eval_stmt(&mut self) {
        let et = self.parser.parse_statement().unwrap();

        match et {
            Etree::Tree(tree) => {
                match tree.token_type {
                    TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                        let value = self.eval_num_calc(&tree);
                        println!("## result = {:?}", value)
                    },
                    _ => {}
                }

                ()
            },
            Etree::IfTree(tree) => {
                // match tree.token_type {
                //     TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                //         let value = self.eval_num_calc(&tree);
                //         println!("## result = {:?}", value)
                //     }
                //     _ => {}
                // }
                //
                ()
            },
            Etree::FuncTree(ftree) => {

            },

            _ => (),
        }
    }

    pub fn eval(&mut self) {
        let ot = self.parser.parse_expr();

        match ot {
            None => {}
            Some(tree) => {
                match tree.token_type {
                    TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                        let value = self.eval_num_calc(&tree);
                        println!("## result = {:?}", value)
                    }
                    _ => {}
                }

                ()
            }
        }
    }

    pub fn eval_num_calc(&mut self, tree: &Tree) -> Value {
        match tree.semantics_type {
            SemanticsType::Direct => {
                return tree.value.clone();
            }

            _ => {
                let left_val = self.eval_num_calc(&tree.left.as_ref().unwrap());
                let right_val = self.eval_num_calc(&tree.right.as_ref().unwrap());
                match tree.token_type {
                    TokenType::MULTIP => {
                        let lv = match left_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        let rv = match right_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        return Value::Float(lv * rv);
                    }

                    TokenType::DIVIDER => {
                        let lv = match left_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        let rv = match right_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        return Value::Float(lv / rv);
                    }

                    TokenType::PLUS => {
                        let lv = match left_val {
                            Value::Float(v) => (v, None),
                            Value::Charset(s) => (0f32, Some(s)),
                            _ => panic!("undefine value type"),
                        };

                        let rv = match right_val {
                            Value::Float(v) => (v, None),
                            Value::Charset(s) => (0f32, Some(s)),
                            _ => panic!("undefine value type"),
                        };

                        if lv.1 == None && rv.1 == None { // 1. 0 + 2.0 = 3.0
                            return Value::Float(lv.0 + rv.0);
                        }

                        if lv.1 == None && rv.1 != None { // 1 + "abc" = "1abc"
                            return Value::Charset(lv.0.to_string() + &rv.1.unwrap());
                        }

                        if lv.1 != None && rv.1 == None { // 1 + "abc" = "1abc"
                            return Value::Charset(lv.1.unwrap() + rv.0.to_string().as_str());
                        }

                        return Value::Charset(lv.1.unwrap() + &rv.1.unwrap());
                    }

                    TokenType::SUBSTRACT => {
                        let lv = match left_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        let rv = match right_val {
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        return Value::Float(lv - rv);
                    }
                    _ => { return Value::Float(0f32); }
                }
            }
        }
    }
}
