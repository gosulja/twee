use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
}

#[derive(Debug, Serialize)]
pub enum Stmt {
    Expression(Expr),
    VariableDecl(String, Expr),
}
