use crate::ast::{Binop, Expr, Stmt, VariableDecl};
use crate::lexer::*;

pub struct TweeParser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
}

impl<'a> TweeParser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn check(&self, target_type: &TokenType) -> bool {
        if let Some(ref token) = self.current {
            token.token_type == *target_type
        } else {
            *target_type == TokenType::EOF
        }
    }

    fn consume(&mut self, expected: TokenType) -> Result<Token, String> {
        if self.check(&expected) {
            let token = self.current.clone();
            self.advance();

            token.ok_or_else(|| "[twee::error] unexpected end of input".to_string())
        } else {
            Err(format!(
                "[twee::error] expected {:?} but got {:?}",
                expected,
                self.current.as_ref().map(|t| &t.token_type)
            ))
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();

        while !self.check(&TokenType::EOF) {
            stmts.push(self.parse_stmt()?);
            if self.check(&TokenType::Semi) {
                self.advance();
            }
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(token) => match token.token_type {
                TokenType::Local => self.parse_local_declaration(),
                _ => {
                    let expr = Stmt::Expression(self.parse_expr()?);
                    if self.check(&TokenType::Semi) {
                        self.advance();
                    }

                    Ok(expr)
                }
            },

            None => Err("[twee::error] unexpected end of input".to_string()),
        }
    }

    /*
        Parse a local variable declaration.
        Syntax:
            local ident = value<Expr>;<Optional>
        Example:
            local number = 23
    */
    fn parse_local_declaration(&mut self) -> Result<Stmt, String> {
        /* Expect and consume the "local" keyword */
        self.consume(TokenType::Local)?;

        /* Expect and consume an identifier, this is the variabels identifier. */
        let name = self.consume(TokenType::Identifier)?.lexeme;

        /* Parse a type annotation, if none is present just imply the type. */
        let type_annotation = if self.check(&TokenType::Colon) {
            self.advance();

            let type_str = self.consume(TokenType::Identifier)?.lexeme;
            type_str
        } else {
            "any".to_string()
        };

        /* Expect and consume an equals symbol. */
        self.consume(TokenType::Equals)?;

        /* Parse an expression for the value of the variable. */
        let value = self.parse_expr()?;

        Ok(Stmt::VariableDecl(VariableDecl {
            name,
            value,
            type_annotation,
        }))
    }

    /*
        Parse an ordinary expression.
    */
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_precedence(0) /* Start with parsing by preceden */
    }

    fn parse_precedence(&mut self, min: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while let Some(op) = self.binop() {
            let precedence = op.precedence();

            if precedence < min {
                break;
            }

            self.advance();

            let right_min = if op.is_left_linked() {
                precedence + 1
            } else {
                precedence
            };

            let right = self.parse_precedence(right_min)?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /*
        Parse primary expressions (literals, identifiers, and grouped expressions).
    */
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(token) => match token.token_type.clone() {
                /* Parse a numeric literal. */
                TokenType::Number => {
                    let value = token.lexeme.parse::<f64>().map_err(|e| e.to_string())?;
                    self.advance();

                    Ok(Expr::Number(value))
                }

                /* Parse a reference to an identifier */
                TokenType::Identifier => {
                    let value = token.lexeme.clone();
                    self.advance();

                    Ok(Expr::Identifier(value))
                }

                /* Parse a string literal. */
                TokenType::String => {
                    let value = token.lexeme.clone();
                    self.advance();

                    Ok(Expr::String(value))
                }

                /* Parse parenthesized expressions */
                TokenType::LParen => {
                    self.advance(); // consume '('
                    let expr = self.parse_expr()?;
                    self.consume(TokenType::RParen)?; // consume ')'
                    Ok(expr)
                }

                _ => Err(format!(
                    "[twee::error] unexpected token {:?}",
                    token.token_type
                )),
            },

            None => Err("[twee::error] unexpected end of input".to_string()),
        }
    }

    /*
        Is the current token a binary operator? (add, sub, mul, div) if so return it as a binop.
    */
    fn binop(&self) -> Option<Binop> {
        match self.peek() {
            Some(tok) => match tok.token_type {
                TokenType::Add => Some(Binop::Add),
                TokenType::Sub => Some(Binop::Sub),
                TokenType::Mul => Some(Binop::Mul),
                TokenType::Div => Some(Binop::Div),
                _ => None,
            },

            None => None,
        }
    }
}
