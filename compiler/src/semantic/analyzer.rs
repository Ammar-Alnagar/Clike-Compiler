use crate::parser::ast::Expr;
use crate::semantic::symbol_table::{SymbolTable, Type};

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(None),
        }
    }

    pub fn analyze(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_type = self.analyze(left)?;
                let right_type = self.analyze(right)?;

                if left_type != right_type {
                    return Err(format!(
                        "Type mismatch: cannot apply operator {:?} to types {:?} and {:?}",
                        operator, left_type, right_type
                    ));
                }

                Ok(left_type)
            }
            Expr::Grouping { expression } => self.analyze(expression),
            Expr::Literal { value } => match value {
                crate::lexer::token::Token::Number(_) => Ok(Type::Float),
                crate::lexer::token::Token::String(_) => Ok(Type::String),
                crate::lexer::token::Token::Reserved(r) => match r {
                    crate::lexer::token::Reserved::True | crate::lexer::token::Reserved::False => Ok(Type::Boolean),
                    crate::lexer::token::Reserved::Null => Ok(Type::Null),
                    _ => Err(format!("Invalid literal type: {:?}", r)),
                },
                _ => Err(format!("Invalid literal type: {:?}", value)),
            },
            Expr::Unary { operator, right } => {
                let right_type = self.analyze(right)?;
                match operator {
                    crate::lexer::token::Token::Operation(op) => match op {
                        crate::lexer::token::Operation::Subtract => {
                            if right_type != Type::Float && right_type != Type::Integer {
                                return Err(format!(
                                    "Type mismatch: cannot apply operator {:?} to type {:?}",
                                    operator, right_type
                                ));
                            }
                            Ok(right_type)
                        }
                        crate::lexer::token::Operation::Not => {
                            if right_type != Type::Boolean {
                                return Err(format!(
                                    "Type mismatch: cannot apply operator {:?} to type {:?}",
                                    operator, right_type
                                ));
                            }
                            Ok(Type::Boolean)
                        }
                        _ => Err(format!("Invalid unary operator: {:?}", operator)),
                    },
                    _ => Err(format!("Invalid unary operator: {:?}", operator)),
                }
            }
        }
    }
}
