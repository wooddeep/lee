use crate::parser::*;
use crate::lexer::*;
use crate::tree::*;
use std::ops::Deref;
use std::borrow::Borrow;
use std::collections::HashMap;


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
        self.eval_etree_list(&etl.0, &etl.1);
        //self.eval_func_map(&etl.1);
    }

    pub fn eval_func_call() {

    }

    pub fn eval_etree(&mut self, et: &Etree, fm: &HashMap<String, Etree>) {
        match et {

            Etree::FuncCallTree(fctree) => {
                println!("## function name: {}", fctree.func_name);
                let fbody = fm.get(&fctree.func_name);
                self.eval_etree(fbody.unwrap(), fm);
            }

            Etree::FuncTree(ftree) => {
                println!("## function call calc"); // 具体的值计算

            }

            Etree::Tree(tree) => {
                match tree.semantics_type {
                    SemanticsType::Calculate | SemanticsType::Compare | SemanticsType::Direct => {
                        let value = self.eval_num_calc(&tree);
                        println!("## result = {:?}", value);
                    }
                    _ => {}
                }
            }

            Etree::IfTree(itree) => {
                let value = self.eval_num_calc(&itree.condition.as_ref().unwrap().tree().unwrap());

                match value {
                    Value::Bool(bv) => {
                        if bv {
                            println!("## in if branch!");
                            self.eval_etree_list(itree.if_branch.as_ref().unwrap(), fm);
                        } else {
                            println!("## in else branch!");
                            self.eval_etree_list(itree.else_branch.as_ref().unwrap(), fm);
                        }
                    }

                    Value::Float(fv) => {
                        if fv > 0f32 {
                            println!("## in if branch!");
                            self.eval_etree_list(itree.if_branch.as_ref().unwrap(), fm);
                        } else {
                            println!("## in else branch!");
                            self.eval_etree_list(itree.else_branch.as_ref().unwrap(), fm);
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    pub fn eval_etree_list(&mut self, etl: &Vec<Etree>, fm: &HashMap<String, Etree>) {
        for et in etl.iter() {
            self.eval_etree(et, fm);
        }
    }

    pub fn eval_func_map(&mut self, fmap: &HashMap<String, Etree>) {
        for (_, tree) in fmap {
           self.eval_etree(tree, fmap);
        }
    }

    pub fn eval(&mut self) {
        let ot = self.parser.parse_expr();

        match ot {
            None => {}
            Some(tree) => {
                match tree.tree().unwrap().token_type {
                    TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                        let value = self.eval_num_calc(&tree.tree().unwrap());
                        println!("## result = {:?}", value)
                    }
                    _ => {}
                }

                ()
            }
        }
    }

    pub fn eval_compare(&mut self, tree: &Tree) -> Value {
        let left_val = self.eval_num_calc(tree.left.as_ref().unwrap().tree().unwrap());
        let right_val = self.eval_num_calc(tree.right.as_ref().unwrap().tree().unwrap());

        match tree.token_type {
            TokenType::EQ => {
                return Value::Bool(left_val == right_val);
            }

            TokenType::UE => {
                return Value::Bool(left_val != right_val);
            }

            TokenType::GT => {
                return Value::Bool(left_val > right_val);
            }

            TokenType::GE => {
                return Value::Bool(left_val >= right_val);
            }

            TokenType::LT => {
                return Value::Bool(left_val < right_val);
            }

            TokenType::LE => {
                return Value::Bool(left_val <= right_val);
            }
            _ => {}
        }

        return Value::Bool(true);
    }

    pub fn eval_num_calc(&mut self, tree: &Tree) -> Value {
        match tree.semantics_type {
            SemanticsType::Direct => {
                return tree.value.clone();
            }

            SemanticsType::Compare => {
                println!("it compare type");
                return self.eval_compare(tree);
            }

            _ => {
                let left_val = self.eval_num_calc(&tree.left.as_ref().unwrap().tree().unwrap());
                let right_val = self.eval_num_calc(&tree.right.as_ref().unwrap().tree().unwrap());
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
