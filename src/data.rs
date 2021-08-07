
#[derive(Debug, Clone)]
pub struct Field {
    pub rule : String,
    pub data : Data,
}

#[derive(Debug, Clone)]
pub enum Data {
    Nil,
    Char(char),
    Field(Box<Field>),
    Table { list : Vec<Data>, structure : Vec<Field> },
}

impl Data {
    pub fn find(&self, test : fn(&Data) -> bool) -> Vec<Data> {
        let mut results : Vec<Vec<Data>> = vec![];
        match self {
            it @ Data::Nil if test(it) => results.push( vec![it.clone()] ),
            it @ Data::Char(_) if test(it) => results.push( vec![it.clone()] ), 
            it @ Data::Field(_) if test(it) => {
                results.push( vec![it.clone()] );
                let field = match it {
                    Data::Field(f) => f,
                    _ => panic!("Data::find expects Field"),
                };
                results.push( field.data.find(test) );
            }
            it @ Data::Table { .. } if test(it) => {
                results.push( vec![it.clone()] );
                let (list, structure) = match it {
                    Data::Table { list, structure } => (list, structure),
                    _ => panic!("Data::find expects Table"),
                };
                for l in list {
                    results.push( l.find(test) );
                }
                for s in structure {
                    results.push( s.data.find(test) );
                }
            }
            _ => { },
        }
        results.into_iter().flatten().collect()
    }
}