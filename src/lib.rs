
use std::collections::HashMap;

mod input;

use input::Input;

enum ParseRule {
    Any,                                                            // Char(char) 
    MatchString(String),                                            // ()
    InvokeRule(String),                                             // Field
    ZeroOrMore(Box<ParseRule>),                                     // Table { list }
    OneOrMore(Box<ParseRule>),                                      // Table { list }
    ZeroOrOne(Box<ParseRule>),                                      // Table { list }
    Or(Vec<ParseRule>),                                             // Data
    And(Vec<ParseRule>),                                            // Table { list, structure }
    Until { target : Box<ParseRule>, end : Box<ParseRule> },        // Table { list }
}

struct Field {
    rule : String,
    data : Data,
}

enum Data {
    Char(char),
    Table { list : Vec<Data>, structure : Vec<Field> },
}

fn parse(rule : &str, rules : HashMap<String, ParseRule>, input : &mut Input) -> Result<Data, ()> {
    let x = rules.get(rule).unwrap();

    match x {
        ParseRule::Any => Ok(Data::Char(input.get_char()?)),
        _ => Err(()),
    }
}

fn main() {
    println!("Hello, world!");
}
