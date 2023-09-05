pub type Program = Vec<Stmt>;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Box<Expr>, Infix, Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Prefix {
    Positive,
    Negative,
}

#[derive(PartialEq, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    PLowest,
    PSum,
    PProduct,
    PPower,
}