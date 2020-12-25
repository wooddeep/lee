pub mod lexer;   // 文件夹名 作为模块名, 文件夹内模块文件的名称必须为mod.rs
pub use crate::lexer::shell;

//pub use crate::dir_as_mod_name::bare_func;

pub use crate::lexer::*;

pub fn eat_at_restaurant() {
    //hosting::add_to_waitlist();
    //hosting::add_to_waitlist();
    //hosting::add_to_waitlist();
    //inner::test();
    shell::test();
    lexer::bare_func(); 
}

mod test; // 文件名 作为模块名, 测试相关程序放在 test.rs里面


/*
    curr_index: usize,
    token_list: Vec<Token>,
    formula: String,
*/

fn main() {
    //eat_at_restaurant();
    //println!("Hello, world!");

    let mut lexer = Lexer{curr_index: 0, token_list: Vec::new(), formula: String::from("/* coment */hello#abc def\nabc \"123\" \n -90.123f123,;:+-=*/abcd &&& ||")};

    lexer.analyze();

    for element in &lexer.token_list {
        println!("{}", element.literal)
    }
}

