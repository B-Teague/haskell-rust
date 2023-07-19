pub mod token;
use crate::lexer::token::*;

use nom::*;
use nom::bytes::complete::{tag};
use nom::character::complete::{multispace0}; 
use nom::combinator::{map};

fn plus_operator<'a>(s: &'a str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(s)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        plus_operator,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}
