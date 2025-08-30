/*
 * Copyright 2025 Mehmet T. AKALIN
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::PathBuf;
use std::fmt;
use crate::ast::{AstNode, Expression, Statement, Literal, BinaryOperator, UnaryOperator, AssignmentOperator};
use crate::error::{CompileError, parse_error};
use crate::types::Type;

/// PHP parser trait
pub trait Parser {
    /// Parse PHP source code into AST
    fn parse(&self, source: &str) -> Result<Vec<AstNode>, CompileError>;
    
    /// Parse PHP file into AST
    fn parse_file(&self, file_path: &PathBuf) -> Result<Vec<AstNode>, CompileError>;
}

/// Default PHP parser implementation
pub struct DefaultParser {
    /// Whether to use strict mode
    strict_mode: bool,
    
    /// Whether to parse attributes
    parse_attributes: bool,
    
    /// Whether to parse doc comments
    parse_doc_comments: bool,
}

impl DefaultParser {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            parse_attributes: true,
            parse_doc_comments: true,
        }
    }
    
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    pub fn with_attributes(mut self, parse: bool) -> Self {
        self.parse_attributes = parse;
        self
    }
    
    pub fn with_doc_comments(mut self, parse: bool) -> Self {
        self.parse_doc_comments = parse;
        self
    }
}

impl Parser for DefaultParser {
    fn parse(&self, source: &str) -> Result<Vec<AstNode>, CompileError> {
        // TODO: Implement actual PHP parsing
        // For now, return a simple placeholder
        Ok(vec![AstNode::Program(vec![AstNode::Expression(Box::new(
            Expression::Literal(Literal::String("Hello, World!".to_string()))
        ))])])
    }
    
    fn parse_file(&self, file_path: &PathBuf) -> Result<Vec<AstNode>, CompileError> {
        let source = std::fs::read_to_string(file_path)
            .map_err(|e| parse_error!(file_path, format!("Failed to read file: {}", e)))?;
        
        self.parse(&source)
    }
}

/// Token types for PHP parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    
    // Keywords
    Function,
    Class,
    Interface,
    Trait,
    Enum,
    Namespace,
    Use,
    If,
    Else,
    While,
    For,
    Foreach,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Return,
    Try,
    Catch,
    Finally,
    Throw,
    New,
    Clone,
    Instanceof,
    Public,
    Protected,
    Private,
    Static,
    Abstract,
    Final,
    Readonly,
    Const,
    Global,
    Echo,
    Print,
    Unset,
    Isset,
    Empty,
    Die,
    Exit,
    Declare,
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
    Yield,
    From,
    Match,
    Fn,
    Arrow,
    
    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Caret,          // ^
    Tilde,          // ~
    Exclamation,    // !
    Equal,          // =
    Less,           // <
    Greater,        // >
    Question,       // ?
    Colon,          // :
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .
    At,             // @
    Dollar,         // $
    Ampersand,      // &
    Pipe,           // |
    Backslash,      // \
    
    // Compound operators
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    CaretEqual,     // ^=
    DotEqual,       // .=
    EqualEqual,     // ==
    ExclamationEqual, // !=
    LessEqual,      // <=
    GreaterEqual,   // >=
    LessLess,       // <<
    GreaterGreater, // >>
    LessLessEqual,  // <<=
    GreaterGreaterEqual, // >>=
    EqualEqualEqual, // ===
    ExclamationEqualEqual, // !==
    LessGreater,    // <>
    LessEqualGreater, // <=>
    QuestionQuestion, // ??
    QuestionQuestionEqual, // ??=
    AmpersandAmpersand, // &&
    PipePipe,       // ||
    AmpersandEqual, // &=
    PipeEqual,      // |=
    CaretEqual,     // ^=
    
    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    
    // Special tokens
    Hash,           // #
    DoubleHash,     // ##
    DoubleSlash,    // //
    SlashStar,      // /*
    StarSlash,      // */
    HashHash,       // ##
    
    // Boolean and null
    Bool(bool),
    Null,
    
    // End of file
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "identifier '{}'", name),
            Token::Integer(n) => write!(f, "integer {}", n),
            Token::Float(x) => write!(f, "float {}", x),
            Token::String(s) => write!(f, "string '{}'", s),
            Token::Function => write!(f, "function"),
            Token::Class => write!(f, "class"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Equal => write!(f, "="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Lexer for PHP source code
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Get current character
    fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    
    /// Get next character
    fn next_char(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }
    
    /// Advance to next character
    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }
    
    /// Peek at next character without advancing
    fn peek(&self) -> Option<char> {
        self.next_char()
    }
    
    /// Check if we've reached the end
    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }
    
    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Skip comments
    fn skip_comments(&mut self) -> bool {
        if let Some(ch) = self.current_char() {
            match ch {
                '/' => {
                    if let Some(next) = self.peek() {
                        match next {
                            '/' => {
                                // Single line comment
                                while let Some(ch) = self.current_char() {
                                    if ch == '\n' {
                                        break;
                                    }
                                    self.advance();
                                }
                                true
                            }
                            '*' => {
                                // Multi-line comment
                                self.advance(); // consume *
                                while let Some(ch) = self.current_char() {
                                    if ch == '*' && self.peek() == Some('/') {
                                        self.advance(); // consume *
                                        self.advance(); // consume /
                                        break;
                                    }
                                    self.advance();
                                }
                                true
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                '#' => {
                    // Hash comment
                    while let Some(ch) = self.current_char() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// Read identifier or keyword
    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let start_pos = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword
        match identifier.as_str() {
            "function" => Token::Function,
            "class" => Token::Class,
            "interface" => Token::Interface,
            "trait" => Token::Trait,
            "enum" => Token::Enum,
            "namespace" => Token::Namespace,
            "use" => Token::Use,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "foreach" => Token::Foreach,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "return" => Token::Return,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "throw" => Token::Throw,
            "new" => Token::New,
            "clone" => Token::Clone,
            "instanceof" => Token::Instanceof,
            "public" => Token::Public,
            "protected" => Token::Protected,
            "private" => Token::Private,
            "static" => Token::Static,
            "abstract" => Token::Abstract,
            "final" => Token::Final,
            "readonly" => Token::Readonly,
            "const" => Token::Const,
            "global" => Token::Global,
            "echo" => Token::Echo,
            "print" => Token::Print,
            "unset" => Token::Unset,
            "isset" => Token::Isset,
            "empty" => Token::Empty,
            "die" => Token::Die,
            "exit" => Token::Exit,
            "declare" => Token::Declare,
            "include" => Token::Include,
            "include_once" => Token::IncludeOnce,
            "require" => Token::Require,
            "require_once" => Token::RequireOnce,
            "yield" => Token::Yield,
            "from" => Token::From,
            "match" => Token::Match,
            "fn" => Token::Fn,
            "true" | "false" => Token::Bool(identifier == "true"),
            "null" => Token::Null,
            _ => Token::Identifier(identifier),
        }
    }
    
    /// Read number literal
    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut is_float = false;
        
        while let Some(ch) = self.current_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                number.push(ch);
                is_float = true;
                self.advance();
            } else if ch == 'e' || ch == 'E' {
                number.push(ch);
                self.advance();
                if let Some(sign) = self.current_char() {
                    if sign == '+' || sign == '-' {
                        number.push(sign);
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        
        if is_float {
            number.parse::<f64>()
                .map(Token::Float)
                .unwrap_or(Token::Identifier(number))
        } else {
            number.parse::<i64>()
                .map(Token::Integer)
                .unwrap_or(Token::Identifier(number))
        }
    }
    
    /// Read string literal
    fn read_string(&mut self) -> Token {
        let quote = self.current_char().unwrap();
        self.advance(); // consume opening quote
        
        let mut string = String::new();
        let mut escaped = false;
        
        while let Some(ch) = self.current_char() {
            if escaped {
                match ch {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    '\'' => string.push('\''),
                    '$' => string.push('$'),
                    _ => string.push(ch),
                }
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                self.advance(); // consume closing quote
                break;
            } else {
                string.push(ch);
            }
            self.advance();
        }
        
        Token::String(string)
    }
    
    /// Get next token
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace and comments
        loop {
            self.skip_whitespace();
            if !self.skip_comments() {
                break;
            }
        }
        
        if self.is_eof() {
            return Token::Eof;
        }
        
        let ch = self.current_char().unwrap();
        
        match ch {
            ch if ch.is_alphabetic() || ch == '_' => {
                self.read_identifier()
            }
            ch if ch.is_digit(10) => {
                self.read_number()
            }
            '"' | '\'' => {
                self.read_string()
            }
            '+' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::StarEqual
                } else {
                    Token::Star
                }
            }
            '/' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            }
            '=' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        if let Some('=') = self.peek() {
                            self.advance();
                            Token::EqualEqualEqual
                        } else {
                            Token::EqualEqual
                        }
                    }
                    Some('>') => {
                        self.advance();
                        Token::Arrow
                    }
                    _ => Token::Equal,
                }
            }
            '<' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        if let Some('>') = self.peek() {
                            self.advance();
                            Token::LessEqualGreater
                        } else {
                            Token::LessEqual
                        }
                    }
                    Some('<') => {
                        self.advance();
                        if let Some('=') = self.peek() {
                            self.advance();
                            Token::LessLessEqual
                        } else {
                            Token::LessLess
                        }
                    }
                    Some('>') => {
                        self.advance();
                        Token::LessGreater
                    }
                    _ => Token::Less,
                }
            }
            '>' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::GreaterEqual
                    }
                    Some('>') => {
                        self.advance();
                        if let Some('=') = self.peek() {
                            self.advance();
                            Token::GreaterGreaterEqual
                        } else {
                            Token::GreaterGreater
                        }
                    }
                    _ => Token::Greater,
                }
            }
            '!' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::ExclamationEqualEqual
                    } else {
                        Token::ExclamationEqual
                    }
                } else {
                    Token::Exclamation
                }
            }
            '&' => {
                self.advance();
                match self.peek() {
                    Some('&') => {
                        self.advance();
                        Token::AmpersandAmpersand
                    }
                    Some('=') => {
                        self.advance();
                        Token::AmpersandEqual
                    }
                    _ => Token::Ampersand,
                }
            }
            '|' => {
                self.advance();
                match self.peek() {
                    Some('|') => {
                        self.advance();
                        Token::PipePipe
                    }
                    Some('=') => {
                        self.advance();
                        Token::PipeEqual
                    }
                    _ => Token::Pipe,
                }
            }
            '^' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::CaretEqual
                } else {
                    Token::Caret
                }
            }
            '?' => {
                self.advance();
                if let Some('?') = self.peek() {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::QuestionQuestionEqual
                    } else {
                        Token::QuestionQuestion
                    }
                } else {
                    Token::Question
                }
            }
            '.' => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::DotEqual
                } else {
                    Token::Dot
                }
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            '[' => {
                self.advance();
                Token::LeftBracket
            }
            ']' => {
                self.advance();
                Token::RightBracket
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            '@' => {
                self.advance();
                Token::At
            }
            '$' => {
                self.advance();
                Token::Dollar
            }
            '\\' => {
                self.advance();
                Token::Backslash
            }
            '#' => {
                self.advance();
                Token::Hash
            }
            _ => {
                // Unknown character
                let ch = self.current_char().unwrap();
                self.advance();
                Token::Identifier(ch.to_string())
            }
        }
    }
}

impl Default for DefaultParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let mut lexer = Lexer::new("function hello() { echo 'world'; }");
        
        assert_eq!(lexer.next_token(), Token::Function);
        assert_eq!(lexer.next_token(), Token::Identifier("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Echo);
        assert_eq!(lexer.next_token(), Token::String("world".to_string()));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_operators() {
        let mut lexer = Lexer::new("a + b * c");
        
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::Star);
        assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_numbers() {
        let mut lexer = Lexer::new("42 3.14");
        
        assert_eq!(lexer.next_token(), Token::Integer(42));
        assert_eq!(lexer.next_token(), Token::Float(3.14));
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
