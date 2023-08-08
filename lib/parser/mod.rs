pub mod ast;

use nom::{
    IResult,
    combinator::{verify,map},
    bytes::complete::take,
    multi::many0,
    sequence::terminated,
}

use crate::lexer::token::*;
use crate::parser::ast::*;

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expr> {
    alt((
        parse_lit_expr,
        parse_prefix_expr,
        parse_paren_expr,
    ))(input)
}

fn parse_pratt_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expr> {
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_pratt_expr(input, Precedence::PLowest)
}

fn parse_expr_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
    map(parse_expr, |expr| {
        Stmt::ExprStmt(expr)
    })(input)
    
}

fn parse_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
    alt((parse_expr_stmt))(input)
}

fn eof_tag(tokens: Tokens) -> IResult<Tokens, Tokens> {
    verify(take(1usize), |t: &Tokens| t.tok[0] == Token::EOF)(tokens)
}

fn parse_program(input: Tokens) -> IResult<Tokens, Program> {
    terminated(many0(parse_stmt), eof_tag)(input)
}
pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> IResult<Tokens, Program> {
        parse_program(tokens)
    }
}