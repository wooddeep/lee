use std::process;
use std::borrow::Borrow;
use crate::lexer::TokenType;
use crate::parser::SemanticsType;
use std::any::Any;
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
    pub if_branch: Option<Box<Vec<Etree>>>,
    pub else_branch: Option<Box<Vec<Etree>>>,
}

pub enum TreeType {
    BaseTree,
    IfTree,
}

#[derive(Clone)]
pub enum Etree {
    Tree(Tree),
    IfTree(IfTree),
}

pub trait TreeAct {
    fn get_type(&self) -> TreeType;
    fn as_any(&self) -> &dyn Any;
}

impl TreeAct for Tree {
    fn get_type(&self) -> TreeType {
        return TreeType::BaseTree;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TreeAct for IfTree {
    fn get_type(&self) -> TreeType {
        return TreeType::IfTree;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
