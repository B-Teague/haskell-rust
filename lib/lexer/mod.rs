pub mod token;
use crate::lexer::token::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};
use std::iter::once;

macro_rules! single_char_token_parser {
    ($name:ident, $char:expr, $variant:expr) => {
        fn $name<'a>(input: &'a str) -> IResult<&str, Token> {
            map(tag($char), |_| $variant)(input)
        }
    };
}

single_char_token_parser!(plus_operator, "+", Token::Plus);
single_char_token_parser!(minus_operator, "-", Token::Minus);
single_char_token_parser!(left_paren, "(", Token::LeftParen);
single_char_token_parser!(right_paren, ")", Token::RightParen);


fn num_token<'a>(input: &'a str) -> IResult<&str, Token> {
    map(digit1, Token::Num)(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        plus_operator,
        minus_operator,
        num_token,
        left_paren,
        right_paren,
    ))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
        let tokens = many0(delimited(multispace0, lex_token, multispace0))(input);
        tokens.map(|(slice, result)| (slice, result.into_iter().chain(once(Token::EOF)).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = "3+(-2)";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Num("3"),
            Token::Plus,
            Token::LeftParen,
            Token::Minus,
            Token::Num("2"),
            Token::RightParen,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }
}
