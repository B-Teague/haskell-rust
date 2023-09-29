use nom::{
    IResult,
    Parser,
    branch::*,
    bytes::complete::{tag, take},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, none_of},
    combinator::{map, recognize, value},
    multi::many0,
    sequence::{delimited, preceded, pair},
};


use std::str;

pub mod token;
use crate::lexer::token::*;

macro_rules! syntax_tag {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a str) -> IResult<&str, Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

macro_rules! syntax_char {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a str) -> IResult<&str, Token> {
            map(char($tag_string), |_| $output_token)(s)
        }
    };
}

// operators
syntax_tag! {equal_operator, "==", Token::Equal}
syntax_tag! {not_equal_operator, "!=", Token::NotEqual}
syntax_char! {assign_operator, '=', Token::Assign}
syntax_char! {plus_operator, '+', Token::Plus}
syntax_char! {minus_operator, '-', Token::Minus}
syntax_char! {multiply_operator, '*', Token::Multiply}
syntax_char! {divide_operator, '/', Token::Divide}
syntax_char! {not_operator, '!', Token::Not}
syntax_tag! {greater_operator_equal, ">=", Token::GreaterThanEqual}
syntax_tag! {lesser_operator_equal, "<=", Token::LessThanEqual}
syntax_char! {greater_operator, '>', Token::GreaterThan}
syntax_char! {lesser_operator, '<', Token::LessThan}

pub fn lex_operator(input: &str) -> IResult<&str, Token> {
    alt((
        equal_operator,
        not_equal_operator,
        assign_operator,
        plus_operator,
        minus_operator,
        multiply_operator,
        divide_operator,
        not_operator,
        greater_operator_equal,
        lesser_operator_equal,
        greater_operator,
        lesser_operator,
    ))(input)
}

// punctuations
syntax_char! {comma_punctuation, ',', Token::Comma}
syntax_char! {semicolon_punctuation, ';', Token::SemiColon}
syntax_char! {colon_punctuation, ':', Token::Colon}
syntax_char! {lparen_punctuation, '(', Token::LParen}
syntax_char! {rparen_punctuation, ')', Token::RParen}
syntax_char! {lbrace_punctuation, '{', Token::LBrace}
syntax_char! {rbrace_punctuation, '}', Token::RBrace}
syntax_char! {lbracket_punctuation, '[', Token::LBracket}
syntax_char! {rbracket_punctuation, ']', Token::RBracket}

pub fn lex_punctuations(input: &str) -> IResult<&str, Token> {
    alt((
        comma_punctuation,
        semicolon_punctuation,
        colon_punctuation,
        lparen_punctuation,
        rparen_punctuation,
        lbrace_punctuation,
        rbrace_punctuation,
        lbracket_punctuation,
        rbracket_punctuation,
    ))(input)
}

fn string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(
            many0(preceded(char('\\'), char('"')).or(none_of("\""))),
            |chars: Vec<char>| chars.into_iter().collect(),
        ),
        char('"'),
    )(input)
}

fn lex_string(input: &str) -> IResult<&str, Token> {
    map(string, Token::StringLiteral)(input)
}

// Reserved or ident
fn lex_reserved_ident(input: &str) -> IResult<&str, Token> {
    alt((
        value(Token::Let, tag("let")),
        value(Token::Function, tag("fn")),
        value(Token::If, tag("if")),
        value(Token::Else, tag("else")),
        value(Token::Return, tag("return")),
        value(Token::BoolLiteral(true), tag("true")),
        value(Token::BoolLiteral(false), tag("false")),
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |s: &str| Token::Ident(s.to_string()),
        ),
    ))(input)
}

// Integers parsing
fn lex_integer(input: &str) -> IResult<&str, Token> {
    map(digit1, |s: &str| {
        Token::IntLiteral(s.parse::<i64>().unwrap())
    })(input)
}

// Illegal tokens
fn lex_illegal(input: &str) -> IResult<&str, Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        lex_operator,
        lex_punctuations,
        lex_string,
        lex_reserved_ident,
        lex_integer,
        lex_illegal,
    ))(input)
}

fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
        lex_tokens(input)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = "=+(){},;";
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer2() {
        let input = "let five = 5;\
             let ten = 10;\
             let add = fn(x, y) {\
                 x + y;\
             };\
             let result = add(five, ten);";

        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::IntLiteral(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_owned()),
            Token::Plus,
            Token::Ident("y".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RParen,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer3() {
        let input = "if (a == 10) {\
                return a;\
             } else if (a != 20) {\
                return !a;\
            } else if (a > 20) {\
                return -30 / 40 * 50;\
            } else if (a < 30) {\
                return true;\
            }\
            return false;\
            ";

        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Equal,
            Token::IntLiteral(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::NotEqual,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Not,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::GreaterThan,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Minus,
            Token::IntLiteral(30),
            Token::Divide,
            Token::IntLiteral(40),
            Token::Multiply,
            Token::IntLiteral(50),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::LessThan,
            Token::IntLiteral(30),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::BoolLiteral(true),
            Token::SemiColon,
            Token::RBrace,
            Token::Return,
            Token::BoolLiteral(false),
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn string_literals() {
        let (_, result) = Lexer::lex_tokens("\"foobar\"").unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foobar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens("\"foo bar\"").unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo bar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens("\"foo\nbar\"").unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\nbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens("\"foo\tbar\"").unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\tbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens("\"foo\\\"bar\"").unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\"bar".to_owned()), Token::EOF]
        );

        let (_, result) =
            Lexer::lex_tokens("\"foo\\\"bar with \u{1F496} emojis\"").unwrap();
        assert_eq!(
            result,
            vec![
                Token::StringLiteral("foo\"bar with 💖 emojis".to_owned()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn id_with_numbers() {
        let (_, result) = Lexer::lex_tokens("hello2 hel301oo120").unwrap();
        let expected = vec![
            Token::Ident("hello2".to_owned()),
            Token::Ident("hel301oo120".to_owned()),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn array_tokens() {
        let (_, result) = Lexer::lex_tokens("[1, 2];").unwrap();
        let expected = vec![
            Token::LBracket,
            Token::IntLiteral(1),
            Token::Comma,
            Token::IntLiteral(2),
            Token::RBracket,
            Token::SemiColon,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn hash_tokens() {
        let (_, result) = Lexer::lex_tokens("{\"hello\": \"world\"}").unwrap();
        let expected = vec![
            Token::LBrace,
            Token::StringLiteral("hello".to_owned()),
            Token::Colon,
            Token::StringLiteral("world".to_owned()),
            Token::RBrace,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }
}
