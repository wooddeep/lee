use crate::lexer::*;
use crate::tree::*;
use std::collections::HashMap;
use std::cell::RefCell;

#[allow(dead_code)]
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
    /*
    constructor(formula) {
        this.lexer = new Lexer(formula)
        this.const_map = {} // 变量值存储 ~ 都是全局的变量
        this.func_map = {} // 函数表
    }
    */

    pub lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    /*
     * term: factor { ("*" | "/") factor }
     */

    /*
    term: factor { ("*" | "/") factor } 
     | factor ">" factor
     | factor ">=" factor
     | factor "<" factor
     | factor "<=" factor
     | factor "==" factor
    */

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

    fn match_mul_div(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::MULTIP | TokenType::DIVIDER => {
                return true;
            }
            _ => { return false; }
        }
    }


    fn parse_mul_div(&self, left: Option<Tree>, token_type: &TokenType) -> Option<Tree> {
        let token = self.lexer.pick();

        match token {
            None => { return left; } // 返回 乘除法的逻辑
            _ => { println!("##: {}", token.unwrap().literal) }
        }

        let right = self.parse_factor();

        let left_node = Some(Box::new(left.unwrap()));
        let right_node = Some(Box::new(right.unwrap()));

        let left = Tree {
            value: 0,
            token_type: TokenType::Number,
            semantics_type: SemanticsType::Calculate,
            left: left_node,
            right: right_node,
        };

        let next = self.lexer.lookup(0);
        let token_type = &next.unwrap().token_type;
        if !self.match_mul_div(token_type) { return Some(left); }

        self.parse_mul_div(Some(left), token_type)
    }


    #[allow(dead_code)]
    pub fn parse_term(&mut self) {
        let left = self.parse_factor();
        let next = self.lexer.lookup(0);
        let token_type = &next.unwrap().token_type;

        self.parse_mul_div(left, token_type);
    }

    #[allow(dead_code)]
    pub fn parse_factor(&self) -> Option<Tree> {
        let token = self.lexer.lookup(0);

        let token_type = &token.unwrap().token_type;

        match token_type {
            TokenType::Number => {
                let int = token.unwrap().literal.parse::<i32>().unwrap();
                println!("value = {}", int);
                self.lexer.pick(); // 取数

                let tree = Tree {
                    value: 0,
                    token_type: TokenType::Number,
                    semantics_type: SemanticsType::Direct,
                    left: None,
                    right: None,
                };

                return Some(tree);
            }
            _ => {
                return None;
            }
        }
    }
}