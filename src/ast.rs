use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
}

#[derive(Debug, Serialize)]
pub enum Stmt {
    Expression(Expr),
    VariableDecl(VariableDecl), /* name, value, type */
}

#[derive(Debug, Serialize)]
pub struct VariableDecl {
    pub name: String,
    pub value: Expr,
    pub type_annotation: String,
}
