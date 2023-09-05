pub mod ast;

use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, verify},
    error::{Error, ErrorKind},
    error_position,
    multi::many0,
    sequence::{delimited, terminated},
    Err, IResult,
};

use crate::lexer::token::*;
use crate::parser::ast::*;

fn eof_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tokens[0] == Token::EOF)(tokens)
}

fn plus_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tokens[0] == Token::Plus)(tokens)
}

fn minus_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tokens[0] == Token::Minus)(tokens)
}

fn left_paren_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tokens[0] == Token::LeftParen)(tokens)
}

fn right_paren_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tokens[0] == Token::RightParen)(tokens)
}

fn parse_literal(input: Tokens) -> IResult<Tokens, Literal> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tokens.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tokens[0] {
            Token::IntLiteral(name) => {
                let parsed_int: Result<i64, _> = name.parse();
                match parsed_int {
                    Ok(num) => Ok((i1, Literal::Int(num))),
                    Err(_) => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
                }
            }
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn parse_lit_expr(input: Tokens) -> IResult<Tokens, Expr> {
    map(parse_literal, Expr::Literal)(input)
}

fn parse_prefix_expr(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, t1) = alt((plus_tag, minus_tag))(input)?;
    if t1.tokens.is_empty() {
        Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    } else {
        let (i2, e) = parse_atom_expr(i1)?;
        match t1.tokens[0].clone() {
            Token::Plus => Ok((i2, Expr::Prefix(Prefix::Positive, Box::new(e)))),
            Token::Minus => Ok((i2, Expr::Prefix(Prefix::Negative, Box::new(e)))),
            _ => Err(Err::Error(error_position!(input, ErrorKind::Tag))),
        }
    }
}

fn parse_paren_expr(input: Tokens) -> IResult<Tokens, Expr> {
    delimited(left_paren_tag, parse_expr, right_paren_tag)(input)
}

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expr> {
    alt((parse_lit_expr, parse_prefix_expr, parse_paren_expr))(input)
}

fn infix_op(t: &Token) -> (Precedence, Option<Infix>) {
    match *t {
        Token::Plus => (Precedence::PSum, Some(Infix::Plus)),
        Token::Minus => (Precedence::PSum, Some(Infix::Minus)),
        _ => (Precedence::PLowest, None),
    }
}

fn parse_infix_expr(input: Tokens, left: Expr) -> IResult<Tokens, Expr> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tokens.is_empty() {
        Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    } else {
        let next = &t1.tokens[0];
        let (precedence, maybe_op) = infix_op(next);
        match maybe_op {
            None => Err(Err::Error(error_position!(input, ErrorKind::Tag))),
            Some(op) => {
                let (i2, right) = parse_pratt_expr(i1, precedence)?;
                Ok((i2, Expr::Infix(Box::new(left), op, Box::new(right))))
            }
        }
    }
}

fn go_parse_pratt_expr(input: Tokens, precedence: Precedence, left: Expr) -> IResult<Tokens, Expr> {
    let (i1, t1) = take(1usize)(input)?;

    if t1.tokens.is_empty() {
        Ok((i1, left))
    } else {
        let preview = &t1.tokens[0];
        let p = infix_op(preview);
        match p {
            (ref peek_precedence, _) if precedence < *peek_precedence => {
                let (i2, left2) = parse_infix_expr(input, left)?;
                go_parse_pratt_expr(i2, precedence, left2)
            }
            _ => Ok((input, left)),
        }
    }
}

fn parse_pratt_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expr> {
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_pratt_expr(input, Precedence::PLowest)
}

fn parse_expr_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
    map(parse_expr, |expr| Stmt::ExprStmt(expr))(input)
}

fn parse_program(input: Tokens) -> IResult<Tokens, Program> {
    terminated(many0(parse_expr_stmt), eof_tag)(input)
}

// fn parse_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
//     alt(parse_expr_stmt)(input)
// }
pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> IResult<Tokens, Program> {
        parse_program(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    fn assert_input_with_program(input: &str, expected_results: Program) {
        let (_, r) = Lexer::lex_tokens(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parse_tokens(tokens).unwrap();
        assert_eq!(result, expected_results);
    }

    fn compare_inputs(input: &str, input2: &str) {
        let (_, r) = Lexer::lex_tokens(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parse_tokens(tokens).unwrap();

        let (_, r) = Lexer::lex_tokens(input2).unwrap();
        let tokens = Tokens::new(&r);
        let (_, expected_results) = Parser::parse_tokens(tokens).unwrap();

        assert_eq!(result, expected_results);
    }

    #[test]
    fn empty() {
        assert_input_with_program("", vec![]);
    }

    #[test]
    fn operator_precedence() {
        let input = "3 - 1 + 4 - 5";
        let input2 = "(((3 - 1) + 4) - 5)";
        compare_inputs(input, input2);
    }

    #[test]
    fn minus_and_unary_operators() {
        let input = "3--1";
        assert_input_with_program(
            input,
            vec![Stmt::ExprStmt(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(3))),
                Infix::Minus,
                Box::new(Expr::Prefix(
                    Prefix::Negative,
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ))],
        );
    }

    #[test]
    fn minus_and_unary_operators2() {
        let input = "3+-5";
        let input2 = "(3 + (- 5))";
        compare_inputs(input, input2);
    }

    #[test]
    fn too_many_operators() {
        let input = "3+++5";
        let input2 = "(3 + (+ (+ 5)))";
        compare_inputs(input, input2);
    }
}
