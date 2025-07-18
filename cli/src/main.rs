use d_compiler::compile;

fn main() {
    let input = "1 + 2 * 3";
    let result = compile(input);
    println!("{:?}", result);
}
