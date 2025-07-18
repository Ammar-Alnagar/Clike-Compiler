pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod hir;
pub mod mir;
pub mod lir;
pub mod codegen;

use crate::codegen::codegen::CodeGenerator;
use crate::lexer::lexer::Lexer;
use crate::parser::pratt_parser::Parser;
use crate::semantic::analyzer::SemanticAnalyzer;

pub fn compile(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(&ast)?;
    let mut codegen = CodeGenerator::new();
    codegen.codegen(ast)
}
