use clap::{self, Parser};
use lexer::Lexer;
use parser::TweeParser;

mod ast;
mod lexer;
mod parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    let output = args.output;

    let content = std::fs::read_to_string(&input).unwrap();
    let lexer = Lexer::new(&content);
    let mut parser = TweeParser::new(lexer);

    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            println!("[twee::parser_error] {}", e);
            return;
        }
    };

    match serde_json::to_string_pretty(&stmts) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&output, json) {
                println!("[twee::io_error] failed to write output: {}", e);
            }
        }
        Err(e) => {
            println!("[twee::serialization_error] {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Stmt};
    use crate::lexer::Lexer;
    use crate::parser::TweeParser;

    #[test]
    fn test_hello_var_type() {
        let input = r#"local hello = "world!""#;

        let lexer = Lexer::new(input);
        let mut parser = TweeParser::new(lexer);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 1);

        match &result[0] {
            Stmt::VariableDecl(name, expr) => {
                assert_eq!(name, "hello");

                match expr {
                    Expr::String(value) => {
                        assert_eq!(value, "world!");
                    }

                    _ => panic!("expected a string expression, got {:?}", expr),
                }
            }

            _ => {
                panic!("expected a variable declaration, got {:?}", result[0]);
            }
        }

        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("serialized:\n{}\n", json);

        assert!(json.contains("VariableDecl"));
        assert!(json.contains("hello"));
        assert!(json.contains("String"));
        assert!(json.contains("world!"));
    }
}
