use crate::lexer::token::{Token, TokenInfo};
use crate::parser::ast::Expr;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

pub struct PrattParser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl PrattParser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr, String> {
        let mut left = self.prefix_rule()?;

        while precedence <= self.get_precedence(&self.peek().token) {
            left = self.infix_rule(left)?;
        }

        Ok(left)
    }

    fn prefix_rule(&mut self) -> Result<Expr, String> {
        let token = self.advance();
        match &token.token {
            Token::Punctuation(crate::lexer::token::Punctuation::OpenParen) => self.grouping(),
            Token::Operation(crate::lexer::token::Operation::Subtract) | Token::Operation(crate::lexer::token::Operation::Not) => self.unary(),
            Token::Number(_) | Token::String(_) | Token::Reserved(_) => self.literal(),
            _ => Err(format!("Expected expression, found {:?}", token.token)),
        }
    }

    fn infix_rule(&mut self, left: Expr) -> Result<Expr, String> {
        let token = self.advance().clone();
        let precedence = self.get_precedence(&token.token);
        let right = self.parse_precedence(precedence)?;
        Ok(Expr::Binary {
            left: Box::new(left),
            operator: token.token,
            right: Box::new(right),
        })
    }

    fn literal(&mut self) -> Result<Expr, String> {
        Ok(Expr::Literal {
            value: self.previous().token.clone(),
        })
    }

    fn grouping(&mut self) -> Result<Expr, String> {
        let expr = self.expression()?;
        self.consume(Token::Punctuation(crate::lexer::token::Punctuation::CloseParen), "Expect ')' after expression.")?;
        Ok(Expr::Grouping {
            expression: Box::new(expr),
        })
    }

    fn unary(&mut self) -> Result<Expr, String> {
        let operator = self.previous().clone();
        let right = self.parse_precedence(Precedence::Unary)?;
        Ok(Expr::Unary {
            operator: operator.token,
            right: Box::new(right),
        })
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Operation(op) => match op {
                crate::lexer::token::Operation::Add | crate::lexer::token::Operation::Subtract => Precedence::Term,
                crate::lexer::token::Operation::Multiply | crate::lexer::token::Operation::Divide => Precedence::Factor,
                crate::lexer::token::Operation::IfEqual | crate::lexer::token::Operation::NotEqual => Precedence::Equality,
                crate::lexer::token::Operation::Greater | crate::lexer::token::Operation::GreaterEqual | crate::lexer::token::Operation::Less | crate::lexer::token::Operation::LessEqual => Precedence::Comparison,
                _ => Precedence::None,
            },
            _ => Precedence::None,
        }
    }

    fn consume(&mut self, token_type: Token, message: &str) -> Result<&TokenInfo, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(message.to_string())
    }

    fn check(&self, token_type: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token) == std::mem::discriminant(&token_type)
    }

    fn advance(&mut self) -> &TokenInfo {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token == Token::Eof
    }

    fn peek(&self) -> &TokenInfo {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenInfo {
        &self.tokens[self.current - 1]
    }
}
