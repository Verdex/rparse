
#[derive(Debug, Clone)]
pub enum Data {
    Nil,
    Char(char),
    Field { rule : String, data: Box<Data> },
    List(Vec<Data>),
}

impl Data {
    pub fn find(&self, test : fn(&Data) -> bool) -> Vec<Data> {
        let mut results : Vec<Vec<Data>> = vec![];
        match self {
            it @ Data::Nil if test(it) => results.push( vec![it.clone()] ),
            it @ Data::Char(_) if test(it) => results.push( vec![it.clone()] ), 
            it @ Data::Field { .. } if test(it) => {
                results.push( vec![it.clone()] );

                let data = match it {
                    Data::Field { data, .. } => data,
                    _ => panic!("Data::find expects Field"),
                };

                results.push( data.find(test) );
            },
            it @ Data::List(_) if test(it) => {
                results.push( vec![it.clone()] );
                let list = match it {
                    Data::List(list) => list,
                    _ => panic!("Data::find expects Table"),
                };
                for l in list {
                    results.push( l.find(test) );
                }
            },
            Data::Field { data, .. } => results.push( data.find(test) ),
            Data::List(list) => {
                for l in list {
                    results.push( l.find(test) );
                }
            },
            _ => { },
        }
        results.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_find_nil() {
        let data = Data::Nil;
        let results = data.find(|d| match d {
            Data::Nil => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Nil ) );
    }

    #[test]
    fn should_find_char() {
        let data = Data::Char('c');
        let results = data.find(|d| match d {
            Data::Char(_) => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Char('c') ) );
    }

    #[test]
    fn should_find_specific_char() {
        let data = Data::Char('c');
        let results = data.find(|d| match d {
            Data::Char('c') => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Char('c') ) );
    }

    #[test]
    fn should_find_field() {
        let data = Data::Field(Box::new( Field {rule: "rule".to_string(), data: Data::Nil} ));
        let results = data.find(|d| match d {
            Data::Field { .. } => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Field { .. } ) );
    }

    #[test]
    fn should_find_table() {
        let data = Data::Table{ list: vec![], structure: vec![] };
        let results = data.find(|d| match d {
            Data::Table { .. } => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Table { .. } ) );
    }

    #[test]
    fn should_find_table_nested_nil() {
        let data = Data::Table { list: vec![Data::Nil], structure: vec![] };
       
        let results = data.find(|d| match d {
            Data::Nil => true,
            _ => false,
        });

        assert_eq!( results.len(), 1 );
        assert!( matches!( results[0], Data::Nil ));
    }
}