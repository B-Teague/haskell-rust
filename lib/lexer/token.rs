#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Num(&'a str),
    Plus,
    Minus,
    EOF,
}

pub struct Tokens<'a> {
    pub tok: &'a [Token<'a>],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}