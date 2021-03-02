use crate::parser::*;
use crate::lexer::*;
use crate::tree::*;
use std::ops::Deref;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


pub struct Executor<'a> {
    parser: &'a mut Parser<'a>,
    para_map: Rc<HashMap<String, HashMap<String, i32>>>, // 记录个函数的参数列表
}

impl<'a> Executor<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Self {
        Executor { parser, para_map: Rc::new(Default::default()) }
    }

    pub fn parser(&'a mut self) -> &'a mut Parser<'a> {
        self.parser
    }

    pub fn eval_program(&mut self) {
        let etl = self.parser.parse_program();
        self.eval_etree_list(&etl.0, &etl.1, None, None);
        //self.eval_func_map(&etl.1);
    }

    pub fn eval_func_call(&mut self, ft: &FuncTree, alist: &Vec<Etree>, fm: &HashMap<String, Etree>) {
        let body = ft.fbody.as_ref().unwrap(); // 函数体
        let plist = ft.plist.as_ref().unwrap();
        self.eval_etree_list(body, fm, Some(plist), Some(alist));
    }

    pub fn eval_etree(&mut self, et: &Etree, fm: &HashMap<String, Etree>,
                      plist: Option<&HashMap<String, i32>>, alist: Option<&Vec<Etree>>) {
        match et {
            Etree::FuncCallTree(fctree) => {
                println!("## function name: {}", fctree.func_name);
                let ftree = fm.get(&fctree.func_name).unwrap().func_tree().unwrap();
                self.eval_func_call(ftree, fctree.alist.as_ref().unwrap(), fm);
            }

            Etree::Tree(tree) => {
                match tree.semantics_type {
                    SemanticsType::Calculate | SemanticsType::Compare | SemanticsType::Direct => {
                        let value = self.eval_num_calc(&tree, plist, alist);
                        println!("## result = {:?}", value);
                    }
                    _ => {}
                }
            }

            Etree::IfTree(itree) => {
                let value = self.eval_num_calc(
                    &itree.condition.as_ref().unwrap().tree().unwrap(), plist, alist);

                match value {
                    Value::Bool(bv) => {
                        if bv {
                            println!("## in if branch!");
                            self.eval_etree_list(itree.if_branch.as_ref().unwrap(), fm, None, None);
                        } else {
                            println!("## in else branch!");
                            self.eval_etree_list(itree.else_branch.as_ref().unwrap(), fm, None, None);
                        }
                    }

                    Value::Float(fv) => {
                        if fv > 0f32 {
                            println!("## in if branch!");
                            self.eval_etree_list(itree.if_branch.as_ref().unwrap(), fm, None, None);
                        } else {
                            println!("## in else branch!");
                            self.eval_etree_list(itree.else_branch.as_ref().unwrap(), fm, None, None);
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    pub fn eval_etree_list(&mut self, etl: &Vec<Etree>, fm: &HashMap<String, Etree>,
                           plist: Option<&HashMap<String, i32>>, alist: Option<&Vec<Etree>>) {
        for et in etl.iter() {
            self.eval_etree(et, fm, plist, alist);
        }
    }

    pub fn eval_func_map(&mut self, fmap: &HashMap<String, Etree>) {
        for (_, tree) in fmap {
            self.eval_etree(tree, fmap, None, None);
        }
    }

    pub fn eval(&mut self) {
        let ot = self.parser.parse_expr();

        match ot {
            None => {}
            Some(tree) => {
                match tree.tree().unwrap().token_type {
                    TokenType::MULTIP | TokenType::DIVIDER | TokenType::PLUS | TokenType::SUBSTRACT => {
                        let value = self.eval_num_calc(&tree.tree().unwrap(), None, None);
                        println!("## result = {:?}", value)
                    }
                    _ => {}
                }

                ()
            }
        }
    }

    pub fn eval_compare(&mut self, tree: &Tree) -> Value {
        let left_val = self.eval_num_calc(tree.left.as_ref().unwrap().tree().unwrap(), None, None);
        let right_val = self.eval_num_calc(tree.right.as_ref().unwrap().tree().unwrap(), None, None);

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

    pub fn eval_num_calc(&mut self, tree: &Tree, plist: Option<&HashMap<String, i32>>,
                         alist: Option<&Vec<Etree>>) -> Value {
        match tree.semantics_type {
            SemanticsType::Direct => {
                return tree.value.clone();
            }

            SemanticsType::Variable => { // 变量
                let para_name = tree.value.get_str().unwrap();
                let para_index = plist.unwrap().get(para_name).unwrap();
                println!("# var name: {:?}, index: {:?}", para_name, para_index);

                let arg = alist.unwrap().get( *para_index as usize).unwrap();
                match arg {
                    Etree::Tree(tree) => {
                        return self.eval_num_calc(tree, plist, alist);
                    },
                    _ => {return Value::Float(0f32)},
                }
            }

            SemanticsType::Compare => {
                println!("it compare type");
                return self.eval_compare(tree);
            }

            _ => {
                let left_val = self.eval_num_calc(&tree.left.as_ref().unwrap().tree().unwrap(), plist, alist);
                let right_val = self.eval_num_calc(&tree.right.as_ref().unwrap().tree().unwrap(), plist, alist);
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
