use crate::lexer::*;
use crate::tree::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::any::Any;
use crate::tree;
use std::sync::atomic::Ordering::SeqCst;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)] // #[derive(Debug, Clone, PartialEq)]
pub enum SemanticsType {
    Direct,
    // 立即数
    Calculate,
    // 计算
    Assignment,
    Compare,
    MapSet,
    Condition,
    FuncDef,
    FuncCall,
    Variable,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    //pub func_map: HashMap<String, Etree>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Parser { lexer }
    }
}

/*
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Parser { lexer, func_map: HashMap::new() }
    }
}
*/


impl<'a> Parser<'a> {
    // /*
    //  * program: statement { ";" statement }
    //  */
    pub fn parse_program(&mut self) -> (Vec<Etree>, HashMap<String, Etree>) {
        let mut func_map: HashMap<String, Etree> = HashMap::new();
        let mut out: Vec<Etree> = Vec::new(); // 输出代码块列表
        let stmt = self.parse_statement().unwrap();
        match &stmt {
            Etree::FuncTree(ftree) => {
                func_map.insert(ftree.func_name.clone(), stmt);
            }
            _ => {
                out.push(stmt);
            }
        }
        //println!("{:?}", self.lexer.lookup(0).unwrap().token_type);

        while self.lexer.lookup(0).unwrap().token_type == TokenType::Semicolon {
            self.lexer.pick();
            let stmt = self.parse_statement();
            match &stmt {
                None => break,
                Some(etree) => {
                    match &etree {
                        Etree::FuncTree(ftree) => {
                            func_map.insert(ftree.func_name.clone(), stmt.unwrap());
                        }
                        _ => {
                            out.push(stmt.unwrap());
                        }
                    }
                }
            }
        }

        return (out, func_map);
    }


    // /*
    //  * statement: "if" expr block [ "else" block ]
    //  *       | "while" expr block
    //  *       | "func" FUNCNAME "(" plist ")" block
    //  *       | expr
    //  */
    pub fn parse_statement(&mut self) -> Option<Etree> {
        let token_type = self.lexer.lookup(0).unwrap().token_type;

        if token_type == TokenType::Func {
            self.lexer.pick(); // 去掉 "func"
            let func_name = self.lexer.pick().unwrap().literal.clone(); // 函数名称
            self.lexer.pick(); // 去掉左括号
            let plist = self.parse_plist(); // 参数列表
            self.lexer.pick(); // 去掉右括号
            let func_body = self.parse_block(); // 函数体
            let func_tree = FuncTree {
                semantics_type: SemanticsType::FuncDef,
                func_name,
                plist: Some(Box::new(plist.unwrap())),
                fbody: Some(Box::new(func_body.unwrap())),
            };

            return Some(Etree::FuncTree(func_tree));
        } else if token_type == TokenType::If {
            self.lexer.pick();
            let condition = self.parse_expr();
            let if_branch = self.parse_block();

            let mut left = IfTree {
                condition: Some(Box::new(condition.unwrap())),
                if_branch: Some(Box::new(if_branch.unwrap())),
                else_branch: None,
            };

            let token_type = self.lexer.lookup(0).unwrap().token_type;
            if token_type == TokenType::Else {
                self.lexer.pick();
                let else_branch = self.parse_block();
                left.else_branch = Some(Box::new(else_branch.unwrap()));
            }

            return Some(Etree::IfTree(left));
        } else {
            let expr = self.parse_expr();
            match &expr {
                Some(tree) => return Some(expr.unwrap()),
                _ => return None
            }
            //return Some(Etree::Tree(expr.unwrap()));
        }
    }

    // /*
    //  * plist: "" | NAME {, NAME}
    //  */
    fn parse_plist(&mut self) -> Option<Vec<String>> {
        let mut out: Vec<String> = Vec::new(); // 输出代码块列表
        if self.lexer.lookup(0).unwrap().token_type != TokenType::RightCurve {
            let para = self.lexer.pick();
            out.push(para.unwrap().literal.clone()); // 存储形参的 字符串 字面量
            while self.lexer.lookup(0).unwrap().token_type == TokenType::Comma {
                self.lexer.pick();
                let para = self.lexer.pick();
                out.push(para.unwrap().literal.clone());
            }
        }
        return Some(out);
    }

    // /*
    //  * block: "{" [ statement ] { ((";" | EOL) [statement]} "}"
    //  */
    fn parse_block(&mut self) -> Option<Vec<Etree>> {
        let mut out: Vec<Etree> = Vec::new(); // 输出代码块列表

        if self.lexer.lookup(0).unwrap().token_type == TokenType::LeftBraces {
            self.lexer.pick();
            let statement = self.parse_statement();
            out.push(statement.unwrap());

            while self.lexer.lookup(0).unwrap().token_type == TokenType::Semicolon {
                self.lexer.pick();
                let stmt = self.parse_statement();
                out.push(stmt.unwrap());
            }

            self.lexer.pick();
        }
        return Some(out);
    }

    // * expr: term { ("+" | "-") term }
    // *   | NAME "=" expr
    // *   | local NAME "=" expr
    // *   | NAME "[" expr "]" "=" expr  // 字典赋值

    fn match_add_sub(&mut self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::PLUS | TokenType::SUBSTRACT => {
                return true;
            }
            _ => { return false; }
        }
    }

    fn parse_add_sub(&mut self, left: Option<Etree>, token_type: &TokenType) -> Option<Etree> {
        if !self.match_add_sub(token_type) { return left; }

        let token = self.lexer.pick();
        match token {
            None => { return left; } // 返回 乘除法的逻辑
            _ => { println!("## parse_add_sub: {}", token.unwrap().literal) }
        }

        let right = self.parse_term();

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

        self.parse_add_sub(Some(Etree::Tree(left)), token_type)
    }

    pub fn parse_expr(&mut self) -> Option<Etree> {
        let left = self.parse_term();

        let next = self.lexer.lookup(0);
        let token_type = &next.unwrap().token_type.clone();

        return self.parse_add_sub(left, token_type);
    }

    fn match_mul_div(&mut self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::MULTIP | TokenType::DIVIDER => {
                return true;
            }
            _ => { return false; }
        }
    }

    fn parse_mul_div(&mut self, left: Option<Etree>, token_type: &TokenType) -> Option<Etree> {
        if !self.match_mul_div(token_type) { return left; }

        let token = self.lexer.pick();
        match token {
            None => { return left; } // 返回 乘除法的逻辑
            _ => { println!("## parse_mul_div: {}", token.unwrap().literal) }
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

        self.parse_mul_div(Some(Etree::Tree(left)), token_type)
    }

    // /*
    // term: factor { ("*" | "/") factor }
    //  | factor ">" factor
    //  | factor ">=" factor
    //  | factor "<" factor
    //  | factor "<=" factor
    //  | factor "==" factor
    // */
    pub fn parse_term(&mut self) -> Option<Etree> {
        let left = self.parse_factor();

        let next = self.lexer.lookup(0);
        let token_type = next.unwrap().token_type;
        match token_type {
            TokenType::GT | TokenType::GE | TokenType::EQ |
            TokenType::LT | TokenType::LE | TokenType::EQ => {
                self.lexer.pick();
                let right = self.parse_factor();
                let tree = Tree {
                    value: Value::Float(0f32),
                    token_type,
                    semantics_type: SemanticsType::Compare,
                    left: Some(Box::new(left.unwrap())),
                    right: Some(Box::new(right.unwrap())),
                };
                return Some(Etree::Tree(tree));
            }
            _ => {}
        }

        let token_type = &next.unwrap().token_type.clone();

        return self.parse_mul_div(left, token_type);
    }

    // /*
    //  * alist: "" | expr {, expr} // 实参列表
    //  */
    pub fn parse_alist(&mut self) -> Option<Vec<Etree>> {
        let mut out: Vec<Etree> = Vec::new();

        if self.lexer.lookup(0).unwrap().token_type != TokenType::RightCurve {
            let para = self.parse_expr();
            out.push(para.unwrap());

            while self.lexer.lookup(0).unwrap().token_type != TokenType::Comma {
                self.lexer.pick();
                let para = self.parse_expr();
                out.push(para.unwrap());
            }
        }

        return Some(out);
    }


    // factor: NUMBER
    // | "(" expr ")"
    // | NAME  // 变量
    // | NMAE "[" STRING "]"  // 字典取数
    // | FUNCNAME "(" alist ")"
    // | STRING
    // | "{" {STRING ":" expr ","} STRING ":" expr "}"    // 字典

    #[allow(dead_code)]
    pub fn parse_factor(&mut self) -> Option<Etree> {
        let token = self.lexer.lookup(0);
        let token_literal = &token.unwrap().literal.clone();
        let token_type = &token.unwrap().token_type;

        return match token_type {
            TokenType::Number => {
                let val = token.unwrap().literal.parse::<f32>().unwrap();
                println!("## parse_factor:number = {}", val);
                self.lexer.pick(); // 取数

                let tree = Tree {
                    value: Value::Float(val),
                    token_type: TokenType::Number,
                    semantics_type: SemanticsType::Direct,
                    left: None,
                    right: None,
                };

                Some(Etree::Tree(tree))
            }

            TokenType::STRING => {
                let usize = token.unwrap().literal.as_str().len();
                let val = token.unwrap().literal.as_str()[1..usize - 1].to_string();
                println!("## parse_factor: string = {}", val);
                self.lexer.pick(); // 取数

                let tree = Tree {
                    value: Value::Charset(val),
                    token_type: TokenType::Number,
                    semantics_type: SemanticsType::Direct,
                    left: None,
                    right: None,
                };

                Some(Etree::Tree(tree))
            }

            TokenType::LeftCurve => {
                self.lexer.pick(); // 去掉左括号
                let expr = self.parse_expr();
                self.lexer.pick(); // 去掉右括号
                expr
            }

            TokenType::Identifier => { // 普通标识符: 变量, 函数参数, 函数调用

                if self.lexer.lookups(2).unwrap().token_type == TokenType::LeftCurve { // 函数调用
                    let func_name = self.lexer.pick().unwrap().literal.clone();
                    self.lexer.pick(); // 去掉左括号
                    let alist = self.parse_alist();
                    self.lexer.pick(); // 去掉右括号
                    let tree = FuncCallTree {
                        semantics_type: SemanticsType::FuncCall,
                        func_name,
                        alist: Some(Box::new(alist.unwrap())),
                    };
                    Some(Etree::FuncCallTree(tree))
                } else {
                    println!("## parse_factor:variable = {}", token_literal); // 变量
                    self.lexer.pick(); // 取数
                    let tree = Tree {
                        value: Value::Charset(String::from(token_literal)),
                        token_type: TokenType::Identifier,
                        semantics_type: SemanticsType::Variable,
                        left: None,
                        right: None,
                    };
                    Some(Etree::Tree(tree))
                }
            }

            _ => {
                None
            }
        };
    }
}