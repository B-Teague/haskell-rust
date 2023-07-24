pub mod token;
use crate::lexer::token::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, digit1},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};
use std::iter::once;

fn plus_operator<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(input)
}

fn minus_operator<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag("-"), |_| Token::Minus)(input)
}

fn num_token<'a>(input: &'a str) -> IResult<&str, Token> {
    map(digit1, Token::Num)(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        plus_operator,
        minus_operator,
        num_token,
    ))(input)
}

// fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
//     many0(delimited(multispace0, lex_token, multispace0))(input)
// }

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
        let tokens = many0(delimited(multispace0, lex_token, multispace0))(input);
        tokens.map(|(slice, result)| {
            (
                slice,
                result
                    .into_iter()
                    .chain(once(Token::EOF))
                    .collect(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = "3+-2";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Num("3"),
            Token::Plus, 
            Token::Minus,
            Token::Num("2"),
            Token::EOF];

        assert_eq!(result, expected_results);
    }
}
