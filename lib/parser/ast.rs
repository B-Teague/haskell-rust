pub type Program = Vec<Stmt>;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    LitExpr(Literal),
    PrefixExpr(Prefix, Box<Expr>),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    IntLiteral(i64),
}

#[derive(PartialEq, Debug)]
pub enum Prefix {
    PrefixPlus,
    PrefixMinus,
}

#[derive(PartialEq, Debug)]
pub enum Infix {
    Plus,
    Minus,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    PLowest,
    PSum,
}