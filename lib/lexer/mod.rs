pub mod token;
use crate::lexer::token::*;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0; 
use nom::combinator::map;
use nom::sequence::delimited;
use nom::branch::alt;
use nom::multi::many0;

fn plus_operator<'a>(s: &'a str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(s)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        plus_operator,
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
            (slice, result.into_iter().chain(std::iter::once(Token::EOF)).collect())
    })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = " + ";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Plus,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }
}