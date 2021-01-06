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
                            Value::Float(v) => v,
                            _ => panic!("undefine value type"),
                        };

                        let rv = match right_val {
                            Value::Float(v) => {
                                if v == 0f32 {
                                    panic!("## divid by zero error!")
                                } else {
                                    v
                                }
                            },
                            _ => panic!("undefine value type"),
                        };

                        return Value::Float(lv + rv);
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
