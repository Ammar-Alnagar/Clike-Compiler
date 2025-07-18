use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::parser::ast::Expr;

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn codegen(&mut self, ast: Expr) -> Result<String, String> {
        self.codegen_expr(ast)
    }

    fn codegen_expr(&mut self, expr: Expr) -> Result<String, String> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.codegen_expr(*left)?;
                let right = self.codegen_expr(*right)?;
                Ok(format!("({} {} {})", operator, left, right))
            }
            Expr::Grouping { expression } => self.codegen_expr(*expression),
            Expr::Literal { value } => Ok(value.to_string()),
            Expr::Unary { operator, right } => {
                let right = self.codegen_expr(*right)?;
                Ok(format!("({} {})", operator, right))
            }
        }
    }
}
