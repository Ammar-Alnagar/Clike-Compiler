use crate::semantic::symbol_table::Type;

#[derive(Debug, Clone)]
pub enum HirExpr {
    Binary {
        op: HirBinaryOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
        expr_type: Type,
    },
    Literal {
        value: HirLiteral,
        expr_type: Type,
    },
    Unary {
        op: HirUnaryOp,
        expr: Box<HirExpr>,
        expr_type: Type,
    },
}

#[derive(Debug, Clone)]
pub enum HirBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum HirUnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum HirLiteral {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
