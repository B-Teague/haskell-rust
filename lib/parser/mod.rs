pub mod ast;

use crate::lexer::token::*;
use crate::parser::ast::*;

// fn parse_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
//     alt(parse_expr_stmt)(input)
// }
pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> Result<Tokens, Program> {
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
}