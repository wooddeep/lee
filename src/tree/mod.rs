use std::process;
use std::borrow::Borrow;
use crate::lexer::TokenType;
use crate::parser::SemanticsType;
use std::any::Any;
use std::cmp::Ordering;
// rust面向对象之继承: https://blog.csdn.net/u010766458/article/details/105403282/

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f32),
    Charset(String),
    Bool(bool),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Value::Float(fs) => {
                match other {
                    Value::Float(fa) => {
                        return fs.partial_cmp(fa);
                    },
                    _ => {panic!("compare tye error!")}
                }
            },
            _ => {panic!("compare tye error!")}
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            Value::Float(fs) => {
                match other {
                    Value::Float(fa) => {
                        return fs.lt(fa);
                    },
                    _ => {panic!("compare tye error!")}
                }
            },
            _ => {panic!("compare tye error!")}
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            Value::Float(fs) => {
                match other {
                    Value::Float(fa) => {
                        return fs.le(fa);
                    },
                    _ => {panic!("compare tye error!")}
                }
            },
            _ => {panic!("compare tye error!")}
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Value::Float(fs) => {
                match other {
                    Value::Float(fa) => {
                        return fs.gt(fa);
                    },
                    _ => {return false}
                }
            },
            _ => {return false;}
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            Value::Float(fs) => {
                match other {
                    Value::Float(fa) => {
                        return fs.ge(fa);
                    },
                    _ => {return false}
                }
            },
            _ => {return false;}
        }
    }
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

#[derive(Clone)]
pub struct FuncTree {
    pub semantics_type: SemanticsType,
    pub func_name: String,
    pub plist: Option<Box<Vec<Etree>>>,
    pub fbody: Option<Box<Vec<Etree>>>,
}

pub enum TreeType {
    BaseTree,
    IfTree,
    FuncTree,
}

#[derive(Clone)]
pub enum Etree {
    Tree(Tree),
    IfTree(IfTree),
    FuncTree(FuncTree),
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

impl TreeAct for FuncTree {
    fn get_type(&self) -> TreeType {
        return TreeType::FuncTree;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}