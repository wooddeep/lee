#[cfg(test)]
mod tests {
    use regex::{Regex, Captures, Error};
    use std::ops::Index;
    //use itertools::Itertools;
    pub use crate::lexer::shell;
    pub use crate::lexer;
    use std::any::Any;

    pub fn test_mod_embed() {
        shell::test();
        lexer::bare_func();
    }
    // this.token_patt = /\/\*.*\*\/|#[^\n]*\n|[_a-zA-Z][\-_a-zA-Z0-9]*|".*?"|\-?[0-9]+\.?[0-9]*[lf]*|[,:;\+\-\*/=\(\)\{\}\[\]]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|/gm
    // //this.token_patt = /([_a-zA-Z][\-_a-zA-Z0-9]*|".*?"|\-?[0-9]+(.[0-9]+)?[fl]?|[,:;+\-*/=\(\)\{\}]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|)/gm
    // this.curr_index = 0
    // this.token_list = []
    // this.formula = formula
    // this.analyze(this.formula) 

    struct Bird {
        wing: String,
    }

    struct Eagle {
        bird: Bird,
        view: f32,
    }


    trait Fly {
        fn as_any(&self) -> &dyn Any;
        // fn as_any(&self) -> &dyn Any {
        //     self
        // }

        fn bird_type(&self) -> i32;
    }

    impl Fly for Bird {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn bird_type(&self) -> i32 {
            0 // base type
        }
    }

    impl Fly for Eagle {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn bird_type(&self) -> i32 {
            1 // eagle
        }
    }

    fn show_bird(bird: &impl Fly) {
        if bird.bird_type() == 1 {
            let eagle = match bird.as_any().downcast_ref::<Eagle>() {
                Some(b) => b,
                None => panic!("&a isn't a B!")
            };
            println!("{}", eagle.bird.wing);
        }
    }

    //#[test] // cargo test -- --nocapture
    fn test() {
        let eagle: Eagle = Eagle { bird: Bird { wing: String::from("eagle's wing") }, view: 1000f32 };
        show_bird(&eagle);
    }

    //#[test]
    fn it_works() {  // cargo test -- --nocapture

        //let s = "/* coment */hello#abc def\nabc \"123\" \n -90.123f123,;:+-=*/abcd &&& ||";
        let s = r#""hello" + "word""#;
        let r = Regex::new(r#"(?s)/\*[^/]*\*/|#[^\n]*|[_a-zA-Z][\-_a-zA-Z0-9]*|"[^"]*"|\-?[0-9]+\.?[0-9]*[lf]*|[,:;\+\-\*/=\(\)\{\}\[\]]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|"#).unwrap();

        let regex_multi_comment = Regex::new(r#"(?s)/\*[^/]*\*/"#).unwrap();
        let regex_single_comment = Regex::new(r#"#[^\n]*"#).unwrap();
        let regex_identifier = Regex::new(r#"^[_a-zA-Z][\-_a-zA-Z0-9]*"#).unwrap();
        let regex_string = Regex::new(r#""[^"]*""#).unwrap();
        let regex_number = Regex::new(r#"\-?[0-9]+\.?[0-9]*[lf]*"#).unwrap();


        for (i, c) in r.captures_iter(&s).enumerate() {
            for j in 0..c.len() {
                if regex_multi_comment.is_match(&c[j]) {
                    println!("{} is mutiple line comment", &c[j]);
                    break;
                }

                if regex_single_comment.is_match(&c[j]) {
                    println!("{} is single line comment", &c[j]);
                    break;
                }

                if regex_identifier.is_match(&c[j]) {
                    println!("{} is identifier", &c[j]);
                    break;
                }

                if regex_string.is_match(&c[j]) {
                    println!("{} is string", &c[j]);
                    break;
                }

                if regex_number.is_match(&c[j]) {
                    println!("{} is number", &c[j]);
                    break;
                }

                match &c[j] {
                    "+" => println!("plus!"),
                    "-" => println!("substract"),
                    "*" => println!("multiple"),
                    "/" => println!("divid"),
                    "=" => println!("assign"),

                    _ => println!("group {},{} : {}", i, j, &c[j]),
                }
            }
        }
    }

    //#[test]
    fn vec_works() {  // cargo test -- --nocapture
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);

        /*
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);

        vec[0] = 7;
        assert_eq!(vec[0], 7);

        vec.extend([1, 2, 3].iter().copied());

        for x in &vec {
            println!("{}", x);
        }
        assert_eq!(vec, [7, 1, 2, 3]);
        */
    }

    use std::cell::*;

    #[derive(Debug)]
    struct Foo {
        age: i32,
        name: String,
        next: Option<Box<Foo>>,
    }

    struct Bar {
        foo: Option<Foo>,
    }

    use std::thread;
    use std::rc::Rc;
    use std::borrow::{Borrow, BorrowMut};

    //#[test]
    fn test_cell() {  // cargo test -- --nocapture

        // let cell = Cell::new(1); // cell 不需要声明为 mut
        // println!("{}", cell.get());
        // cell.set(2);
        // println!("{}", cell.get());

        // //相对于 Cell 只能包裹实现了 Copy 的类型，RefCell 用于更普遍的情况（其它情况都用 RefCell）
        // let ref_cell = RefCell::new(Foo{age: 38, name: String::from("lee")});
        // println!("{:?}", ref_cell.borrow().name);
        // ref_cell.borrow_mut().name = String::from("lihan");
        // ref_cell.borrow_mut().name = String::from("lihan1");
        // println!("{:?}", ref_cell.borrow().name);

        // 运行期错误
        // let c = RefCell::new(5);
        // let b = c.borrow(); // this causes a panic
        // println!("{:?}", b);
        // let m = c.borrow_mut();
        // println!("{:?}", m);

        // // correct
        // let sent_messages: RefCell<Vec<String>> = RefCell::new(Vec::new());
        // sent_messages.borrow_mut().push(String::from("message"));
        // sent_messages.borrow_mut().push(String::from("message"));

        // error
        // let sent_messages: RefCell<Vec<String>> = RefCell::new(Vec::new());
        // let mut one_borrow = sent_messages.borrow_mut();
        // let mut two_borrow = sent_messages.borrow_mut();
        // one_borrow.push(String::from("message"));
        // two_borrow.push(String::from("message"));

        // Rc
        // Rc 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点：
        //
        // 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
        // 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
        // Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
        // Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。

        // Rc结合 RefCell
        // let sent_messages: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        // let mut one_borrow = Rc::clone(&sent_messages);
        // let mut two_borrow = Rc::clone(&sent_messages);
        // one_borrow.borrow_mut().push(String::from("message0"));
        // println!("{:?}", sent_messages);
        // two_borrow.borrow_mut().push(String::from("message1"));
        // println!("{:?}", sent_messages.borrow());
        //
        // let s = String::from("raw string0!");
        // let mut s = String::from("raw string1!");
        // let rs = &mut s;
        // let mut rs = &mut String::from("raw string1!");
        // rs.push_str("abc");
    }

    fn call_ref(input: &Foo) {
        println!("## call ref: {:?}", input);
        //call_ref(input);
    }

    //#[test]
    fn test_option() {  // cargo test -- --nocapture
        let data = &Bar { foo: Some(Foo { age: 38, name: String::from("lee"), next: None }) };
        call_ref(data.foo.as_ref().unwrap().borrow());
    }

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
        z: String,
    }

    impl Point {
        fn bar(&mut self, msg: &String) {
            println!("{:?}", self)
        }

        fn foo(&mut self) {
            println!("{:?}", self.z)
        }
    }

    #[test]
    fn yy_test() {
        let mut p = Point { x: 1.0, y: 3.0, z: String::from("a") };
        let mut pp = &mut p;

        //pp.bar(& pp.z);

        pp.foo();
        pp.foo();
    }

    fn negate(p: Point) -> Point {
        Point {
            x: -p.x,
            y: -p.y,
            z: String::from("lee"),
        }
    }

    //#[test]
    fn xx_test() {
        let mut p = Point { x: 1.0, y: 3.0, z: String::from("a") };
        //p.x = 2f64;
        let p_ref = &p;
        //print!("{:?}", *p_ref);
        //negate(*p_ref);
        // error: cannot move out of `*p_ref` which is behind a shared reference
        let b = p;
    }

    #[test]
    fn rc_test() {
        //let mut v: Rc<RefCell<_>> = Rc::new(RefCell::new(1));
        //v.borrow_mut().replace(2);
        //println!("v: {}", v.into_inner());

        //let v: Rc<_> = Rc::new(1);   // rc 用户共享，记住链表的例子
        //println!("v: {}", v.clone());

        let v = RefCell::new(1);
        println!("v: {}", v.into_inner());

    }
}

