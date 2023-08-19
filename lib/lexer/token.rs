#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    IntLiteral(&'a str),
    LeftParen,
    RightParen,
    Plus,
    Minus,
    EOF,
}

#[derive(PartialEq, Debug)]
pub struct Tokens<'a> {
    tokens: &'a [Token<'a>],
    index: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tokens: vec,
            index: 0
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a Token<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            self.index += 1;
            return Some(&self.tokens[self.index - 1]);
        }
        return None;
    }
}