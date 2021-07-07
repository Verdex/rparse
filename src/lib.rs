
use std::collections::HashMap;

mod input;

use input::Input;

pub enum ParseRule {
    Any,                                                            // Char(char) 
    MatchString(String),                                            // NIL 
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
        ParseRule::ZeroOrMore(target_rule) => {
            let mut datas = vec![];
            loop {
                match apply(target_rule, rules, input) {
                    Ok(data) => datas.push(data),
                    Err(_) => break,
                }
            }
            Ok(Data::Table { list: datas, structure: vec![] })
        },
        ParseRule::OneOrMore(target_rule) => {
            let mut datas = vec![];

            let data = apply(target_rule, rules, input)?;
            
            datas.push(data);

            loop {
                match apply(target_rule, rules, input) {
                    Ok(data) => datas.push(data),
                    Err(_) => break,
                }
            }
            Ok(Data::Table { list: datas, structure: vec![] })
        },
        ParseRule::ZeroOrOne(target_rule) => {
            match apply(target_rule, rules, input) {
                Ok(data) => Ok(Data::Table { list: vec![data], structure: vec![] }),
                Err(_) => Ok(Data::Table { list: vec![], structure: vec![] }),
            }
        },
        ParseRule::Or(target_rules) => {
            for target_rule in target_rules {
                match apply(target_rule, rules, input) {
                    Ok(data) => return Ok(data),
                    Err(_) => { },
                }
            }
            Err(())
        },
        ParseRule::And(target_rules) => {
            let rp = input.restore_point();
            let mut list = vec![];
            let mut structure = vec![]; 
            for target_rule in target_rules {
                match apply(target_rule, rules, input) {
                    Ok(Data::Field(field)) => structure.push(*field),
                    Ok(data) => list.push(data),
                    Err(_) => {
                        input.restore(rp);
                        return Err(());
                    },
                }
            }
            Ok(Data::Table {list, structure})
        },
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