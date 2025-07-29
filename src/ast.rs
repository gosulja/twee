use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    BinaryOp {
        left: Box<Expr>,
        op: Binop,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
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

impl Binop {
    /*
        A precedence of operators system to ensure proper parsing.
    */
    pub fn precedence(&self) -> u8 {
        match self {
            Binop::Add | Binop::Sub => 1,
            Binop::Mul | Binop::Div => 2,
        }
    }

    /*
        All operators are linked with something to the left, for example:
        2 + 5, the left link is 2.
    */
    pub fn is_left_linked(&self) -> bool {
        true
    }
}
