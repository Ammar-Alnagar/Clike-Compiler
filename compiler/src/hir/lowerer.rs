use crate::hir::hir::{HirBinaryOp, HirExpr, HirLiteral, HirUnaryOp};
use crate::parser::ast::Expr;
use crate::semantic::symbol_table::Type;

pub struct HirLowerer;

impl HirLowerer {
    pub fn new() -> Self {
        Self
    }

    pub fn lower(&self, expr: &Expr) -> Result<HirExpr, String> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = self.lower(left)?;
                let right = self.lower(right)?;

                let op = match operator {
                    crate::lexer::token::Token::Operation(op) => match op {
                        crate::lexer::token::Operation::Add => HirBinaryOp::Add,
                        crate::lexer::token::Operation::Subtract => HirBinaryOp::Subtract,
                        crate::lexer::token::Operation::Multiply => HirBinaryOp::Multiply,
                        crate::lexer::token::Operation::Divide => HirBinaryOp::Divide,
                        crate::lexer::token::Operation::IfEqual => HirBinaryOp::Equal,
                        crate::lexer::token::Operation::NotEqual => HirBinaryOp::NotEqual,
                        crate::lexer::token::Operation::Greater => HirBinaryOp::GreaterThan,
                        crate::lexer::token::Operation::Less => HirBinaryOp::LessThan,
                        crate::lexer::token::Operation::GreaterEqual => HirBinaryOp::GreaterThanOrEqual,
                        crate::lexer::token::Operation::LessEqual => HirBinaryOp::LessThanOrEqual,
                        _ => return Err(format!("Invalid binary operator: {:?}", op)),
                    },
                    _ => return Err(format!("Invalid binary operator: {:?}", operator)),
                };

                Ok(HirExpr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    expr_type: Type::Float, // TODO: Get type from semantic analyzer
                })
            }
            Expr::Grouping { expression } => self.lower(expression),
            Expr::Literal { value } => {
                let (literal, expr_type) = match value {
                    crate::lexer::token::Token::Number(n) => (HirLiteral::Number(*n), Type::Float),
                    crate::lexer::token::Token::String(s) => (HirLiteral::String(s.clone()), Type::String),
                    crate::lexer::token::Token::Reserved(r) => match r {
                        crate::lexer::token::Reserved::True => (HirLiteral::Boolean(true), Type::Boolean),
                        crate::lexer::token::Reserved::False => (HirLiteral::Boolean(false), Type::Boolean),
                        crate::lexer::token::Reserved::Null => (HirLiteral::Null, Type::Null),
                        _ => return Err(format!("Invalid literal type: {:?}", r)),
                    },
                    _ => return Err(format!("Invalid literal type: {:?}", value)),
                };
                Ok(HirExpr::Literal {
                    value: literal,
                    expr_type,
                })
            }
            Expr::Unary { operator, right } => {
                let expr = self.lower(right)?;

                let op = match operator {
                    crate::lexer::token::Token::Operation(op) => match op {
                        crate::lexer::token::Operation::Subtract => HirUnaryOp::Negate,
                        crate::lexer::token::Operation::Not => HirUnaryOp::Not,
                        _ => return Err(format!("Invalid unary operator: {:?}", op)),
                    },
                    _ => return Err(format!("Invalid unary operator: {:?}", operator)),
                };

                Ok(HirExpr::Unary {
                    op,
                    expr: Box::new(expr),
                    expr_type: Type::Float, // TODO: Get type from semantic analyzer
                })
            }
        }
    }
}
