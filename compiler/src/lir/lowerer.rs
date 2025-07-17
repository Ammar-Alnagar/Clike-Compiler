use crate::lir::lir::{LirInstruction, LirInstructionKind, LirOperand};
use crate::mir::mir::{MirBasicBlock, MirBinaryOp, MirInstructionKind, MirOperand};

pub struct LirLowerer;

impl LirLowerer {
    pub fn new() -> Self {
        Self
    }

    pub fn lower(&self, blocks: Vec<MirBasicBlock>) -> Result<Vec<LirInstruction>, String> {
        let mut instructions = Vec::new();

        for block in blocks {
            for instruction in block.instructions {
                match instruction.kind {
                    MirInstructionKind::BinaryOp(op, src1, src2) => {
                        let kind = match op {
                            MirBinaryOp::Add => LirInstructionKind::Add,
                            MirBinaryOp::Subtract => LirInstructionKind::Sub,
                            MirBinaryOp::Multiply => LirInstructionKind::Mul,
                            MirBinaryOp::Divide => LirInstructionKind::Div,
                            _ => return Err(format!("Invalid binary operator: {:?}", op)),
                        };
                        let dest = self.lower_operand(instruction.dest)?;
                        let src1 = self.lower_operand(src1)?;
                        let src2 = self.lower_operand(src2)?;
                        instructions.push(LirInstruction {
                            kind,
                            dest,
                            src1,
                            src2: Some(src2),
                        });
                    }
                    MirInstructionKind::UnaryOp(_, _) => {
                        // TODO: Handle unary operations
                    }
                    MirInstructionKind::Load(_) => {
                        // TODO: Handle load operations
                    }
                }
            }
        }

        Ok(instructions)
    }

    fn lower_operand(&self, operand: MirOperand) -> Result<LirOperand, String> {
        match operand {
            MirOperand::Literal(literal) => match literal {
                crate::hir::hir::HirLiteral::Number(n) => Ok(LirOperand::Constant(n)),
                _ => Err(format!("Invalid literal type: {:?}", literal)),
            },
            MirOperand::Register(r) => Ok(LirOperand::Register(r)),
        }
    }
}
