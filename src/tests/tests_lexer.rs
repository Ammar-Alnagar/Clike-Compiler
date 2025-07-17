use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Operation, Punctuation, Reserved, Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_char_operators() {
        let mut lexer = Lexer::new("== != >= <=");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Operation(Operation::IfEqual));
        assert_eq!(tokens[1].token, Token::Operation(Operation::NotEqual));
        assert_eq!(tokens[2].token, Token::Operation(Operation::GreaterEqual));
        assert_eq!(tokens[3].token, Token::Operation(Operation::LessEqual));
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let fn true false");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::Let));
        assert_eq!(tokens[1].token, Token::Reserved(Reserved::Fn));
        assert_eq!(tokens[2].token, Token::Reserved(Reserved::True));
        assert_eq!(tokens[3].token, Token::Reserved(Reserved::False));
    }

    #[test]
    fn test_string_with_escapes() {
        let mut lexer = Lexer::new(r#""hello\nworld""#);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("variable_name _private __dunder");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0].token,
            Token::Identifier("variable_name".to_string())
        );
        assert_eq!(tokens[1].token, Token::Identifier("_private".to_string()));
        assert_eq!(tokens[2].token, Token::Identifier("__dunder".to_string()));
    }

    #[test]
    fn test_basic_numbers() {
        let mut lexer = Lexer::new("123 456.789");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Number(123.0));
        assert_eq!(tokens[1].token, Token::Number(456.789));
    }

    #[test]
    fn test_negative_numbers() {
        let mut lexer = Lexer::new("-123 -456.789");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Operation(Operation::Subtract));
        assert_eq!(tokens[1].token, Token::Number(123.0));
        assert_eq!(tokens[2].token, Token::Operation(Operation::Subtract));
        assert_eq!(tokens[3].token, Token::Number(456.789));
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Eof);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("   \t  \n  \r\n  ");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3); // Should only contain 2 newlines + EOF
        assert_eq!(tokens[0].token, Token::Newline);
        assert_eq!(tokens[1].token, Token::Newline);
        assert_eq!(tokens[2].token, Token::Eof);
    }

    #[test]
    fn test_all_operators() {
        let mut lexer = Lexer::new("+ - * / = == != > < >= <= !");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[1].token, Token::Operation(Operation::Subtract));
        assert_eq!(tokens[2].token, Token::Operation(Operation::Multiply));
        assert_eq!(tokens[3].token, Token::Operation(Operation::Divide));
        assert_eq!(tokens[4].token, Token::Operation(Operation::Assign));
        assert_eq!(tokens[5].token, Token::Operation(Operation::IfEqual));
        assert_eq!(tokens[6].token, Token::Operation(Operation::NotEqual));
        assert_eq!(tokens[7].token, Token::Operation(Operation::Greater));
        assert_eq!(tokens[8].token, Token::Operation(Operation::Less));
        assert_eq!(tokens[9].token, Token::Operation(Operation::GreaterEqual));
        assert_eq!(tokens[10].token, Token::Operation(Operation::LessEqual));
        assert_eq!(tokens[11].token, Token::Operation(Operation::Not));
    }

    #[test]
    fn test_all_punctuation() {
        let mut lexer = Lexer::new("( ) { } [ ] , ; . : ?");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[1].token, Token::Punctuation(Punctuation::CloseParen));
        assert_eq!(tokens[2].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[3].token, Token::Punctuation(Punctuation::CloseBrace));
        assert_eq!(
            tokens[4].token,
            Token::Punctuation(Punctuation::OpenBracket)
        );
        assert_eq!(
            tokens[5].token,
            Token::Punctuation(Punctuation::CloseBracket)
        );
        assert_eq!(tokens[6].token, Token::Punctuation(Punctuation::Comma));
        assert_eq!(tokens[7].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(tokens[8].token, Token::Punctuation(Punctuation::Dot));
        assert_eq!(tokens[9].token, Token::Punctuation(Punctuation::Colon));
        assert_eq!(
            tokens[10].token,
            Token::Punctuation(Punctuation::QuestionMark)
        );
    }

    #[test]
    fn test_all_reserved_words() {
        let mut lexer = Lexer::new(
            "null void let fn if else while for continue break return public private static print true false",
        );
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::Null));
        assert_eq!(tokens[1].token, Token::Reserved(Reserved::Void));
        assert_eq!(tokens[2].token, Token::Reserved(Reserved::Let));
        assert_eq!(tokens[3].token, Token::Reserved(Reserved::Fn));
        assert_eq!(tokens[4].token, Token::Reserved(Reserved::If));
        assert_eq!(tokens[5].token, Token::Reserved(Reserved::Else));
        assert_eq!(tokens[6].token, Token::Reserved(Reserved::While));
        assert_eq!(tokens[7].token, Token::Reserved(Reserved::For));
        assert_eq!(tokens[8].token, Token::Reserved(Reserved::Continue));
        assert_eq!(tokens[9].token, Token::Reserved(Reserved::Break));
        assert_eq!(tokens[10].token, Token::Reserved(Reserved::Return));
        assert_eq!(tokens[11].token, Token::Reserved(Reserved::Public));
        assert_eq!(tokens[12].token, Token::Reserved(Reserved::Private));
        assert_eq!(tokens[13].token, Token::Reserved(Reserved::Static));
        assert_eq!(tokens[14].token, Token::Reserved(Reserved::Print));
        assert_eq!(tokens[15].token, Token::Reserved(Reserved::True));
        assert_eq!(tokens[16].token, Token::Reserved(Reserved::False));
    }

    #[test]
    fn test_identifiers_with_numbers() {
        let mut lexer = Lexer::new("var1 v2ar var3_ _4var _5_var");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Identifier("var1".to_string()));
        assert_eq!(tokens[1].token, Token::Identifier("v2ar".to_string()));
        assert_eq!(tokens[2].token, Token::Identifier("var3_".to_string()));
        assert_eq!(tokens[3].token, Token::Identifier("_4var".to_string()));
        assert_eq!(tokens[4].token, Token::Identifier("_5_var".to_string()));
    }

    #[test]
    fn test_string_with_spaces() {
        let mut lexer = Lexer::new(r#""hello   world""#);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::String("hello   world".to_string()));
    }

    #[test]
    fn test_string_with_special_chars() {
        let mut lexer = Lexer::new(r#""!@#$%^&*()""#);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::String("!@#$%^&*()".to_string()));
    }

    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new(r#""""#);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::String("".to_string()));
    }

    #[test]
    fn test_string_with_quotes() {
        let mut lexer = Lexer::new(r#""Hello \"World\"!""#);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0].token,
            Token::String("Hello \"World\"!".to_string())
        );
    }

    #[test]
    fn test_string_with_escapes_mixed() {
        let mut lexer = Lexer::new(r#""Hello\n\t\"World\"\n!""#);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0].token,
            Token::String("Hello\n\t\"World\"\n!".to_string())
        );
    }

    #[test]
    fn test_invalid_tokens() {
        let mut lexer = Lexer::new("@ # $");
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[2].token, Token::Invalid(_)));
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"Hello");
        let tokens = lexer.tokenize();

        match &tokens[0].token {
            Token::Invalid(msg) => assert!(msg.contains("Unterminated string")),
            _ => panic!("Expected Invalid token for unterminated string"),
        }
    }

    #[test]
    fn test_function_declaration() {
        let mut lexer = Lexer::new("fn test() { return; }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::Fn));
        assert_eq!(tokens[1].token, Token::Identifier("test".to_string()));
        assert_eq!(tokens[2].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[3].token, Token::Punctuation(Punctuation::CloseParen));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[5].token, Token::Reserved(Reserved::Return));
        assert_eq!(tokens[6].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(tokens[7].token, Token::Punctuation(Punctuation::CloseBrace));
    }

    #[test]
    fn test_nested_blocks() {
        let mut lexer = Lexer::new("{ { { } } }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[1].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[2].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[3].token, Token::Punctuation(Punctuation::CloseBrace));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::CloseBrace));
        assert_eq!(tokens[5].token, Token::Punctuation(Punctuation::CloseBrace));
    }

    #[test]
    fn test_mixed_expressions() {
        let mut lexer = Lexer::new("x + y * (z - w)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Identifier("x".to_string()));
        assert_eq!(tokens[1].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[2].token, Token::Identifier("y".to_string()));
        assert_eq!(tokens[3].token, Token::Operation(Operation::Multiply));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[5].token, Token::Identifier("z".to_string()));
        assert_eq!(tokens[6].token, Token::Operation(Operation::Subtract));
        assert_eq!(tokens[7].token, Token::Identifier("w".to_string()));
        assert_eq!(tokens[8].token, Token::Punctuation(Punctuation::CloseParen));
    }

    #[test]
    fn test_if_else_structure() {
        let mut lexer = Lexer::new("if x > 0 { print x; } else { print 0; }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::If));
        assert_eq!(tokens[1].token, Token::Identifier("x".to_string()));
        assert_eq!(tokens[2].token, Token::Operation(Operation::Greater));
        assert_eq!(tokens[3].token, Token::Number(0.0));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[5].token, Token::Reserved(Reserved::Print));
        assert_eq!(tokens[6].token, Token::Identifier("x".to_string()));
        assert_eq!(tokens[7].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(tokens[8].token, Token::Punctuation(Punctuation::CloseBrace));
        assert_eq!(tokens[9].token, Token::Reserved(Reserved::Else));
        assert_eq!(tokens[10].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[11].token, Token::Reserved(Reserved::Print));
        assert_eq!(tokens[12].token, Token::Number(0.0));
        assert_eq!(tokens[13].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(
            tokens[14].token,
            Token::Punctuation(Punctuation::CloseBrace)
        );
    }

    #[test]
    fn test_while_loop() {
        let mut lexer = Lexer::new("while i < 10 { i = i + 1; }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::While));
        assert_eq!(tokens[1].token, Token::Identifier("i".to_string()));
        assert_eq!(tokens[2].token, Token::Operation(Operation::Less));
        assert_eq!(tokens[3].token, Token::Number(10.0));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[5].token, Token::Identifier("i".to_string()));
        assert_eq!(tokens[6].token, Token::Operation(Operation::Assign));
        assert_eq!(tokens[7].token, Token::Identifier("i".to_string()));
        assert_eq!(tokens[8].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[9].token, Token::Number(1.0));
        assert_eq!(tokens[10].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(
            tokens[11].token,
            Token::Punctuation(Punctuation::CloseBrace)
        );
    }

    #[test]
    fn test_complex_nested_expression() {
        let mut lexer = Lexer::new("(a + b) * (c - d) / (e + f)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[1].token, Token::Identifier("a".to_string()));
        assert_eq!(tokens[2].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[3].token, Token::Identifier("b".to_string()));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::CloseParen));
        assert_eq!(tokens[5].token, Token::Operation(Operation::Multiply));
        assert_eq!(tokens[6].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[7].token, Token::Identifier("c".to_string()));
        assert_eq!(tokens[8].token, Token::Operation(Operation::Subtract));
        assert_eq!(tokens[9].token, Token::Identifier("d".to_string()));
        assert_eq!(
            tokens[10].token,
            Token::Punctuation(Punctuation::CloseParen)
        );
        assert_eq!(tokens[11].token, Token::Operation(Operation::Divide));
        assert_eq!(tokens[12].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[13].token, Token::Identifier("e".to_string()));
        assert_eq!(tokens[14].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[15].token, Token::Identifier("f".to_string()));
        assert_eq!(
            tokens[16].token,
            Token::Punctuation(Punctuation::CloseParen)
        );
    }

    #[test]
    fn test_function_with_multiple_parameters() {
        let mut lexer = Lexer::new("fn sum(a, b, c) { return a + b + c; }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token, Token::Reserved(Reserved::Fn));
        assert_eq!(tokens[1].token, Token::Identifier("sum".to_string()));
        assert_eq!(tokens[2].token, Token::Punctuation(Punctuation::OpenParen));
        assert_eq!(tokens[3].token, Token::Identifier("a".to_string()));
        assert_eq!(tokens[4].token, Token::Punctuation(Punctuation::Comma));
        assert_eq!(tokens[5].token, Token::Identifier("b".to_string()));
        assert_eq!(tokens[6].token, Token::Punctuation(Punctuation::Comma));
        assert_eq!(tokens[7].token, Token::Identifier("c".to_string()));
        assert_eq!(tokens[8].token, Token::Punctuation(Punctuation::CloseParen));
        assert_eq!(tokens[9].token, Token::Punctuation(Punctuation::OpenBrace));
        assert_eq!(tokens[10].token, Token::Reserved(Reserved::Return));
        assert_eq!(tokens[11].token, Token::Identifier("a".to_string()));
        assert_eq!(tokens[12].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[13].token, Token::Identifier("b".to_string()));
        assert_eq!(tokens[14].token, Token::Operation(Operation::Add));
        assert_eq!(tokens[15].token, Token::Identifier("c".to_string()));
        assert_eq!(tokens[16].token, Token::Punctuation(Punctuation::Semicolon));
        assert_eq!(
            tokens[17].token,
            Token::Punctuation(Punctuation::CloseBrace)
        );
    }
}
