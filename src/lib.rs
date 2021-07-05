
use std::collections::HashMap;

mod input;

use input::Input;

pub enum ParseRule {
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

pub struct Field {
    pub rule : String,
    pub data : Data,
}

pub enum Data {
    Nil,
    Char(char),
    Field(Box<Field>),
    Table { list : Vec<Data>, structure : Vec<Field> },
}

fn data_field(rule : &str, data : Data) -> Result<Data, ()> {
    let rule = rule.to_string();
    Ok(Data::Field( Box::new(Field { rule, data })))
}

fn apply(rule : &ParseRule, rules : &HashMap<String, ParseRule>, input : &mut Input) -> Result<Data, ()> {
    match rule {
        ParseRule::Any => Ok(Data::Char(input.get_char()?)),
        ParseRule::MatchString(target) => {
            input.match_string(target)?;
            Ok(Data::Nil)
        },
        ParseRule::InvokeRule(target_rule) => data_field(target_rule, lookup_apply(target_rule, rules, input)?),
        _ => Err(()),
    }
}

fn lookup_apply(rule : &str, rules : &HashMap<String, ParseRule>, input : &mut Input) -> Result<Data, ()> {
    let x = rules.get(rule).expect(&format!("Encountered unknown rule: {}", rule));
    apply(x, rules, input)
}

pub fn parse(start_rule : &str, rules : HashMap<String, ParseRule>, input : &str) -> Result<Data, ()> {
    lookup_apply(start_rule, &rules, &mut Input::new(input))
}