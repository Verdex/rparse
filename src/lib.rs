
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
    }
}

fn lookup_apply(rule : &str, rules : &HashMap<String, ParseRule>, input : &mut Input) -> Result<Data, ()> {
    let x = rules.get(rule).expect(&format!("Encountered unknown rule: {}", rule));
    apply(x, rules, input)
}

pub fn parse(start_rule : &str, rules : &HashMap<String, ParseRule>, input : &str) -> Result<Data, ()> {
    lookup_apply(start_rule, &rules, &mut Input::new(input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_any() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("any".to_string(), ParseRule::Any);

        let data = parse("any", &rules, "string")?;

        assert!( matches!(data, Data::Char('s') ) );

        Ok(())
    }

    #[test]
    fn should_parse_string() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("match_string".to_string(), ParseRule::MatchString("this[]".to_string()));

        let data = parse("match_string", &rules, "this[]")?;

        assert!( matches!(data, Data::Nil ) );

        Ok(())
    }

    #[test]
    fn should_parse_string_with_extra() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("match_string".to_string(), ParseRule::MatchString("this[]".to_string()));

        let data = parse("match_string", &rules, "this[]blah")?;

        assert!( matches!(data, Data::Nil ) );

        Ok(())
    }

    #[test]
    fn should_parse_any_and_any() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("any".to_string(), ParseRule::And(vec! [ ParseRule::Any, ParseRule::Any ]));

        let data = parse("any", &rules, "this[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 2);
                assert!( matches!( list[0], Data::Char('t')));
                assert!( matches!( list[1], Data::Char('h')));
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_any_or_any() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("any".to_string(), ParseRule::Or(vec! [ ParseRule::Any, ParseRule::Any ]));

        let data = parse("any", &rules, "this[]blah")?;

        assert!( matches!( data, Data::Char('t')));

        Ok(())
    }


    #[test]
    fn should_parse_zero_zero_or_more_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("zero_or_more".to_string(), ParseRule::ZeroOrMore(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("zero_or_more", &rules, "this[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 0);
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_more_zero_or_more_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("zero_or_more".to_string(), ParseRule::ZeroOrMore(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("zero_or_more", &rules, "aathis[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 2);
                assert!(matches!(list[0], Data::Nil));
                assert!(matches!(list[1], Data::Nil));
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_one_one_or_more_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("one_or_more".to_string(), ParseRule::OneOrMore(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("one_or_more", &rules, "athis[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 1);
                assert!(matches!(list[0], Data::Nil));
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_more_one_or_more_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("zero_or_more".to_string(), ParseRule::OneOrMore(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("zero_or_more", &rules, "aathis[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 2);
                assert!(matches!(list[0], Data::Nil));
                assert!(matches!(list[1], Data::Nil));
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_zero_zero_or_one_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("zero_or_one".to_string(), ParseRule::ZeroOrOne(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("zero_or_one", &rules, "this[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 0);
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_one_zero_or_one_a() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("zero_or_one".to_string(), ParseRule::ZeroOrOne(Box::new(ParseRule::MatchString("a".to_string()))));

        let data = parse("zero_or_one", &rules, "athis[]blah")?;

        match data {
            Data::Table { list, .. } => {
                assert_eq!(list.len(), 1);
                assert!(matches!(list[0], Data::Nil));
            },
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn should_parse_invoke() -> Result<(), ()> {
        let mut rules = HashMap::new();

        rules.insert("any".to_string(), ParseRule::Any);
        rules.insert("invoke".to_string(), ParseRule::InvokeRule("any".to_string()));

        let data = parse("invoke", &rules, "athis[]blah")?;

        match data {
            Data::Field(f) => {
                assert_eq!(f.rule, "any");
                assert!( matches!(f.data, Data::Char('a')));
            },
            _ => assert!(false),
        }

        Ok(())
    }
}