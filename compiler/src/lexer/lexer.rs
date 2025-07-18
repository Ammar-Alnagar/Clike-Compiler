use crate::lexer::token::{Operation, Punctuation, Reserved, Token, TokenInfo};

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token_info = self.next_token();
            // Skip whitespace tokens for cleaner output
            if !matches!(token_info.token, Token::Whitespace) {
                if let Token::Invalid(msg) = &token_info.token {
                    return Err(format!(
                        "Invalid token at line {}, column {}: {}",
                        token_info.line, token_info.column, msg
                    ));
                }
                tokens.push(token_info);
            }
        }

        tokens.push(TokenInfo::new(
            Token::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    fn next_token(&mut self) -> TokenInfo {
        let start_line = self.line;
        let start_column = self.column;
        let start_pos = self.current;

        let ch = self.advance();

        let token = match ch {
            // Whitespace
            ' ' | '\r' | '\t' => {
                while self.peek().is_ascii_whitespace() && self.peek() != '\n' {
                    self.advance();
                }
                Token::Whitespace
            }
            '\n' => {
                self.line += 1;
                self.column = 1;
                Token::Newline
            }

            // Multi-character operators (must come before single-char)
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::Operation(Operation::IfEqual)
                } else {
                    Token::Operation(Operation::Assign)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::Operation(Operation::NotEqual)
                } else {
                    Token::Operation(Operation::Not)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::Operation(Operation::GreaterEqual)
                } else {
                    Token::Operation(Operation::Greater)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::Operation(Operation::LessEqual)
                } else {
                    Token::Operation(Operation::Less)
                }
            }

            // Single character operators
            '+' => Token::Operation(Operation::Add),
            '-' => Token::Operation(Operation::Subtract),
            '*' => Token::Operation(Operation::Multiply),
            '%' => {
                if self.peek() == '%' {
                    self.advance();
                    Token::Operation(Operation::Remainder)
                } else {
                    Token::Operation(Operation::Modulo)
                }
            }

            // Handle division and comments
            '/' => {
                if self.peek() == '/' {
                    self.advance();
                    Token::Punctuation(Punctuation::Comment)
                } else if (self.peek() == '*') {
                    self.advance();
                    Token::Punctuation(Punctuation::CommentBlkStr)
                } else {
                    Token::Operation(Operation::Divide)
                }
            }

            // Punctuation
            '(' => Token::Punctuation(Punctuation::OpenParen),
            ')' => Token::Punctuation(Punctuation::CloseParen),
            '{' => Token::Punctuation(Punctuation::OpenBrace),
            '}' => Token::Punctuation(Punctuation::CloseBrace),
            '[' => Token::Punctuation(Punctuation::OpenBracket),
            ']' => Token::Punctuation(Punctuation::CloseBracket),
            ',' => Token::Punctuation(Punctuation::Comma),
            ';' => Token::Punctuation(Punctuation::Semicolon),
            '.' => Token::Punctuation(Punctuation::Dot),
            ':' => Token::Punctuation(Punctuation::Colon),
            '?' => Token::Punctuation(Punctuation::QuestionMark),
            '#' => Token::Punctuation(Punctuation::Hashtag),
            '@' => Token::Punctuation(Punctuation::At),

            // String literals
            '"' | '\'' => self.string(ch),

            // Numbers
            c if c.is_ascii_digit() => self.number(),

            // Identifiers and keywords
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),

            // Invalid character
            c => Token::Invalid(c.to_string()),
        };

        let lexeme = self.input[start_pos..self.current].iter().collect();

        TokenInfo::new(token, lexeme, start_line, start_column)
    }

    fn string(&mut self, quote_char: char) -> Token {
        let mut value = String::new();

        while self.peek() != quote_char && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            // Basic escape sequence handling
            if self.peek() == '\\' {
                self.advance(); // consume backslash
                match self.peek() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    c => {
                        value.push('\\');
                        value.push(c);
                    }
                }
                self.advance();
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Token::Invalid(format!("Unterminated string starting with {}", quote_char));
        }

        // Consume closing quote
        self.advance();
        Token::String(value)
    }

    fn number(&mut self) -> Token {
        let start = self.current - 1;

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Handle decimal point
        if self.peek() == '.' && self.peek_ahead(1).is_ascii_digit() {
            self.advance(); // consume '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_str: String = self.input[start..self.current].iter().collect();
        match number_str.parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => Token::Invalid(format!("Invalid number: {}", number_str)),
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.current - 1;

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.input[start..self.current].iter().collect();

        // Check for reserved keywords
        let reserved = match text.as_str() {
            "Null" | "null" => Some(Reserved::Null),
            "Void" | "void" => Some(Reserved::Void),
            "let" => Some(Reserved::Let),
            "fn" => Some(Reserved::Fn),
            "if" => Some(Reserved::If),
            "else" => Some(Reserved::Else),
            "while" => Some(Reserved::While),
            "for" => Some(Reserved::For),
            "continue" => Some(Reserved::Continue),
            "break" => Some(Reserved::Break),
            "Return" | "return" => Some(Reserved::Return),
            "public" => Some(Reserved::Public),
            "private" => Some(Reserved::Private),
            "static" => Some(Reserved::Static),
            "Print" | "print" => Some(Reserved::Print),
            "True" | "true" => Some(Reserved::True),
            "False" | "false" => Some(Reserved::False),
            "Type" | "type" => Some(Reserved::Type),
            "Alias" | "alias" => Some(Reserved::TypeAlias),
            "Def" | "def" => Some(Reserved::TypeDef),
            "Struct" | "struct" => Some(Reserved::Struct),
            "Enum" | "enum" => Some(Reserved::Enum),
            "Impl" | "impl" => Some(Reserved::Impl),
            "Trait" | "trait" => Some(Reserved::Trait),
            "Use" | "use" => Some(Reserved::Use),
            "Union" | "union" => Some(Reserved::Union),
            "Impl" | "impl" => Some(Reserved::Impl),
            "import" => Some(Reserved::Import),
            "export" => Some(Reserved::Export),
            "define" | "Define" => Some(Reserved::Define),

            _ => None,
        };

        match reserved {
            Some(keyword) => Token::Reserved(keyword),
            None => Token::Identifier(text),
        }
    }

    // Peek methods
    fn peek(&self) -> char {
        self.peek_ahead(0)
    }

    fn peek_ahead(&self, offset: usize) -> char {
        let pos = self.current + offset;
        if pos >= self.input.len() {
            '\0'
        } else {
            self.input[pos]
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let ch = self.input[self.current];
        self.current += 1;
        self.column += 1;
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
