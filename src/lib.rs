
use std::str::CharIndices;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Input<'a> {
    cs : CharIndices<'a>,
}

impl<'a> Input<'a> {

    fn new(s : &'a str) -> Input<'a> {
        Input { cs : s.char_indices() }
    }

    fn get_char(&mut self) -> Result<char, ()> {
        match self.cs.next() {
            Some((_,c)) => Ok(c),
            None => Err(()),
        }
    }

    fn match_string(&mut self, s : &str) -> Result<(), ()> {
        let mut n = self.cs.clone();

        for c in s.chars() {
            match n.next() {
                Some((_, target)) if c == target => { }, 
                Some(_) => return Err(()),
                None => return Err(()),
            }
        }

        self.cs = n;
        Ok(())
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_get_char() {
        let mut input = Input::new("string");

        let c = input.get_char().expect("Should be able to get 's'");
        assert_eq!( 's', c );

        let c = input.get_char().expect("Should be able to get 't'");
        assert_eq!( 't', c );

        let c = input.get_char().expect("Should be able to get 'r'");
        assert_eq!( 'r', c );

        let c = input.get_char().expect("Should be able to get 'i'");
        assert_eq!( 'i', c );

        let c = input.get_char().expect("Should be able to get 'n'");
        assert_eq!( 'n', c );

        let c = input.get_char().expect("Should be able to get 'g'");
        assert_eq!( 'g', c );
    }

    #[test]
    fn match_string_failure_should_not_change_index() {

    }

    #[test]
    fn match_string_success_should_change_index() {

    }

}
