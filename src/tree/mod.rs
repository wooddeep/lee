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
    pub right: Option<Box<Tree>>
}


impl Tree {


    /*
    pub fn get_val(&self) -> i32 {
        return self.value;
    }

    pub fn set_val(&mut self, val: i32) -> i32 {
        self.value = val;
        return self.value;
    }

    pub fn insert(&mut self, dir: &String, val: Tree) {
        assert!(dir == "left" || dir == "right");
        match dir.as_ref() {
            "left" => self.left = Some(Box::new(val)),
            "right" => self.right = Some(Box::new(val)),
            _ => { 
                println!("Insert Error: only left and right supported");
                process::exit(1);
            }
        }
    }

    pub fn delete(&mut self, dir: &String) {
        assert!(dir == "left" || dir == "right");
        match dir.as_ref() {
                "left" => self.left = None,
                "right" => self.right = None,
                 _ => { 
                    println!("Insert Error: only left and right supported");
                    process::exit(1);
                }
        }
    }
    */
}


// 非消耗性遍历
// fn traverse(tree: &Tree) {
//     println!("Node Value: {:?}", tree.value);
//     match tree.left {
//         Some(ref x) => traverse(x),
//         _ => {}
//     }
//     match tree.right {
//         Some(ref x) => traverse(x),
//         _ => {}
//     }
// }

// 消耗性遍历：
// fn traverse(tree: Tree) {
//     println!("Node Value: {:?}", tree.value);
//     if tree.left.is_some() {
//         traverse(*tree.left.unwrap()); // 手动解引用
//     }
//     if tree.right.is_some() {
//         traverse(*tree.right.unwrap()); // 手动解引用
//     }
// }

// pub fn tree_main() {
//     println!("begin rust tree test:");
//     let mut tree = Tree { value : 12, ..Default::default() };
//     let mut left = Tree { value : 121, ..Default::default() };
//     tree.insert(&String::from("left"), left);
//     let mut right = Tree { value : 122, ..Default::default() };
//     tree.insert(&String::from("right"), right);
//     // tree.delete(&String::from("right"));
//     // println!("Tree val: {:?}", left.get_val()); 不能这样写，所有权已经被移动
//     traverse(&tree);
//     // traverse(tree);
// }