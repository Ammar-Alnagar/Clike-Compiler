use crate::hir::hir::{HirBinaryOp, HirExpr, HirUnaryOp};
use crate::mir::mir::{MirBasicBlock, MirBinaryOp, MirInstruction, MirInstructionKind, MirOperand, MirTerminator, MirUnaryOp};

pub struct MirLowerer {
    instructions: Vec<MirInstruction>,
    registers: usize,
}

impl MirLowerer {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            registers: 0,
        }
    }

    pub fn lower(&mut self, expr: HirExpr) -> Result<Vec<MirBasicBlock>, String> {
        let result = self.lower_expr(expr)?;
        let terminator = MirTerminator::Return(result);
        let block = MirBasicBlock {
            instructions: self.instructions.drain(..).collect(),
            terminator,
        };
        Ok(vec![block])
    }

    fn lower_expr(&mut self, expr: HirExpr) -> Result<MirOperand, String> {
        match expr {
            HirExpr::Binary { op, left, right, expr_type } => {
                let left = self.lower_expr(*left)?;
                let right = self.lower_expr(*right)?;
                let op = match op {
                    HirBinaryOp::Add => MirBinaryOp::Add,
                    HirBinaryOp::Subtract => MirBinaryOp::Subtract,
                    HirBinaryOp::Multiply => MirBinaryOp::Multiply,
                    HirBinaryOp::Divide => MirBinaryOp::Divide,
                    HirBinaryOp::Equal => MirBinaryOp::Equal,
                    HirBinaryOp::NotEqual => MirBinaryOp::NotEqual,
                    HirBinaryOp::GreaterThan => MirBinaryOp::GreaterThan,
                    HirBinaryOp::LessThan => MirBinaryOp::LessThan,
                    HirBinaryOp::GreaterThanOrEqual => MirBinaryOp::GreaterThanOrEqual,
                    HirBinaryOp::LessThanOrEqual => MirBinaryOp::LessThanOrEqual,
                };
                let dest = self.new_register();
                self.instructions.push(MirInstruction {
                    kind: MirInstructionKind::BinaryOp(op, left, right),
                    dest: dest.clone(),
                    ty: expr_type,
                });
                Ok(dest)
            }
            HirExpr::Literal { value, .. } => Ok(MirOperand::Literal(value)),
            HirExpr::Unary { op, expr, expr_type } => {
                let expr = self.lower_expr(*expr)?;
                let op = match op {
                    HirUnaryOp::Negate => MirUnaryOp::Negate,
                    HirUnaryOp::Not => MirUnaryOp::Not,
                };
                let dest = self.new_register();
                self.instructions.push(MirInstruction {
                    kind: MirInstructionKind::UnaryOp(op, expr),
                    dest: dest.clone(),
                    ty: expr_type,
                });
                Ok(dest)
            }
        }
    }

    fn new_register(&mut self) -> MirOperand {
        let register = MirOperand::Register(self.registers);
        self.registers += 1;
        register
    }
}
