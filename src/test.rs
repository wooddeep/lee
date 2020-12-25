
#[cfg(test)]
mod tests {

    use regex::{Regex, Captures, Error};
    use std::ops::Index;
    //use itertools::Itertools;


    // this.token_patt = /\/\*.*\*\/|#[^\n]*\n|[_a-zA-Z][\-_a-zA-Z0-9]*|".*?"|\-?[0-9]+\.?[0-9]*[lf]*|[,:;\+\-\*/=\(\)\{\}\[\]]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|/gm
    // //this.token_patt = /([_a-zA-Z][\-_a-zA-Z0-9]*|".*?"|\-?[0-9]+(.[0-9]+)?[fl]?|[,:;+\-*/=\(\)\{\}]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|)/gm
    // this.curr_index = 0
    // this.token_list = []
    // this.formula = formula
    // this.analyze(this.formula) 


    //#[test]
    fn it_works(){  // cargo test -- --nocapture 
        
        let s = "/* coment */hello#abc def\nabc \"123\" \n -90.123f123,;:+-=*/abcd &&& ||";
        let r = Regex::new(r#"(?s)/\*[^/]*\*/|#[^\n]*|[_a-zA-Z][\-_a-zA-Z0-9]*|".*"|\-?[0-9]+\.?[0-9]*[lf]*|[,:;\+\-\*/=\(\)\{\}\[\]]|>|<|>=|<=|==|!=|>>|<<|&&|\|\|"#).unwrap();

        let regex_multi_comment = Regex::new(r#"(?s)/\*[^/]*\*/"#).unwrap();
        let regex_single_comment = Regex::new(r#"#[^\n]*"#).unwrap();
        let regex_identifier = Regex::new(r#"^[_a-zA-Z][\-_a-zA-Z0-9]*"#).unwrap();
        let regex_string = Regex::new(r#"".*""#).unwrap();
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

    #[test]
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

}

