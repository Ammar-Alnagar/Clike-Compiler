use d_compiler::codegen::codegen::CodeGenerator;
use d_compiler::hir::lowerer::HirLowerer;
use d_compiler::lexer::lexer::Lexer;
use d_compiler::lir::lowerer::LirLowerer;
use d_compiler::mir::lowerer::MirLowerer;
use d_compiler::parser::pratt_parser::PrattParser;
use d_compiler::semantic::analyzer::SemanticAnalyzer;

fn main() {
    let input = "1 + 2 * 3";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = PrattParser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let analysis_result = analyzer.analyze(&ast).unwrap();

    let hir_lowerer = HirLowerer::new();
    let hir = hir_lowerer.lower(&ast).unwrap();

    let mut mir_lowerer = MirLowerer::new();
    let mir = mir_lowerer.lower(hir.clone()).unwrap();

    let lir_lowerer = LirLowerer::new();
    let lir = lir_lowerer.lower(mir.clone()).unwrap();

    let mut codegen = CodeGenerator::new();
    let wasm = codegen.codegen(lir.clone()).unwrap();

    println!("Input Program:\n{}\n", input);
    println!("AST:\n{:#?}", ast);
    println!("Analysis result:\n{:#?}", analysis_result);
    println!("HIR:\n{:#?}", hir);
    println!("MIR:\n{:#?}", mir);
    println!("LIR:\n{:#?}", lir);
    println!("WASM:\n{:?}", wasm);
}
