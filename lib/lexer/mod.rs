pub mod token;
use crate::lexer::token::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};
use std::iter::once;

fn plus_operator<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(input)
}
fn minus_operator<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag("-"), |_| Token::Minus)(input)
}
fn left_paren<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag("("), |_| Token::LeftParen)(input)
}
fn right_paren<'a>(input: &'a str) -> IResult<&str, Token> {
    map(tag(")"), |_| Token::RightParen)(input)
}

fn int_token<'a>(input: &'a str) -> IResult<&str, Token> {
    map(digit1, Token::IntLiteral)(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        plus_operator,
        minus_operator,
        int_token,
        left_paren,
        right_paren,
    ))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
        let tokens = terminated(many0(preceded(multispace0, lex_token)), multispace0)(input);
        tokens.map(|(slice, result)| (slice, result.into_iter().chain(once(Token::EOF)).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = " 3\t+ (-2) \n";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::IntLiteral("3"),
            Token::Plus,
            Token::LeftParen,
            Token::Minus,
            Token::IntLiteral("2"),
            Token::RightParen,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer2() {
        let input = "3--2";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::IntLiteral("3"),
            Token::Minus,
            Token::Minus,
            Token::IntLiteral("2"),
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }
}
