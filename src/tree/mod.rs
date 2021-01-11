use std::process;
use std::borrow::Borrow;
use crate::lexer::TokenType;
use crate::parser::SemanticsType;

// rust面向对象之继承: https://blog.csdn.net/u010766458/article/details/105403282/

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Charset(String),
}

#[derive(Clone)]
pub struct Tree {
    pub value: Value,
    pub token_type: TokenType,
    pub semantics_type: SemanticsType,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

#[derive(Clone)]
pub struct IfTree {
    pub condition: Option<Box<Tree>>,
    pub if_branch: Option<Box<Tree>>,
    pub else_branch: Option<Box<Tree>>,
}

pub trait TreeAct {

}

impl TreeAct for Tree {

}

impl TreeAct for IfTree {

}
