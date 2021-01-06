use crate::parser::*;
use crate::lexer::*;
use crate::tree::*;
use std::ops::Deref;


pub struct Executor<'a> {
    parser: &'a mut Parser <'a>,
}

impl<'a> Executor<'a> {

    pub fn new(parser: &'a mut Parser<'a>) -> Self {
        Executor { parser }
    }

    pub fn parser(&'a mut self) -> &'a mut Parser<'a> {
        self.parser
    }

    pub fn eval(&mut self) {
        let ot = self.parser.parse_term();

        match ot {
            None => {},
            Some(tree) => {
                match tree.token_type {
                    TokenType::MULTIP => {

                    },
                    _ => {}
                }

                ()
            }
        }
    }

    pub fn eval_num_cal(&mut self, tree: &Tree) -> i32 {
        let left_val = self.eval_num_cal(&tree.left.as_ref().unwrap());
        let right_val = self.eval_num_cal(&tree.right.as_ref().unwrap());
        //println!();
        0
    }



}
