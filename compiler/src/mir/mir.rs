use crate::hir::hir::HirLiteral;
use crate::semantic::symbol_table::Type;

#[derive(Debug, Clone)]
pub struct MirBasicBlock {
    pub instructions: Vec<MirInstruction>,
    pub terminator: MirTerminator,
}

#[derive(Debug, Clone)]
pub struct MirInstruction {
    pub kind: MirInstructionKind,
    pub dest: MirOperand,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum MirInstructionKind {
    BinaryOp(MirBinaryOp, MirOperand, MirOperand),
    UnaryOp(MirUnaryOp, MirOperand),
    Load(MirOperand),
}

#[derive(Debug, Clone)]
pub enum MirTerminator {
    Return(MirOperand),
    Goto(usize),
    Branch(MirOperand, usize, usize),
}

#[derive(Debug, Clone)]
pub enum MirOperand {
    Literal(HirLiteral),
    Register(usize),
}

#[derive(Debug, Clone)]
pub enum MirBinaryOp {
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
pub enum MirUnaryOp {
    Negate,
    Not,
}
