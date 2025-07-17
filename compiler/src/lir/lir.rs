#[derive(Debug, Clone)]
pub struct LirInstruction {
    pub kind: LirInstructionKind,
    pub dest: LirOperand,
    pub src1: LirOperand,
    pub src2: Option<LirOperand>,
}

#[derive(Debug, Clone)]
pub enum LirInstructionKind {
    Add,
    Sub,
    Mul,
    Div,
    Mov,
}

#[derive(Debug, Clone)]
pub enum LirOperand {
    Register(usize),
    Constant(f64),
}
