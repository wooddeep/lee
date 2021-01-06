use crate::lexer::*;
use crate::tree::*;
use std::collections::HashMap;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SemanticsType {
    Direct,
    // 立即数
    Calculate,
    // 计算
    Assignment,
    Compare,
    MapSet,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Parser { lexer }
    }
}

impl<'a> Parser<'a> {


    // * expr: term { ("+" | "-") term }
    // *   | NAME "=" expr
    // *   | local NAME "=" expr
    // *   | NAME "[" expr "]" "=" expr  // 字典赋值

    pub fn parse_expr(&mut self) -> Option<Tree> {
        let left = self.parse_term();

        loop {

            let token = self.lexer.lookup(0).unwrap();

            match token.token_type {
                TokenType::PLUS | TokenType::SUBSTRACT => (), // + 获取 - 则循环
                _ => break,
            }

            let token_type = token.token_type.clone();

            self.lexer.pick(); // 去掉加减号

            let right = self.parse_term();

            let left = Tree {
                value: Value::Float(0f32),
                token_type,//TokenType::Number,
                semantics_type: SemanticsType::Calculate,
                left: Some(Box::new(left.clone().unwrap())),
                right: Some(Box::new(right.clone().unwrap())),
            };
        }

        left
    }


    // term: factor { ("*" | "/") factor }
    //  | factor ">" factor
    //  | factor ">=" factor
    //  | factor "<" factor
    //  | factor "<=" factor
    //  | factor "==" factor

    /*
    parse_term() {
        var left =  this.parse_factor()

        var next = this.lexer.lookup()

        if (/(>)|(>=)|(<)|(<=)|(==)|(!=)/.exec(next) != null) {
            var token = this.lexer.pick()
            var right = this.parse_factor()
            var out = {
                "oper": "cmp", // 比较
                "token": token,
                "left": left,
                "right": right
            }
            return out
        }

        while ( this.lexer.lookup() == "*" ||  this.lexer.lookup() == "/") {
            var oper =  this.lexer.pick()
            var right = this.parse_factor()
            var left = {
                "left": left,
                "oper": oper,
                "right": right
            }
        }

        return left
    }
    */

    fn match_mul_div(&mut self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::MULTIP | TokenType::DIVIDER => {
                return true;
            }
            _ => { return false; }
        }
    }


    fn parse_mul_div(&mut self, left: Option<Tree>, token_type: &TokenType) -> Option<Tree> {
        let token = self.lexer.pick();

        match token {
            None => { return left; } // 返回 乘除法的逻辑
            _ => { println!("##: {}", token.unwrap().literal) }
        }

        let right = self.parse_factor();

        let left_node = Some(Box::new(left.unwrap()));
        let right_node = Some(Box::new(right.unwrap()));

        let left = Tree {
            value: Value::Float(0f32),
            token_type: *token_type,//TokenType::Number,
            semantics_type: SemanticsType::Calculate,
            left: left_node,
            right: right_node,
        };

        let next = self.lexer.lookup(0);
        let token_type = &next.unwrap().token_type.clone();
        //let token_type = &TokenType::MULTIP;
        if !self.match_mul_div(token_type) { return Some(left); }

        self.parse_mul_div(Some(left), token_type)
    }


    #[allow(dead_code)]
    pub fn parse_term(&mut self) -> Option<Tree> {
        let left = self.parse_factor();
        let next = self.lexer.lookup(0);
        let token_type = &next.unwrap().token_type.clone();

        return self.parse_mul_div(left, token_type);
    }


    // factor: NUMBER  // TODO，加上 "-" NUMBER
    // | "(" expr ")"
    // | NAME
    // | NMAE "[" STRING "]"  // 字典取数
    // | FUNCNAME "(" alist ")"
    // | STRING
    // | "{" {STRING ":" expr ","} STRING ":" expr "}"    // 字典

    #[allow(dead_code)]
    pub fn parse_factor(&mut self) -> Option<Tree> {
        let token = self.lexer.lookup(0);

        let token_type = &token.unwrap().token_type;

        return match token_type {
            TokenType::Number => {
                let val = token.unwrap().literal.parse::<f32>().unwrap();
                println!("value = {}", val);
                self.lexer.pick(); // 取数

                let tree = Tree {
                    value: Value::Float(val),
                    token_type: TokenType::Number,
                    semantics_type: SemanticsType::Direct,
                    left: None,
                    right: None,
                };

                Some(tree)
            }

            TokenType::LeftCurve => {
                self.lexer.pick(); // 去掉左括号
                let expr = self.parse_expr();
                self.lexer.pick(); // 去掉右括号
                expr
            }

            _ => {
                None
            }
        };
    }
}