
use std::str::CharIndices;

pub struct Input<'a> {
    cs : CharIndices<'a>,
}

pub struct RestorePoint<'a> {
    cs : CharIndices<'a>,
}

impl<'a> Input<'a> {

    pub fn new(s : &'a str) -> Input<'a> {
        Input { cs : s.char_indices() }
    }

    pub fn restore_point(&self) -> RestorePoint<'a> {
        RestorePoint { cs: self.cs.clone() }
    }

    pub fn restore(&mut self, rp : RestorePoint<'a>) {
        self.cs = rp.cs;
    }

    pub fn get_char(&mut self) -> Result<char, ()> {
        match self.cs.next() {
            Some((_,c)) => Ok(c),
            None => Err(()),
        }
    }

    pub fn match_string(&mut self, s : &str) -> Result<(), ()> {
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
        let mut input = Input::new("string");

        let result = input.match_string("yy");

        assert!( matches!( result, Err(_) ) );

        let result = input.match_string("string");

        assert!( matches!( result, Ok(_) ) );
    }

    #[test]
    fn match_string_success_should_change_index() {
        let mut input = Input::new("string");

        let result = input.match_string("st");

        assert!( matches!( result, Ok(_) ) );

        let result = input.match_string("ring");

        assert!( matches!( result, Ok(_) ) );
    }
}