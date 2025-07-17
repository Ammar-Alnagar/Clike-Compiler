use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::lir::lir::{LirInstruction, LirInstructionKind, LirOperand};

pub struct CodeGenerator {
    variables: Vec<u32>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn codegen(&mut self, instructions: Vec<LirInstruction>) -> Result<Vec<u8>, String> {
        let mut module = Module::new();

        // Create a type section with a single function type: () -> f64
        let mut types = TypeSection::new();
        types.function(vec![], vec![ValType::F64]);
        module.section(&types);

        // Create a function section with a single function
        let mut functions = FunctionSection::new();
        functions.function(0);
        module.section(&functions);

        // Create an export section to export the function as "main"
        let mut exports = ExportSection::new();
        exports.export("main", ExportKind::Func, 0);
        module.section(&exports);

        // Create a code section with the code for the function
        let mut code = CodeSection::new();
        let mut func = Function::new(vec![]);

        for instruction in instructions {
            self.codegen_instruction(&mut func, instruction)?;
        }

        func.instruction(&Instruction::End);
        code.function(&func);
        module.section(&code);

        Ok(module.finish())
    }

    fn codegen_instruction(
        &mut self,
        func: &mut Function,
        instruction: LirInstruction,
    ) -> Result<(), String> {
        match instruction.kind {
            LirInstructionKind::Add => {
                self.codegen_operand(func, instruction.src1)?;
                self.codegen_operand(func, instruction.src2.unwrap())?;
                func.instruction(&Instruction::F64Add);
                self.variables.push(0); // TODO: Use real variable indices
            }
            LirInstructionKind::Sub => {
                self.codegen_operand(func, instruction.src1)?;
                self.codegen_operand(func, instruction.src2.unwrap())?;
                func.instruction(&Instruction::F64Sub);
                self.variables.push(0);
            }
            LirInstructionKind::Mul => {
                self.codegen_operand(func, instruction.src1)?;
                self.codegen_operand(func, instruction.src2.unwrap())?;
                func.instruction(&Instruction::F64Mul);
                self.variables.push(0);
            }
            LirInstructionKind::Div => {
                self.codegen_operand(func, instruction.src1)?;
                self.codegen_operand(func, instruction.src2.unwrap())?;
                func.instruction(&Instruction::F64Div);
                self.variables.push(0);
            }
            LirInstructionKind::Mov => {
                self.codegen_operand(func, instruction.src1)?;
                self.variables.push(0);
            }
        }
        Ok(())
    }

    fn codegen_operand(&self, func: &mut Function, operand: LirOperand) -> Result<(), String> {
        match operand {
            LirOperand::Register(r) => {
                func.instruction(&Instruction::LocalGet(r as u32));
            }
            LirOperand::Constant(c) => {
                func.instruction(&Instruction::F64Const(c));
            }
        }
        Ok(())
    }
}
